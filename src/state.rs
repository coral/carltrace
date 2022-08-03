use anyhow::Result;
use sdl2::keyboard::Keycode;

pub struct State {
    pub map: Map,
    pub camera: Camera,
}

impl State {
    pub fn new() -> Result<State> {
        let map = Map::new(10, 10)?;
        //map.print_map();

        Ok(State {
            map,
            camera: Camera {
                x: 100.0,
                y: 100.0,
                angle: 120.0,
                fov: 70.0,
                angle_draw_distance: 40.0,
                movement_speed: 4.81,
                turn_speed: 4.0,
            },
        })
    }

    pub fn handle_key(&mut self, k: sdl2::keyboard::Keycode) {
        let cx = self.camera.x;
        let cy = self.camera.y;
        let mx = self.camera.movement_speed;
        let a = self.camera.angle;

        match k {
            Keycode::Up | Keycode::W => {
                self.camera.x = cx + (mx * a.to_radians().cos());
                self.camera.y = cy + (mx * a.to_radians().sin());
            }
            Keycode::Down | Keycode::S => {
                self.camera.x = cx - (mx * a.to_radians().cos());
                self.camera.y = cy - (mx * a.to_radians().sin());
            }
            Keycode::Left | Keycode::A => {
                self.camera.angle = self.camera.angle - self.camera.turn_speed;
            }
            Keycode::Right | Keycode::D => {
                self.camera.angle = self.camera.angle + self.camera.turn_speed;
            }
            _ => {}
        }
    }
}

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub fov: f64,

    pub angle_draw_distance: f64,
    pub movement_speed: f64,
    pub turn_speed: f64,
}

pub struct Map {
    pub structure: Vec<Vec<bool>>,
    pub width: i64,
    pub height: i64,

    pub ew: f64,
    pub eh: f64,
}

impl Map {
    pub fn new(w: i64, h: i64) -> Result<Map> {
        let str = vec![
            vec![true, true, true, true, true, true, true, true, true, true],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, false, true, true, true, false, false, false, true,
            ],
            vec![
                true, false, false, true, false, true, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, false, false, false, false, false, false, false, true,
            ],
            vec![true, true, true, true, true, true, true, true, true, true],
        ];

        Ok(Map {
            structure: str,
            width: 10,
            height: 10,

            ew: (w as f64) * 10.0,
            eh: (h as f64) * 10.0,
        })
    }

    #[allow(dead_code)]
    pub fn print_map(&self) {
        println!("\n");
        self.structure.iter().for_each(|row| {
            println!("");
            row.iter().for_each(|i| match i {
                true => print!("1 "),
                false => print!("0 "),
            })
        });
        println!("");
    }

    pub fn intersects(&self, x: f64, y: f64) -> bool {
        let xx = (x / 80.0).floor() as usize;
        let yy = (y / 80.0).floor() as usize;

        if xx > self.structure.len() - 1 {
            return false;
        }

        if yy > self.structure[xx].len() - 1 {
            return false;
        }

        self.structure[xx][yy]
    }
}
