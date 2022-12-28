pub mod colors;
pub mod style;

pub use colors::Color;
pub use style::Style;

/// Way of determining whether something is foreground or background.
///
/// Used to determine foreground and background ansi codes.
pub enum COLOR {
    Foreground = 30,
    Background = 40,
}

/// Toggle states for bold formatting.
#[derive(PartialEq, Eq)]
pub enum Bold {
    On = 1,
    Off = 22,
}

/// Toggle states for underline formatting.
#[derive(PartialEq, Eq)]
pub enum Uline {
    On = 4,
    Off = 24,
}
