use glutin_window::GlutinWindow;
use piston::input::{Button, ButtonArgs, ButtonState, Event, Input, Key};
use piston::window::{AdvancedWindow, Window};
use piston_window::Transformed;
use piston_window::{clear, rectangle, text, PistonWindow, Position, WindowSettings};
use std::time::Instant;

mod colours;
mod layout;

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

    let mut box_contents = String::new();

    while let Some(event) = window.next() {
        match &event {
            Event::Input(ev, _) => match ev {
                Input::Text(t) => box_contents += t,

                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(Key::Backspace),
                    ..
                }) => {
                    box_contents.pop();
                }

                _ => (),
            },
            _ => (),
        }
        window.draw_2d(&event, |context, graphics, device| {
            clear(colours::BACKGROUND, graphics);

            let textbox_padding = 5.;
            let textbox_height = 50.;

            rectangle::Rectangle::new_round(colours::TEXT_BOX, 5.).draw(
                [
                    textbox_padding as f64,
                    textbox_padding as f64,
                    layout::WINDOW_DIMS.width as f64 - 2. * textbox_padding as f64,
                    textbox_height as f64,
                ],
                &context.draw_state,
                context.transform,
                graphics,
            );
            text::Text::new_color([1.; 4], (textbox_height / 1.5) as u32)
                .draw(
                    &box_contents,
                    &mut gcache,
                    &context.draw_state,
                    context.transform.trans(
                        textbox_padding as f64 * 2.,
                        textbox_height as f64 - textbox_padding,
                    ),
                    graphics,
                )
                .unwrap();
            // text(
            //     [1.0, 1.0, 1.0, 1.0],
            //     2,
            //     "aaaaa",
            //     &mut gcache,
            //     [[0.11, 0., -1.], [0., 0.11, 0.]],
            //     graphics,
            // )
            // .unwrap();

            // magic, see https://github.com/PistonDevelopers/piston_window/issues/258#issuecomment-514442002
            gcache.factory.encoder.flush(device);
        });
    }
    println!("Hello, world!");
}
