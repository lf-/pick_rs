use piston_window::types::Color;

/// Takes in RGBA in hex and returns the colour in float format
/// 0xRRGGBBAA => [R, G, B, A]
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

pub const BACKGROUND: Color = hex_colour!(0x00_00_00_ffu32);
pub const TEXT_BOX: Color = hex_colour!(0x55_55_55_ffu32);
pub const RESULTS: Color = hex_colour!(0xdd_dd_dd_ffu32);
pub const SELECTED_RESULT: Color = hex_colour!(0x44_44_44_ffu32);
