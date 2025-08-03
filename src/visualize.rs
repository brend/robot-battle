use crate::ast::Robot;
use macroquad::math::Vec2;
use macroquad::prelude::*;
use macroquad::shapes::{DrawRectangleParams, draw_rectangle_ex};

/// Size of the arena (in logical units)
const ARENA_WIDTH: f32 = 400.0;
const ARENA_HEIGHT: f32 = 400.0;

/// Size of each robot (in logical units)
const ROBOT_WIDTH: f32 = 10.0;
const ROBOT_HEIGHT: f32 = 10.0;

/// Colors for robots
const ROBOT_COLORS: [Color; 2] = [RED, BLUE];

/// Convert logical arena coordinates to screen coordinates
fn to_screen_coords(x: f32, y: f32, screen_w: f32, screen_h: f32) -> (f32, f32) {
    let sx = x / ARENA_WIDTH * screen_w;
    let sy = y / ARENA_HEIGHT * screen_h;
    (sx, sy)
}

/// Draw a single robot as a rectangle, rotated according to its heading
fn draw_robot(robot: &Robot, color: Color, screen_w: f32, screen_h: f32) {
    let (x, y) = (robot.position.0 as f32, robot.position.1 as f32);
    let (sx, sy) = to_screen_coords(x, y, screen_w, screen_h);

    let rw = ROBOT_WIDTH / ARENA_WIDTH * screen_w;
    let rh = ROBOT_HEIGHT / ARENA_HEIGHT * screen_h;

    // Use draw_rectangle_ex to rotate around the center using offset
    draw_rectangle_ex(
        sx - rw / 2.0,
        sy - rh / 2.0,
        rw,
        rh,
        DrawRectangleParams {
            rotation: robot.heading,
            offset: Vec2::new(0.5, 0.5),
            color,
        },
    );

    // Draw robot ID (not rotated)
    let id_text = format!("{}", robot.id);
    draw_text(&id_text, sx - 8.0, sy - 8.0, 24.0, WHITE);
}

/// Visualize the robots in the arena.
/// This function runs a macroquad window and draws the robots in their positions.
pub async fn visualize_robots(robots: &[Robot]) {
    clear_background(BLACK);

    let screen_w = screen_width();
    let screen_h = screen_height();

    // Draw arena border
    draw_rectangle_lines(0.0, 0.0, screen_w, screen_h, 4.0, LIGHTGRAY);

    // Draw all robots
    for (i, robot) in robots.iter().enumerate() {
        let color = ROBOT_COLORS.get(i).copied().unwrap_or(GREEN);
        draw_robot(robot, color, screen_w, screen_h);
    }

    // Draw HUD for each robot
    let mut hud_y = 20.0;
    for (i, robot) in robots.iter().enumerate() {
        let color = ROBOT_COLORS.get(i).copied().unwrap_or(GREEN);
        let hud_text = format!(
            "Robot {:>2} | Pos: ({:>6.1}, {:>6.1}) | Heading: {:>7.2}",
            robot.id, robot.position.0, robot.position.1, robot.heading
        );
        draw_text(&hud_text, 20.0, hud_y, 28.0, color);
        hud_y += 32.0;
    }

    next_frame().await;
}
