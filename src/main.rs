mod r#move;
mod fen;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use r#move::MoveHandler;


const TILE_AXIS_PIXELS: f64 = 100.0;
const AXIS_LEN: u32 = 800;

fn main() {
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Chess Engine Board", [AXIS_LEN, AXIS_LEN])
        .opengl(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut board = Board {
        gl: GlGraphics::new(opengl),
        tiles_per_axis: 8,
        tile_len: TILE_AXIS_PIXELS,
        tiles: vec![],
    };

    let mut move_handler = MoveHandler::new();

    let mut events = Events::new(EventSettings::new())
        .ups(2);

    while let Some(e) = events.next(&mut window) {
        move_handler.event(
            800.0,
            &e,
            &board.tiles.to_vec()
        );

        if let Some(r) = e.render_args() {
            board.render(&r);
        }
    }
}

pub struct Piece {
    worth: i32,
    name: String,
    capturable: bool
}

pub struct Board {
    gl: GlGraphics,
    tiles_per_axis: u32,
    tile_len: f64,
    tiles: Vec<Tile>
}

#[derive(Clone)]
pub struct Tile {
    color: [f32; 4],
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    owning_piece: Option<Piece>
}

impl Tile {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let square = graphics::rectangle::square(self.x1 as f64, self.y1 as f64, 100.0);

        gl.draw(
            args.viewport(),
            |c, gl| {
                graphics::rectangle(
                    self.color,
                    square,
                    c.transform,
                    gl
                );
            }
        );
    }

    fn contained_inside(&self, cx: u32, cy: u32) -> bool {
        let x1 = self.x1;
        let y1 = self.y1;

        let x2 = self.x2;
        let y2 = self.y2;

        return (cx > x1 && cx < x2) && (cy > y1 && cy < y2)
    }
}

impl Board {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        for file in 0..8 {
            for rank in 0..8 {
                let is_light: bool = (file + rank) % 2 != 0;

                let mut tile = Tile {
                    color: if is_light { WHITE } else {BLACK},
                    x1: rank * 100,
                    y1: file * 100,
                    x2: (rank * 100) + 99,
                    y2: (file * 100) + 99,
                    owning_piece: None
                };

                tile.render(&mut self.gl, args);
                self.add_tile(tile);
            }
        }
    }
    
    fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile)
    }
}
