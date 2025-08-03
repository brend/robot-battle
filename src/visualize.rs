use crate::ast::Robot;
use macroquad::prelude::*;

/// Size of the arena (in logical units)
const ARENA_WIDTH: f32 = 10.0;
const ARENA_HEIGHT: f32 = 10.0;

/// Size of each robot (in logical units)
const ROBOT_WIDTH: f32 = 0.8;
const ROBOT_HEIGHT: f32 = 0.8;

/// Colors for robots
const ROBOT_COLORS: [Color; 2] = [RED, BLUE];

/// Convert logical arena coordinates to screen coordinates
fn to_screen_coords(x: f32, y: f32, screen_w: f32, screen_h: f32) -> (f32, f32) {
    let sx = x / ARENA_WIDTH * screen_w;
    let sy = y / ARENA_HEIGHT * screen_h;
    (sx, sy)
}

/// Draw a single robot as a rectangle (optionally rotated in the future)
fn draw_robot(robot: &Robot, color: Color, screen_w: f32, screen_h: f32) {
    let (x, y) = (robot.position.0 as f32, robot.position.1 as f32);
    let (sx, sy) = to_screen_coords(x, y, screen_w, screen_h);

    let rw = ROBOT_WIDTH / ARENA_WIDTH * screen_w;
    let rh = ROBOT_HEIGHT / ARENA_HEIGHT * screen_h;

    // Center the rectangle on (sx, sy)
    draw_rectangle(sx - rw / 2.0, sy - rh / 2.0, rw, rh, color);

    // Draw robot ID
    let id_text = format!("{}", robot.id);
    draw_text(&id_text, sx - 8.0, sy - 8.0, 24.0, WHITE);
}

/// Visualize the robots in the arena.
/// This function runs a macroquad window and draws the robots in their positions.
pub async fn visualize_robots(robots: &[Robot]) {
    clear_background(DARKGRAY);

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Draw arena border
    draw_rectangle_lines(0.0, 0.0, screen_w, screen_h, 4.0, LIGHTGRAY);

    // Draw all robots
    for (i, robot) in robots.iter().enumerate() {
        let color = ROBOT_COLORS.get(i).copied().unwrap_or(GREEN);
        draw_robot(robot, color, screen_w, screen_h);
    }

    next_frame().await;
}
