mod r#move;
mod fen;
mod timer;
mod modules;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate rand;

use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::Mul;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use graphics::{DrawState, Ellipse, Image, Transformed};
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL, Texture, TextureSettings};
use fen::Fen;
use std::path::Path;
use std::time::Instant;
use modules::*;
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
        fen_string: "RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbkqbnr".to_string()
    };

    let mut events = Events::new(EventSettings::new())
        .ups(30);

    let mut started: i32 = 0;
    let mut glyphs = GlyphCache::new("bin/views/FiraSans-Regular.ttf", (), TextureSettings::new())
        .expect("Could not load font");

    let mut timer_handler = Timer {
        white_turn: false,
        started: false,
        white_time: 180,
        black_time: 180,
        last_tick: Instant::now()
    };

    let mut chess_buttons: Vec<ChessButton> = vec![];

    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        // Start the timer if the game has started
        if timer_handler.started {
            if Instant::now().duration_since(timer_handler.last_tick).as_secs() >= 1 {
                timer_handler.last_tick = Instant::now();

                if timer_handler.white_turn {
                    timer_handler.white_time -= 1;
                } else {
                    timer_handler.black_time -= 1;
                }
            }
        }

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
                board.update(&r);

                for update in move_handler.move_circle_tiles.clone() {
                    update.render_move_circle(
                        &mut board.gl,
                        &r
                    );
                }
            }
        }
    }
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
    }

    fn map_textures_to_pieces(&self) -> HashMap<String, Texture> {
        let mut map: HashMap<String, Texture> = HashMap::new();

        // Black pieces
        map.insert(String::from("BlackKing"), Texture::from_path(
            Path::new("bin/assets/black/black_king.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackRook"), Texture::from_path(
            Path::new("bin/assets/black/black_rook.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackBishop"), Texture::from_path(
            Path::new("bin/assets/black/black_bishop.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackPawn"), Texture::from_path(
            Path::new("bin/assets/black/black_pawn.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackKnight"), Texture::from_path(
            Path::new("bin/assets/black/black_knight.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("BlackQueen"), Texture::from_path(
            Path::new("bin/assets/black/black_queen.png"),
            &TextureSettings::new(),
        ).unwrap());

        // White pieces
        map.insert(String::from("WhiteKing"), Texture::from_path(
            Path::new("bin/assets/white/white_king.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("WhiteRook"), Texture::from_path(
            Path::new("bin/assets/white/white_rook.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("WhiteBishop"), Texture::from_path(
            Path::new("bin/assets/white/white_bishop.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("WhitePawn"), Texture::from_path(
            Path::new("bin/assets/white/white_pawn.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("WhiteKnight"), Texture::from_path(
            Path::new("bin/assets/white/white_knight.png"),
            &TextureSettings::new(),
        ).unwrap());

        map.insert(String::from("WhiteQueen"), Texture::from_path(
            Path::new("bin/assets/white/white_queen.png"),
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
    
    fn get_horizontal_moves(&self, current_index: u32) -> Vec<Tile> {
        let mut moves: Vec<Tile> = vec![];

        for i in 0..9 {
            // Max row position to the left
            let max_row_position = Wrapping(8) * Wrapping(i);

            // Max row position to the right
            let min_index: Wrapping<u32> = max_row_position - Wrapping(1);
            let min_row_position = Wrapping(min_index.0);

            let position_right = Wrapping(current_index) + Wrapping(i);
            let position_left = Wrapping(current_index) - Wrapping(i);

            let optional_horizontal_tile_downwards = self.get_tile_based_on_index(position_left.0);
            

            if optional_horizontal_tile_downwards.is_some() {
                if position_left.0 < min_row_position.0 {
                    continue;
                }

                moves.push(optional_horizontal_tile_downwards.unwrap());
            }

            let optional_horizontal_tile_upwards = self.get_tile_based_on_index(position_right.0);

            if optional_horizontal_tile_upwards.is_some() {
                if position_right.0 > max_row_position.0 {
                    continue;
                }

                moves.push(optional_horizontal_tile_upwards.unwrap());
            }
        }

        return moves;
    }

    fn get_vertical_moves(&self, current_index: u32) -> Vec<Tile> {
        let mut moves: Vec<Tile> = vec![];

        for i in 0..9 {
            let addition = Wrapping(8 * i);

            let position_upwards = Wrapping(current_index) + addition;
            let position_downwards = Wrapping(current_index) - addition;

            let optional_vertical_tile_downwards = self.get_tile_based_on_index(position_downwards.0);
            

            if optional_vertical_tile_downwards.is_some() {
                moves.push(optional_vertical_tile_downwards.unwrap());
            }

            let optional_vertical_tile_upwards = self.get_tile_based_on_index(position_upwards.0);

            if optional_vertical_tile_upwards.is_some() {
                moves.push(optional_vertical_tile_upwards.unwrap());
            }
        }

        return moves;
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

                // Pawn Logic
                if name == "Pawn" {
                    if !unwrapped_piece.white {
                        // First tile for pawn
                        let first_tile_down_i = unwrapped_tile.board_index + 16;
                        let to_render = board.get_tile_based_on_index(first_tile_down_i);

                        if to_render.is_some() {
                            // Only start adding stuff if the owning piece is none.
                            if to_render.clone().unwrap().owning_piece.is_none() {
                                moves.push(to_render.unwrap());
                            }
                        }

                        // Second tile if it is the first move
                        let second_tile_down_i = unwrapped_tile.board_index + 8;
                        let second_tile = board.get_tile_based_on_index(second_tile_down_i);

                        if second_tile.is_some() {
                            // Only start adding stuff if the owning piece is none.
                            if second_tile.clone().unwrap().owning_piece.is_none() {
                                moves.push(second_tile.unwrap());
                            }
                        }
                    } else {
                        // First tile for pawn
                        let first_tile_down_i = unwrapped_tile.board_index - 16;
                        let to_render = board.get_tile_based_on_index(first_tile_down_i);

                        if to_render.is_some() {
                            // Only start adding stuff if the owning piece is none.
                            if to_render.clone().unwrap().owning_piece.is_none() {
                                moves.push(to_render.unwrap());
                            }
                        }

                        // Second tile if it is the first move
                        let second_tile_down_i = unwrapped_tile.board_index - 8;
                        let second_tile = board.get_tile_based_on_index(second_tile_down_i);

                        if second_tile.is_some() {
                            // Only start adding stuff if the owning piece is none.
                            if second_tile.clone().unwrap().owning_piece.is_none() {
                                moves.push(second_tile.unwrap());
                            }
                        }
                    }
                }

                // Bishop Logic
                if name == "Bishop" {
                    if !unwrapped_piece.white {
                        for i in 0..9 {
                            let position = Wrapping((8 * i) + (1 * i));
                            let position_backwards = Wrapping((8 * i) + (1 * i));

                            let diagonal = Wrapping(unwrapped_tile.board_index) + position;
                            let backwards_diagonal = Wrapping(unwrapped_tile.board_index) - position_backwards;
                            let max_distance_allowed = 8 - (unwrapped_tile.x1 / 100);

                            if i > max_distance_allowed {
                                continue;
                            }


                            if diagonal.0 > 63 {
                                continue;
                            }


                            let optional_diag_tile = board.get_tile_based_on_index(diagonal.0);

                            if optional_diag_tile.is_some() {
                                moves.push(optional_diag_tile.unwrap());
                            }

                            if backwards_diagonal.0 > 63 {
                                continue;
                            }

                            let optional_backwards_diag_tile = board.get_tile_based_on_index(backwards_diagonal.0);

                            if optional_backwards_diag_tile.is_some() {
                                moves.push(optional_backwards_diag_tile.unwrap())
                            }
                        }
                    } else {

                    }
                }

                // Logic for Rooks
                if name == "Rook" {
                    let vertical_moves = board.get_vertical_moves(current_tile_index);

                    for tile in vertical_moves {
                        moves.push(tile);
                    }

                    let horizontal_moves = board.get_horizontal_moves(current_tile_index);

                    for tile in horizontal_moves {
                        moves.push(tile);
                    }
                }
            }
        }

        return moves;
    }
}

impl ChessButton {
    pub fn is_inside(&self, cx: u64, cy: u64) -> bool {
        let x1 = self.x1;
        let y1 = self.y1;

        let x2 = self.x2;
        let y2 = self.y2;

        return (cx > x1 && cx < x2) && (cy > y1 && cy < y2);
    }
}