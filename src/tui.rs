

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

fn gradient_at(t: f32) -> Color {
    const STOPS: [(u8, u8, u8); 3] = [(253, 240, 155), (243, 213, 74), (188, 148, 45)];
    let t = t.clamp(0.0, 1.0) * (STOPS.len() - 1) as f32;
    let i = (t.floor() as usize).min(STOPS.len() - 1);
    let j = (i + 1).min(STOPS.len() - 1);
    let f = t - i as f32;
    let mix = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * f).round() as u8;
    Color::Rgb(
        mix(STOPS[i].0, STOPS[j].0),
        mix(STOPS[i].1, STOPS[j].1),
        mix(STOPS[i].2, STOPS[j].2),
    )
}


pub fn bolt_art() -> Vec<Line<'static>> {
    let indent = BOLT_ART
        .iter()
        .filter(|r| !r.trim().is_empty())
        .map(|r| r.len() - r.trim_start().len())
        .min()
        .unwrap_or(0);
    let rows: Vec<&str> = BOLT_ART
        .iter()
        .map(|r| &r[indent.min(r.len())..])
        .collect();
    let widest = rows.iter().map(|r| width(r)).max().unwrap_or(0);
    let last = (rows.len() - 1).max(1) as f32;
    rows.iter()
        .enumerate()
        .map(|(i, row)| {
            let padded = format!("{}{}", row, " ".repeat(widest - width(row)));
            Line::from(ratatui::text::Span::styled(
                padded,
                Style::default()
                    .fg(gradient_at(i as f32 / last))
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect()
}


pub fn wordmark() -> Vec<Line<'static>> {
    let widest = WORDMARK.iter().map(|r| width(r)).max().unwrap_or(0);
    let gradient = [BRAND_HI, BRAND, BRAND, BRAND_LO];
    WORDMARK
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let padded = format!("{}{}", row, " ".repeat(widest - width(row)));
            Line::from(ratatui::text::Span::styled(
                padded,
                Style::default()
                    .fg(gradient[i.min(gradient.len() - 1)])
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect()
}
//Some like greetings using ur time lmao for like top center part
pub fn greeting() -> &'static str {
    let hour = chrono::Local::now().hour();
    if hour < 5 {
        "still up?"
    } else if hour < 12 {
        "good morning"
    } else if hour < 18 {
        "good afternoon"
    } else if hour < 22 {
        "good evening"
    } else {
        "good night"
    }
}



pub const COLUMN_MAX: u16 = 118;


pub fn column(area: Rect) -> Rect {
    let margin = if area.width >= 56 { 4 } else { 0 };
    let avail = area.width.saturating_sub(margin * 2);
    let width = avail.min(COLUMN_MAX);
    Rect {
        x: area.x + margin + (avail - width) / 2,
        y: area.y,
        width,
        height: area.height,
    }
}

pub fn band(area: Rect, y: u16, height: u16) -> Rect {
    let bottom = area.y.saturating_add(area.height);
    let y = y.min(bottom);
    Rect {
        x: area.x,
        y,
        width: area.width,
        height: height.min(bottom.saturating_sub(y)),
    }
}

pub fn width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}


pub fn fit(s: &str, w: usize) -> String {
    if w == 0 {
        return String::new();
    }
    if width(s) <= w {
        return s.to_string();
    }
    let mut out = String::new();
    let mut used = 0;
    for ch in s.chars() {
        let cw = UnicodeWidthStr::width(ch.to_string().as_str());
        if used + cw > w.saturating_sub(1) {
            break;
        }
        out.push(ch);
        used += cw;
    }
    out.push('…');
    out
}


pub fn tail(s: &str, w: usize) -> String {
    if width(s) <= w {
        return s.to_string();
    }
    let mut out: Vec<char> = Vec::new();
    let mut used = 0;
    for ch in s.chars().rev() {
        let cw = UnicodeWidthStr::width(ch.to_string().as_str());
        if used + cw > w {
            break;
        }
        used += cw;
        out.push(ch);
    }
    out.reverse();
    format!("…{}", out.into_iter().collect::<String>())
}


pub fn pad(s: &str, w: usize) -> String {
    let s = fit(s, w);
    let gap = w.saturating_sub(width(&s));
    format!("{s}{}", " ".repeat(gap))
}


// ima test it since I CANT FINISH THE MAIN LOGIC ITS SO LONGGGGG I DONT EVEN GOT INSPO ANYMORE
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_is_centered_and_capped() {
        let wide = column(Rect::new(0, 0, 200, 40));
        assert_eq!(wide.width, COLUMN_MAX);
        assert_eq!(wide.x, (200 - COLUMN_MAX) / 2);

        let narrow = column(Rect::new(0, 0, 50, 20));
        assert_eq!(narrow.width, 50);
        assert_eq!(narrow.x, 0);
    }

    #[test]
    fn fit_and_pad_keep_cell_width() {
        assert_eq!(width(&fit("frieren beyond journey's end", 10)), 10);
        assert_eq!(width(&pad("ab", 6)), 6);
        assert_eq!(fit("short", 10), "short");
    }

    #[test]
    fn tail_keeps_the_end_of_a_long_query() {
        assert_eq!(tail("abc", 10), "abc");
        assert!(tail("a very long query", 6).ends_with("query"));
    }

    #[test]
    fn band_stays_inside_its_area() {
        let area = Rect::new(0, 5, 40, 4);
        let b = band(area, 12, 3);
        assert_eq!(b.y, 9);
        assert_eq!(b.height, 0);
    }
}
