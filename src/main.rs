use glutin_window::GlutinWindow;
use piston::input::{Button, ButtonArgs, ButtonState, Event, Input, Key};
use piston::window::{AdvancedWindow, Window};
use piston_window::Transformed;
use piston_window::{clear, rectangle, text, PistonWindow, Position, WindowSettings};
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

    let mut gcache = window.load_font("SourceCodePro.ttf").unwrap();

    let mut app = App::new();

    while let Some(event) = window.next() {
        if let Event::Input(ev, _) = &event {
            app.on_input(ev);
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear(colours::BACKGROUND, graphics);

            app.render(&context, graphics, &mut gcache);

            // magic, see https://github.com/PistonDevelopers/piston_window/issues/258#issuecomment-514442002
            gcache.factory.encoder.flush(device);
        });
    }
    println!("Hello, world!");
}
