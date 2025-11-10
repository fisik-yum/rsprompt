use crate::modules;
#[allow(dead_code)]
#[repr(u8)]
pub enum Format {
    Default = 0,
    Bold,
    Dim,

    Underlined = 4,
    Blink,

    Invert = 7,
    Hidden,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum FgColor {
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LGrey,

    Default = 39,

    Grey = 90,
    LRed,
    LGreen,
    LYellow,
    LBlue,
    LMagenta,
    LCyan,
    White,
}

#[allow(dead_code)]
#[repr(u8)]
pub enum BgColor {
    Black = 40,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LGrey,

    Default = 49,

    Grey = 100,
    LRed,
    LGreen,
    LYellow,
    LBlue,
    LMagenta,
    LCyan,
    White,
}

pub struct Text;
impl modules::Module for Text {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() < 3 {
                panic!("!invalid option count")
            } else {
                let fg = o.get(0).unwrap();
                let bg = o.get(1).unwrap();
                let cursor = o.get(2).unwrap();
                return Text::style(fg, bg, cursor);
            }
        } else {
            panic!("!invalid option count")
        }
    }
}

impl Text {
    fn style(fg: &String, bg: &String, cursor: &String) -> String {
        let fg_t = Text::fg(fg) as u8;
        let bg_t = Text::bg(bg) as u8;
        let cursor_t = Text::cursor(cursor) as u8;
        format!("\\[\\e[{};{};{}m\\]", cursor_t, fg_t, bg_t)
    }
    fn fg(fg: &String) -> FgColor {
        match fg.as_str() {
            "black" => FgColor::Black,
            "red" => FgColor::Red,
            "green" => FgColor::Green,
            "yellow" => FgColor::Yellow,
            "blue" => FgColor::Blue,
            "magenta" => FgColor::Magenta,
            "cyan" => FgColor::Cyan,
            "lgrey" => FgColor::LGrey,
            "grey" => FgColor::Grey,
            "lred" => FgColor::Red,
            "lgreen" => FgColor::LGreen,
            "lyellow" => FgColor::LYellow,
            "lblue" => FgColor::LBlue,
            "lcyan" => FgColor::LCyan,
            "lmagenta" => FgColor::LMagenta,
            "white" => FgColor::White,
            _ => FgColor::Default,
        }
    }
    fn bg(bg: &String) -> BgColor {
        match bg.as_str() {
            "black" => BgColor::Black,
            "red" => BgColor::Red,
            "green" => BgColor::Green,
            "yellow" => BgColor::Yellow,
            "blue" => BgColor::Blue,
            "magenta" => BgColor::Magenta,
            "cyan" => BgColor::Cyan,
            "lgrey" => BgColor::LGrey,
            "grey" => BgColor::Grey,
            "lred" => BgColor::Red,
            "lgreen" => BgColor::LGreen,
            "lyellow" => BgColor::LYellow,
            "lblue" => BgColor::LBlue,
            "lcyan" => BgColor::LCyan,
            "lmagenta" => BgColor::LMagenta,
            "white" => BgColor::White,
            _ => BgColor::Default,
        }
    }
    fn cursor(cursor: &String) -> Format {
        match cursor.as_str() {
            "bold" => Format::Bold,
            "dim" => Format::Dim,
            "underline" => Format::Underlined,
            "blink" => Format::Blink,
            "invert" => Format::Invert,
            "hidden" => Format::Hidden,
            _ => Format::Default,
        }
    }
}
