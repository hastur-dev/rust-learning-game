#[cfg(windows)]
use winapi::um::errhandlingapi::SetUnhandledExceptionFilter;
#[cfg(windows)]
use winapi::shared::ntdef::LONG;
#[cfg(windows)]
use winapi::um::winnt::PEXCEPTION_POINTERS;

// Windows exception constants
#[cfg(windows)]
const EXCEPTION_ACCESS_VIOLATION: u32 = 0xC0000005;
#[cfg(windows)]
const EXCEPTION_CONTINUE_EXECUTION: i32 = -1;
#[cfg(windows)]  
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;
use log::{error, info, warn};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// Global crash tracking
static CRASH_COUNT: AtomicUsize = AtomicUsize::new(0);
static SYSTEM_CRASH_ACTIVE: AtomicBool = AtomicBool::new(false);
static mut LAST_CRASH_ADDRESS: usize = 0;
static mut SAME_ADDRESS_COUNT: usize = 0;
static mut BLACKLISTED_ADDRESSES: Vec<usize> = Vec::new();
static PERMANENT_CRASH_PROTECTION: AtomicBool = AtomicBool::new(false);
static EMERGENCY_SHUTDOWN_MODE: AtomicBool = AtomicBool::new(false);

pub fn is_system_crash_active() -> bool {
    SYSTEM_CRASH_ACTIVE.load(Ordering::SeqCst)
}

pub fn reset_system_crash_state() {
    SYSTEM_CRASH_ACTIVE.store(false, Ordering::SeqCst);
}

pub fn is_permanent_protection_active() -> bool {
    PERMANENT_CRASH_PROTECTION.load(Ordering::SeqCst)
}

pub fn get_blacklisted_count() -> usize {
    unsafe { BLACKLISTED_ADDRESSES.len() }
}

pub fn is_emergency_shutdown_active() -> bool {
    EMERGENCY_SHUTDOWN_MODE.load(Ordering::SeqCst)
}

#[cfg(windows)]
unsafe extern "system" fn unhandled_exception_filter(
    exception_info: PEXCEPTION_POINTERS,
) -> LONG {
    if exception_info.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    let exception_record = (*exception_info).ExceptionRecord;
    if exception_record.is_null() {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    let exception_code = (*exception_record).ExceptionCode;
    let exception_address = (*exception_record).ExceptionAddress;
    
    // Handle specific exceptions we want to recover from
    match exception_code {
        EXCEPTION_ACCESS_VIOLATION => {
            let crash_count = CRASH_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
            let current_address = exception_address as usize;
            
            // Check if this is the same address as before
            let same_address_count = unsafe {
                if LAST_CRASH_ADDRESS == current_address {
                    SAME_ADDRESS_COUNT += 1;
                    SAME_ADDRESS_COUNT
                } else {
                    LAST_CRASH_ADDRESS = current_address;
                    SAME_ADDRESS_COUNT = 1;
                    1
                }
            };
            
            error!(
                "SYSTEM CRASH CAUGHT - Access violation at address {:p} (crash #{}, same address: {}x)!", 
                exception_address, crash_count, same_address_count
            );
            
            // Set system crash recovery flag
            SYSTEM_CRASH_ACTIVE.store(true, Ordering::SeqCst);
            
            // Be more aggressive about stopping repeated crashes at the same address
            if same_address_count >= 3 {
                error!("INFINITE CRASH LOOP DETECTED - ACTIVATING EMERGENCY SHUTDOWN");
                
                // Add address to blacklist and enable ALL protection modes
                unsafe {
                    BLACKLISTED_ADDRESSES.push(current_address);
                }
                PERMANENT_CRASH_PROTECTION.store(true, Ordering::SeqCst);
                EMERGENCY_SHUTDOWN_MODE.store(true, Ordering::SeqCst);
                
                error!("EMERGENCY: Disabling all risky operations permanently");
                
                // CRITICAL: Do NOT continue execution - let Windows handle it to break the loop
                return EXCEPTION_CONTINUE_SEARCH;
            } else if crash_count < 10 {
                warn!("Attempting to continue execution after access violation...");
                
                // Try to continue execution - this is risky but may work
                // for some types of access violations
                return EXCEPTION_CONTINUE_EXECUTION;
            } else {
                error!("Too many total crashes ({}), allowing normal crash handling", crash_count);
                return EXCEPTION_CONTINUE_SEARCH;
            }
        }
        _ => {
            // For other exceptions, let Windows handle them normally
            return EXCEPTION_CONTINUE_SEARCH;
        }
    }
}

pub fn setup_system_crash_protection() {
    #[cfg(windows)]
    unsafe {
        let previous_filter = SetUnhandledExceptionFilter(Some(unhandled_exception_filter));
        if previous_filter.is_some() {
            info!("Installed system-level crash protection (replaced existing handler)");
        } else {
            info!("Installed system-level crash protection");
        }
    }
    
    #[cfg(not(windows))]
    {
        info!("System-level crash protection not available on this platform");
    }
}

// Safe wrapper for potentially dangerous system operations
pub fn safe_system_operation<F, R>(operation: F, operation_name: &str, default_result: R) -> R
where 
    F: FnOnce() -> R,
    R: Clone,
{
    // In permanent protection mode, be extra cautious with risky operations
    if is_permanent_protection_active() {
        // Skip potentially dangerous operations entirely
        if operation_name.contains("key") || operation_name.contains("mouse") || operation_name.contains("screen") {
            warn!("Permanent protection: Skipping risky operation '{}'", operation_name);
            return default_result;
        }
    }
    
    if is_system_crash_active() {
        warn!("Skipping {} due to active system crash recovery", operation_name);
        return default_result;
    }

    // First try our normal panic-based protection
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(result) => result,
        Err(_) => {
            error!("System operation '{}' panicked, using default", operation_name);
            default_result
        }
    }
}

pub fn get_crash_count() -> usize {
    CRASH_COUNT.load(Ordering::SeqCst)
}

pub fn reset_crash_count() {
    CRASH_COUNT.store(0, Ordering::SeqCst);
    reset_system_crash_state();
    unsafe {
        LAST_CRASH_ADDRESS = 0;
        SAME_ADDRESS_COUNT = 0;
    }
}

pub fn get_crash_info() -> (usize, usize, usize) {
    let total_crashes = CRASH_COUNT.load(Ordering::SeqCst);
    unsafe {
        (total_crashes, LAST_CRASH_ADDRESS, SAME_ADDRESS_COUNT)
    }
}

// Safe wrappers for potentially crash-prone macroquad operations
use macroquad::prelude::{KeyCode, is_key_pressed as macroquad_is_key_pressed, is_key_down as macroquad_is_key_down};

pub fn safe_is_key_pressed(key: KeyCode) -> bool {
    safe_system_operation(|| macroquad_is_key_pressed(key), "is_key_pressed", false)
}

pub fn safe_is_key_down(key: KeyCode) -> bool {
    safe_system_operation(|| macroquad_is_key_down(key), "is_key_down", false)
}

pub fn safe_screen_width() -> f32 {
    safe_system_operation(|| macroquad::prelude::screen_width(), "screen_width", 800.0)
}

pub fn safe_screen_height() -> f32 {
    safe_system_operation(|| macroquad::prelude::screen_height(), "screen_height", 600.0)
}

pub fn safe_mouse_position() -> (f32, f32) {
    safe_system_operation(|| macroquad::prelude::mouse_position(), "mouse_position", (0.0, 0.0))
}