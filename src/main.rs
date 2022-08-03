extern crate sdl2;
use anyhow::{Context, Result};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
mod overview;
mod state;
mod tracer;

use overview::Overview;

pub fn main() -> Result<()> {
    let mut st = state::State::new()?;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position(0, 0)
        .build()?;

    let ovw = video_subsystem
        .window("overview", 800, 800)
        .position(820, 0)
        .build()?;

    let mut overview = Overview::bootstrap(ovw)?;

    let mut tr = tracer::Tracer::bootstrap(window)?;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Draw overview
        match overview.draw(&st) {
            Err(e) => println!("could not draw state: {}", e),
            _ => {}
        }

        // Draw raycaster
        tr.draw(&st);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }

            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(k) => st.handle_key(k),
                    None => {}
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
