use crate::state::State;
//use anyhow::{Context, Result};
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
    pub fn bootstrap(w: sdl2::video::Window) -> anyhow::Result<Overview> {
        let canvas = w.into_canvas().build()?;
        let (w, h) = canvas.window().size();

        Ok(Overview {
            c: canvas,
            w,
            h,
            scaling_factor: w as f64 / 100.0,
        })
    }

    pub fn trace(
        angle: f64,
        c: &mut sdl2::render::Canvas<sdl2::video::Window>,
        s: &State,
    ) -> Result<(), String> {
        for n in (0..800).step_by(10) {
            let xx = s.camera.x + (n as f64 * angle.to_radians().cos());
            let yy = s.camera.y + (n as f64 * angle.to_radians().sin());

            match s.map.intersects(xx, yy) {
                true => {
                    c.filled_circle(xx as i16, yy as i16, 5, Color::RGB(0, 255, 0))?;
                    return Ok(());
                }
                false => c.filled_circle(xx as i16, yy as i16, 1, Color::RGB(255, 255, 0))?,
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, s: &State) -> Result<(), String> {
        let c = &mut self.c;
        c.set_draw_color(Color::RGB(0, 0, 0));
        c.clear();

        let sw = (self.w as f64 / s.map.width as f64) as u32;

        // Draw Map
        s.map.structure.iter().enumerate().for_each(|(row, rd)| {
            rd.iter().enumerate().for_each(|(col, i)| {
                let x = 0.0;

                match i {
                    true => {
                        c.set_draw_color(Color::RGB(100, 50, 0));
                    }
                    false => {
                        c.set_draw_color(Color::RGB(40, 40, 40));
                    }
                }
                c.fill_rect(Rect::new(
                    (row as i32) * sw as i32,
                    (col as i32) * sw as i32,
                    sw,
                    sw,
                ))
                .unwrap();
            })
        });

        // Draw outlines
        c.set_draw_color(Color::RGB(255, 255, 255));
        for i in 0..s.map.width {
            // X
            c.draw_line(
                Point::new((i as u32 * sw) as i32, 0),
                Point::new((i as u32 * sw) as i32, 800),
            )?;

            // Y
            c.draw_line(
                Point::new(0, (i as u32 * sw) as i32),
                Point::new(800, (i as u32 * sw) as i32),
            )?;
        }

        // Draw camera
        let la = s.camera.angle - (s.camera.fov / 2.0);
        let ll = s.camera.angle + (s.camera.fov / 2.0);
        // Frustrum
        c.thick_line(
            s.camera.x as i16,
            s.camera.y as i16,
            (s.camera.x + (s.camera.angle_draw_distance * 2.0 * la.to_radians().cos())) as i16,
            (s.camera.y + (s.camera.angle_draw_distance * 2.0 * la.to_radians().sin())) as i16,
            1,
            Color::RGB(135, 135, 135),
        )?;
        c.thick_line(
            s.camera.x as i16,
            s.camera.y as i16,
            (s.camera.x + (s.camera.angle_draw_distance * 2.0 * ll.to_radians().cos())) as i16,
            (s.camera.y + (s.camera.angle_draw_distance * 2.0 * ll.to_radians().sin())) as i16,
            1,
            Color::RGB(135, 135, 135),
        )?;

        c.filled_circle(
            s.camera.x as i16,
            s.camera.y as i16,
            10,
            Color::RGB(255, 0, 0),
        )?;

        // Pointive
        c.thick_line(
            s.camera.x as i16,
            s.camera.y as i16,
            (s.camera.x + (s.camera.angle_draw_distance * s.camera.angle.to_radians().cos()))
                as i16,
            (s.camera.y + (s.camera.angle_draw_distance * s.camera.angle.to_radians().sin()))
                as i16,
            2,
            Color::RGB(255, 0, 0),
        )?;

        let mut a = s.camera.angle - (s.camera.fov / 2.0);
        let fv = s.camera.fov / 800.0;
        for na in 0..800 {
            Overview::trace(a + (na as f64 * fv), c, s)?;
        }

        c.present();
        Ok(())
    }
}
