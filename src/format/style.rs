use super::{colors::Ansi, Bold, Uline, COLOR};

/// Builder struct that has reasonable defaults for styling text.
/// 
/// Initialize the class and use method chaining to set certain values. Set the foreground and background
/// colors, url value, bold, underline, and reversed colors. When ready call the `style` method and pass
/// the text you want to style. The method will return the stylized string.
/// 
/// # Example
/// ```
/// let style = Style::new().foreground(Color::from_system("red"))
/// 
/// println!("{}", style.style("Hello World!"))
/// ```
/// 
/// Will result in the string `Hello World!` to have the font color of `red`
pub struct Style {
    foreground: Option<Box<dyn Ansi>>,
    background: Option<Box<dyn Ansi>>,
    url: Option<String>,
    bold: Bold,
    underline: Uline,
    reverse: bool,
}

impl Style {
    /// Create a default style which then can have it's values changed with the provided builder methods.
    ///
    /// ## Defualts:
    /// - foreground: `Color::neutral(255)` == White
    /// - background: `Color::neutral(0)` == Black
    /// - bold: `Bold::Off`
    /// - underline: `Uline:Off`
    /// - reverse: `false`
    pub fn new() -> Style {
        Style {
            foreground: None,
            background: None,
            url: None,
            bold: Bold::Off,
            underline: Uline::Off,
            reverse: false,
        }
    }

    /// Set the foreground color.
    pub fn foreground(mut self, color: impl Ansi + 'static) -> Style {
        self.foreground = Some(Box::new(color));
        self
    }

    /// Set the background color.
    pub fn background(mut self, color: impl Ansi + 'static) -> Style {
        self.background = Some(Box::new(color));
        self
    }

    /// Set the url value.
    pub fn url(mut self, url: &str) -> Style {
        match url {
            "" => self.url = None,
            _ => self.url = Some(url.to_string()),
        }
        self
    }

    /// Toggle the bold state.
    pub fn bold(mut self) -> Style {
        match self.bold {
            Bold::On => self.bold = Bold::Off,
            Bold::Off => self.bold = Bold::On,
        }
        self
    }

    /// Toggle the underlined state.
    pub fn underline(mut self) -> Style {
        match self.underline {
            Uline::On => self.underline = Uline::Off,
            Uline::Off => self.underline = Uline::On,
        }
        self
    }

    /// Toggle whether the colors are reversed for foreground and background.
    pub fn reverse(mut self) -> Style {
        self.reverse = !self.reverse;
        self
    }

    /// Style a provided string slice into a new string slice given the style.
    pub fn style(&self, text: &str) -> String {
        let mut result: Vec<String> = Vec::new();

        // Add foreground and background colors
        match &self.reverse {
            true => {
                match &self.foreground {
                    Some(color) => {
                        result.push(color.escape_seq(COLOR::Background));
                    }
                    _ => (),
                }

                match &self.background {
                    Some(color) => {
                        result.push(color.escape_seq(COLOR::Foreground));
                    }
                    _ => (),
                }
            }
            false => {
                match &self.foreground {
                    Some(color) => {
                        result.push(color.escape_seq(COLOR::Foreground));
                    }
                    _ => (),
                }

                match &self.background {
                    Some(color) => {
                        result.push(color.escape_seq(COLOR::Background));
                    }
                    _ => (),
                }
            }
        }

        // Add bold
        if self.bold == Bold::On {
            result.push("1".to_string());
        }

        // Add underline
        if self.underline == Uline::On {
            result.push("4".to_string());
        }

        let mut markup = false;
        if result.len() > 0 {
            markup = true;
            result = Vec::from([String::from("\x1b["), result.join(";"), String::from("m")]);
        }

        let mut is_url = false;
        match &self.url {
            Some(url) => {
                result.push(["\x1b]8;;".to_string(), url.clone(), "\x1b\\".to_string()].join(""));
                is_url = true;
            }
            None => (),
        }

        // Add text
        result.push(text.to_string());

        if is_url {
            result.push("\x1b]8;;\x1b\\".to_string());
        }

        // Reset style
        if markup {
            result.push("\x1b[0m".to_string());
        }

        return result.join("");
    }
}
