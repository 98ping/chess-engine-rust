use piston::input::GenericEvent;
use ::{Tile};

pub struct MoveHandler {
    pub selected_cell: Option<Tile>,
    pub cursor_pos: [f64; 2],
}

impl MoveHandler {

    pub fn new() -> MoveHandler {
        MoveHandler {
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub(crate) fn event<E: GenericEvent>(&mut self, size: f64, e: &E, tiles: &Vec<Tile>) {
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
                        self.selected_cell = Option::from(tile.clone())
                    }
                }
            }
        }
    }
}