use piston_window::{
    rectangle, text, Button, ButtonArgs, ButtonState, Context, Event, G2d, Glyphs, Input, Key,
    Transformed,
};

use crate::colours;
use crate::layout;

pub struct App {
    box_content: String,
}

impl App {
    pub fn new() -> App {
        App {
            box_content: String::new(),
        }
    }

    pub fn on_input(&mut self, event: &Input) {
        match event {
            Input::Text(t) => self.box_content += &t,

            Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Keyboard(Key::Backspace),
                ..
            }) => {
                self.box_content.pop();
            }

            _ => (),
        }
    }

    fn render_textbox(&self, ctx: &Context, graphics: &mut G2d, gcache: &mut Glyphs) {
        let textbox_padding = 5.;
        let textbox_height = 50.;

        rectangle::Rectangle::new_round(colours::TEXT_BOX, 5.).draw(
            [
                textbox_padding as f64,
                textbox_padding as f64,
                layout::WINDOW_DIMS.width as f64 - 2. * textbox_padding as f64,
                textbox_height as f64,
            ],
            &ctx.draw_state,
            ctx.transform,
            graphics,
        );
        text::Text::new_color([1.; 4], (textbox_height / 1.5) as u32)
            .draw(
                &self.box_content,
                gcache,
                &ctx.draw_state,
                ctx.transform.trans(
                    textbox_padding as f64 * 2.,
                    textbox_height as f64 - textbox_padding,
                ),
                graphics,
            )
            .unwrap();
    }

    pub fn render(&self, ctx: &Context, graphics: &mut G2d, gcache: &mut Glyphs) {
        self.render_textbox(ctx, graphics, gcache);
    }
}
