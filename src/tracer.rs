use crate::state::State;
use sdl2::pixels::Color;

pub struct Tracer {
    c: sdl2::render::Canvas<sdl2::video::Window>,
    i: u8,
}

impl Tracer {
    pub fn bootstrap(w: sdl2::video::Window) -> anyhow::Result<Tracer> {
        let canvas = w.into_canvas().build()?;
        let (w, h) = canvas.window().size();

        Ok(Tracer { c: canvas, i: 0 })
    }

    pub fn draw(&mut self, s: &State) {
        let mut canvas = &mut self.c;

        //self.i = (self.i + 1) % 255;
        let i = (s.camera.angle as i64 % 255) as u8;

        //println!("calc: {}, angle: {}", i, s.camera.angle);
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.present();
    }
}
