use piston_window::types::Color;
/// Takes in RGBA in hex and returns the colour in float format
macro_rules! hex_colour {
    ( $colour:expr ) => {
        [
            (($colour >> 24) & 0xff) as f32 / 255.,
            (($colour >> 16) & 0xff) as f32 / 255.,
            (($colour >> 8) & 0xff) as f32 / 255.,
            (($colour >> 0) & 0xff) as f32 / 255.,
        ]
    };
}

pub const BACKGROUND: Color = hex_colour!(0x000000ff);
pub const TEXT_BOX: Color = hex_colour!(0x555555ff);
