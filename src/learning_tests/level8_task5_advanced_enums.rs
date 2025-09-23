// Level 8 Task 5 Test: Advanced Enum Patterns and Complex State Management
// Tests if the user code creates sophisticated robot AI using advanced enum patterns

#[cfg(test)]
mod level8_task5_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_ai_state_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum AIState") ||
            analyzer.code.contains("enum AI") ||
            analyzer.code.contains("enum State"),
            "❌ Your code should define an AIState enum"
        );
    }

    #[test]
    fn test_ai_state_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_initializing = analyzer.code.contains("Initializing");
        let has_planning = analyzer.code.contains("Planning");
        let has_executing = analyzer.code.contains("Executing");
        let has_adapting = analyzer.code.contains("Adapting");
        assert!(
            has_initializing && has_planning && has_executing && has_adapting,
            "❌ Your AIState enum should have Initializing, Planning, Executing, and Adapting variants"
        );
    }

    #[test]
    fn test_decision_factor_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum DecisionFactor") ||
            analyzer.code.contains("enum Factor"),
            "❌ Your code should define a DecisionFactor enum"
        );
    }

    #[test]
    fn test_ai_action_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum AIAction") ||
            analyzer.code.contains("enum Action"),
            "❌ Your code should define an AIAction enum"
        );
    }

    #[test]
    fn test_complex_enum_data() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_objectives = analyzer.code.contains("objectives: Vec<String>");
        let has_retry_count = analyzer.code.contains("retry_count");
        let has_steps_remaining = analyzer.code.contains("steps_remaining");
        assert!(
            has_objectives && has_retry_count && has_steps_remaining,
            "❌ Your enum variants should carry complex data (objectives, retry_count, steps_remaining)"
        );
    }

    #[test]
    fn test_advanced_robot_ai_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct AdvancedRobotAI") ||
            analyzer.code.contains("struct RobotAI") ||
            analyzer.code.contains("struct AI"),
            "❌ Your code should define an AdvancedRobotAI struct"
        );
    }

    #[test]
    fn test_analyze_situation_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn analyze_situation") ||
            analyzer.code.contains("fn analyze"),
            "❌ Your AI should have an analyze_situation method"
        );
    }

    #[test]
    fn test_decide_next_action_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn decide_next_action") ||
            analyzer.code.contains("fn decide"),
            "❌ Your AI should have a decide_next_action method"
        );
    }

    #[test]
    fn test_execute_action_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn execute_action"),
            "❌ Your AI should have an execute_action method"
        );
    }

    #[test]
    fn test_update_state_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn update_state"),
            "❌ Your AI should have an update_state method"
        );
    }

    #[test]
    fn test_complex_pattern_matching() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let complex_matching = analyzer.code.contains("match") &&
                              analyzer.code.contains("current_state") &&
                              analyzer.code.contains("action_result");
        assert!(
            complex_matching,
            "❌ Your code should use complex pattern matching for state transitions"
        );
    }

    #[test]
    fn test_decision_making_logic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_decision_logic = analyzer.code.contains("decision_factors") &&
                                analyzer.code.contains("for factor in");
        assert!(
            has_decision_logic,
            "❌ Your AI should analyze decision factors for action selection"
        );
    }

    #[test]
    fn test_energy_level_factor() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("EnergyLevel") ||
            analyzer.code.contains("energy"),
            "❌ Your decision factors should include energy level"
        );
    }

    #[test]
    fn test_threat_level_factor() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("ThreatLevel") ||
            analyzer.code.contains("threat"),
            "❌ Your decision factors should include threat level"
        );
    }

    #[test]
    fn test_mission_progress_tracking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let tracks_progress = analyzer.code.contains("MissionProgress") ||
                             analyzer.code.contains("progress");
        assert!(
            tracks_progress,
            "❌ Your AI should track mission progress"
        );
    }

    #[test]
    fn test_retry_mechanism() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_retry = analyzer.code.contains("retry_count") &&
                       analyzer.code.contains("max_retries");
        assert!(
            has_retry,
            "❌ Your AI should implement a retry mechanism for failed actions"
        );
    }

    #[test]
    fn test_action_history() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("action_history"),
            "❌ Your AI should maintain an action history"
        );
    }

    #[test]
    fn test_ai_cycle_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn run_ai_cycle") ||
            analyzer.code.contains("fn cycle"),
            "❌ Your AI should have a run_ai_cycle method"
        );
    }

    #[test]
    fn test_completion_detection() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let detects_completion = analyzer.code.contains("Completed") &&
                                analyzer.code.contains("success");
        assert!(
            detects_completion,
            "❌ Your AI should detect mission completion"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output AI system information
        let has_ai_output = result.stdout.contains("AI") ||
                           result.stdout.contains("Cycle") ||
                           result.stdout.contains("State") ||
                           result.stdout.contains("Mission") ||
                           result.stdout.contains("Action");

        assert!(
            has_ai_output,
            "❌ Your program should output information about AI system execution"
        );
    }
}

// Reference implementation combining all advanced enum concepts
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
enum RobotError {
    InsufficientEnergy { required: u32, available: u32 },
    PathBlocked { obstacle: String },
    InvalidPosition { x: i32, y: i32 },
    SystemFailure { component: String },
}

#[derive(Debug)]
struct RobotController {
    position: (i32, i32),
    energy: u32,
    map_bounds: (i32, i32),
}

impl RobotController {
    fn new() -> Self {
        RobotController {
            position: (0, 0),
            energy: 100,
            map_bounds: (14, 12),
        }
    }

    fn move_to(&mut self, x: i32, y: i32) -> Result<(), RobotError> {
        let energy_cost = ((x - self.position.0).abs() + (y - self.position.1).abs()) as u32 * 5;

        if self.energy < energy_cost {
            return Err(RobotError::InsufficientEnergy {
                required: energy_cost,
                available: self.energy,
            });
        }

        if x < 0 || y < 0 || x >= self.map_bounds.0 || y >= self.map_bounds.1 {
            return Err(RobotError::InvalidPosition { x, y });
        }

        let blocked_positions = vec![(4, 3), (9, 6), (3, 9), (11, 4)];
        if blocked_positions.contains(&(x, y)) {
            return Err(RobotError::PathBlocked {
                obstacle: "Enemy".to_string(),
            });
        }

        self.position = (x, y);
        self.energy -= energy_cost;
        Ok(())
    }

    fn recharge(&mut self) -> Result<(), RobotError> {
        self.energy = 100;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
enum AIState {
    Initializing,
    Planning { objectives: Vec<String> },
    Executing { current_objective: String, steps_remaining: u32 },
    Adapting { problem: RobotError, retry_count: u32 },
    Completed { success: bool, items_collected: u32 },
}

#[derive(Debug, Clone)]
enum DecisionFactor {
    EnergyLevel(u32),
    ThreatLevel(u32),
    ItemsNearby(Vec<String>),
    PathClearance(bool),
    MissionProgress(f32),
}

#[derive(Debug, Clone)]
enum AIAction {
    Move(Direction),
    Scan { radius: u32 },
    Collect { item: String },
    Avoid { threat_pos: (i32, i32) },
    Recharge,
    Wait { duration: u32 },
    Abort { reason: String },
}

struct AdvancedRobotAI {
    controller: RobotController,
    current_state: AIState,
    decision_factors: Vec<DecisionFactor>,
    action_history: Vec<AIAction>,
    objectives: Vec<String>,
    max_retries: u32,
}

impl AdvancedRobotAI {
    fn new() -> Self {
        AdvancedRobotAI {
            controller: RobotController::new(),
            current_state: AIState::Initializing,
            decision_factors: Vec::new(),
            action_history: Vec::new(),
            objectives: vec![
                "Collect enum_core".to_string(),
                "Collect state_machine".to_string(),
                "Collect option_handler".to_string(),
                "Collect result_processor".to_string(),
                "Reach goal".to_string(),
            ],
            max_retries: 3,
        }
    }

    fn analyze_situation(&mut self) {
        self.decision_factors.clear();

        // Analyze energy
        self.decision_factors.push(DecisionFactor::EnergyLevel(self.controller.energy));

        // Analyze threats (simulate enemy detection)
        let enemy_positions = vec![(4, 3), (9, 6), (3, 9), (11, 4)];
        let mut threat_level = 0;

        for enemy_pos in enemy_positions {
            let distance = (self.controller.position.0 - enemy_pos.0).abs() +
                          (self.controller.position.1 - enemy_pos.1).abs();
            if distance <= 3 {
                threat_level += 10 / (distance + 1) as u32;
            }
        }

        self.decision_factors.push(DecisionFactor::ThreatLevel(threat_level));

        // Analyze items nearby
        let item_positions = vec![(3, 1), (12, 3), (1, 8), (8, 10)];
        let mut nearby_items = Vec::new();

        for item_pos in item_positions {
            let distance = (self.controller.position.0 - item_pos.0).abs() +
                          (self.controller.position.1 - item_pos.1).abs();
            if distance <= 4 {
                nearby_items.push(format!("item_at_{:?}", item_pos));
            }
        }

        self.decision_factors.push(DecisionFactor::ItemsNearby(nearby_items));

        // Calculate mission progress
        let completed_objectives = 5 - self.objectives.len();
        let progress = completed_objectives as f32 / 5.0;
        self.decision_factors.push(DecisionFactor::MissionProgress(progress));
    }

    fn decide_next_action(&self) -> AIAction {
        // Decision-making based on current state and factors
        match &self.current_state {
            AIState::Initializing => AIAction::Scan { radius: 3 },
            AIState::Planning { objectives } => {
                if objectives.is_empty() {
                    AIAction::Move(Direction::East) // Head to goal
                } else {
                    AIAction::Move(Direction::North) // Start first objective
                }
            }
            AIState::Executing { current_objective, steps_remaining } => {
                // Check decision factors
                for factor in &self.decision_factors {
                    match factor {
                        DecisionFactor::EnergyLevel(energy) if *energy < 30 => {
                            return AIAction::Recharge;
                        }
                        DecisionFactor::ThreatLevel(threat) if *threat > 20 => {
                            return AIAction::Avoid { threat_pos: (4, 3) };
                        }
                        DecisionFactor::ItemsNearby(items) if !items.is_empty() => {
                            return AIAction::Collect { item: items[0].clone() };
                        }
                        _ => {}
                    }
                }

                if *steps_remaining > 0 {
                    AIAction::Move(Direction::East)
                } else {
                    AIAction::Scan { radius: 2 }
                }
            }
            AIState::Adapting { problem, retry_count } => {
                match problem {
                    RobotError::InsufficientEnergy { .. } => AIAction::Recharge,
                    RobotError::PathBlocked { .. } => AIAction::Move(Direction::South),
                    _ => {
                        if *retry_count >= self.max_retries {
                            AIAction::Abort { reason: "Too many failures".to_string() }
                        } else {
                            AIAction::Move(Direction::West)
                        }
                    }
                }
            }
            AIState::Completed { .. } => AIAction::Wait { duration: 0 },
        }
    }

    fn execute_action(&mut self, action: AIAction) -> Result<(), RobotError> {
        println!("Executing: {:?}", action);
        self.action_history.push(action.clone());

        match action {
            AIAction::Move(direction) => {
                let (dx, dy) = match direction {
                    Direction::North => (0, 1),
                    Direction::South => (0, -1),
                    Direction::East => (1, 0),
                    Direction::West => (-1, 0),
                };

                let new_x = self.controller.position.0 + dx;
                let new_y = self.controller.position.1 + dy;

                self.controller.move_to(new_x, new_y)
            }
            AIAction::Scan { radius } => {
                println!("Scanning with radius {}", radius);
                Ok(()) // Simulate successful scan
            }
            AIAction::Collect { item } => {
                println!("Successfully collected: {}", item);
                self.objectives.retain(|obj| !obj.contains(&item));
                Ok(())
            }
            AIAction::Avoid { threat_pos } => {
                // Move away from threat
                let away_x = if self.controller.position.0 < threat_pos.0 {
                    self.controller.position.0 - 1
                } else {
                    self.controller.position.0 + 1
                };

                self.controller.move_to(away_x, self.controller.position.1)
            }
            AIAction::Recharge => self.controller.recharge(),
            AIAction::Wait { duration } => {
                println!("Waiting for {} cycles", duration);
                Ok(())
            }
            AIAction::Abort { reason } => {
                println!("Mission aborted: {}", reason);
                Err(RobotError::SystemFailure { component: reason })
            }
        }
    }

    fn update_state(&mut self, action_result: Result<(), RobotError>) {
        self.current_state = match (&self.current_state, action_result) {
            (AIState::Initializing, Ok(())) => {
                AIState::Planning { objectives: self.objectives.clone() }
            }
            (AIState::Planning { .. }, Ok(())) => {
                if self.objectives.is_empty() {
                    AIState::Completed { success: true, items_collected: 4 }
                } else {
                    AIState::Executing {
                        current_objective: self.objectives[0].clone(),
                        steps_remaining: 10,
                    }
                }
            }
            (AIState::Executing { steps_remaining, .. }, Ok(())) => {
                if *steps_remaining <= 1 {
                    AIState::Planning { objectives: self.objectives.clone() }
                } else {
                    AIState::Executing {
                        current_objective: self.objectives.get(0).unwrap_or(&"Complete".to_string()).clone(),
                        steps_remaining: steps_remaining - 1,
                    }
                }
            }
            (_, Err(error)) => {
                let retry_count = match &self.current_state {
                    AIState::Adapting { retry_count, .. } => retry_count + 1,
                    _ => 1,
                };

                if retry_count > self.max_retries {
                    AIState::Completed { success: false, items_collected: 0 }
                } else {
                    AIState::Adapting {
                        problem: error,
                        retry_count,
                    }
                }
            }
            (AIState::Adapting { retry_count, .. }, Ok(())) => {
                AIState::Planning { objectives: self.objectives.clone() }
            }
            (state, _) => state.clone(),
        };
    }

    fn run_ai_cycle(&mut self) -> bool {
        println!("\n--- AI Cycle ---");
        println!("State: {:?}", self.current_state);
        println!("Position: {:?}", self.controller.position);
        println!("Energy: {}", self.controller.energy);
        println!("Objectives remaining: {}", self.objectives.len());

        // Analyze situation
        self.analyze_situation();

        // Decide action
        let action = self.decide_next_action();

        // Execute action
        let result = self.execute_action(action);

        // Update state
        self.update_state(result);

        // Check if mission is complete
        matches!(self.current_state, AIState::Completed { .. })
    }
}

fn main() {
    println!("🤖 Advanced Robot AI System - Level 8 Mission");

    let mut ai = AdvancedRobotAI::new();

    // Run AI for up to 25 cycles
    for cycle in 1..=25 {
        println!("\n🔄 === Cycle {} ===", cycle);

        if ai.run_ai_cycle() {
            println!("\n🎯 Mission Complete!");
            match &ai.current_state {
                AIState::Completed { success, items_collected } => {
                    if *success {
                        println!("✅ Success! Collected {} items", items_collected);
                    } else {
                        println!("❌ Mission failed after collecting {} items", items_collected);
                    }
                }
                _ => unreachable!(),
            }
            break;
        }
    }

    println!("\n📊 Final Statistics:");
    println!("Actions taken: {}", ai.action_history.len());
    println!("Objectives remaining: {}", ai.objectives.len());
    println!("Final position: {:?}", ai.controller.position);
    println!("Final energy: {}", ai.controller.energy);

    println!("\n📝 Action History:");
    for (i, action) in ai.action_history.iter().enumerate().take(10) {
        println!("  {}: {:?}", i + 1, action);
    }
}