use std::process::exit;
use macroquad::input::KeyCode::{Escape, Space};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("game_two"),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

struct GameState {
    angle: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    r: f32,
    active_orange: bool,
    p1x: f32,
    p1y: f32,
    time: f64,
    best_time: f64,
    stop_time: f64,
    points: Vec<(f32, f32, bool)>,
    fps: i64,
}

fn new_points() -> Vec<(f32, f32, bool)> {
    vec![
        (100.0, 600.0, true),
        (600.0, 600.0, true),
        (1100.0, 600.0, true),
        (200.0, 350.0, true),
        (600.0, 350.0, true),
        (1000.0, 350.0, true),
        (600.0, 100.0, true),
        (1100.0, 100.0, true),
    ]
}

fn input(s: &mut GameState) {
    if is_key_pressed(Escape) {
        exit(0);
    }

    if is_key_pressed(Space) {
        s.active_orange = !s.active_orange;
        s.angle += std::f32::consts::PI;
    }
}

fn draw(s: &GameState) {
    clear_background(BLACK);

    draw_circle(s.p1x, s.p1y, 10.0, GREEN);
    for (x, y, b) in &s.points {
        if *b { draw_circle(*x, *y, 10.0, BLUE) }
    }

    // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(s.x1, s.y1, 20.0, YELLOW);
    draw_circle(s.x2, s.y2, 20.0, ORANGE);

    let x = 1000.0;
    draw_text(&(s.best_time).to_string(), x, 20.0, 30.0, DARKGRAY);
    draw_text(&(if s.points.iter().all(|&(_, _, b)| !b) { s.stop_time } else { get_time() - s.time }).to_string(),
              x, 50.0, 30.0, DARKGRAY);
    draw_text(format!("{:?}", (s.x1, s.y1)).as_str(), x, 80.0, 30.0, DARKGRAY);
    draw_text(&s.fps.to_string(), x, 100.0, 30.0, DARKGRAY);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut frame_time = 0.0;
    let mut frames = 0;

    let mut state = GameState {
        angle: 0.0,
        x1: 50.0,
        y1: 50.0,
        x2: 150.0,
        y2: 50.0,
        r: 100.0,
        active_orange: false,
        p1x: 50.0,
        p1y: 50.0,
        time: 0.0,
        best_time: 0.0,
        stop_time: 0.0,
        points: new_points(),
        fps: 0,
    };

    loop {
        input(&mut state);

        // println!("{}", state.angle);
        if frame_time == 0.0 {
            println!("{}", f32::cos(30.0));
            println!("{}", f32::cos(0.0));
            println!("{}", f32::cos(std::f32::consts::FRAC_PI_3));
            println!("{}", f32::cos(std::f32::consts::FRAC_PI_2));
            println!("{}", f32::cos(std::f32::consts::PI));
        }

        let current_time = get_time();

        if current_time - frame_time > 1.0 {
            frame_time = current_time;
            state.fps = frames;
            frames = 0;
        }

        if state.active_orange {
            state.angle += get_frame_time() * 6.0;
            state.x2 = state.x1 + state.r * f32::cos(state.angle);
            state.y2 = state.y1 + state.r * f32::sin(state.angle);
        } else {
            state.angle -= get_frame_time() * 6.0;
            state.x1 = state.x2 + state.r * f32::cos(state.angle);
            state.y1 = state.y2 + state.r * f32::sin(state.angle);
        }

        if state.angle > std::f32::consts::PI * 2.0 {
            state.angle -= std::f32::consts::PI * 2.0
        } else if state.angle < -std::f32::consts::PI * 2.0 {
            state.angle += std::f32::consts::PI * 2.0
        }

        if (state.p1x - state.x1).powi(2) + (state.p1y - state.y1).powi(2) <= 900.0 {
            state.time = current_time;
            state.points = new_points();
        }

        for i in 0..state.points.len() {
            if state.points[i].2 && (state.points[i].0 - state.x1).powi(2) + (state.points[i].1 - state.y1).powi(2) <= 900.0 {
                state.points[i].2 = false;
                if state.points.iter().all(|&(_x, _y, b)| b == false) {
                    state.stop_time = current_time - state.time;
                    state.best_time = if state.best_time == 0.0 { state.stop_time } else { f64::min(state.stop_time, state.best_time) };
                }
            }
        }

        draw(&state);
        frames += 1;
        next_frame().await
    }
}
