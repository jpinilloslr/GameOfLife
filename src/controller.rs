use crate::game_of_life::GameOfLife;
use anyhow::Result;
use graphics::{clear, rectangle, Transformed};
use opengl_graphics::GlGraphics;
use piston::{RenderArgs, UpdateArgs};

pub struct Controller {
    cell_size: f64,
    pub active: bool,
    pub dragging: bool,
    graphics: GlGraphics,
    game_of_life: GameOfLife,
}

impl Controller {
    pub fn new(gl: GlGraphics, max_x: usize, max_y: usize, cell_size: f64) -> Self {
        Self {
            graphics: gl,
            cell_size,
            active: true,
            dragging: false,
            game_of_life: GameOfLife::new(max_x, max_y),
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        self.game_of_life.load(filename)
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let square = rectangle::square(0.0, 0.0, self.cell_size);

        self.graphics.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            for cell in self.game_of_life.cells() {
                if cell.alive {
                    let transform = c.transform.trans(
                        cell.x as f64 * self.cell_size,
                        cell.y as f64 * self.cell_size,
                    );
                    rectangle([1.0, 1.0, 1.0, 1.0], square, transform, gl);
                }
            }
        });
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        if self.active {
            self.game_of_life.step();
        }
    }

    pub fn mouse_move(&mut self, x: f64, y: f64) {
        if !self.dragging {
            return;
        }
        let cell_x = (x / self.cell_size) as usize;
        let cell_y = (y / self.cell_size) as usize;
        self.game_of_life.set_alive(cell_x, cell_y);
    }
}
