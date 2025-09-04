use log::{debug, error, warn};

#[derive(Debug, Clone, Copy)]
pub struct GlobalCoordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowCoordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct EditorCoordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct CoordinateTransformer {
    window_info: Option<WindowInfo>,
}

impl CoordinateTransformer {
    pub fn new() -> Self {
        Self {
            window_info: None,
        }
    }

    pub fn get_global_mouse_position(enable_logs: bool) -> Option<GlobalCoordinate> {
        #[cfg(windows)]
        {
            use winapi::um::winuser::{GetCursorPos};
            use winapi::shared::windef::POINT;
            use std::mem;

            unsafe {
                let mut point: POINT = mem::zeroed();
                if GetCursorPos(&mut point) != 0 {
                    if enable_logs {
                        debug!("Global mouse position: ({}, {})", point.x, point.y);
                    }
                    Some(GlobalCoordinate {
                        x: point.x as f32,
                        y: point.y as f32,
                    })
                } else {
                    error!("Failed to get global cursor position");
                    None
                }
            }
        }
        
        #[cfg(not(windows))]
        {
            warn!("Global mouse position not implemented for this platform");
            None
        }
    }

    pub fn get_window_position() -> Option<WindowInfo> {
        #[cfg(windows)]
        {
            use winapi::um::winuser::{FindWindowW, GetWindowRect, EnumWindows, GetWindowTextW, IsWindowVisible, GetClientRect, ClientToScreen, IsWindow};
            use winapi::shared::windef::{RECT, HWND};
            use winapi::shared::minwindef::{BOOL, TRUE, LPARAM};
            use std::mem;
            use std::ptr;
            use std::sync::{Arc, Mutex};

            unsafe {
                // First try the foreground window (most likely to be our game)
                let hwnd = winapi::um::winuser::GetForegroundWindow();
                if !hwnd.is_null() {
                    // Verify the window is still valid before making API calls
                    if IsWindow(hwnd) == 0 {
                        debug!("Foreground window handle is no longer valid, skipping");
                    } else {
                        // Get the client area coordinates instead of window rect
                        let mut client_rect: RECT = mem::zeroed();
                        if winapi::um::winuser::GetClientRect(hwnd, &mut client_rect) != 0 {
                            // Convert client area to screen coordinates
                            let mut top_left = winapi::shared::windef::POINT { x: 0, y: 0 };
                            if winapi::um::winuser::ClientToScreen(hwnd, &mut top_left) != 0 {
                                let window_info = WindowInfo {
                                    x: top_left.x,
                                    y: top_left.y,
                                    width: client_rect.right - client_rect.left,
                                    height: client_rect.bottom - client_rect.top,
                                };
                                
                                // Validate that this looks like a reasonable game window
                                if Self::is_valid_game_window(&window_info, false) {
                                    debug!("Found valid foreground window (client area): {:?}", window_info);
                                    return Some(window_info);
                                } else {
                                    debug!("Foreground window client area invalid: {:?}", window_info);
                                }
                            } else {
                                debug!("ClientToScreen failed, falling back to GetWindowRect");
                                // Fallback to window rect if client area conversion fails
                                let mut rect: RECT = mem::zeroed();
                                if GetWindowRect(hwnd, &mut rect) != 0 {
                                    let window_info = WindowInfo {
                                        x: rect.left,
                                        y: rect.top,
                                        width: rect.right - rect.left,
                                        height: rect.bottom - rect.top,
                                    };
                                    if Self::is_valid_game_window(&window_info, false) {
                                        debug!("Found valid foreground window (window rect fallback): {:?}", window_info);
                                        return Some(window_info);
                                    }
                                }
                            }
                        } else {
                            debug!("GetClientRect failed, falling back to GetWindowRect");
                            // Fallback to window rect if client rect fails
                            let mut rect: RECT = mem::zeroed();
                            if GetWindowRect(hwnd, &mut rect) != 0 {
                                let window_info = WindowInfo {
                                    x: rect.left,
                                    y: rect.top,
                                    width: rect.right - rect.left,
                                    height: rect.bottom - rect.top,
                                };
                                if Self::is_valid_game_window(&window_info, false) {
                                    debug!("Found valid foreground window (window rect fallback): {:?}", window_info);
                                    return Some(window_info);
                                }
                            }
                        }
                    }
                }

                // If foreground window isn't suitable, try finding by class name
                let window_classes = [
                    "macroquad",
                    "GLFW30", 
                    "SDL_app",
                ];

                for &class_name in &window_classes {
                    let class_name_wide: Vec<u16> = class_name.encode_utf16().chain(std::iter::once(0)).collect();
                    let hwnd = FindWindowW(class_name_wide.as_ptr(), ptr::null());
                    
                    if !hwnd.is_null() && IsWindow(hwnd) != 0 && IsWindowVisible(hwnd) == TRUE {
                        // Use client area for class-based window detection too
                        let mut client_rect: RECT = mem::zeroed();
                        if GetClientRect(hwnd, &mut client_rect) != 0 {
                            let mut top_left = winapi::shared::windef::POINT { x: 0, y: 0 };
                            if ClientToScreen(hwnd, &mut top_left) != 0 {
                                let window_info = WindowInfo {
                                    x: top_left.x,
                                    y: top_left.y,
                                    width: client_rect.right - client_rect.left,
                                    height: client_rect.bottom - client_rect.top,
                                };
                                
                                if Self::is_valid_game_window(&window_info, false) {
                                    debug!("Found valid window with class '{}' (client area): {:?}", class_name, window_info);
                                    return Some(window_info);
                                } else {
                                    debug!("Window with class '{}' client area invalid: {:?}", class_name, window_info);
                                }
                            } else {
                                // Fallback to window rect if client area fails
                                let mut rect: RECT = mem::zeroed();
                                if GetWindowRect(hwnd, &mut rect) != 0 {
                                    let window_info = WindowInfo {
                                        x: rect.left,
                                        y: rect.top,
                                        width: rect.right - rect.left,
                                        height: rect.bottom - rect.top,
                                    };
                                    if Self::is_valid_game_window(&window_info, false) {
                                        debug!("Found valid window with class '{}' (window rect fallback): {:?}", class_name, window_info);
                                        return Some(window_info);
                                    }
                                }
                            }
                        } else {
                            // Fallback to window rect if client rect fails
                            let mut rect: RECT = mem::zeroed();
                            if GetWindowRect(hwnd, &mut rect) != 0 {
                                let window_info = WindowInfo {
                                    x: rect.left,
                                    y: rect.top,
                                    width: rect.right - rect.left,
                                    height: rect.bottom - rect.top,
                                };
                                if Self::is_valid_game_window(&window_info, false) {
                                    debug!("Found valid window with class '{}' (window rect fallback): {:?}", class_name, window_info);
                                    return Some(window_info);
                                }
                            }
                        }
                    }
                }

                // Last resort: enumerate all windows and find the largest visible one that looks like a game
                let best_window: Arc<Mutex<Option<WindowInfo>>> = Arc::new(Mutex::new(None));
                let best_window_clone = best_window.clone();
                
                unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
                    let best_window = &*(lparam as *const Arc<Mutex<Option<WindowInfo>>>);
                    
                    if IsWindowVisible(hwnd) == TRUE {
                        let mut rect: RECT = mem::zeroed();
                        if GetWindowRect(hwnd, &mut rect) != 0 {
                            let window_info = WindowInfo {
                                x: rect.left,
                                y: rect.top,
                                width: rect.right - rect.left,
                                height: rect.bottom - rect.top,
                            };
                            
                            // Check if this looks like a reasonable game window
                            if CoordinateTransformer::is_valid_game_window(&window_info, false) {
                                let mut best = best_window.lock().unwrap();
                                let should_replace = match *best {
                                    None => true,
                                    Some(ref current) => {
                                        // Prefer larger windows (more likely to be the game)
                                        let new_area = window_info.width * window_info.height;
                                        let current_area = current.width * current.height;
                                        new_area > current_area
                                    }
                                };
                                
                                if should_replace {
                                    *best = Some(window_info);
                                }
                            }
                        }
                    }
                    TRUE
                }

                EnumWindows(Some(enum_proc), &*best_window_clone as *const _ as LPARAM);
                
                let result = best_window.lock().unwrap().clone();
                if let Some(window_info) = result {
                    debug!("Found best enumerated window: {:?}", window_info);
                    return Some(window_info);
                }

                // Debug: List all visible windows for troubleshooting
                debug!("Failed to find game window. Listing all visible windows for debugging:");
                
                unsafe extern "system" fn debug_enum_proc(hwnd: HWND, _lparam: LPARAM) -> BOOL {
                    if IsWindowVisible(hwnd) == TRUE {
                        let mut rect: RECT = mem::zeroed();
                        if GetWindowRect(hwnd, &mut rect) != 0 {
                            let window_info = WindowInfo {
                                x: rect.left,
                                y: rect.top,
                                width: rect.right - rect.left,
                                height: rect.bottom - rect.top,
                            };
                            
                            // Get window title for debugging
                            let mut buffer = [0u16; 256];
                            let title_len = winapi::um::winuser::GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
                            let title = if title_len > 0 {
                                String::from_utf16_lossy(&buffer[..title_len as usize])
                            } else {
                                "No Title".to_string()
                            };
                            
                            debug!("  Window: '{}' - {:?}", title, window_info);
                        }
                    }
                    TRUE
                }
                
                EnumWindows(Some(debug_enum_proc), 0);
                
                error!("Failed to get window position");
                None
            }
        }

        #[cfg(not(windows))]
        {
            warn!("Window position not implemented for this platform");
            None
        }
    }

    pub fn update_window_info(&mut self) {
        self.window_info = Self::get_window_position();
    }

    fn is_valid_game_window(window_info: &WindowInfo, enable_logs: bool) -> bool {
        // Filter out obviously invalid windows
        // Minimum reasonable game window size
        let min_width = 800;
        let min_height = 600;
        
        // Check for reasonable coordinates (not minimized/hidden)
        let reasonable_coords = window_info.x >= -100 && window_info.x <= 10000 && 
                               window_info.y >= -100 && window_info.y <= 10000;
        
        // Check size
        let reasonable_size = window_info.width >= min_width && 
                             window_info.height >= min_height &&
                             window_info.width <= 4000 && 
                             window_info.height <= 4000;
        
        let valid = reasonable_coords && reasonable_size;
        
        if !valid && enable_logs {
            debug!("Window validation failed: coords_ok={}, size_ok={}, window={:?}", 
                   reasonable_coords, reasonable_size, window_info);
        }
        
        valid
    }

    pub fn global_to_window(&self, global: GlobalCoordinate, enable_logs: bool) -> Option<WindowCoordinate> {
        if let Some(window_info) = self.window_info {
            let window_coord = WindowCoordinate {
                x: global.x - window_info.x as f32,
                y: global.y - window_info.y as f32,
            };
            if enable_logs {
                debug!("Converted global ({}, {}) to window ({}, {})", 
                       global.x, global.y, window_coord.x, window_coord.y);
            }
            Some(window_coord)
        } else {
            if enable_logs {
                error!("Window info not available for coordinate transformation");
            }
            None
        }
    }

    pub fn window_to_editor(&self, window: WindowCoordinate, editor_bounds: (f32, f32, f32, f32), enable_logs: bool) -> EditorCoordinate {
        let (editor_x, editor_y, _editor_width, _editor_height) = editor_bounds;
        
        let editor_coord = EditorCoordinate {
            x: window.x - editor_x,
            y: window.y - editor_y,
        };
        
        if enable_logs {
            debug!("Converted window ({}, {}) to editor ({}, {}) with bounds {:?}", 
                   window.x, window.y, editor_coord.x, editor_coord.y, editor_bounds);
        }
        
        editor_coord
    }

    pub fn get_precise_mouse_position_in_editor(&mut self, editor_bounds: (f32, f32, f32, f32), enable_logs: bool) -> Option<EditorCoordinate> {
        // Add safety check to prevent crashes from rapid calls
        use std::panic;
        
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            // Get global mouse position
            Self::get_global_mouse_position(enable_logs)
        }));
        
        let global_pos = match result {
            Ok(Some(pos)) => pos,
            Ok(None) => {
                if enable_logs {
                    debug!("Failed to get global mouse position");
                }
                return None;
            }
            Err(_) => {
                error!("Panic caught in get_global_mouse_position - falling back to None");
                return None;
            }
        };
        
        // Note: Window info is updated by the throttled update_window_coordinates method
        // This ensures we don't hammer the Windows API on every mouse click
        
        // Validate we have window info
        if self.window_info.is_none() {
            if enable_logs {
                error!("No valid window info available for coordinate transformation");
            }
            return None;
        }
        
        // Convert global to window coordinates
        let window_pos = self.global_to_window(global_pos, enable_logs)?;
        
        // Validate that the window coordinates are reasonable
        let (editor_x, editor_y, editor_width, editor_height) = editor_bounds;
        
        // Check if the window coordinates are within a reasonable range of the editor bounds
        if window_pos.x < -100.0 || window_pos.y < -100.0 || 
           window_pos.x > editor_x + editor_width + 100.0 || 
           window_pos.y > editor_y + editor_height + 100.0 {
            if enable_logs {
                debug!("Window coordinates seem invalid: ({:.2}, {:.2}) for editor bounds {:?}", 
                       window_pos.x, window_pos.y, editor_bounds);
            }
            return None;
        }
        
        // Convert window to editor coordinates
        let editor_pos = self.window_to_editor(window_pos, editor_bounds, enable_logs);
        
        if enable_logs {
            debug!("Final editor coordinates: ({:.2}, {:.2})", editor_pos.x, editor_pos.y);
        }
        Some(editor_pos)
    }

    pub fn get_window_info(&self) -> Option<WindowInfo> {
        self.window_info
    }
}

impl Default for CoordinateTransformer {
    fn default() -> Self {
        Self::new()
    }
}