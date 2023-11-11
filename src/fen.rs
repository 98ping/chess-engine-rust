use std::collections::HashMap;
use ::{Piece, Tile};
use main;

pub struct Fen {
    pub fen_string: String
}

impl Fen {
    pub(crate) fn interpret(&self) -> Vec<Tile> {
        let mut new_tiles = vec![];
        let piece_hashmap = self.load_hashmap_for_pieces();

        let mut i = 0;
        let mut row = 1;

        for c in self.fen_string.chars() {
            if c.is_digit(10) {
                let spaces = c as i32 - 0x30;

                for _slot in 0..spaces {
                    new_tiles.push(self.create_blank_tile(row, i));

                    i += 1;
                }
            } else if c == '/' {
                row += 1;
                i = 0;
            } else {
                let found_piece = piece_hashmap.get(&c);

                if found_piece.is_none() {
                    new_tiles.push(self.create_blank_tile(row, i));
                } else {
                    new_tiles.push(self.create_pieced_tile(row, i, found_piece.unwrap().clone()))
                }

                i += 1;
            }
        }

        return new_tiles
    }

    fn load_hashmap_for_pieces(&self) -> HashMap<char, Piece> {
        let mut chars_to_piece: HashMap<char, Piece> = HashMap::new();

        chars_to_piece.insert('R', self.create_generic_piece("Rook", 5, false, false));
        chars_to_piece.insert('K', self.create_generic_piece("King", 0, false, false));

        return chars_to_piece
    }

    fn create_blank_tile(&self, row: u32, i: u32) -> Tile {
        return Tile {
            color: if (row + i) % 2 != 0 { graphics::color::hex("ccac95") } else { graphics::color::hex("a67a5a") },
            x1: i * 100,
            y1: (row - 1) * 100,
            x2: (i * 100) + 99,
            y2: ((row - 1) * 100) + 99,
            owning_piece: None,
            board_index: (i * row)
        }
    }

    fn create_pieced_tile(&self, row: u32, i: u32, piece: Piece) -> Tile {
        return Tile {
            color: if (row + i) % 2 != 0 { graphics::color::hex("ccac95") } else { graphics::color::hex("a67a5a") },
            x1: i * 100,
            y1: (row - 1) * 100,
            x2: (i * 100) + 99,
            y2: ((row - 1) * 100) + 99,
            owning_piece: Some(piece),
            board_index: (i * row)
        }
    }

    fn create_generic_piece(&self, name_str: &str, piece_worth: i32, is_white: bool, can_be_captured: bool) -> Piece {
        return Piece {
            worth: piece_worth,
            name: name_str.to_string(),
            capturable: can_be_captured,
            white: is_white,
        }
    }
}