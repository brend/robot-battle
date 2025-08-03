mod ast;
mod parser;
mod tokenizer;
mod visualize;

const ROBOT_TURN_SPEED: f32 = 0.3;
const ROBOT_MOVE_SPEED: f32 = 0.01;

#[macroquad::main("Robot Battle")]
async fn main() {
    use ast::Robot;

    // Example scripts for two robots
    let script1 = r#"
loop {
    move forward 10
    rotate right 1
}
"#;

    //     let script2 = r#"
    // loop {
    //     move forward 2
    //     fire
    //     scan
    // }
    // "#;

    // Tokenize and parse scripts
    let tokens1 = tokenizer::tokenize_script(script1);
    let ast1 = parser::parse_tokens(&tokens1).unwrap_or_else(|e| {
        println!("Parse error for robot 1: {:?}", e);
        vec![]
    });

    // let tokens2 = tokenizer::tokenize_script(script2);
    // let ast2 = parser::parse_tokens(&tokens2).unwrap_or_else(|e| {
    //     println!("Parse error for robot 2: {:?}", e);
    //     vec![]
    // });

    // Initialize robots with translated instructions, registers, and instruction pointer
    let mut robots = vec![
        Robot {
            id: 1,
            position: (2.0, 2.0),
            heading: 0.0,
            health: 10,
            instruction_queue: ast::translate_commands_to_instructions(&ast1),
            ip: 0,
            registers: std::collections::HashMap::new(),
            command_queue: ast1.clone(),
            busy_ticks: 0,
            current_command: None,
        },
        // Robot {
        //     id: 2,
        //     position: (200.0, 200.0),
        //     heading: 0.0,
        //     health: 10,
        //     instruction_queue: ast::translate_commands_to_instructions(&ast2),
        //     ip: 0,
        //     registers: std::collections::HashMap::new(),
        //     command_queue: ast2.clone(),
        //     busy_ticks: 0,
        //     current_command: None,
        // },
    ];

    println!("Robot 1 commands: {:?}", ast1);
    println!("Robot 1 instructions: {:?}", robots[0].instruction_queue);

    // Simulation loop
    loop {
        let mut damage_events = Vec::new();

        let robots_len = robots.len(); // Avoid multiple mutable borrows

        for i in 0..robots_len {
            let robot = &mut robots[i];

            if robot.health <= 0 {
                continue;
            }

            // Execute one instruction per tick
            execute_robot_instruction(robot);

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
        // for robot in robots.iter() {
        //     println!(
        //         "Robot {}: pos={:?}, health={}, ip={}, registers={:?}",
        //         robot.id, robot.position, robot.health, robot.ip, robot.registers
        //     );
        // }

        visualize::visualize_robots(&robots).await;
    }
}

/// Execute the instruction at the current instruction pointer for a robot.
/// Advances the instruction pointer and updates robot state as needed.
fn execute_robot_instruction(robot: &mut ast::Robot) {
    use ast::Instruction;
    if robot.ip < robot.instruction_queue.len() {
        let instr = &robot.instruction_queue[robot.ip];
        match instr {
            Instruction::MoveForward => {
                println!(
                    "Robot {} moves forward with heading {}",
                    robot.id, robot.heading
                );
                robot.position.0 += ROBOT_MOVE_SPEED * robot.heading.cos();
                robot.position.1 += ROBOT_MOVE_SPEED * robot.heading.sin();
                robot.ip += 1;
            }
            Instruction::TurnLeft => {
                println!("Robot {} turns left", robot.id);
                robot.heading -= ROBOT_TURN_SPEED;
                robot.ip += 1;
            }
            Instruction::TurnRight => {
                println!("Robot {} turns right", robot.id);
                robot.heading += ROBOT_TURN_SPEED;
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
                    if let Some(target) = robot.instruction_queue.iter().position(|i| match i {
                        Instruction::Label(l) => l == label,
                        _ => false,
                    }) {
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
}
