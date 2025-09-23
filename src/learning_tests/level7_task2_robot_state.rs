// Level 7 Task 2 Test: Robot State Management Struct
// Tests if the user code creates a comprehensive Robot struct

#[cfg(test)]
mod level7_task2_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_robot_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Robot"),
            "❌ Your code should define a Robot struct"
        );
    }

    #[test]
    fn test_has_position_field() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("position: Position") || analyzer.code.contains("position:Position"),
            "❌ Your Robot struct should have a position field of type Position"
        );
    }

    #[test]
    fn test_has_health_energy_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_health = analyzer.code.contains("health: u32") || analyzer.code.contains("health:u32");
        let has_energy = analyzer.code.contains("energy: u32") || analyzer.code.contains("energy:u32");
        assert!(
            has_health && has_energy,
            "❌ Your Robot struct should have health and energy fields of type u32"
        );
    }

    #[test]
    fn test_has_is_active_field() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("is_active: bool") || analyzer.code.contains("is_active:bool"),
            "❌ Your Robot struct should have an is_active field of type bool"
        );
    }

    #[test]
    fn test_has_robot_impl_block() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("impl Robot"),
            "❌ Your code should have an impl block for Robot"
        );
    }

    #[test]
    fn test_has_move_to_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn move_to"),
            "❌ Your Robot should have a move_to() method"
        );
    }

    #[test]
    fn test_has_scan_area_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn scan_area"),
            "❌ Your Robot should have a scan_area() method"
        );
    }

    #[test]
    fn test_has_recharge_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn recharge"),
            "❌ Your Robot should have a recharge() method"
        );
    }

    #[test]
    fn test_has_take_damage_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn take_damage"),
            "❌ Your Robot should have a take_damage() method"
        );
    }

    #[test]
    fn test_has_status_report_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn status_report"),
            "❌ Your Robot should have a status_report() method"
        );
    }

    #[test]
    fn test_energy_consumption() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_energy_check = analyzer.code.contains("self.energy < ") || analyzer.code.contains("energy < ");
        let has_energy_sub = analyzer.code.contains("self.energy -= ") || analyzer.code.contains("energy -= ");
        assert!(
            has_energy_check && has_energy_sub,
            "❌ Your methods should check and consume energy"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output robot status and actions
        let has_robot_output = result.stdout.contains("Robot") ||
                              result.stdout.contains("energy") ||
                              result.stdout.contains("health") ||
                              result.stdout.contains("moved");

        assert!(
            has_robot_output,
            "❌ Your program should output information about robot state and actions"
        );
    }
}

// Reference implementation for comparison
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

#[derive(Debug)]
struct Robot {
    position: Position,
    health: u32,
    energy: u32,
    is_active: bool,
    scan_range: u32,
}

impl Robot {
    fn new(x: i32, y: i32) -> Self {
        Robot {
            position: Position::new(x, y),
            health: 100,
            energy: 100,
            is_active: true,
            scan_range: 2,
        }
    }

    fn move_to(&mut self, x: i32, y: i32) -> bool {
        if self.energy < 10 {
            println!("Insufficient energy to move!");
            return false;
        }

        self.position.x = x;
        self.position.y = y;
        self.energy -= 10;
        println!("Robot moved to ({}, {}), energy: {}", x, y, self.energy);
        true
    }

    fn scan_area(&mut self) -> Vec<String> {
        if self.energy < 5 {
            println!("Insufficient energy to scan!");
            return vec![];
        }

        self.energy -= 5;
        let mut scan_results = vec![];

        for dx in -1..=1 {
            for dy in -1..=1 {
                let scan_x = self.position.x + dx;
                let scan_y = self.position.y + dy;

                if scan_x >= 0 && scan_y >= 0 {
                    scan_results.push(format!("({},{}):clear", scan_x, scan_y));
                }
            }
        }

        println!("Scanned {} positions, energy: {}", scan_results.len(), self.energy);
        scan_results
    }

    fn recharge(&mut self, amount: u32) {
        self.energy = (self.energy + amount).min(100);
        println!("Robot recharged, energy: {}", self.energy);
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

    fn status_report(&self) {
        println!("=== Robot Status ===");
        println!("Position: {:?}", self.position);
        println!("Health: {}/100", self.health);
        println!("Energy: {}/100", self.energy);
        println!("Active: {}", self.is_active);
    }
}

fn main() {
    let mut robot = Robot::new(0, 0);
    robot.status_report();

    let navigation_path = vec![(1, 0), (2, 0), (2, 1)];

    for (x, y) in navigation_path {
        if robot.move_to(x, y) {
            robot.scan_area();
        }
    }

    println!("Enemy encounter!");
    robot.take_damage(25);
    robot.recharge(30);
    robot.status_report();
}