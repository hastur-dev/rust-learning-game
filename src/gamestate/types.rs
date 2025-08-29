use crate::level::LevelSpec;
use crate::grid::Grid;
use crate::robot::Robot;
use crate::item::ItemManager;
use crate::menu::Menu;
use crate::popup::PopupSystem;
use rand::rngs::StdRng;

#[cfg(not(target_arch = "wasm32"))]
use crossbeam_channel::Receiver;
#[cfg(not(target_arch = "wasm32"))]
use notify::Event;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RustFunction {
    Move,
    Grab,
    Scan,
    LaserDirection,
    LaserTile,
    OpenDoor,
    SkipLevel,
    GotoLevel,
    Println,
    Eprintln, // Error messages
    Panic,    // Critical errors
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub function: RustFunction,
    pub direction: Option<(i32, i32)>, // for move, scan, and laser direction
    pub coordinates: Option<(i32, i32)>, // for laser tile targeting
    pub level_number: Option<usize>, // for goto_level
    pub boolean_param: Option<bool>, // for open_door
    pub message: Option<String>, // for println
}

#[derive(Clone, Debug)]
pub struct TutorialState {
    pub task_completed: [bool; 5], // Track completion of 5 tutorial tasks
    pub current_task: usize,       // Current active task (0-4)
    pub variables_used: Vec<String>, // Track variables created by user
    pub scan_output_stored: bool,  // Track if scan output was stored in variable
    pub u32_move_used: bool,      // Track if move with u32 was used
}

#[derive(Debug)]
pub struct Game {
    pub level_idx: usize,
    pub levels: Vec<LevelSpec>,
    pub grid: Grid,
    pub robot: Robot,
    pub item_manager: ItemManager,
    pub rng: StdRng,
    pub credits: u32,
    pub turns: usize,
    pub max_turns: usize,
    pub discovered_this_level: usize,
    pub finished: bool,
    pub scan_armed: bool,
    pub execution_result: String,
    pub code_editor_active: bool,
    pub selected_function_to_view: Option<RustFunction>,
    pub robot_code_path: String,
    #[cfg(not(target_arch = "wasm32"))]
    pub file_watcher_receiver: Option<Receiver<notify::Result<Event>>>,
    pub robot_code_modified: bool,
    pub current_code: String,
    pub cursor_position: usize,
    pub code_scroll_offset: usize, // Top line displayed in editor
    pub code_lines_visible: usize, // Number of lines visible in editor
    pub enemy_step_paused: bool,
    pub time_slow_active: bool,
    pub time_slow_duration_ms: u32,
    pub menu: Menu,
    pub popup_system: PopupSystem,
    pub stunned_enemies: std::collections::HashMap<usize, u8>, // enemy_index -> remaining_stun_turns
    pub temporary_removed_obstacles: std::collections::HashMap<(i32, i32), u8>, // position -> remaining_turns
    pub println_outputs: Vec<String>, // Track println outputs for completion conditions
    pub error_outputs: Vec<String>, // Track error/eprintln outputs for completion conditions
    pub panic_occurred: bool, // Track if panic occurred for completion conditions
    pub tutorial_state: TutorialState, // Tutorial system for progressive learning
    #[cfg(not(target_arch = "wasm32"))]
    pub rust_checker: Option<crate::rust_checker::RustChecker>, // Cargo integration for syntax checking
}