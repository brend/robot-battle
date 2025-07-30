mod ast;
mod parser;
mod tokenizer;

fn main() {
    use ast::{Command, Robot};

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

    // Initialize robots
    let mut robots = vec![
        Robot {
            id: 1,
            position: (0, 0),
            direction: "forward".to_string(),
            health: 10,
            command_queue: ast1.clone(),
            busy_ticks: 0,
            current_command: None,
        },
        Robot {
            id: 2,
            position: (5, 5),
            direction: "forward".to_string(),
            health: 10,
            command_queue: ast2.clone(),
            busy_ticks: 0,
            current_command: None,
        },
    ];

    // Simulation loop
    let max_ticks = 20;
    for tick in 0..max_ticks {
        println!("Tick {}", tick);

        for robot in robots.iter_mut() {
            if robot.health <= 0 {
                continue;
            }

            if robot.busy_ticks > 0 {
                robot.busy_ticks -= 1;
                // Example: update position if moving
                if let Some(Command::Move {
                    direction,
                    distance,
                }) = &robot.current_command
                {
                    // Move one unit per tick
                    if *distance > 0 {
                        match direction.as_str() {
                            "forward" => robot.position.1 += 1,
                            "backward" => robot.position.1 -= 1,
                            "left" => robot.position.0 -= 1,
                            "right" => robot.position.0 += 1,
                            _ => {}
                        }
                    }
                }
            } else {
                if let Some(command) = robot.command_queue.first().cloned() {
                    match &command {
                        Command::Move {
                            direction: _,
                            distance,
                        } => {
                            robot.busy_ticks = *distance as u32;
                            robot.current_command = Some(command.clone());
                        }
                        Command::Fire => {
                            robot.busy_ticks = 1;
                            robot.current_command = Some(command.clone());
                        }
                        Command::Scan => {
                            robot.busy_ticks = 1;
                            robot.current_command = Some(command.clone());
                        }
                        Command::Rotate {
                            section: _,
                            angle: _,
                        } => {
                            robot.busy_ticks = 1;
                            robot.current_command = Some(command.clone());
                        }
                        Command::Loop { block } => {
                            // Replace command queue with loop block (repeat forever)
                            robot.command_queue = block.clone();
                            robot.current_command = None;
                            continue;
                        }
                    }
                    // Remove the command from the queue
                    robot.command_queue.remove(0);
                } else {
                    robot.current_command = None;
                }
            }
        }

        // Collect interactions to process after borrow ends
        let mut damage_events = vec![];

        // Example interaction: if robots are close and firing, schedule damage
        if robots[0].position == robots[1].position
            && robots[0].current_command == Some(Command::Fire)
            && robots[1].health > 0
        {
            println!("Robot 1 fires at Robot 2!");
            damage_events.push((1, 2));
        }
        if robots[1].position == robots[0].position
            && robots[1].current_command == Some(Command::Fire)
            && robots[0].health > 0
        {
            println!("Robot 2 fires at Robot 1!");
            damage_events.push((0, 2));
        }

        // Apply damage after borrow ends
        for (idx, dmg) in damage_events {
            if let Some(robot) = robots.get_mut(idx) {
                robot.health -= dmg;
            }
        }

        // Print robot states
        for robot in robots.iter() {
            println!(
                "Robot {}: pos={:?}, health={}, busy_ticks={}, current_command={:?}",
                robot.id, robot.position, robot.health, robot.busy_ticks, robot.current_command
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
