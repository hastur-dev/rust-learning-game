// Level 8 Task 3 Test: Option Enum for Safe Item Handling
// Tests if the user code uses Option<T> enum for safe item collection

#[cfg(test)]
mod level8_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_game_item_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct GameItem") ||
            analyzer.code.contains("struct Item"),
            "❌ Your code should define a GameItem struct"
        );
    }

    #[test]
    fn test_item_scanner_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct ItemScanner") ||
            analyzer.code.contains("Scanner"),
            "❌ Your code should define an ItemScanner struct"
        );
    }

    #[test]
    fn test_returns_option_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Option<GameItem>") ||
            analyzer.code.contains("Option<Item>") ||
            analyzer.code.contains("-> Option"),
            "❌ Your scan methods should return Option<GameItem>"
        );
    }

    #[test]
    fn test_scan_for_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn scan_for_item"),
            "❌ Your code should have a scan_for_item method"
        );
    }

    #[test]
    fn test_uses_some_none_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_some = analyzer.code.contains("Some(");
        let has_none = analyzer.code.contains("None");
        assert!(
            has_some && has_none,
            "❌ Your code should use Some() and None variants of Option"
        );
    }

    #[test]
    fn test_pattern_matching_option() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let matches_option = analyzer.code.contains("match") &&
                             (analyzer.code.contains("Some(") || analyzer.code.contains("None"));
        assert!(
            matches_option,
            "❌ Your code should use pattern matching with Option (Some/None)"
        );
    }

    #[test]
    fn test_item_collector_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct ItemCollector") ||
            analyzer.code.contains("struct Collector"),
            "❌ Your code should define an ItemCollector struct"
        );
    }

    #[test]
    fn test_collect_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn collect_item"),
            "❌ Your collector should have a collect_item method"
        );
    }

    #[test]
    fn test_find_item_by_name() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn find_item_by_name") ||
            analyzer.code.contains("find") ||
            analyzer.code.contains("get"),
            "❌ Your collector should have a method to find items by name"
        );
    }

    #[test]
    fn test_use_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn use_item"),
            "❌ Your collector should have a use_item method that returns Option"
        );
    }

    #[test]
    fn test_option_chaining() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_chaining = analyzer.code.contains("and_then") ||
                          analyzer.code.contains("unwrap_or") ||
                          analyzer.code.contains("if let Some");
        assert!(
            has_chaining,
            "❌ Your code should demonstrate Option chaining methods"
        );
    }

    #[test]
    fn test_safe_item_access() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let safe_access = analyzer.code.contains("if let Some") ||
                         analyzer.code.contains("match") ||
                         analyzer.code.contains(".unwrap_or");
        assert!(
            safe_access,
            "❌ Your code should safely access Option values without panicking"
        );
    }

    #[test]
    fn test_distance_calculation() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let calculates_distance = analyzer.code.contains("distance") ||
                                 analyzer.code.contains("abs()");
        assert!(
            calculates_distance,
            "❌ Your scanner should calculate distance for range checking"
        );
    }

    #[test]
    fn test_inventory_management() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let manages_inventory = analyzer.code.contains("inventory") &&
                               analyzer.code.contains("Vec<");
        assert!(
            manages_inventory,
            "❌ Your collector should manage an inventory using Vec"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output Option handling information
        let has_option_output = result.stdout.contains("Found") ||
                               result.stdout.contains("item") ||
                               result.stdout.contains("Collected") ||
                               result.stdout.contains("None") ||
                               result.stdout.contains("Some");

        assert!(
            has_option_output,
            "❌ Your program should output information about Option handling"
        );
    }
}

// Reference implementation for comparison
#[derive(Debug, Clone)]
struct GameItem {
    name: String,
    value: u32,
    item_type: String,
}

#[derive(Debug)]
struct ItemScanner {
    position: (i32, i32),
    scan_range: u32,
}

impl ItemScanner {
    fn new(x: i32, y: i32) -> Self {
        ItemScanner {
            position: (x, y),
            scan_range: 2,
        }
    }

    // Simulate scanning for items - returns Option
    fn scan_for_item(&self, search_pos: (i32, i32)) -> Option<GameItem> {
        let distance = ((self.position.0 - search_pos.0).abs() +
                       (self.position.1 - search_pos.1).abs()) as u32;

        if distance <= self.scan_range {
            // Simulate finding different items based on position
            match search_pos {
                (3, 1) => Some(GameItem {
                    name: "Enum Core".to_string(),
                    value: 100,
                    item_type: "Data".to_string(),
                }),
                (12, 3) => Some(GameItem {
                    name: "State Machine".to_string(),
                    value: 150,
                    item_type: "Logic".to_string(),
                }),
                (1, 8) => Some(GameItem {
                    name: "Option Handler".to_string(),
                    value: 75,
                    item_type: "Safety".to_string(),
                }),
                (8, 10) => Some(GameItem {
                    name: "Result Processor".to_string(),
                    value: 125,
                    item_type: "Error".to_string(),
                }),
                _ => None, // No item at this position
            }
        } else {
            None // Out of scan range
        }
    }

    fn find_nearest_item(&self, positions: Vec<(i32, i32)>) -> Option<(GameItem, (i32, i32))> {
        let mut nearest_item = None;
        let mut nearest_distance = u32::MAX;

        for pos in positions {
            if let Some(item) = self.scan_for_item(pos) {
                let distance = ((self.position.0 - pos.0).abs() +
                               (self.position.1 - pos.1).abs()) as u32;

                if distance < nearest_distance {
                    nearest_distance = distance;
                    nearest_item = Some((item, pos));
                }
            }
        }

        nearest_item
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.position = (x, y);
        println!("Scanner moved to ({}, {})", x, y);
    }
}

#[derive(Debug)]
struct ItemCollector {
    inventory: Vec<GameItem>,
    max_capacity: usize,
}

impl ItemCollector {
    fn new(capacity: usize) -> Self {
        ItemCollector {
            inventory: Vec::new(),
            max_capacity: capacity,
        }
    }

    fn collect_item(&mut self, item_option: Option<GameItem>) -> Option<String> {
        match item_option {
            Some(item) => {
                if self.inventory.len() < self.max_capacity {
                    let item_name = item.name.clone();
                    self.inventory.push(item);
                    Some(format!("Collected: {}", item_name))
                } else {
                    Some("Inventory full!".to_string())
                }
            }
            None => Some("No item to collect".to_string()),
        }
    }

    fn find_item_by_name(&self, name: &str) -> Option<&GameItem> {
        self.inventory.iter().find(|item| item.name == name)
    }

    fn use_item(&mut self, name: &str) -> Option<GameItem> {
        if let Some(index) = self.inventory.iter().position(|item| item.name == name) {
            Some(self.inventory.remove(index))
        } else {
            None
        }
    }

    fn get_total_value(&self) -> u32 {
        self.inventory.iter().map(|item| item.value).sum()
    }
}

fn main() {
    println!("=== Option Enum Item Collection System ===");

    let mut scanner = ItemScanner::new(0, 0);
    let mut collector = ItemCollector::new(10);

    // Item positions from level 8
    let item_positions = vec![(3, 1), (12, 3), (1, 8), (8, 10), (5, 5)];

    println!("Starting scan from position {:?}", scanner.position);

    // Scan from starting position
    for pos in &item_positions {
        match scanner.scan_for_item(*pos) {
            Some(item) => {
                println!("Found item at {:?}: {:?}", pos, item);
                if let Some(message) = collector.collect_item(Some(item)) {
                    println!("  {}", message);
                }
            }
            None => {
                println!("No item detected at {:?}", pos);
            }
        }
    }

    println!("\n--- Moving closer to items ---");

    // Move scanner and try again
    scanner.move_to(2, 2);

    // Find nearest item
    match scanner.find_nearest_item(item_positions.clone()) {
        Some((item, position)) => {
            println!("Nearest item: {:?} at {:?}", item, position);
            if let Some(message) = collector.collect_item(Some(item)) {
                println!("  {}", message);
            }
        }
        None => {
            println!("No items in range");
        }
    }

    // Move to collect more items
    for pos in item_positions {
        scanner.move_to(pos.0, pos.1);

        if let Some(item) = scanner.scan_for_item(pos) {
            if let Some(message) = collector.collect_item(Some(item)) {
                println!("  {}", message);
            }
        }
    }

    println!("\n=== Inventory Management ===");
    println!("Items in inventory: {}", collector.inventory.len());
    println!("Total value: {}", collector.get_total_value());

    // Try to find specific items
    match collector.find_item_by_name("Option Handler") {
        Some(item) => println!("Found: {:?}", item),
        None => println!("Option Handler not found"),
    }

    // Use an item
    match collector.use_item("Enum Core") {
        Some(used_item) => println!("Used: {:?}", used_item),
        None => println!("Enum Core not available"),
    }

    println!("Items remaining: {}", collector.inventory.len());

    // Demonstrate Option chaining
    let result = scanner.scan_for_item((1, 8))
        .and_then(|item| collector.collect_item(Some(item)))
        .unwrap_or_else(|| "No item found or collection failed".to_string());

    println!("Chained operation: {}", result);
}