use glutin_window::GlutinWindow;
use piston::window::{AdvancedWindow, Window};
use piston_window::{clear, PistonWindow, Position, WindowSettings};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod app;
mod colours;
mod layout;

use app::App;

fn main() {
    let now = Instant::now();
    let mut window: PistonWindow<GlutinWindow> =
        WindowSettings::new("Hello Piston!", layout::WINDOW_DIMS)
            .exit_on_esc(true)
            .decorated(false)
            .build()
            .unwrap();
    println!("build window {}", now.elapsed().as_millis());
    let now = Instant::now();
    let win = window.window.ctx.window();
    let monitor_id = win.get_primary_monitor();
    let msize = monitor_id.get_dimensions();
    let mpos = monitor_id.get_position();
    println!("get window dims {}", now.elapsed().as_millis());

    let center = Position {
        x: (mpos.x + msize.width / 2. - window.size().width / 2.) as i32,
        y: (mpos.y + msize.height / 2. - window.size().height / 2.) as i32,
    };

    let now = Instant::now();
    window.set_position(center);
    println!("center window {}", now.elapsed().as_millis());

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
