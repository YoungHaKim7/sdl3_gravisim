use std::f32::consts::PI;

use crate::cam::Cam;
use crate::GRAVITY_CONST;
use sdl3::gfx::primitives::DrawRenderer;
use sdl3::pixels::Color;
use sdl3::render::WindowCanvas;

#[derive(Copy, Clone)]
pub struct Body {
    pub past_a_x: f32,
    pub past_a_y: f32,
    pub past_x: f32,
    pub past_y: f32,
    pub x: f32,
    pub y: f32,
    pub a_x: f32,
    pub a_y: f32,
    pub v_x: f32,
    pub v_y: f32,
    pub mass: f32,
    pub size: f32,
    pub density: f32,
    pub color: (u8, u8, u8, u8),
}

impl Body {
    pub fn new(x: f32, y: f32, v_x: f32, v_y: f32, density: f32, size: f32) -> Body {
        Body {
            past_a_x: 0f32,
            past_a_y: 0f32,
            past_x: 0f32,
            past_y: 0f32,
            x,
            y,
            v_x,
            v_y,
            a_x: 0f32,
            a_y: 0f32,
            mass: (4.0 / 3.0) * PI * size.powi(3) * density,
            size,
            density,
            color: (255, 255, 255, 255),
        }
    }

    pub fn update_self(&mut self, mult: &f32, total_time: &f32) {
        self.past_x = self.x;
        self.past_y = self.y;

        self.x += (self.v_x * mult) + (0.5 * self.a_x * mult.powi(2));
        self.y += (self.v_y * mult) + (0.5 * self.a_y * mult.powi(2));

        self.v_x += (self.a_x + self.past_a_x) * mult * 0.5;
        self.v_y += (self.a_y + self.past_a_y) * mult * 0.5;

        self.past_a_x = self.a_x * mult;
        self.past_a_y = self.a_y * mult;
    }

    pub fn compute_gravity(&mut self, body: Body, mult: &f32) {
        let min_distance = 0.0001;
        let direction = (body.x - self.x, body.y - self.y);
        let mut distance = ((body.x - self.x).powi(2) + (body.y - self.y).powi(2)).sqrt();
        if distance < min_distance {
            distance = min_distance;
        }
        let unit_direction = (direction.0 / distance, direction.1 / distance);
        let force_scalar = GRAVITY_CONST * self.mass * body.mass / distance.powi(2);
        let acc_scalar = force_scalar / self.mass;
        let acc_vector = (unit_direction.0 * acc_scalar, unit_direction.1 * acc_scalar);
        self.a_x += acc_vector.0;
        self.a_y += acc_vector.1;
    }

    pub fn render(&self, canvas: &mut WindowCanvas, cam: &Cam) {
        let color_g = 255u8.saturating_sub(self.density as u8);

        let t = cam.transform((self.x, self.y));
        let _ = canvas.filled_circle(
            t.0 as i16,
            t.1 as i16,
            (self.size * cam.zoom) as i16,
            (255, color_g, 255, 255),
        );
    }
}
