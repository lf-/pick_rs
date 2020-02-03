#![windows_subsystem = "windows"]

use piston::window::Window;
use piston_window::{clear, EventLoop, PistonWindow, WindowSettings};
use sdl2::video::WindowPos;
use sdl2_window::Sdl2Window;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod app;
mod colours;
mod layout;

use app::App;

fn main() {
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Hello Piston!", layout::WINDOW_DIMS)
            .exit_on_esc(true)
            .decorated(false)
            .build()
            .unwrap();

    window.set_max_fps(60);
    window.set_ups(60);

    window
        .window
        .window
        .set_position(WindowPos::Centered, WindowPos::Centered);

    let mut gcache = window.load_font("FiraSans-Regular.ttf").unwrap();

    let choices = BufReader::new(File::open("choices.txt").unwrap())
        .lines()
        .filter_map(|ln| ln.ok())
        .collect::<Vec<String>>();

    let mut app = App::new(choices);

    while let Some(event) = window.next() {
        if app.on_event(&event) {
            window.set_should_close(true);
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear(colours::BACKGROUND, graphics);

            app.render(&context, graphics, &mut gcache);

            // magic, see https://github.com/PistonDevelopers/piston_window/issues/258#issuecomment-514442002
            gcache.factory.encoder.flush(device);
        });
    }
}
