mod r#move;
mod fen;
mod timer;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use std::collections::HashMap;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use graphics::{DrawState, Ellipse, Image, Text, text, Transformed};
use graphics::types::{Color, Rectangle};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use fen::Fen;
use std::path::Path;
use r#move::MoveHandler;
use timer::Timer;


const TILE_AXIS_PIXELS: f64 = 100.0;

fn main() {
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Chess Engine Board", [1000, 800])
        .opengl(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut board = Board {
        gl: GlGraphics::new(opengl),
        tiles_per_axis: 8,
        tile_len: TILE_AXIS_PIXELS,
        tiles: HashMap::new(),
    };

    let mut move_handler = MoveHandler::new();

    let fen_manager = Fen {
        fen_string: "RNBQKBNR/PPPPPPPP/8/8/8/8/8/8".to_string()
    };

    let mut events = Events::new(EventSettings::new())
        .ups(2);

    let mut started: i32 = 0;
    let mut glyphs = GlyphCache::new("bin/views/FiraSans-Regular.ttf", (), TextureSettings::new())
        .expect("Could not load font");

    let timer_handler = Timer { };

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        move_handler.event(
            800.0,
            &e,
            &mut board,
            e.render_args().get_or_insert(RenderArgs {
                ext_dt: 0.1,
                width: 800,
                height: 800,
                draw_width: 800,
                draw_height: 800,
            })
        );

        if let Some(r) = e.render_args() {
            timer_handler.draw_timers(&mut board.gl, &r, &mut glyphs);

            if started == 0 {
                board.render_fen(&r, &fen_manager);
                let tile = move_handler.get_tile_from_position("a1", &board);
                if tile.is_some() {
                    println!("Tile was some: {}", tile.clone().unwrap().board_index)
                }

                let string_pos = move_handler.get_position_from_tile(&tile.clone().unwrap());

                if string_pos.is_none() {
                    println!("String position is none");
                }

                let transformed = move_handler.get_position_from_transformations("a1", 1, 1);

                if transformed.is_none() {
                    println!("Was none");
                } else {
                    let unwrapped = transformed.unwrap();

                    println!("New pos: {}", unwrapped);
                }

                println!("String pos: {}", string_pos.unwrap());

                started = 1;
            } else {
                board.update(&r)
            }
        }
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Piece {
    worth: i32,
    name: String,
    capturable: bool,
    white: bool
}

pub struct Board {
    gl: GlGraphics,
    tiles_per_axis: u32,
    tile_len: f64,
    tiles: HashMap<u32, Tile>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Tile {
    color: Color,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    owning_piece: Option<Piece>,
    board_index: u32,
}

impl Tile {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let square = graphics::rectangle::square(self.x1 as f64, self.y1 as f64, 100.0);
        let resource_map = self.map_textures_to_pieces();

        gl.draw(
            args.viewport(),
            |c, gl| {
                graphics::rectangle(
                    self.color,
                    square,
                    c.transform,
                    gl,
                );
            },
        );

        if self.owning_piece.is_some() {
            let piece = self.owning_piece.clone().unwrap();

            // Kind of cancerous lol.
            // Way it has to be done though
            let image = resource_map.get(
                &(if piece.white { "White" } else { "Black" }.to_owned() + &piece.name)
            );

            if image.is_none() {
                return;
            }

            let unwrapped_image = image.unwrap();

            gl.draw(
                args.viewport(),
                |c, gl| {
                    Image::new()
                        .rect([(self.x1 + 10) as f64, (self.y1 + 10) as f64, 80.0, 80.0])
                        .draw(unwrapped_image,
                              &DrawState::default(),
                              c.transform,
                              gl);
                },
            );
        }
    }

    fn render_move_circle(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let x: u32 = self.x1;
        let y: u32 = self.y1;

        println!("Rendering move circle");
        gl.draw(
            args.viewport(),
            |c, gl| {
                Ellipse::new([0.0, 0.0, 0.0, 1.0])
                    .draw(
                        [(x + 35) as f64, (y + 35) as f64, 30.0, 30.0],
                        &DrawState::default(),
                        c.transform,
                        gl
                    );
            },
        );
        println!("Rendering at: {}, {}", x, y);
    }

    fn map_textures_to_pieces(&self) -> HashMap<String, Texture> {
        let mut map: HashMap<String, Texture> = HashMap::new();

        map.insert(String::from("BlackKing"), Texture::from_path(
            Path::new("bin/assets/black_king.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackRook"), Texture::from_path(
            Path::new("bin/assets/black_rook.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackBishop"), Texture::from_path(
            Path::new("bin/assets/black_bishop.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackPawn"), Texture::from_path(
            Path::new("bin/assets/black_pawn.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackKnight"), Texture::from_path(
            Path::new("bin/assets/black_knight.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackQueen"), Texture::from_path(
            Path::new("bin/assets/black_queen.png"),
            &TextureSettings::new(),
        ).unwrap());

        return map;
    }

    fn contained_inside(&self, cx: u32, cy: u32) -> bool {
        let x1 = self.x1;
        let y1 = self.y1;

        let x2 = self.x2;
        let y2 = self.y2;

        return (cx > x1 && cx < x2) && (cy > y1 && cy < y2);
    }
}

impl Board {
    fn render_fen(&mut self, args: &RenderArgs, fen: &Fen) {
        use graphics;

        for mut tile in fen.interpret() {
            tile.render(&mut self.gl, args);
            self.tiles.insert(tile.board_index, tile);
        }
    }

    fn update(&mut self, args: &RenderArgs) {
        use graphics;

        for tile in self.tiles.values_mut() {
            tile.render(&mut self.gl, args);
        }
    }

    fn get_tile_based_on_piece(&self, piece: &Piece) -> Option<&Tile> {
        for tile in self.tiles.values() {
            if tile.owning_piece.is_some() && tile.owning_piece.clone().unwrap() == *piece {
                return Some(tile);
            }
        }

        return None;
    }

    fn get_tile_based_on_index(&self, index: u32) -> Option<Tile> {
        for tile in self.tiles.values() {
            if tile.board_index == index {
                return Some(tile.clone());
            }
        }

        return None;
    }

    fn render_no_fen(&mut self, args: &RenderArgs) {
        use graphics;

        // Our custom colors b/c they look pretty
        let white = graphics::color::hex("ccac95");
        let black = graphics::color::hex("a67a5a");

        // Track board position (0-63)
        let mut board_pos = 0;

        for file in 0..8 {
            for rank in 0..8 {
                let is_light: bool = (file + rank) % 2 != 0;

                let mut tile = Tile {
                    color: if is_light { white } else { black },
                    x1: rank * 100,
                    y1: file * 100,
                    x2: (rank * 100) + 99,
                    y2: (file * 100) + 99,
                    owning_piece: None,
                    board_index: board_pos,
                };

                board_pos += 1;
                tile.render(&mut self.gl, args);

                self.tiles.insert(tile.board_index, tile);
            }
        }
    }
}

impl Piece {
    fn get_move_tiles(&self, board: &Board, current_tile_index: u32) -> Vec<Tile> {
        let mut moves: Vec<Tile> = vec![];
        let current_tile = board.get_tile_based_on_index(current_tile_index);

        if current_tile.is_some() {
            let piece = current_tile.clone().unwrap().owning_piece;

            if piece.is_some() {
                let unwrapped_piece = piece.unwrap();
                let unwrapped_tile = current_tile.clone().unwrap();
                let name = unwrapped_piece.name;

                if name == "Pawn" && !unwrapped_piece.white {
                    let first_tile_down_i = unwrapped_tile.board_index + 16;
                    let to_render = board.get_tile_based_on_index(first_tile_down_i);

                    if to_render.is_some() {
                        moves.push(to_render.unwrap());
                    }

                    let second_tile_down_i = unwrapped_tile.board_index + 8;
                    let second_tile = board.get_tile_based_on_index(second_tile_down_i);

                    if second_tile.is_some() {
                        moves.push(second_tile.unwrap());
                    }
                }
            }
        }

        return moves;
    }
}
