use chrono::Timelike;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use unicode_width::UnicodeWidthStr;

pub const BG: Color = Color::Rgb(35, 34, 27);
pub const FG: Color = Color::Rgb(223, 218, 191);
pub const MUTED: Color = Color::Rgb(146, 139, 110);
pub const FAINT: Color = Color::Rgb(104, 99, 78);
pub const BORDER: Color = Color::Rgb(110, 104, 78);
pub const BRAND: Color = Color::Rgb(243, 213, 74);
pub const BRAND_HI: Color = Color::Rgb(253, 240, 155);
pub const BRAND_LO: Color = Color::Rgb(188, 148, 45);
pub const LABEL: Color = Color::Rgb(178, 186, 122);
pub const SEL_BG: Color = Color::Rgb(58, 55, 38);

pub const TABS: [&str; 5] = ["search", "vibes", "picks", "details", "help"];

pub fn text() -> Style {
    Style::default().fg(FG)
}

pub fn dim() -> Style {
    Style::default().fg(MUTED)
}

pub fn faint() -> Style {
    Style::default().fg(FAINT)
}

pub fn border() -> Style {
    Style::default().fg(BORDER)
}

pub fn brand() -> Style {
    Style::default().fg(BRAND).add_modifier(Modifier::BOLD)
}

pub fn brand2() -> Style {
    Style::default().fg(BRAND_LO)
}

pub fn label() -> Style {
    Style::default().fg(LABEL)
}

pub fn header() -> Style {
    Style::default().fg(MUTED).add_modifier(Modifier::BOLD)
}
//This IS MY WATERMARK DONT STEALLL
pub const BOLT_ART: [&str; 19] = [
    "                          =.",
    "                        :%#.",
    "                      =*=*",
    "                    ++-:#:",
    "                   +=.--@",
    "                :*=.:-=*",
    "              .+=.:---*:",
    "             -+:.:---=*********#=",
    "          .=+:.::--------=====+=.",
    "         :+-.:-----------===**:",
    "         =*********#%----==*:.",
    "                   ++--==+=",
    "                  :+--=++=",
    "                 .+--=+-",
    "                 @-=%",
    "                *-+*.",
    "               -**-.",
    "              .%-",
    "              #=",
];

const WORDMARK: [&str; 4] = [
    "▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▄ █▀▀▀▀",
    "█   █ █▄▄▄▀ █   █ █▄▄▄ ",
    "█▀▀▀█ █  ▀▄ █   █ █    ",
    "█   █ █   █ ▀▄▄▄▀ █▄▄▄▄",
];

pub const WORDMARK_ROWS: u16 = 4;