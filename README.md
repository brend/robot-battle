robot-battle/README.md
# Robot Battle Simulator

## Overview

**Robot Battle Simulator** is a Rust-based project for simulating battles between programmable robots. Each robot runs its own script written in a custom domain-specific language (DSL), allowing for autonomous movement, scanning, and combat. The simulation advances in discrete ticks, with robots executing commands concurrently and interacting within a shared world. The project features a real-time graphical visualization using [macroquad](https://github.com/not-fl3/macroquad).

## Features

- Custom DSL for robot scripting (`move`, `scan`, `fire`, `rotate`, `loop`, etc.)
- Tick-based simulation: robots act simultaneously, with command durations
- Multiple robots, each with independent scripts and state
- Basic combat and movement mechanics
- Real-time graphical visualization of the arena and robots
- Extensible architecture for new commands and features

## DSL Example

Below is an example robot script (`robot-scripts/circler.robo`) that makes a robot move in a circle:

```text
# This is a simple robot script
# that makes the robot move in a circle.
loop {
    move forward 2    # Move the robot forward by 2 units
    rotate main 6     # Rotate the robot by 6 degrees counterclockwise
}
```

## Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Build and Run

```sh
cargo run
```

This will launch the simulator with a visualization window. By default, two robots are loaded, each running the same script from `robot-scripts/circler.robo`. The simulation displays the state and position of each robot in real time. The simulation ends when only one robot remains alive.

### Project Structure

- `src/ast.rs` — AST definitions and the `Robot` struct
- `src/tokenizer.rs` — Tokenizer for the DSL
- `src/parser.rs` — Parser for converting tokens to AST
- `src/visualize.rs` — Visualization of the arena and robots using macroquad
- `src/main.rs` — Simulation loop and entry point
- `robot-scripts/` — Example robot scripts (e.g., `circler.robo`)

## Simulation Model

- **Tick-based:** The world advances in discrete steps ("ticks").
- **Robot State:** Each robot tracks its position, heading, health, command queue, and registers.
- **Command Execution:** Commands like `move` and `fire` take time to complete. Robots are "busy" while executing long-running commands.
- **Interactions:** Robots can scan, move, rotate, and fire at each other. Combat and movement are resolved each tick.
- **Visualization:** The arena and robots are rendered in real time, showing positions, headings, and actions.

## Extending the Project

- Add new commands to the DSL (e.g., conditional logic, variables)
- Implement more sophisticated combat and movement rules
- Add more robot scripts to `robot-scripts/`
- Expand the visualization (e.g., display health bars, effects, or a web-based UI)
- Expand the world model (obstacles, power-ups, etc.)

## Dependencies

- [macroquad](https://github.com/not-fl3/macroquad) for visualization

## License

MIT License

---