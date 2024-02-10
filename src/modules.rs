use std::collections::HashMap;
use graphics::types::Color;
use opengl_graphics::GlGraphics;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Piece {
    pub worth: i32,
    pub name: String,
    pub capturable: bool,
    pub white: bool
}

pub struct Board {
    pub gl: GlGraphics,
    pub tiles_per_axis: u32,
    pub tile_len: f64,
    pub tiles: HashMap<u32, Tile>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct Tile {
    pub color: Color,
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
    pub owning_piece: Option<Piece>,
    pub board_index: u32,
}

#[derive(Clone)]
pub struct ChessButton {
    pub x1: u64,
    pub y1: u64,
    pub x2: u64,
    pub y2: u64,
    pub name: String
}
