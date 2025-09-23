// Learning Tests for Level 18, Task 5: FFI and Interoperability
// C interop, extern functions, and cross-language data structures

use std::ffi::{CStr, CString, c_void};
use std::os::raw::{c_char, c_int, c_uint, c_double, c_float};
use std::ptr;
use std::slice;
use std::mem;

// C-compatible robot structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CRobot {
    pub id: c_uint,
    pub x: c_float,
    pub y: c_float,
    pub battery: c_double,
    pub status: c_int,
    pub name: *const c_char,
}

// Ensure C compatibility
impl CRobot {
    pub fn new(id: u32, x: f32, y: f32, battery: f64, status: i32, name: &str) -> Self {
        let c_name = CString::new(name).expect("CString conversion failed");
        let name_ptr = c_name.into_raw();

        CRobot {
            id: id as c_uint,
            x: x as c_float,
            y: y as c_float,
            battery: battery as c_double,
            status: status as c_int,
            name: name_ptr,
        }
    }

    pub unsafe fn get_name(&self) -> String {
        if self.name.is_null() {
            return String::new();
        }

        CStr::from_ptr(self.name)
            .to_string_lossy()
            .into_owned()
    }

    pub unsafe fn free_name(&mut self) {
        if !self.name.is_null() {
            let _ = CString::from_raw(self.name as *mut c_char);
            self.name = ptr::null();
        }
    }
}

// Callbacks and function pointers
pub type RobotCallback = unsafe extern "C" fn(*const CRobot) -> c_int;
pub type DataProcessor = unsafe extern "C" fn(*const c_void, usize) -> c_int;

// Simulated external C functions
#[no_mangle]
pub extern "C" fn robot_init(robot: *mut CRobot, id: c_uint) -> c_int {
    if robot.is_null() {
        return -1;
    }

    unsafe {
        (*robot).id = id;
        (*robot).x = 0.0;
        (*robot).y = 0.0;
        (*robot).battery = 100.0;
        (*robot).status = 1;
        (*robot).name = ptr::null();
    }

    0
}

#[no_mangle]
pub extern "C" fn robot_move(robot: *mut CRobot, dx: c_float, dy: c_float) -> c_int {
    if robot.is_null() {
        return -1;
    }

    unsafe {
        (*robot).x += dx;
        (*robot).y += dy;
        (*robot).battery -= 0.1;
    }

    0
}

#[no_mangle]
pub extern "C" fn robot_get_distance(robot: *const CRobot) -> c_double {
    if robot.is_null() {
        return -1.0;
    }

    unsafe {
        let x = (*robot).x as f64;
        let y = (*robot).y as f64;
        (x * x + y * y).sqrt()
    }
}

// Array handling
#[no_mangle]
pub extern "C" fn process_robot_array(
    robots: *const CRobot,
    count: usize,
    callback: Option<RobotCallback>
) -> c_int {
    if robots.is_null() || count == 0 {
        return -1;
    }

    let robot_slice = unsafe { slice::from_raw_parts(robots, count) };

    let mut result = 0;
    for robot in robot_slice {
        if let Some(cb) = callback {
            result += unsafe { cb(robot) };
        }
    }

    result
}

// Opaque pointer handling
#[repr(C)]
pub struct OpaqueRobotController {
    _private: [u8; 0],
}

pub struct RobotController {
    robots: Vec<CRobot>,
    active: bool,
}

#[no_mangle]
pub extern "C" fn controller_create() -> *mut OpaqueRobotController {
    let controller = Box::new(RobotController {
        robots: Vec::new(),
        active: true,
    });

    Box::into_raw(controller) as *mut OpaqueRobotController
}

#[no_mangle]
pub extern "C" fn controller_destroy(controller: *mut OpaqueRobotController) {
    if !controller.is_null() {
        unsafe {
            let _ = Box::from_raw(controller as *mut RobotController);
        }
    }
}

#[no_mangle]
pub extern "C" fn controller_add_robot(
    controller: *mut OpaqueRobotController,
    robot: *const CRobot
) -> c_int {
    if controller.is_null() || robot.is_null() {
        return -1;
    }

    let controller = unsafe { &mut *(controller as *mut RobotController) };
    let robot_copy = unsafe { *robot };

    controller.robots.push(robot_copy);
    0
}

#[no_mangle]
pub extern "C" fn controller_get_robot_count(controller: *const OpaqueRobotController) -> usize {
    if controller.is_null() {
        return 0;
    }

    let controller = unsafe { &*(controller as *const RobotController) };
    controller.robots.len()
}

// Error handling with errno-style codes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RobotError {
    Success = 0,
    InvalidInput = -1,
    OutOfMemory = -2,
    InvalidState = -3,
    NetworkError = -4,
}

static mut LAST_ERROR: RobotError = RobotError::Success;

#[no_mangle]
pub extern "C" fn get_last_error() -> c_int {
    unsafe { LAST_ERROR as c_int }
}

#[no_mangle]
pub extern "C" fn set_error(error: c_int) {
    unsafe {
        LAST_ERROR = match error {
            -1 => RobotError::InvalidInput,
            -2 => RobotError::OutOfMemory,
            -3 => RobotError::InvalidState,
            -4 => RobotError::NetworkError,
            _ => RobotError::Success,
        };
    }
}

// String handling
#[no_mangle]
pub extern "C" fn create_robot_name(prefix: *const c_char, id: c_uint) -> *mut c_char {
    if prefix.is_null() {
        set_error(RobotError::InvalidInput as c_int);
        return ptr::null_mut();
    }

    let prefix_str = unsafe {
        CStr::from_ptr(prefix).to_str().unwrap_or("Robot")
    };

    let name = format!("{}_{}", prefix_str, id);
    match CString::new(name) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => {
            set_error(RobotError::InvalidInput as c_int);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn free_robot_name(name: *mut c_char) {
    if !name.is_null() {
        unsafe {
            let _ = CString::from_raw(name);
        }
    }
}

// Buffer manipulation
#[no_mangle]
pub extern "C" fn encode_robot_data(
    robot: *const CRobot,
    buffer: *mut u8,
    buffer_size: usize
) -> c_int {
    if robot.is_null() || buffer.is_null() {
        return -1;
    }

    let required_size = mem::size_of::<CRobot>();
    if buffer_size < required_size {
        return -2;
    }

    unsafe {
        let robot_bytes = slice::from_raw_parts(
            robot as *const u8,
            required_size
        );

        let buffer_slice = slice::from_raw_parts_mut(buffer, buffer_size);
        buffer_slice[..required_size].copy_from_slice(robot_bytes);
    }

    required_size as c_int
}

#[no_mangle]
pub extern "C" fn decode_robot_data(
    buffer: *const u8,
    buffer_size: usize,
    robot: *mut CRobot
) -> c_int {
    if buffer.is_null() || robot.is_null() {
        return -1;
    }

    let required_size = mem::size_of::<CRobot>();
    if buffer_size < required_size {
        return -2;
    }

    unsafe {
        ptr::copy_nonoverlapping(buffer, robot as *mut u8, required_size);
    }

    0
}

// Variadic functions simulation
#[repr(C)]
pub struct VarArgs {
    args: Vec<c_int>,
}

#[no_mangle]
pub extern "C" fn varargs_create() -> *mut VarArgs {
    Box::into_raw(Box::new(VarArgs { args: Vec::new() }))
}

#[no_mangle]
pub extern "C" fn varargs_add(va: *mut VarArgs, value: c_int) {
    if !va.is_null() {
        unsafe {
            (*va).args.push(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn varargs_sum(va: *const VarArgs) -> c_int {
    if va.is_null() {
        return 0;
    }

    unsafe { (*va).args.iter().sum() }
}

#[no_mangle]
pub extern "C" fn varargs_destroy(va: *mut VarArgs) {
    if !va.is_null() {
        unsafe {
            let _ = Box::from_raw(va);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_robot_creation() {
        let mut robot = CRobot::new(1, 10.0, 20.0, 95.5, 1, "TestBot");

        unsafe {
            assert_eq!(robot.get_name(), "TestBot");
            robot.free_name();
        }

        assert_eq!(robot.id, 1);
        assert_eq!(robot.x, 10.0);
        assert_eq!(robot.y, 20.0);
    }

    #[test]
    fn test_robot_functions() {
        let mut robot: CRobot = unsafe { mem::zeroed() };
        let result = robot_init(&mut robot, 42);

        assert_eq!(result, 0);
        assert_eq!(robot.id, 42);
        assert_eq!(robot.battery, 100.0);

        robot_move(&mut robot, 3.0, 4.0);
        let distance = robot_get_distance(&robot);

        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_controller() {
        let controller = controller_create();
        assert!(!controller.is_null());

        let robot = CRobot::new(1, 0.0, 0.0, 100.0, 1, "Bot1");
        let result = controller_add_robot(controller, &robot);
        assert_eq!(result, 0);

        let count = controller_get_robot_count(controller);
        assert_eq!(count, 1);

        controller_destroy(controller);
    }

    #[test]
    fn test_string_handling() {
        let prefix = CString::new("Robot").unwrap();
        let name = create_robot_name(prefix.as_ptr(), 123);

        assert!(!name.is_null());

        unsafe {
            let name_str = CStr::from_ptr(name).to_str().unwrap();
            assert_eq!(name_str, "Robot_123");
        }

        free_robot_name(name);
    }

    #[test]
    fn test_buffer_encoding() {
        let robot = CRobot::new(1, 10.0, 20.0, 100.0, 1, "TestBot");
        let mut buffer = vec![0u8; 256];

        let encoded_size = encode_robot_data(&robot, buffer.as_mut_ptr(), buffer.len());
        assert!(encoded_size > 0);

        let mut decoded_robot: CRobot = unsafe { mem::zeroed() };
        let result = decode_robot_data(buffer.as_ptr(), buffer.len(), &mut decoded_robot);

        assert_eq!(result, 0);
        assert_eq!(decoded_robot.id, robot.id);
        assert_eq!(decoded_robot.x, robot.x);
        assert_eq!(decoded_robot.y, robot.y);
    }

    #[test]
    fn test_varargs() {
        let va = varargs_create();
        assert!(!va.is_null());

        varargs_add(va, 10);
        varargs_add(va, 20);
        varargs_add(va, 30);

        let sum = varargs_sum(va);
        assert_eq!(sum, 60);

        varargs_destroy(va);
    }
}

// Student exercises
pub mod exercises {
    use super::*;

    // Exercise 1: Create safe wrapper for C library
    pub struct SafeRobotWrapper {
        inner: CRobot,
    }

    impl SafeRobotWrapper {
        pub fn new(id: u32, name: &str) -> Self {
            // TODO: Create safe wrapper
            unimplemented!("Create safe wrapper")
        }

        pub fn move_robot(&mut self, dx: f32, dy: f32) {
            // TODO: Safely call robot_move
            unimplemented!("Safe move")
        }

        pub fn get_distance(&self) -> f64 {
            // TODO: Safely call robot_get_distance
            unimplemented!("Safe distance")
        }
    }

    impl Drop for SafeRobotWrapper {
        fn drop(&mut self) {
            // TODO: Clean up resources
            unimplemented!("Cleanup")
        }
    }

    // Exercise 2: Implement callback registration
    pub struct CallbackRegistry {
        callbacks: Vec<Box<dyn Fn(&CRobot) -> i32>>,
    }

    impl CallbackRegistry {
        pub fn new() -> Self {
            // TODO: Initialize registry
            unimplemented!("Initialize callback registry")
        }

        pub fn register<F>(&mut self, callback: F) -> usize
        where
            F: Fn(&CRobot) -> i32 + 'static
        {
            // TODO: Register Rust closure as C callback
            unimplemented!("Register callback")
        }

        pub unsafe extern "C" fn c_callback_wrapper(robot: *const CRobot) -> c_int {
            // TODO: Bridge C callback to Rust closure
            unimplemented!("Callback wrapper")
        }
    }

    // Exercise 3: Create bindgen-style struct mapping
    #[repr(C)]
    pub struct BindgenRobot {
        // TODO: Define C-compatible struct with bitfields
        // Use proper repr and alignment
    }

    impl BindgenRobot {
        pub fn from_c_struct(c_robot: &CRobot) -> Self {
            // TODO: Convert from C struct
            unimplemented!("Convert from C")
        }

        pub fn to_c_struct(&self) -> CRobot {
            // TODO: Convert to C struct
            unimplemented!("Convert to C")
        }
    }
}