// Level 7 Task 5 Test: Complete Robot Control System
// Tests if the user code creates a master robot controller combining all subsystems

#[cfg(test)]
mod level7_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_master_robot_controller() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("MasterRobotController") ||
            analyzer.code.contains("RobotController") ||
            analyzer.code.contains("struct Robot"),
            "❌ Your code should define a master robot controller struct"
        );
    }

    #[test]
    fn test_has_objective_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Objective") ||
            analyzer.code.contains("objective") ||
            analyzer.code.contains("mission"),
            "❌ Your code should define an Objective struct or mission system"
        );
    }

    #[test]
    fn test_has_objective_type_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum ObjectiveType") ||
            analyzer.code.contains("CollectItem") ||
            analyzer.code.contains("ReachGoal"),
            "❌ Your code should define an ObjectiveType enum with variants like CollectItem, ReachGoal"
        );
    }

    #[test]
    fn test_controller_has_subsystems() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot = analyzer.code.contains("robot:") || analyzer.code.contains("robot ");
        let has_map = analyzer.code.contains("game_map") || analyzer.code.contains("map");
        let has_objectives = analyzer.code.contains("objectives") || analyzer.code.contains("mission");
        assert!(
            has_robot && (has_map || has_objectives),
            "❌ Your controller should contain robot, map, and objectives subsystems"
        );
    }

    #[test]
    fn test_has_execute_mission_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn execute_mission") ||
            analyzer.code.contains("execute") ||
            analyzer.code.contains("run_mission"),
            "❌ Your controller should have an execute_mission() or similar method"
        );
    }

    #[test]
    fn test_mission_returns_bool() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let returns_bool = analyzer.code.contains("-> bool") ||
                          analyzer.code.contains("return true") ||
                          analyzer.code.contains("return false");
        assert!(
            returns_bool,
            "❌ Your execute_mission method should return bool to indicate success/failure"
        );
    }

    #[test]
    fn test_has_level_setup() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let sets_up_level = analyzer.code.contains("12, 10") ||
                           (analyzer.code.contains("set_cell") && analyzer.code.contains("Enemy"));
        assert!(
            sets_up_level,
            "❌ Your code should set up the level 7 map with enemies and items"
        );
    }

    #[test]
    fn test_uses_pathfinding() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("find_path") ||
            analyzer.code.contains("path"),
            "❌ Your controller should use pathfinding for navigation"
        );
    }

    #[test]
    fn test_handles_enemy_encounters() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let handles_enemies = analyzer.code.contains("Enemy") &&
                             (analyzer.code.contains("damage") || analyzer.code.contains("health"));
        assert!(
            handles_enemies,
            "❌ Your controller should handle enemy encounters and damage"
        );
    }

    #[test]
    fn test_mission_completion_tracking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let tracks_completion = analyzer.code.contains("completed") ||
                               analyzer.code.contains("finished") ||
                               (analyzer.code.contains("objective") && analyzer.code.contains("true"));
        assert!(
            tracks_completion,
            "❌ Your code should track mission objective completion"
        );
    }

    #[test]
    fn test_item_collection_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let collects_items = analyzer.code.contains("collect_item") ||
                            analyzer.code.contains("add_item") ||
                            (analyzer.code.contains("Item") && analyzer.code.contains("inventory"));
        assert!(
            collects_items,
            "❌ Your controller should handle item collection"
        );
    }

    #[test]
    fn test_emergency_procedures() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("emergency") ||
            analyzer.code.contains("health") ||
            analyzer.code.contains("recharge"),
            "❌ Your controller should have emergency procedures for low health/energy"
        );
    }

    #[test]
    fn test_status_reporting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let reports_status = analyzer.code.contains("status") ||
                            analyzer.code.contains("report") ||
                            analyzer.code.contains("summary");
        assert!(
            reports_status,
            "❌ Your controller should provide status reporting"
        );
    }

    #[test]
    fn test_goal_reaching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let reaches_goal = analyzer.code.contains("goal") ||
                          analyzer.code.contains("11, 9") ||
                          analyzer.code.contains("ReachGoal");
        assert!(
            reaches_goal,
            "❌ Your controller should include reaching the goal as an objective"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output mission information
        let has_mission_output = result.stdout.contains("mission") ||
                                result.stdout.contains("objective") ||
                                result.stdout.contains("controller") ||
                                result.stdout.contains("robot") ||
                                result.stdout.contains("complete");

        assert!(
            has_mission_output,
            "❌ Your program should output information about mission execution"
        );
    }
}

// Reference implementation combining all previous systems
#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    item_type: ItemType,
    value: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum ItemType {
    Energy,
    Health,
    Key,
    Tool,
    Data,
}

#[derive(Debug)]
struct Inventory {
    items: Vec<Item>,
    max_capacity: usize,
}

impl Inventory {
    fn new(capacity: usize) -> Self {
        Inventory {
            items: Vec::new(),
            max_capacity: capacity,
        }
    }

    fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() >= self.max_capacity {
            println!("Inventory full! Cannot add {}", item.name);
            return false;
        }
        println!("Added {} to inventory", item.name);
        self.items.push(item);
        true
    }

    fn count_by_type(&self, item_type: &ItemType) -> usize {
        self.items.iter().filter(|item| &item.item_type == item_type).count()
    }

    fn list_items(&self) {
        println!("=== Inventory ({}/{}) ===", self.items.len(), self.max_capacity);
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {} ({:?})", i + 1, item.name, item.item_type);
        }
    }
}

#[derive(Debug)]
struct Robot {
    position: Position,
    health: u32,
    energy: u32,
    is_active: bool,
}

impl Robot {
    fn new(x: i32, y: i32) -> Self {
        Robot {
            position: Position::new(x, y),
            health: 100,
            energy: 100,
            is_active: true,
        }
    }

    fn take_damage(&mut self, damage: u32) {
        self.health = self.health.saturating_sub(damage);
        if self.health == 0 {
            self.is_active = false;
            println!("Robot disabled!");
        } else {
            println!("Robot took {} damage, health: {}", damage, self.health);
        }
    }

    fn recharge(&mut self, amount: u32) {
        self.energy = (self.energy + amount).min(100);
        println!("Robot recharged, energy: {}", self.energy);
    }

    fn status_report(&self) {
        println!("=== Robot Status ===");
        println!("Position: {:?}", self.position);
        println!("Health: {}/100", self.health);
        println!("Energy: {}/100", self.energy);
        println!("Active: {}", self.is_active);
    }
}

#[derive(Debug)]
struct RobotWithInventory {
    robot: Robot,
    inventory: Inventory,
}

impl RobotWithInventory {
    fn new(x: i32, y: i32) -> Self {
        RobotWithInventory {
            robot: Robot::new(x, y),
            inventory: Inventory::new(10),
        }
    }

    fn collect_item(&mut self, item: Item) -> bool {
        if self.inventory.add_item(item.clone()) {
            println!("Robot collected: {}", item.name);
            true
        } else {
            false
        }
    }

    fn use_health_item(&mut self) {
        if let Some(item) = self.inventory.items.iter().find(|i| i.item_type == ItemType::Health).cloned() {
            if let Some(index) = self.inventory.items.iter().position(|i| i.name == item.name) {
                let removed_item = self.inventory.items.remove(index);
                self.robot.health = (self.robot.health + removed_item.value).min(100);
                println!("Health restored to {}", self.robot.health);
            }
        } else {
            println!("No health items available");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Obstacle,
    Enemy,
    Door,
    Item,
    Goal,
}

#[derive(Debug)]
struct GameMap {
    width: i32,
    height: i32,
    cells: Vec<Vec<CellType>>,
    robot_position: Position,
}

impl GameMap {
    fn new(width: i32, height: i32) -> Self {
        let cells = vec![vec![CellType::Empty; height as usize]; width as usize];
        GameMap {
            width,
            height,
            cells,
            robot_position: Position::new(0, 0),
        }
    }

    fn set_cell(&mut self, x: i32, y: i32, cell_type: CellType) {
        if self.is_valid_position(x, y) {
            self.cells[x as usize][y as usize] = cell_type;
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> CellType {
        if self.is_valid_position(x, y) {
            self.cells[x as usize][y as usize]
        } else {
            CellType::Obstacle
        }
    }

    fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn can_move_to(&self, x: i32, y: i32) -> bool {
        match self.get_cell(x, y) {
            CellType::Empty | CellType::Item | CellType::Goal => true,
            _ => false,
        }
    }

    fn find_path_to(&self, target: Position) -> Vec<Position> {
        let mut path = vec![];
        let current = self.robot_position;
        let mut next = current;

        // Simple pathfinding
        while next != target && path.len() < 50 {
            let dx = if target.x > next.x { 1 } else if target.x < next.x { -1 } else { 0 };
            let dy = if target.y > next.y { 1 } else if target.y < next.y { -1 } else { 0 };

            if dx != 0 && self.can_move_to(next.x + dx, next.y) {
                next.x += dx;
            } else if dy != 0 && self.can_move_to(next.x, next.y + dy) {
                next.y += dy;
            } else {
                break;
            }

            path.push(next);
        }

        path
    }

    fn scan_around_robot(&self, range: i32) -> Vec<(Position, CellType)> {
        let mut scan_results = vec![];
        for dx in -range..=range {
            for dy in -range..=range {
                if dx == 0 && dy == 0 { continue; }
                let scan_x = self.robot_position.x + dx;
                let scan_y = self.robot_position.y + dy;
                if self.is_valid_position(scan_x, scan_y) {
                    let cell_type = self.get_cell(scan_x, scan_y);
                    scan_results.push((Position::new(scan_x, scan_y), cell_type));
                }
            }
        }
        scan_results
    }

    fn move_robot(&mut self, new_position: Position) -> bool {
        if self.can_move_to(new_position.x, new_position.y) {
            self.robot_position = new_position;
            println!("Robot moved to {:?}", new_position);
            true
        } else {
            println!("Cannot move to {:?} - blocked", new_position);
            false
        }
    }
}

#[derive(Debug, Clone)]
struct Objective {
    objective_type: ObjectiveType,
    target_position: Position,
    completed: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ObjectiveType {
    CollectItem(String),
    OpenDoor,
    AvoidEnemy,
    ReachGoal,
}

struct MasterRobotController {
    robot: RobotWithInventory,
    game_map: GameMap,
    mission_objectives: Vec<Objective>,
}

impl MasterRobotController {
    fn new() -> Self {
        let mut game_map = GameMap::new(12, 10);

        // Set up level 7 layout
        game_map.set_cell(3, 2, CellType::Enemy);
        game_map.set_cell(8, 5, CellType::Enemy);
        game_map.set_cell(5, 8, CellType::Enemy);
        game_map.set_cell(6, 3, CellType::Door);
        game_map.set_cell(4, 7, CellType::Door);
        game_map.set_cell(2, 1, CellType::Item);
        game_map.set_cell(10, 3, CellType::Item);
        game_map.set_cell(1, 6, CellType::Item);
        game_map.set_cell(7, 9, CellType::Item);
        game_map.set_cell(11, 9, CellType::Goal);

        let objectives = vec![
            Objective {
                objective_type: ObjectiveType::CollectItem("struct_blueprint".to_string()),
                target_position: Position::new(2, 1),
                completed: false,
            },
            Objective {
                objective_type: ObjectiveType::CollectItem("method_chip".to_string()),
                target_position: Position::new(10, 3),
                completed: false,
            },
            Objective {
                objective_type: ObjectiveType::ReachGoal,
                target_position: Position::new(11, 9),
                completed: false,
            },
        ];

        MasterRobotController {
            robot: RobotWithInventory::new(0, 0),
            game_map,
            mission_objectives: objectives,
        }
    }

    fn execute_mission(&mut self) -> bool {
        println!("🚀 Starting Robot Mission: Struct Collection");

        for objective in &mut self.mission_objectives {
            if objective.completed {
                continue;
            }

            println!("\n--- Executing objective: {:?} ---", objective.objective_type);

            // Plan path to objective
            let path = self.game_map.find_path_to(objective.target_position);

            if path.is_empty() {
                println!("No path found to objective!");
                continue;
            }

            // Execute path
            for step in path {
                // Check for enemies before moving
                let scan_results = self.game_map.scan_around_robot(1);
                let enemy_nearby = scan_results.iter().any(|(_, cell)| *cell == CellType::Enemy);

                if enemy_nearby {
                    println!("⚠️ Enemy detected! Taking evasive action...");
                    self.robot.robot.take_damage(10);

                    // Use health item if available
                    if self.robot.robot.health < 50 {
                        self.robot.use_health_item();
                    }
                }

                // Move robot
                if self.game_map.move_robot(step) {
                    self.robot.robot.position = step;
                }

                // Check if we reached the objective
                if step == objective.target_position {
                    match &objective.objective_type {
                        ObjectiveType::CollectItem(item_name) => {
                            let item = Item {
                                name: item_name.clone(),
                                item_type: ItemType::Data,
                                value: 50,
                            };
                            if self.robot.collect_item(item) {
                                objective.completed = true;
                                println!("✅ Collected: {}", item_name);
                            }
                        }
                        ObjectiveType::ReachGoal => {
                            objective.completed = true;
                            println!("✅ Reached goal!");
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }

        // Check mission completion
        let completed_objectives = self.mission_objectives.iter().filter(|obj| obj.completed).count();
        let total_objectives = self.mission_objectives.len();

        println!("\n=== Mission Summary ===");
        println!("Objectives completed: {}/{}", completed_objectives, total_objectives);

        self.robot.inventory.list_items();
        self.robot.robot.status_report();

        completed_objectives == total_objectives
    }

    fn emergency_procedures(&mut self) {
        println!("🚨 Emergency procedures activated!");

        if self.robot.robot.health < 30 {
            println!("Critical health - using health items");
            while self.robot.inventory.count_by_type(&ItemType::Health) > 0 {
                self.robot.use_health_item();
            }
        }

        if self.robot.robot.energy < 20 {
            println!("Low energy - emergency recharge");
            self.robot.robot.recharge(50);
        }
    }
}

fn main() {
    let mut controller = MasterRobotController::new();

    println!("Initializing Master Robot Controller...");
    controller.robot.robot.status_report();

    // Execute the mission
    let success = controller.execute_mission();

    if success {
        println!("\n🎉 MISSION ACCOMPLISHED!");
        println!("All struct-based systems functioning perfectly!");
    } else {
        println!("\n❌ Mission incomplete - activating emergency procedures");
        controller.emergency_procedures();
    }

    println!("\n🤖 Robot systems demonstration complete!");
}