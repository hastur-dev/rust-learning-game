// Level 21 Task 1 Test: Arithmetic Operator Traits
// Tests that user implements basic arithmetic operators for custom geometric types

#[cfg(test)]
mod level21_task1_tests {
    use super::super::test_utils::*;

    #[test]
    fn test_has_custom_types() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_types = (analyzer.code.contains("struct Point2D") ||
                        analyzer.code.contains("struct Vector2D")) &&
                       (analyzer.code.contains("x:") && analyzer.code.contains("y:"));
        assert!(
            has_types,
            "❌ You need to define Point2D and/or Vector2D structs with x, y fields"
        );
    }

    #[test]
    fn test_implements_add_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_add = analyzer.code.contains("impl Add") ||
                     analyzer.code.contains("impl std::ops::Add");
        assert!(
            has_add,
            "❌ You need to implement Add trait for + operator"
        );
    }

    #[test]
    fn test_implements_sub_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_sub = analyzer.code.contains("impl Sub") ||
                     analyzer.code.contains("impl std::ops::Sub");
        assert!(
            has_sub,
            "❌ You need to implement Sub trait for - operator"
        );
    }

    #[test]
    fn test_implements_mul_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_mul = analyzer.code.contains("impl Mul") ||
                     analyzer.code.contains("impl std::ops::Mul");
        assert!(
            has_mul,
            "❌ You need to implement Mul trait for * operator"
        );
    }

    #[test]
    fn test_implements_div_trait() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_div = analyzer.code.contains("impl Div") ||
                     analyzer.code.contains("impl std::ops::Div");
        assert!(
            has_div,
            "❌ You need to implement Div trait for / operator"
        );
    }

    #[test]
    fn test_has_add_method() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_add_fn = analyzer.code.contains("fn add(") &&
                        analyzer.code.contains("Output");
        assert!(
            has_add_fn,
            "❌ Add trait should have fn add() method with Output type"
        );
    }

    #[test]
    fn test_handles_vector_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_vector_ops = analyzer.code.contains("position") ||
                           analyzer.code.contains("velocity") ||
                           analyzer.code.contains("movement");
        assert!(
            has_vector_ops,
            "❌ You should implement vector operations for position/velocity calculations"
        );
    }

    #[test]
    fn test_handles_scalar_operations() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_scalar = analyzer.code.contains("scalar") ||
                        analyzer.code.contains("f32") ||
                        analyzer.code.contains("f64") ||
                        analyzer.code.contains("time");
        assert!(
            has_scalar,
            "❌ You should implement scalar multiplication for time/scaling operations"
        );
    }

    #[test]
    fn test_demonstrates_arithmetic() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        let has_demo = analyzer.code.contains("+") && analyzer.code.contains("-") &&
                      analyzer.code.contains("*") && analyzer.code.contains("/");
        assert!(
            has_demo,
            "❌ You should demonstrate all arithmetic operations (+, -, *, /)"
        );
    }

    #[test]
    fn test_shows_geometric_usage() {
        let analyzer = create_analyzer().expect("Failed to load user code");
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate geometric calculations with output"
        );
    }
}

// Reference implementation
fn main() {
    println!("Level 21 Task 1: Arithmetic Operator Traits");
    // Reference pattern for arithmetic trait implementation
}

// Reference arithmetic traits pattern
// use std::ops::{Add, Sub, Mul, Div};
//
// #[derive(Debug, Clone, Copy)]
// struct Point2D {
//     x: f32,
//     y: f32,
// }
//
// #[derive(Debug, Clone, Copy)]
// struct Vector2D {
//     dx: f32,
//     dy: f32,
// }
//
// impl Add<Vector2D> for Point2D {
//     type Output = Point2D;
//
//     fn add(self, velocity: Vector2D) -> Point2D {
//         Point2D {
//             x: self.x + velocity.dx,
//             y: self.y + velocity.dy,
//         }
//     }
// }
//
// impl Sub for Point2D {
//     type Output = Vector2D;
//
//     fn sub(self, other: Point2D) -> Vector2D {
//         Vector2D {
//             dx: self.x - other.x,
//             dy: self.y - other.y,
//         }
//     }
// }
//
// impl Mul<f32> for Vector2D {
//     type Output = Vector2D;
//
//     fn mul(self, scalar: f32) -> Vector2D {
//         Vector2D {
//             dx: self.dx * scalar,
//             dy: self.dy * scalar,
//         }
//     }
// }
//
// fn demonstrate_geometric_operations() {
//     let current = Point2D { x: 15.5, y: 23.7 };
//     let velocity = Vector2D { dx: 2.1, dy: -1.3 };
//     let time = 2.0;
//
//     let displacement = velocity * time;
//     let new_position = current + displacement;
//
//     println!("New position: {:?}", new_position);
// }