use graphics::{clear, color, Rectangle, Text, Transformed};
use graphics::rectangle::rectangle_by_corners;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub struct Timer {}

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
                .draw("00:00", glyphs, &c.draw_state, black_transform_text, g).unwrap();

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
                .draw("00:00", glyphs, &c.draw_state, white_transform_text, g).unwrap();
        });
    }
}