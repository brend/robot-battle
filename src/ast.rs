// AST type definitions for the robot-battle DSL.
// This module defines the core structures for representing parsed robot scripts.

/// State for a robot in the simulation.
#[derive(Debug, Clone)]
pub struct Robot {
    pub id: usize, // Unique identifier for the robot
    pub position: (f32, f32),
    pub heading: f32,
    pub health: i32,
    // Assembly-like execution state:
    pub instruction_queue: Vec<Instruction>, // Instructions to execute
    pub ip: usize,                           // Instruction pointer
    pub registers: std::collections::HashMap<String, i32>, // Counter registers
    // Optionally keep AST for reference or debugging:
    pub command_queue: Vec<Command>, // Commands to execute (from AST)
    pub busy_ticks: u32,             // Ticks remaining for current command
    pub current_command: Option<Command>, // Command being executed
                                     // Add more fields as needed (e.g., ammo, scan results, etc.)
}

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

/// Low-level assembly-like instructions for robot execution.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Turn left by 1 unit.
    TurnLeft,
    /// Turn right by 1 unit.
    TurnRight,
    /// Move forward by 1 unit.
    MoveForward,
    /// Fire weapon.
    Fire,
    /// Load counter register with value.
    LoadCounter { reg: String, value: i32 },
    /// Decrement register.
    Dec { reg: String },
    /// Jump to label if register is not zero.
    Jnz { reg: String, label: String },
    /// Label definition.
    Label(String),
    // Future: Add more instructions as needed.
}

/// A block is a sequence of commands.
pub type Block = Vec<Command>;

/// Translate a high-level Command AST into a sequence of low-level Instructions.
/// For repeated actions (e.g., turn left 90), generates a loop using labels and jumps.
pub fn translate_commands_to_instructions(commands: &[Command]) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut label_count = 0;

    for cmd in commands {
        match cmd {
            Command::Move {
                direction,
                distance,
            } => {
                // Only support "forward" for now; can be extended
                if direction == "forward" {
                    // Use a loop for repeated moves
                    let reg = format!("mv{}", label_count);
                    let label = format!("move_loop{}", label_count);
                    instructions.push(Instruction::LoadCounter {
                        reg: reg.clone(),
                        value: *distance,
                    });
                    instructions.push(Instruction::Label(label.clone()));
                    instructions.push(Instruction::MoveForward);
                    instructions.push(Instruction::Dec { reg: reg.clone() });
                    instructions.push(Instruction::Jnz {
                        reg: reg.clone(),
                        label: label.clone(),
                    });
                    label_count += 1;
                }
                // Extend for other directions if needed
            }
            Command::Rotate { section, angle } => {
                // Only support "left" and "right" for now; section can be ignored
                let reg = format!("rot{}", label_count);
                let label = format!("turn_loop{}", label_count);
                let turns = angle.abs();
                let turn_instr = if *angle > 0 {
                    Instruction::TurnLeft
                } else {
                    Instruction::TurnRight
                };
                instructions.push(Instruction::LoadCounter {
                    reg: reg.clone(),
                    value: turns,
                });
                instructions.push(Instruction::Label(label.clone()));
                instructions.push(turn_instr);
                instructions.push(Instruction::Dec { reg: reg.clone() });
                instructions.push(Instruction::Jnz {
                    reg: reg.clone(),
                    label: label.clone(),
                });
                label_count += 1;
            }
            Command::Scan => {
                // No atomic scan instruction yet; could add if needed
                // For now, ignore or extend as needed
            }
            Command::Fire => {
                instructions.push(Instruction::Fire);
            }
            Command::Loop { block } => {
                // Infinite loop: label at start, jump to start at end
                let label = format!("loop{}", label_count);
                instructions.push(Instruction::Label(label.clone()));
                let inner = translate_commands_to_instructions(block);
                instructions.extend(inner);
                instructions.push(Instruction::Jnz {
                    reg: "always".to_string(),
                    label: label.clone(),
                });
                label_count += 1;
            }
        }
    }
    instructions
}

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
