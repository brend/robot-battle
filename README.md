# Robot Battle Simulator

## Overview

**Robot Battle Simulator** is a Rust-based project for simulating battles between programmable robots. Each robot runs its own script written in a custom domain-specific language (DSL), allowing for autonomous movement, scanning, and combat. The simulation advances in discrete ticks, with robots executing commands concurrently and interacting within a shared world.

## Features

- Custom DSL for robot scripting (move, scan, fire, loop, etc.)
- Tick-based simulation: robots act simultaneously, with command durations
- Multiple robots, each with independent scripts and state
- Basic combat and movement mechanics
- Extensible architecture for new commands and features

## DSL Example

```text
loop {
    scan
    move forward 3
    fire
}
```

## Usage

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Build and Run

```sh
cargo run
```

The simulator will execute a demo with two robots, each running its own script. The simulation prints the state of each robot at every tick and ends when only one robot remains alive.

### Project Structure

- `src/ast.rs` — AST definitions and the `Robot` struct
- `src/tokenizer.rs` — Tokenizer for the DSL
- `src/parser.rs` — Parser for converting tokens to AST
- `src/main.rs` — Simulation loop and entry point

## Simulation Model

- **Tick-based:** The world advances in discrete steps ("ticks").
- **Robot State:** Each robot tracks its position, health, command queue, and current activity.
- **Command Execution:** Commands like `move` and `fire` take time to complete. Robots are "busy" while executing long-running commands.
- **Interactions:** Robots can scan, move, and fire at each other. Combat and movement are resolved each tick.

## Extending the Project

- Add new commands to the DSL (e.g., conditional logic, variables)
- Implement more sophisticated combat and movement rules
- Add a graphical or web-based visualization
- Expand the world model (obstacles, power-ups, etc.)

## License

MIT License

---
