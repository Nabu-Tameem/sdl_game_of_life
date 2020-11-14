use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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

        let mut mouse_dragging = false;
        let mut mouse_setting = false;
        let mut mouse_clearing = false;

        let mut previous_mouse_pos_x: i32 = 0;
        let mut previous_mouse_pos_y: i32 = 0;

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
                    // Enable dragging, cell revive mode, and cell kill mode.
                    Event::MouseButtonDown {mouse_btn, x, y, ..} => {
                        match mouse_btn {
                            MouseButton::Left => {
                                mouse_setting = true;
                                self.universe.revive(x, y);
                                self.universe.render(&mut self.canvas);
                                self.canvas.present();
                            },
                            MouseButton::Right => {
                                mouse_clearing = true;
                                self.universe.kill(x, y);
                            },
                            MouseButton::Middle => mouse_dragging = true,
                            _ => {}
                        };
                        previous_mouse_pos_x = x;
                        previous_mouse_pos_y = y;
                    },
                    // Disable dragging, cell revive mode, and cell kill mode.
                    Event::MouseButtonUp {mouse_btn, ..} => {
                        match mouse_btn {
                            MouseButton::Left => mouse_setting = false,
                            MouseButton::Right => mouse_clearing = false,
                            MouseButton::Middle => mouse_dragging = false,
                            _ => {}
                        };
                    },

                    // Apply motion event like dragging, cell batch revive, cell batch kill.
                    Event::MouseMotion {x, y, ..} => {
                        if mouse_dragging {
                            let x_dif = x - previous_mouse_pos_x;
                            let y_dif = y - previous_mouse_pos_y;
    
                            self.universe.shift(x_dif, y_dif);

                            previous_mouse_pos_x = x;
                            previous_mouse_pos_y = y;
                        } else if mouse_setting {
                            self.universe.revive(x, y);
                        } else if mouse_clearing {
                            self.universe.kill(x, y);
                        }
                    },
                    _ => {
                    }
                }
            }

            self.universe.render(&mut self.canvas);
            self.universe.tick();
            self.canvas.present();
        }
    }
}