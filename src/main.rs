use std::process::exit;

use controller::Controller;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, Key, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent};

mod controller;
mod game_of_life;

const BORDER: usize = 16;
const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 768;
const PIXEL_TO_CELL_RATIO: usize = 3;
const CELL_SIZE: f64 = PIXEL_TO_CELL_RATIO as f64;
const INITIAL_DATA: &str = "./data/gosper_glider_gun.json";

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("Game of Life", [SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut app = Controller::new(
        GlGraphics::new(opengl),
        (SCREEN_WIDTH + BORDER) / PIXEL_TO_CELL_RATIO,
        (SCREEN_HEIGHT + BORDER) / PIXEL_TO_CELL_RATIO,
        CELL_SIZE,
    );

    if let Err(e) = app.load(INITIAL_DATA) {
        println!("Failed to load file {}: {}", INITIAL_DATA, e);
        exit(1);
    }

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == MouseButton::Left {
                app.dragging = true;
            }
        }

        if let Some(Button::Mouse(button)) = e.release_args() {
            if button == MouseButton::Left {
                app.dragging = false;
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space {
                app.active = !app.active;
            }
        }

        if let Some(mouse_pos) = e.mouse_cursor_args() {
            app.mouse_move(mouse_pos[0], mouse_pos[1]);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
