use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::WindowCanvas;
use sdl3::surface;
use sdl3::ttf::*;
use std::path::Path;

pub struct Text {
    context: &Sdl3TtfContext,
    filename: &str,
    font: Font,
    color: Color,
}

impl Text {
    // With color
    pub fn new(
        ctx: &Sdl3TtfContext,
        filename: &'str,
        size: u16,
        color: Color,
    ) -> Result<Text, String> {
        let font_result = ctx.load_font(Path::new(filename), size);
        if font_result.is_err() {
            return Err(format!("Failed to initialize font for {}", filename));
        }
        Ok(Text {
            context: ctx,
            filename: filename,
            font: font_result.unwrap(),
            color: color,
        })
    }

    pub fn render_surface(&self, text: &'a str) -> Result<surface::Surface, FontError> {
        let partial = self.font.render(text);
        partial.solid(self.color)
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_font_size(&mut self, size: u16) {
        let new_font = self
            .context
            .load_font(Path::new(self.filename), size)
            .expect("Failed to set font size");
        self.font = new_font;
    }

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        text: &str,
        x: i32,
        y: i32,
        right_justify: bool,
    ) {
        let surface = self
            .render_surface(text)
            .expect("Failed creating surface for font");
        let creator = canvas.texture_creator();
        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("Failed creating texture");
        let query = texture.query();

        let updated_x = if right_justify {
            canvas.window().drawable_size().0 as i32 - x - query.width as i32
        } else {
            x
        };

        canvas
            .copy(
                &texture,
                None,
                Rect::new(updated_x, y, query.width, query.height),
            )
            .expect("Failed copying font texture");
    }

    pub fn draw_multiline(
        &self,
        canvas: &mut WindowCanvas,
        text: &str,
        x: i32,
        y: i32,
        right_justify: bool,
        linewidth: i32,
    ) {
        let split_string = text.split("\n");
        let mut line_num = 0;
        for line in split_string {
            self.draw(canvas, line, x, y + (line_num * linewidth), right_justify);
            line_num += 1;
        }
    }
}
