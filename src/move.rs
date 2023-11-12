use piston::input::GenericEvent;
use ::{Tile};
use Board;

pub struct MoveHandler {
    pub selected_cell: Option<Tile>,
    pub cursor_pos: [f64; 2],
}

impl MoveHandler {

    pub fn get_tile_from_position(&self, position: &str, board: &Board) -> Option<Tile> {
        let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
        let mut rank: u32 = 0;
        let mut file: u32 = 0;

        // Alphabetized character
        let first_character: char = position.chars().nth(0).unwrap();

        for (i, c) in alphabet.char_indices() {
            if c == first_character {
                rank = i as u32
            }
        }

        // File index
        let second_character: char = position.chars().nth(1).unwrap();

        if second_character.is_digit(10) {
            let parsed_integer = second_character as i32 - 0x30;

            if parsed_integer < 10 {
                file = parsed_integer as u32;
            }
        }

        let optional_tile = board.tiles.get(&(rank + file)).cloned();

        if optional_tile.is_some() {
            println!("Optional tile was some")
        }

        return optional_tile

    }

    pub fn new() -> MoveHandler {
        MoveHandler {
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub(crate) fn event<E: GenericEvent>(&mut self, size: f64, e: &E, tiles: &Vec<Tile>, board: &Board) {
        use piston::input::{Button, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0];
            let y = self.cursor_pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= size && y >= 0.0 && y <= size {
                // Compute the tile position.
                for mut tile in tiles.iter() {
                    if tile.contained_inside(x as u32, y as u32) {
                        self.selected_cell = Option::from(tile.clone());

                        let cloned_cell = self.selected_cell.clone();

                        // Ensure optional for cell is validated
                        if cloned_cell.is_some() {
                            let piece = cloned_cell.clone().unwrap().owning_piece.clone();

                            // Valid piece in cell
                            if piece.is_none() {
                                return
                            }

                            let moves = piece.unwrap().get_move_tiles(board);

                            println!("Got to moves part but didn't really do anything with it")
                        }
                    }
                }
            }
        }
    }
}