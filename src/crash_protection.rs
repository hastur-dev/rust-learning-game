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

pub fn safe_is_mouse_button_pressed(button: macroquad::prelude::MouseButton) -> bool {
    safe_system_operation(|| macroquad::prelude::is_mouse_button_pressed(button), "is_mouse_button_pressed", false)
}

pub fn safe_is_mouse_button_down(button: macroquad::prelude::MouseButton) -> bool {
    safe_system_operation(|| macroquad::prelude::is_mouse_button_down(button), "is_mouse_button_down", false)
}

pub fn safe_is_mouse_button_released(button: macroquad::prelude::MouseButton) -> bool {
    safe_system_operation(|| macroquad::prelude::is_mouse_button_released(button), "is_mouse_button_released", false)
}

pub fn safe_mouse_wheel() -> (f32, f32) {
    safe_system_operation(|| macroquad::prelude::mouse_wheel(), "mouse_wheel", (0.0, 0.0))
}

pub fn safe_get_time() -> f64 {
    safe_system_operation(|| macroquad::prelude::get_time(), "get_time", 0.0)
}

pub fn safe_get_frame_time() -> f32 {
    safe_system_operation(|| macroquad::prelude::get_frame_time(), "get_frame_time", 0.016)
}

// Window focus management
static WINDOW_FOCUSED: AtomicBool = AtomicBool::new(true);
static LAST_FOCUS_CHECK: AtomicUsize = AtomicUsize::new(0);

pub fn is_window_focused() -> bool {
    WINDOW_FOCUSED.load(Ordering::SeqCst)
}

pub fn update_window_focus_state() {
    // Update window focus state using existing coordinate system
    let is_focused = crate::coordinate_system::CoordinateTransformer::is_game_window_active(false);
    let was_focused = WINDOW_FOCUSED.load(Ordering::SeqCst);

    // Update focus state
    WINDOW_FOCUSED.store(is_focused, Ordering::SeqCst);

    // If focus changed, log it
    if was_focused != is_focused {
        if is_focused {
            info!("Window gained focus - resuming input handling");
        } else {
            warn!("Window lost focus - disabling input handling to prevent crashes");
            // Try to release any cursor constraints when focus is lost
            release_cursor_constraints();
        }
    }
}

fn release_cursor_constraints() {
    // Attempt to release cursor grab/constraints when window loses focus
    safe_system_operation(|| {
        #[cfg(windows)]
        {
            use winapi::um::winuser::{ClipCursor, SetCursor, LoadCursorW, IDC_ARROW};
            use winapi::um::libloaderapi::GetModuleHandleW;
            use std::ptr;

            unsafe {
                // Release cursor clipping/confinement
                ClipCursor(ptr::null());

                // Set cursor to default arrow
                let h_instance = GetModuleHandleW(ptr::null());
                let cursor = LoadCursorW(h_instance, IDC_ARROW);
                SetCursor(cursor);

                info!("Released cursor constraints and set to default cursor");
            }
        }

        #[cfg(not(windows))]
        {
            info!("Cursor constraint release not implemented for this platform");
        }
    }, "release_cursor_constraints", ());
}

// Enhanced safe wrappers that check window focus
pub fn safe_mouse_position_with_focus() -> (f32, f32) {
    if !is_window_focused() {
        return (0.0, 0.0);
    }
    safe_mouse_position()
}

pub fn safe_is_key_pressed_with_focus(key: KeyCode) -> bool {
    if !is_window_focused() {
        return false;
    }
    safe_is_key_pressed(key)
}

pub fn safe_is_key_down_with_focus(key: KeyCode) -> bool {
    if !is_window_focused() {
        return false;
    }
    safe_is_key_down(key)
}

pub fn safe_is_mouse_button_pressed_with_focus(button: macroquad::prelude::MouseButton) -> bool {
    if !is_window_focused() {
        return false;
    }
    safe_is_mouse_button_pressed(button)
}

pub fn safe_is_mouse_button_down_with_focus(button: macroquad::prelude::MouseButton) -> bool {
    if !is_window_focused() {
        return false;
    }
    safe_is_mouse_button_down(button)
}

// Public function to manually release cursor constraints
pub fn force_release_cursor() {
    release_cursor_constraints();
}

// Enhanced update function that always releases cursor when unfocused
pub fn update_window_focus_state_with_cursor_release() {
    // Update window focus state using existing coordinate system
    let is_focused = crate::coordinate_system::CoordinateTransformer::is_game_window_active(false);
    let was_focused = WINDOW_FOCUSED.load(Ordering::SeqCst);

    // Update focus state
    WINDOW_FOCUSED.store(is_focused, Ordering::SeqCst);

    // ALWAYS release cursor when not focused
    if !is_focused {
        // Force release cursor constraints every time window is not focused
        release_cursor_constraints();

        if was_focused {
            warn!("Window lost focus - releasing cursor and disabling input handling");
        }
    } else if !was_focused {
        info!("Window gained focus - resuming input handling");
    }
}

// Safe clipboard operations that prevent crashes when window loses focus
pub fn safe_clipboard_copy(text: &str) -> bool {
    if !is_window_focused() {
        warn!("Skipping clipboard copy - window not focused");
        return false;
    }

    safe_system_operation(|| {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            clipboard.set_text(text).is_ok()
        } else {
            false
        }
    }, "clipboard_copy", false)
}

pub fn safe_clipboard_paste() -> Option<String> {
    if !is_window_focused() {
        warn!("Skipping clipboard paste - window not focused");
        return None;
    }

    safe_system_operation(|| {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            if let Ok(text) = clipboard.get_text() {
                if !text.is_empty() {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }, "clipboard_paste", None)
}

// Critical macroquad rendering pipeline safety wrappers
pub fn safe_clear_background(color: macroquad::prelude::Color) {
    if !is_window_focused() {
        // Completely skip all background clearing when unfocused
        // This prevents any graphics operations on invalid context
        return;
    }

    safe_system_operation(|| {
        macroquad::prelude::clear_background(color);
    }, "clear_background", ());
}

pub async fn safe_next_frame() {
    // When window is not focused, completely skip all rendering
    // This leaves the last drawn frame visible and prevents any graphics context issues
    while !is_window_focused() {
        warn!("Window not focused - pausing all rendering, keeping last frame visible");

        // Use standard async timer to prevent blocking and allow focus state updates
        use std::time::Duration;

        // Create a future that completes after 16ms (~60fps)
        let delay = async {
            let start = std::time::Instant::now();
            while start.elapsed() < Duration::from_millis(16) {
                // Small yield to prevent busy waiting
                std::thread::yield_now();
            }
        };
        delay.await;

        // Continue checking focus state in the loop
        // This ensures we resume immediately when focus returns
    }

    // Only call next_frame when window is focused and graphics context should be valid
    info!("Window focused - resuming rendering");
    macroquad::prelude::next_frame().await;
}

// Safe wrapper for generic draw operations that may involve Game state
pub fn safe_draw_operation_with_focus<F>(operation: F, operation_name: &str)
where
    F: FnOnce(),
{
    if !is_window_focused() {
        // Completely skip ALL drawing operations when unfocused
        // This ensures the last rendered frame remains visible without any graphics context access
        return;
    }

    // Use AssertUnwindSafe to handle non-UnwindSafe types like &mut Game
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)) {
        Ok(_) => {
            // Operation completed successfully
        }
        Err(_) => {
            error!("Draw operation '{}' panicked, likely due to invalid graphics context", operation_name);
            // Mark system crash state to prevent further crashes
            SYSTEM_CRASH_ACTIVE.store(true, Ordering::SeqCst);
        }
    }
}