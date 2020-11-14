use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Duration;
use std::thread;

use crate::universe::Universe;


pub struct Engine {
    universe: Universe,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Engine {
    pub fn new() -> Result<Engine, String> {
        let universe = Universe::new(64, 64);

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut window = match video_subsystem.window("SDL Game of Life", 1000, 1000)
                .position_centered()
                .build() {
            Ok(sub_system) => sub_system,
            Err(e) => return Err(format!("Could not build window: {:?}", e))
        };

        window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;

        let canvas = match window.into_canvas().build() {
            Ok(canvas) => canvas,
            Err(e) => return Err(format!("Could not convert window into canvas: {:?}", e))
        };

        let event_pump = sdl_context.event_pump().unwrap();

        // canvas.set_draw_color(Color::RGB(120, 120, 120));
        // canvas.clear();
        // canvas.present();

        Ok(Engine {
            canvas,
            universe,
            event_pump
        })
    }

    // Starts the game loop
    pub fn run(&mut self) {
        self.canvas.set_draw_color(Color::RGB(120, 120, 120));
        self.canvas.clear();
        self.canvas.present();

        'running: loop {
            self.canvas.set_draw_color(Color::RGB(120, 120, 120));
            self.canvas.clear();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                        break 'running
                    },
                    Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                        self.universe.toggle_state();
                    },
                    Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                        self.universe.run();
                        self.universe.tick();
                        self.universe.pause();
                    },
                    _ => {
                    }
                }
            }

            self.universe.render(&mut self.canvas);
            self.universe.tick();
            self.canvas.present();
            thread::sleep(Duration::from_millis(100));
        }
    }
}