use clipboard::{ClipboardContext, ClipboardProvider};
use piston_window::{
    keyboard::ModifierKey, rectangle, text, Button, ButtonArgs, ButtonState, Context, Event, G2d,
    Glyphs, Input, Key, Transformed,
};
use std::cmp;

use crate::colours;
use crate::layout::{MAX_RESULTS, RESULT_FONT_SIZE, TEXTBOX_HEIGHT, TEXTBOX_PADDING, WINDOW_DIMS};

/// Finds the closest `n` strings to the `input` string in the collection `input`
///
/// # Returns
/// [`Vec`] of indices into the `input` collection for these strings
fn find_closest<'a>(input: &str, options: &'a [String], n: usize) -> Vec<usize> {
    let mut distances: Vec<_> = options
        .iter()
        .enumerate()
        .map(|(idx, elem)| (idx, strsim::normalized_damerau_levenshtein(input, elem)))
        .collect();

    // sort by distance
    distances
        .as_mut_slice()
        .sort_unstable_by(|b, a| a.1.partial_cmp(&b.1).unwrap());

    // limit to n results
    distances.truncate(n);

    distances.iter().map(|e| e.0).collect()
}

pub struct App {
    box_content: String,
    choices: Vec<String>,
    results_cache: Vec<usize>,
    selected: usize,
    modifiers_state: ModifierKey,
    clipboard: ClipboardContext,
}

impl App {
    pub fn new(choices: Vec<String>) -> App {
        App {
            box_content: String::new(),
            choices,
            results_cache: Vec::new(),
            selected: 0,
            modifiers_state: ModifierKey::default(),
            clipboard: ClipboardContext::new().unwrap(),
        }
    }

    /// Runs when the user accepts a choice
    /// # Returns
    /// Whether the program is done
    fn on_accept(&mut self) -> bool {
        let choice_num = self.results_cache.get(self.selected);
        if choice_num.is_none() {
            return false;
        }
        let chosen_line = &self.choices[*choice_num.unwrap()];
        let first_char = chosen_line.chars().take(1).collect::<String>();
        let res = self.clipboard.set_contents(first_char);
        if let Err(e) = res {
            println!("Error copying: {}", e);
        }
        true
    }

    /// Run on each input event, updates internal state
    pub fn on_event(&mut self, event: &Event) -> bool {
        let mut terminate = false;
        self.modifiers_state.event(event);
        if let Event::Input(ev, _) = event {
            match &ev {
                Input::Text(t) => {
                    self.box_content += &t;
                    self.update_results();
                }

                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(k),
                    ..
                }) => match k {
                    Key::Backspace => {
                        // Ctrl-backspace clears the text box
                        if self.modifiers_state.contains(ModifierKey::CTRL) {
                            self.box_content.clear();
                        } else {
                            self.box_content.pop();
                        }
                        self.update_results();
                    }
                    Key::Return => {
                        terminate = self.on_accept();
                    }
                    // note: results are counted from top to bottom (i.e. 0 is at the top of the screen)
                    Key::Up => self.selected = self.selected.saturating_sub(1),
                    Key::Down => self.selected = cmp::min(MAX_RESULTS, self.selected + 1),
                    _ => (),
                },

                _ => (),
            }
        }
        terminate
    }

    /// Update the cache of search results: we only regenerate results when the user types something
    fn update_results(&mut self) {
        // don't emit useless results when there is no query
        if self.box_content.is_empty() {
            return;
        }
        self.results_cache = find_closest(self.box_content.as_ref(), &self.choices, MAX_RESULTS);
    }

    /// Renders the rounded input textbox onto the screen
    fn render_textbox(&self, ctx: &Context, graphics: &mut G2d, gcache: &mut Glyphs) {
        rectangle::Rectangle::new_round(colours::TEXT_BOX, 5.).draw(
            [
                TEXTBOX_PADDING as f64,
                TEXTBOX_PADDING as f64,
                WINDOW_DIMS.width as f64 - 2. * TEXTBOX_PADDING as f64,
                TEXTBOX_HEIGHT as f64,
            ],
            &ctx.draw_state,
            ctx.transform,
            graphics,
        );
        text::Text::new_color([1.; 4], (TEXTBOX_HEIGHT / 1.5) as u32)
            .draw(
                &self.box_content,
                gcache,
                &ctx.draw_state,
                ctx.transform.trans(
                    TEXTBOX_PADDING as f64 * 2.,
                    TEXTBOX_HEIGHT as f64 - TEXTBOX_PADDING as f64,
                ),
                graphics,
            )
            .unwrap();
    }

    fn result_pos(idx: usize) -> f64 {
        TEXTBOX_HEIGHT as f64
            + 2. * TEXTBOX_PADDING as f64
            + 1.5 * RESULT_FONT_SIZE as f64 * (idx + 1) as f64
    }

    fn render_results(&self, ctx: &Context, graphics: &mut G2d, gcache: &mut Glyphs) {
        for (idx, &res) in self.results_cache.iter().enumerate() {
            let name = &self.choices[res];

            text::Text::new_color(colours::RESULTS, RESULT_FONT_SIZE)
                .draw(
                    name,
                    gcache,
                    &ctx.draw_state,
                    ctx.transform
                        .trans(TEXTBOX_PADDING as f64, Self::result_pos(idx)),
                    graphics,
                )
                .unwrap();
        }
    }

    pub fn render_result_selection(&self, ctx: &Context, graphics: &mut G2d) {
        // this is full of magic numbers because text is given a baseline and does not really have
        // predictable dimensions. Part of it goes under the baseline and more than the font size (!!)
        // goes over the baseline
        rectangle::Rectangle::new(colours::SELECTED_RESULT).draw(
            [
                0.,
                // cursed math: we have to move down half the font width because some of the text renders below
                // its baseline and we want the highlight to cover the entire text
                Self::result_pos(self.selected as usize) + RESULT_FONT_SIZE as f64 * 0.4,
                WINDOW_DIMS.width,
                // normally the rectangle draws in +y of the x, y position it is given.
                // This is not desirable in this case because Text renders upwards, so we use a
                // negative y size so it draws -y of the given position
                RESULT_FONT_SIZE as f64 * -1.5,
            ],
            &ctx.draw_state,
            ctx.transform,
            graphics,
        );
    }

    pub fn render(&self, ctx: &Context, graphics: &mut G2d, gcache: &mut Glyphs) {
        self.render_textbox(ctx, graphics, gcache);
        self.render_result_selection(ctx, graphics);
        self.render_results(ctx, graphics, gcache);
    }
}

#[test]
fn find_closest_works() {
    let corpus = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("abb"),
    ];

    assert_eq!(find_closest("a", &corpus, 3), vec![0, 2, 1]);
    assert_eq!(find_closest("b", &corpus, 3), vec![1, 2, 0]);
    assert_eq!(find_closest("b", &corpus, 2), vec![1, 2]);
}
