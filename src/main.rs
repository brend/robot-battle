mod ast;
mod parser;
mod tokenizer;

fn main() {
    use ast::Robot;

    // Example scripts for two robots
    let script1 = r#"
loop {
    scan
    move forward 3
    fire
}
"#;

    let script2 = r#"
loop {
    move forward 2
    fire
    scan
}
"#;

    // Tokenize and parse scripts
    let tokens1 = tokenizer::tokenize_script(script1);
    let ast1 = parser::parse_tokens(&tokens1).unwrap_or_else(|e| {
        println!("Parse error for robot 1: {:?}", e);
        vec![]
    });

    let tokens2 = tokenizer::tokenize_script(script2);
    let ast2 = parser::parse_tokens(&tokens2).unwrap_or_else(|e| {
        println!("Parse error for robot 2: {:?}", e);
        vec![]
    });

    // Initialize robots with translated instructions, registers, and instruction pointer
    let mut robots = vec![
        Robot {
            id: 1,
            position: (0, 0),
            direction: "forward".to_string(),
            health: 10,
            instruction_queue: ast::translate_commands_to_instructions(&ast1),
            ip: 0,
            registers: std::collections::HashMap::new(),
            command_queue: ast1.clone(),
            busy_ticks: 0,
            current_command: None,
        },
        Robot {
            id: 2,
            position: (5, 5),
            direction: "forward".to_string(),
            health: 10,
            instruction_queue: ast::translate_commands_to_instructions(&ast2),
            ip: 0,
            registers: std::collections::HashMap::new(),
            command_queue: ast2.clone(),
            busy_ticks: 0,
            current_command: None,
        },
    ];

    // Simulation loop
    let max_ticks = 20;
    for tick in 0..max_ticks {
        println!("Tick {}", tick);

        let mut damage_events = Vec::new();

        let robots_len = robots.len(); // Avoid multiple mutable borrows

        for i in 0..robots_len {
            let robot = &mut robots[i];

            if robot.health <= 0 {
                continue;
            }

            // Execute one instruction per tick
            if robot.ip < robot.instruction_queue.len() {
                use ast::Instruction;
                let instr = &robot.instruction_queue[robot.ip];
                match instr {
                    Instruction::MoveForward => {
                        robot.position.1 += 1;
                        robot.ip += 1;
                    }
                    Instruction::TurnLeft => {
                        println!("Robot {} turns left", robot.id);
                        robot.ip += 1;
                    }
                    Instruction::TurnRight => {
                        println!("Robot {} turns right", robot.id);
                        robot.ip += 1;
                    }
                    Instruction::Fire => {
                        println!("Robot {} fires!", robot.id);
                        robot.ip += 1;
                    }
                    Instruction::LoadCounter { reg, value } => {
                        robot.registers.insert(reg.clone(), *value);
                        robot.ip += 1;
                    }
                    Instruction::Dec { reg } => {
                        if let Some(val) = robot.registers.get_mut(reg) {
                            *val -= 1;
                        }
                        robot.ip += 1;
                    }
                    Instruction::Jnz { reg, label } => {
                        let jump = match robot.registers.get(reg) {
                            Some(val) => *val != 0,
                            None => reg == "always",
                        };
                        if jump {
                            if let Some(target) =
                                robot.instruction_queue.iter().position(|i| match i {
                                    Instruction::Label(l) => l == label,
                                    _ => false,
                                })
                            {
                                robot.ip = target;
                            } else {
                                robot.ip += 1;
                            }
                        } else {
                            robot.ip += 1;
                        }
                    }
                    Instruction::Label(_) => {
                        robot.ip += 1;
                    }
                }
            }

            // Interaction: If last instruction was Fire
            if robot.ip > 0
                && robot.instruction_queue.get(robot.ip - 1) == Some(&ast::Instruction::Fire)
            {
                let robot_id = robot.id;
                let robot_pos = robot.position;

                // Search for targets **without borrowing robots again mutably**
                for (j, other) in robots.iter().enumerate() {
                    if i != j && other.health > 0 && other.position == robot_pos {
                        damage_events.push((robot_id, other.id, j, 2));
                    }
                }
            }
        }

        // Apply damage after borrow ends
        for (firing_id, target_id, idx, dmg) in damage_events {
            println!("Robot {} fires at Robot {}!", firing_id, target_id);
            if let Some(robot) = robots.get_mut(idx) {
                robot.health -= dmg;
            }
        }

        // Print robot states
        for robot in robots.iter() {
            println!(
                "Robot {}: pos={:?}, health={}, ip={}, registers={:?}",
                robot.id, robot.position, robot.health, robot.ip, robot.registers
            );
        }

        // End condition: only one robot left alive
        let alive = robots.iter().filter(|r| r.health > 0).count();
        if alive <= 1 {
            println!("Simulation ended at tick {}", tick);
            break;
        }
    }
}
