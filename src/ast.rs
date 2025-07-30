// AST type definitions for the robot-battle DSL.
// This module defines the core structures for representing parsed robot scripts.

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Move the robot in a direction by a certain distance.
    Move { direction: String, distance: i32 },
    /// Rotate a section (treads, turret, scanner) by an angle.
    Rotate { section: String, angle: i32 },
    /// Scan for enemies.
    Scan,
    /// Fire weapon.
    Fire,
    /// Infinite loop: executes the block repeatedly.
    Loop { block: Vec<Command> },
    // Future extensions:
    // If { condition: Expr, block: Vec<Command>, else_block: Option<Vec<Command>> },
    // Assignment { name: String, expr: Expr },
    // Let { name: String, expr: Expr },
}

/// A block is a sequence of commands.
pub type Block = Vec<Command>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_command() {
        let cmd = Command::Move {
            direction: "forward".to_string(),
            distance: 5,
        };
        assert_eq!(
            cmd,
            Command::Move {
                direction: "forward".to_string(),
                distance: 5
            }
        );
    }

    #[test]
    fn test_rotate_command() {
        let cmd = Command::Rotate {
            section: "turret".to_string(),
            angle: 90,
        };
        assert_eq!(
            cmd,
            Command::Rotate {
                section: "turret".to_string(),
                angle: 90
            }
        );
    }

    #[test]
    fn test_loop_command() {
        let block = vec![Command::Scan, Command::Fire];
        let cmd = Command::Loop {
            block: block.clone(),
        };
        assert_eq!(cmd, Command::Loop { block });
    }
}
