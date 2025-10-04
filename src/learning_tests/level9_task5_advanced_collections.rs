// Level 9 Task 5 Test: Advanced Collection Operations and Real-World Usage
// Tests if the user code combines all collection types in sophisticated ways

#[cfg(test)]
mod level9_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_imports_all_collections() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_all_imports = analyzer.code.contains("HashMap") &&
                             analyzer.code.contains("HashSet") &&
                             analyzer.code.contains("VecDeque");
        assert!(
            has_all_imports,
            "❌ Your code should import HashMap, HashSet, and VecDeque"
        );
    }

    #[test]
    fn test_defines_robot_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_robot_struct = analyzer.code.contains("struct Robot") ||
                              analyzer.code.contains("Robot {");
        assert!(
            has_robot_struct,
            "❌ Your code should define a Robot struct"
        );
    }

    #[test]
    fn test_defines_fleet_command_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_fleet_struct = analyzer.code.contains("struct FleetCommand") ||
                              analyzer.code.contains("FleetCommand {");
        assert!(
            has_fleet_struct,
            "❌ Your code should define a FleetCommand struct"
        );
    }

    #[test]
    fn test_implements_fleet_methods() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_impl_block = analyzer.code.contains("impl FleetCommand") ||
                           analyzer.code.contains("impl ");
        assert!(
            has_impl_block,
            "❌ Your code should implement methods for FleetCommand"
        );
    }

    #[test]
    fn test_uses_vecdeque() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let uses_vecdeque = analyzer.code.contains("VecDeque") &&
                          (analyzer.code.contains("push_back") ||
                           analyzer.code.contains("pop_front"));
        assert!(
            uses_vecdeque,
            "❌ Your code should use VecDeque for task queuing"
        );
    }

    #[test]
    fn test_robot_has_inventory() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let robot_has_inventory = analyzer.code.contains("inventory: Vec<String>") ||
                                 analyzer.code.contains("inventory");
        assert!(
            robot_has_inventory,
            "❌ Robot struct should have an inventory field using Vec"
        );
    }

    #[test]
    fn test_robot_has_visited_positions() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_visited = analyzer.code.contains("visited_positions: HashSet") ||
                         analyzer.code.contains("visited_positions");
        assert!(
            has_visited,
            "❌ Robot struct should track visited positions using HashSet"
        );
    }

    #[test]
    fn test_fleet_manages_robots() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let manages_robots = analyzer.code.contains("robots: HashMap") ||
                           analyzer.code.contains("add_robot");
        assert!(
            manages_robots,
            "❌ FleetCommand should manage robots using HashMap"
        );
    }

    #[test]
    fn test_task_queue_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_task_ops = analyzer.code.contains("assign_task") ||
                          analyzer.code.contains("execute_next_task") ||
                          analyzer.code.contains("task_queue");
        assert!(
            has_task_ops,
            "❌ Your code should implement task queue operations"
        );
    }

    #[test]
    fn test_optimal_robot_selection() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let finds_optimal = analyzer.code.contains("find_optimal_robot") ||
                           analyzer.code.contains("optimal") ||
                           analyzer.code.contains("best_robot");
        assert!(
            finds_optimal,
            "❌ Your code should implement optimal robot selection"
        );
    }

    #[test]
    fn test_fleet_status_reporting() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_status = analyzer.code.contains("get_fleet_status") ||
                        analyzer.code.contains("status");
        assert!(
            has_status,
            "❌ Your code should implement fleet status reporting"
        );
    }

    #[test]
    fn test_inventory_consolidation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let consolidates = analyzer.code.contains("consolidate_inventory") ||
                         analyzer.code.contains("consolidate");
        assert!(
            consolidates,
            "❌ Your code should implement inventory consolidation across robots"
        );
    }

    #[test]
    fn test_exploration_coverage() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let tracks_coverage = analyzer.code.contains("get_exploration_coverage") ||
                             analyzer.code.contains("coverage") ||
                             analyzer.code.contains("flat_map");
        assert!(
            tracks_coverage,
            "❌ Your code should track exploration coverage across the fleet"
        );
    }

    #[test]
    fn test_fleet_optimization() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let optimizes_fleet = analyzer.code.contains("optimize_fleet_positions") ||
                            analyzer.code.contains("optimize");
        assert!(
            optimizes_fleet,
            "❌ Your code should implement fleet position optimization"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        let has_fleet_output = result.stdout.contains("Fleet") ||
                              result.stdout.contains("robot") ||
                              result.stdout.contains("Task") ||
                              result.stdout.contains("Alpha") ||
                              result.stdout.contains("completed");

        assert!(
            has_fleet_output,
            "❌ Your program should output information about fleet management operations"
        );
    }
}

// Reference implementation
fn main() {
    use std::collections::{HashMap, HashSet, VecDeque};

    #[derive(Debug, Clone)]
    struct Robot {
        id: String,
        position: (i32, i32),
        energy: u32,
        inventory: Vec<String>,
        visited_positions: HashSet<(i32, i32)>,
    }

    struct FleetCommand {
        robots: HashMap<String, Robot>,
        task_queue: VecDeque<String>,
        completed_objectives: HashSet<String>,
        resource_cache: HashMap<String, u32>,
    }

    impl FleetCommand {
        fn new() -> Self {
            FleetCommand {
                robots: HashMap::new(),
                task_queue: VecDeque::new(),
                completed_objectives: HashSet::new(),
                resource_cache: HashMap::new(),
            }
        }

        fn add_robot(&mut self, id: String, start_pos: (i32, i32)) {
            let robot = Robot {
                id: id.clone(),
                position: start_pos,
                energy: 100,
                inventory: Vec::new(),
                visited_positions: HashSet::new(),
            };
            self.robots.insert(id, robot);
        }

        fn assign_task(&mut self, task: String) {
            self.task_queue.push_back(task);
        }

        fn execute_next_task(&mut self) -> Option<String> {
            if let Some(task) = self.task_queue.pop_front() {
                // Find best robot for task
                let best_robot_id = self.find_optimal_robot(&task)?;

                if let Some(robot) = self.robots.get_mut(&best_robot_id) {
                    println!("Assigning '{}' to robot {}", task, robot.id);

                    // Execute task
                    match task.as_str() {
                        "collect_vector_core" => {
                            robot.position = (3, 1);
                            robot.inventory.push("vector_core".to_string());
                            robot.energy -= 20;
                        }
                        "collect_hashmap_engine" => {
                            robot.position = (13, 3);
                            robot.inventory.push("hashmap_engine".to_string());
                            robot.energy -= 30;
                        }
                        "collect_iterator_module" => {
                            robot.position = (2, 8);
                            robot.inventory.push("iterator_module".to_string());
                            robot.energy -= 25;
                        }
                        "collect_collection_analyzer" => {
                            robot.position = (12, 2);
                            robot.inventory.push("collection_analyzer".to_string());
                            robot.energy -= 35;
                        }
                        _ => println!("Unknown task: {}", task),
                    }

                    robot.visited_positions.insert(robot.position);
                    self.completed_objectives.insert(task.clone());
                    Some(format!("Task '{}' completed by {}", task, robot.id))
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn find_optimal_robot(&self, _task: &str) -> Option<String> {
            // Find robot with highest energy and shortest path
            self.robots.iter()
                .filter(|(_, robot)| robot.energy > 30)
                .max_by(|(_, a), (_, b)| a.energy.cmp(&b.energy))
                .map(|(id, _)| id.clone())
        }

        fn get_fleet_status(&self) -> HashMap<String, String> {
            self.robots.iter()
                .map(|(id, robot)| {
                    let status = format!(
                        "Pos: {:?}, Energy: {}, Items: {}, Visited: {}",
                        robot.position,
                        robot.energy,
                        robot.inventory.len(),
                        robot.visited_positions.len()
                    );
                    (id.clone(), status)
                })
                .collect()
        }

        fn consolidate_inventory(&self) -> HashMap<String, u32> {
            let mut consolidated = HashMap::new();

            for robot in self.robots.values() {
                for item in &robot.inventory {
                    *consolidated.entry(item.clone()).or_insert(0) += 1;
                }
            }

            consolidated
        }

        fn get_exploration_coverage(&self) -> HashSet<(i32, i32)> {
            self.robots.values()
                .flat_map(|robot| robot.visited_positions.iter())
                .cloned()
                .collect()
        }

        fn optimize_fleet_positions(&mut self) {
            // Spread robots across the level for better coverage
            let target_positions = vec![(0, 0), (7, 0), (14, 0), (0, 9), (14, 9)];

            for (i, robot) in self.robots.values_mut().enumerate() {
                if i < target_positions.len() {
                    robot.position = target_positions[i];
                    robot.visited_positions.insert(robot.position);
                    robot.energy = robot.energy.saturating_sub(10);
                }
            }
        }
    }

    println!("🤖 Advanced Robot Fleet Management System");

    let mut fleet = FleetCommand::new();

    // Initialize robot fleet
    fleet.add_robot("Alpha".to_string(), (0, 0));
    fleet.add_robot("Beta".to_string(), (7, 5));
    fleet.add_robot("Gamma".to_string(), (14, 9));
    fleet.add_robot("Delta".to_string(), (5, 2));

    println!("Fleet initialized with {} robots", fleet.robots.len());

    // Queue up collection tasks
    let tasks = vec![
        "collect_vector_core",
        "collect_hashmap_engine",
        "collect_iterator_module",
        "collect_collection_analyzer",
    ];

    for task in &tasks {
        fleet.assign_task(task.to_string());
    }

    println!("Queued {} tasks", fleet.task_queue.len());

    // Execute all tasks
    println!("\n=== Task Execution ===");
    while let Some(result) = fleet.execute_next_task() {
        println!("✅ {}", result);
    }

    // Fleet status report
    println!("\n=== Fleet Status Report ===");
    let status_map = fleet.get_fleet_status();
    for (robot_id, status) in status_map {
        println!("{}: {}", robot_id, status);
    }

    // Consolidated inventory
    let inventory = fleet.consolidate_inventory();
    println!("\n=== Fleet Inventory ===");
    let total_items: u32 = inventory.values().sum();
    println!("Total items collected: {}", total_items);
    for (item, count) in inventory {
        println!("  {}: {}", item, count);
    }

    // Exploration coverage
    let coverage = fleet.get_exploration_coverage();
    println!("\n=== Exploration Coverage ===");
    println!("Unique positions visited: {}", coverage.len());

    // Show some visited positions
    let sample_positions: Vec<_> = coverage.iter().take(10).collect();
    println!("Sample positions: {:?}", sample_positions);

    // Mission completion analysis
    let required_objectives: HashSet<String> = tasks.iter()
        .map(|s| s.to_string())
        .collect();

    let completion_rate = (fleet.completed_objectives.len() as f32 /
                         required_objectives.len() as f32) * 100.0;

    println!("\n=== Mission Analysis ===");
    println!("Objectives completed: {}/{}",
             fleet.completed_objectives.len(),
             required_objectives.len());
    println!("Completion rate: {:.1}%", completion_rate);

    if fleet.completed_objectives == required_objectives {
        println!("🎉 ALL OBJECTIVES COMPLETED!");
    } else {
        let missing: HashSet<_> = required_objectives
            .difference(&fleet.completed_objectives)
            .collect();
        println!("Missing objectives: {:?}", missing);
    }

    // Optimize fleet for next mission
    println!("\n=== Fleet Optimization ===");
    fleet.optimize_fleet_positions();

    let optimized_status = fleet.get_fleet_status();
    println!("Fleet repositioned for optimal coverage:");
    for (robot_id, status) in optimized_status {
        println!("  {}: {}", robot_id, status);
    }

    // Performance metrics
    let total_energy_used: u32 = fleet.robots.values()
        .map(|robot| 100 - robot.energy)
        .sum();

    let avg_energy_efficiency = total_energy_used as f32 / fleet.robots.len() as f32;

    println!("\n=== Performance Metrics ===");
    println!("Total energy consumed: {}", total_energy_used);
    println!("Average energy per robot: {:.1}", avg_energy_efficiency);
    println!("Fleet operational efficiency: {:.1}%",
             (400.0 - total_energy_used as f32) / 4.0);
}