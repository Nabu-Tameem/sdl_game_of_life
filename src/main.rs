extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl_game_of_life::Universe;
use std::time::Duration;
use std::thread;



fn main() {
    let mut universe = Universe::new(64, 64);
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL Game of Life", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(128, 86, 32));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(Color::RGB(128, 86, 32));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    universe.toggle_state();
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    universe.run();
                    universe.tick();
                    universe.pause();
                },
                _ => {
                }
            }
        }
        
        universe.render(&mut canvas, 2, 2);
        universe.tick();
        canvas.present();
        thread::sleep(Duration::from_millis(0));
    }
}
