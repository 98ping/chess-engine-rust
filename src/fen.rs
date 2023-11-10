use graphics::color::{BLACK, WHITE};
use Tile;
use main;

pub struct Fen {
    pub fen_string: String
}

impl Fen {
    fn interpret(self) -> Vec<Tile> {
        let mut new_tiles = vec![];

        let mut i = 0;
        let mut row = 1;
        for c in self.fen_string.chars() {
            if c.is_digit(10) {
                let spaces = c as i32 - 0x30;

                for slot in 0..spaces {
                    new_tiles.push(
                        Tile {
                            color: if i % 2 != 0 { WHITE } else {BLACK},
                            x1: i * 100,
                            y1: (row - 1) * 100,
                            x2: (i * 100) + 99,
                            y2: ((row - 1) * 100) + 99,
                            owning_piece: None
                        }
                    );

                    i += 1;

                    // Check if we have to move rows.
                    // For example:
                    //
                    if i >= 8 * row {
                        row += 1;
                        i = 0;
                    }
                }
            } else {

            }
        }

        return new_tiles

    }
}