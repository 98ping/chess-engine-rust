use std::cmp::min;
use std::time::Instant;
use graphics::{clear, color, Rectangle, Text, Transformed};
use graphics::rectangle::rectangle_by_corners;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub struct Timer {
    pub white_turn: bool,
    pub started: bool,
    pub white_time: u64,
    pub black_time: u64,
    pub last_tick: Instant
}

impl Timer {
    pub fn draw_timers(&self, gl: &mut GlGraphics, r: &RenderArgs, glyphs: &mut GlyphCache) {
        gl.draw(r.viewport(), |c, g| {
            let black_transform_text = c.transform.trans(860.0, 137.0);
            let black_rect = rectangle_by_corners(850.0, 100.0, 960.0, 150.0);
            Rectangle::new_border([1.0, 1.0, 1.0, 1.0], 2.0).draw(black_rect, &c.draw_state, c.transform, g);

            clear(color::hex("899499"), g);

            graphics::rectangle(
                [0.0, 0.0, 0.0, 1.0],
                black_rect,
                c.transform,
                g,
            );

            Text::new_color([1.0, 1.0, 1.0, 1.0], 32)
                .draw(Self::format_string_time(self.black_time).as_str(), glyphs, &c.draw_state, black_transform_text, g).unwrap();

            let white_rect = rectangle_by_corners(850.0, 600.0, 960.0, 650.0);
            let white_transform_text = c.transform.trans(860.0, 637.0);
            Rectangle::new_border([0.0, 0.0, 0.0, 1.0], 2.0).draw(white_rect, &c.draw_state, c.transform, g);

            graphics::rectangle(
                [1.0, 1.0, 1.0, 1.0],
                white_rect,
                c.transform,
                g,
            );

            Text::new_color([0.0, 0.0, 0.0, 1.0], 32)
                .draw(Self::format_string_time(self.black_time).as_str(), glyphs, &c.draw_state, white_transform_text, g).unwrap();
        });
    }

    pub fn format_string_time(time: u64) -> String {
        let duration = std::time::Duration::from_secs(time);
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;

        return format!("{:0>2}:{:0>2}", minutes, seconds)
    }
}
