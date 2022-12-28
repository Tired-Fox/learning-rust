mod xterm_data;

use crate::format::COLOR;
use xterm_data::CODE_TO_RGB;

/// Required methods to use the colors ansi code in text styling.
pub trait Ansi {
    /// Get the full ansi code for the color representation.
    fn ansi(&self, fob: COLOR) -> String;

    /// Get the full ansi sequence between `\x1b[` and `m`
    ///
    /// # Example
    /// `\x1b[38;5;15m` == `38;5;15`
    fn escape_seq(&self, fob: COLOR) -> String;
}

/// RGB color representation.
/// 
/// red, green, blue values are from 0 - 255. A neutral color is when all three of red, green, and blue
/// are the same value.
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    pub fn neutral(color: u8) -> Self {
        RGB {
            r: color,
            g: color,
            b: color,
        }
    }
}

impl Ansi for RGB {
    fn ansi(&self, fob: COLOR) -> String {
        format!("\x1b[{};2;{};{};{}m", fob as u8 + 8, self.r, self.g, self.b)
    }

    fn escape_seq(&self, fob: COLOR) -> String {
        format!("{};2;{};{};{}", fob as u8 + 8, self.r, self.g, self.b)
    }
}


/// XTERM color representation.
/// 
/// This color object represents on of 256 xterm colors.
/// These colors are represented by a signle digit so the code is a `u8`.
/// The xterm code can be any number from 0 - 255
pub struct XTERM {
    code: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl XTERM {
    pub fn new(code: u8) -> Self {
        let rgb = match CODE_TO_RGB.get(&code) {
            Some(rgb) => rgb,
            _ => panic!("Couldn't find rgb for code ({})", code),
        };

        XTERM {
            code,
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }
}

impl Ansi for XTERM {
    fn ansi(&self, fob: COLOR) -> String {
        format!("\x1b[{};5;{}m", fob as u8 + 8, self.code)
    }

    fn escape_seq(&self, fob: COLOR) -> String {
        format!("{};5;{}", fob as u8 + 8, self.code)
    }
}

/// System representation of a color.
/// 
/// These colors are the base colors for the operating system.
/// These colors include:
/// - black
/// - red
/// - green
/// - yellow
/// - blue
/// - magenta
/// - cyan
/// - white
pub struct System {
    name: String,
    code: u8,
}

impl System {
    pub fn new(name: &str) -> Self {
        let name: String = name.to_lowercase();
        let mut code: u8 = 0;

        match name.as_str() {
            "black" => code = 0,
            "red" => code = 1,
            "green" => code = 2,
            "yellow" => code = 3,
            "blue" => code = 4,
            "magenta" => code = 5,
            "cyan" => code = 6,
            "white" => code = 7,
            _ => panic!("Invalid system color {}", name),
        }

        System { name, code }
    }
}

impl Ansi for System {
    fn ansi(&self, fob: COLOR) -> String {
        format!("\x1b[{}m", fob as u8 + self.code)
    }

    fn escape_seq(&self, fob: COLOR) -> String {
        format!("{}", fob as u8 + self.code)
    }
}

/// Generic struct to generate different types of colors.
///
/// Can generate RGB, XTERM, and System color objects as each of these have
/// unique patterns to their ansi codes.
///
/// # Examples
///
/// **Creating RGB**
/// ```
/// Color::from_rgb(123, 210, 15) // => rgb(123, 210, 15)
/// Color::from_system("red") // => system("red")
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {}

impl Color {
    pub fn form_system(name: &str) -> System {
        System::new(name)
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> RGB {
        RGB::new(r, g, b)
    }

    pub fn from_neutral(color: u8) -> RGB {
        RGB::neutral(color)
    }

    pub fn from_xterm(code: u8) -> XTERM {
        XTERM::new(code)
    }

    pub fn from_hex(code: &str) -> RGB {
        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let mut hex = String::from(code);

        if hex.starts_with("#") {
            hex = String::from(hex.as_str().get(1..).unwrap());
        }

        if hex.len() == 3 {
            let mut new_hex = String::new();
            for char in hex.chars() {
                new_hex.push(char);
                new_hex.push(char);
            }
            hex = new_hex;
        }

        if hex.len() != 6 {
            panic!("{} is not a valid length hex code!", hex);
        }

        let r: u8 = match u8::from_str_radix(&hex[..2], 16) {
            Ok(color) => color,
            Err(_) => panic!("{} in {} is not a valid hex!", &hex[..2], hex),
        };

        let g: u8 = match u8::from_str_radix(&hex[2..4], 16) {
            Ok(color) => color,
            Err(_) => panic!("{} in {} is not a valid hex!", &hex[2..4], hex),
        };

        let b: u8 = match u8::from_str_radix(&hex[4..6], 16) {
            Ok(color) => color,
            Err(_) => panic!("{} in {} is not a valid hex!", &hex[4..6], hex),
        };

        RGB { r, g, b }
    }
}
