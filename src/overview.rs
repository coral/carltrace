use crate::state::State;
use anyhow::{Context, Result};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub struct Overview {
    c: sdl2::render::Canvas<sdl2::video::Window>,

    w: u32,
    h: u32,
    scaling_factor: f64,
}

impl Overview {
    pub fn bootstrap(w: sdl2::video::Window) -> Result<Overview> {
        let canvas = w.into_canvas().build()?;
        let (w, h) = canvas.window().size();

        Ok(Overview {
            c: canvas,
            w,
            h,
            scaling_factor: w as f64 / 100.0,
        })
    }

    pub fn draw_state(&mut self, s: &State) -> Result<()> {
        let c = &mut self.c;
        c.set_draw_color(Color::RGB(0, 0, 0));
        c.clear();

        let sw = (self.w as f64 / s.map.width as f64) as u32;

        // Draw Map

        //
        s.map.structure.iter().enumerate().for_each(|(row, rd)| {
            rd.iter().enumerate().for_each(|(col, i)| {
                let x = 0.0;

                match i {
                    true => {
                        c.set_draw_color(Color::RGB(100, 50, 0));
                        c.fill_rect(Rect::new(
                            (row as i32) * sw as i32,
                            (col as i32) * sw as i32,
                            sw,
                            sw,
                        ));
                    }
                    false => {
                        c.set_draw_color(Color::RGB(40, 40, 40));
                        c.fill_rect(Rect::new(
                            (row as i32) * sw as i32,
                            (col as i32) * sw as i32,
                            sw,
                            sw,
                        ));
                    }
                }
            })
        });

        // Draw outlines
        c.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..s.map.width {
            // X
            c.draw_line(
                Point::new((i as u32 * sw) as i32, 0),
                Point::new((i as u32 * sw) as i32, 800),
            );

            // Y
            c.draw_line(
                Point::new(0, (i as u32 * sw) as i32),
                Point::new(800, (i as u32 * sw) as i32),
            );
        }

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
