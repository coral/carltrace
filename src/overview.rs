use crate::state::State;
use anyhow::{Context, Result};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Overview {
    c: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Overview {
    pub fn bootstrap(w: sdl2::video::Window) -> Result<Overview> {
        let canvas = w.into_canvas().build()?;

        Ok(Overview { c: canvas })
    }

    pub fn draw_state(&mut self, s: &State) -> Result<()> {
        let c = &mut self.c;
        c.set_draw_color(Color::RGB(0, 0, 0));
        c.clear();

        // Draw Map
        s.map.structure.iter().enumerate().for_each(|(row, rd)| {
            rd.iter().enumerate().for_each(|(col, i)| {
                let x = 0.0;

                match i {
                    true => {
                        c.set_draw_color(Color::RGB(100, 50, 0));
                        c.fill_rect(Rect::new((row as i32) * 40, (col as i32) * 40, 40, 40));
                    }
                    false => {
                        c.set_draw_color(Color::RGB(40, 40, 40));
                        c.fill_rect(Rect::new((row as i32) * 40, (col as i32) * 40, 40, 40));
                    }
                }
            })
        });

        // Draw camera
        c.filled_circle(
            s.camera.x as i16,
            s.camera.y as i16,
            10,
            Color::RGB(255, 0, 0),
        );

        c.thick_line(
            s.camera.x as i16,
            s.camera.y as i16,
            (s.camera.x + (s.camera.angle_draw_distance * s.camera.angle.to_radians().cos()))
                as i16,
            (s.camera.y + (s.camera.angle_draw_distance * s.camera.angle.to_radians().sin()))
                as i16,
            2,
            Color::RGB(255, 0, 0),
        );

        c.present();

        Ok(())
    }
}
