// Level 7 Task 3 Test: Inventory Management System
// Tests if the user code creates inventory management with structs and collections

#[cfg(test)]
mod level7_task3_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_item_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Item"),
            "❌ Your code should define an Item struct"
        );
    }

    #[test]
    fn test_has_item_type_enum() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("enum ItemType"),
            "❌ Your code should define an ItemType enum"
        );
    }

    #[test]
    fn test_has_inventory_struct() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("struct Inventory"),
            "❌ Your code should define an Inventory struct"
        );
    }

    #[test]
    fn test_item_has_required_fields() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_name = analyzer.code.contains("name: String") || analyzer.code.contains("name:String");
        let has_item_type = analyzer.code.contains("item_type: ItemType") || analyzer.code.contains("item_type:ItemType");
        let has_value = analyzer.code.contains("value: u32") || analyzer.code.contains("value:u32");
        assert!(
            has_name && has_item_type && has_value,
            "❌ Your Item struct should have name (String), item_type (ItemType), and value (u32) fields"
        );
    }

    #[test]
    fn test_inventory_has_vec_items() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_items_vec = analyzer.code.contains("items: Vec<Item>") ||
                           analyzer.code.contains("items:Vec<Item>");
        assert!(
            has_items_vec,
            "❌ Your Inventory struct should have an items field of type Vec<Item>"
        );
    }

    #[test]
    fn test_inventory_has_capacity() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_capacity = analyzer.code.contains("max_capacity: usize") ||
                          analyzer.code.contains("capacity: usize");
        assert!(
            has_capacity,
            "❌ Your Inventory struct should have a capacity field"
        );
    }

    #[test]
    fn test_has_inventory_impl() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("impl Inventory"),
            "❌ Your code should have an impl block for Inventory"
        );
    }

    #[test]
    fn test_has_add_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn add_item"),
            "❌ Your Inventory should have an add_item() method"
        );
    }

    #[test]
    fn test_has_use_item_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn use_item"),
            "❌ Your Inventory should have a use_item() method"
        );
    }

    #[test]
    fn test_has_count_by_type_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("fn count_by_type"),
            "❌ Your Inventory should have a count_by_type() method"
        );
    }

    #[test]
    fn test_has_robot_with_inventory() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("RobotWithInventory") || analyzer.code.contains("struct Robot"),
            "❌ Your code should define a robot struct that includes inventory"
        );
    }

    #[test]
    fn test_uses_option_return_type() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.code.contains("Option<") || analyzer.code.contains("-> Option"),
            "❌ Your use_item method should return Option<Item> for safe item retrieval"
        );
    }

    #[test]
    fn test_enum_variants() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_energy = analyzer.code.contains("Energy");
        let has_health = analyzer.code.contains("Health");
        let has_key = analyzer.code.contains("Key");
        assert!(
            has_energy && has_health && has_key,
            "❌ Your ItemType enum should have Energy, Health, and Key variants"
        );
    }

    #[test]
    fn test_inventory_capacity_checking() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let checks_capacity = analyzer.code.contains("max_capacity") &&
                             (analyzer.code.contains("items.len()") || analyzer.code.contains("len()"));
        assert!(
            checks_capacity,
            "❌ Your add_item method should check inventory capacity"
        );
    }

    #[test]
    fn test_code_compiles_and_runs() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let result = analyzer.execute_and_capture_output()
            .expect("❌ Your code should compile and run successfully");

        assert_eq!(result.exit_code, 0, "❌ Your program should exit successfully");

        // Should output inventory information
        let has_inventory_output = result.stdout.contains("inventory") ||
                                  result.stdout.contains("item") ||
                                  result.stdout.contains("Added") ||
                                  result.stdout.contains("collected");

        assert!(
            has_inventory_output,
            "❌ Your program should output information about inventory management"
        );
    }
}

// Reference implementation for comparison
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

    fn use_item(&mut self, name: &str) -> Option<Item> {
        if let Some(index) = self.items.iter().position(|item| item.name == name) {
            let item = self.items.remove(index);
            println!("Used item: {}", item.name);
            Some(item)
        } else {
            println!("Item '{}' not found in inventory", name);
            None
        }
    }

    fn count_by_type(&self, item_type: &ItemType) -> usize {
        self.items.iter().filter(|item| &item.item_type == item_type).count()
    }

    fn total_value(&self) -> u32 {
        self.items.iter().map(|item| item.value).sum()
    }

    fn list_items(&self) {
        println!("=== Inventory ({}/{}) ===", self.items.len(), self.max_capacity);
        for (i, item) in self.items.iter().enumerate() {
            println!("{}: {} ({:?}) - Value: {}", i + 1, item.name, item.item_type, item.value);
        }
    }
}

fn main() {
    let mut inventory = Inventory::new(10);

    // Simulate collecting items from level
    let level_items = vec![
        Item { name: "Energy Cell".to_string(), item_type: ItemType::Energy, value: 20 },
        Item { name: "Door Key".to_string(), item_type: ItemType::Key, value: 1 },
        Item { name: "Health Pack".to_string(), item_type: ItemType::Health, value: 30 },
        Item { name: "Struct Blueprint".to_string(), item_type: ItemType::Data, value: 100 },
    ];

    for item in level_items {
        inventory.add_item(item);
    }

    inventory.list_items();

    // Use items for level navigation
    println!("Energy items: {}", inventory.count_by_type(&ItemType::Energy));
    println!("Total inventory value: {}", inventory.total_value());

    // Use key to open door
    if inventory.use_item("Door Key").is_some() {
        println!("Door opened with key!");
    }

    inventory.list_items();
}