
mod catalog;
mod tui;

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use catalog::{Anime, Details, Profile, Rating, Rec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Search,
    Vibes,
    Picks,
    Details,
    Help,
}

impl Screen {
    fn index(self) -> usize {
        match self {
            Screen::Search => 0,
            Screen::Vibes => 1,
            Screen::Picks => 2,
            Screen::Details => 3,
            Screen::Help => 4,
        }
    }

    fn from_index(i: usize) -> Screen {
        match i {
            1 => Screen::Vibes,
            2 => Screen::Picks,
            3 => Screen::Details,
            4 => Screen::Help,
            _ => Screen::Search,
        }
    }
}

enum DetailState {
    Idle,
    Loading,
    Ready(Details),
    Failed,
}

const MENU: [Screen; 4] = [Screen::Vibes, Screen::Picks, Screen::Details, Screen::Help];

const PAGE: usize = 10;

#[derive(Clone, Copy)]
struct RatingDialog {
    idx: usize,
    sel: usize,
}

fn rating_key(rating: Rating) -> char {
    match rating {
        Rating::Amazing => 'a',
        Rating::Good => 'g',
        Rating::Meh => 'm',
        Rating::Bad => 'b',
    }
}

fn star_style(rating: Option<Rating>) -> Style {
    match rating {
        Some(Rating::Amazing) => tui::brand(),
        Some(_) => tui::brand2(),
        None => tui::text(),
    }
}

struct App {
    catalog: Option<Vec<Anime>>,
    catalog_path: PathBuf,
    profile: Profile,
    profile_path: PathBuf,
    screen: Screen,
    prev: Screen,
    query: String,
    hits: Vec<usize>,
    sel: usize,
    menu_focus: Option<usize>,
    vibe_sel: usize,
    vibe_picks: Vec<usize>,
    vibe_shown: Vec<String>,
    pick_sel: usize,
    recs: Vec<Rec>,
    detail_idx: Option<usize>,
    detail: DetailState,
    rx: Option<Receiver<(usize, Option<Details>)>>,
    rating: Option<RatingDialog>,
    scroll: u16,
    quit: bool,
}

impl App {
    fn new() -> Self {
        let catalog_path = catalog::default_catalog_path();
        let catalog = catalog::load(&catalog_path).ok();
        let profile_path = catalog::default_profile_path();
        let profile = Profile::load(&profile_path);
        let mut app = Self {
            catalog,
            catalog_path,
            profile,
            profile_path,
            screen: Screen::Search,
            prev: Screen::Search,
            query: String::new(),
            hits: Vec::new(),
            sel: 0,
            menu_focus: None,
            vibe_sel: 0,
            vibe_picks: Vec::new(),
            vibe_shown: Vec::new(),
            pick_sel: 0,
            recs: Vec::new(),
            detail_idx: None,
            detail: DetailState::Idle,
            rx: None,
            rating: None,
            scroll: 0,
            quit: false,
        };
        app.rebuild_recs();
        app
    }

    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> anyhow::Result<()> {
        loop {
            self.poll_details();
            terminal.draw(|f| self.draw(f))?;
            if self.quit {
                break;
            }
            if event::poll(Duration::from_millis(80))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.on_key(key);
                    }
                }
            }
        }
        Ok(())
    }


    fn anime(&self, idx: usize) -> Option<&Anime> {
        self.catalog.as_ref().and_then(|c| c.get(idx))
    }

    fn refresh_hits(&mut self) {
        let Some(catalog) = &self.catalog else { return };
        self.hits = catalog::search(catalog, &self.query, 200);
        self.sel = self.sel.min(self.hits.len().saturating_sub(1));
    }

    fn current_idx(&self) -> Option<usize> {
        match self.screen {
            Screen::Search => self.hits.get(self.sel).copied(),
            Screen::Vibes => self.vibe_picks.get(self.pick_sel).copied(),
            Screen::Picks => self.recs.get(self.pick_sel).map(|r| r.idx),
            Screen::Details => self.detail_idx,
            Screen::Help => None,
        }
    }
    //ima add a bunch of features like removing the picks and ratings etc

    fn open_rating(&mut self, idx: usize) {
        let Some(anime) = self.anime(idx) else { return };
        let sel = self
            .profile
            .rating_of(anime.mal_id)
            .and_then(|r| Rating::ALL.iter().position(|x| *x == r))
            .unwrap_or(0);
        self.rating = Some(RatingDialog { idx, sel });
    }

    fn pick_rating(&mut self, idx: usize, rating: Rating) {
        let Some(anime) = self.anime(idx).cloned() else { return };
        self.profile.set_rating(&anime, rating);
        self.profile.save(&self.profile_path);
        self.rating = None;
        self.rebuild_recs();
    }

    fn remove_pick(&mut self, idx: usize) {
        let Some(anime) = self.anime(idx).cloned() else { return };
        self.profile.remove(anime.mal_id);
        self.profile.save(&self.profile_path);
        self.rating = None;
        self.rebuild_recs();
    }

    fn page_bounds(len: usize, sel: usize) -> (usize, usize) {
        let start = (sel / PAGE) * PAGE;
        (start, (start + PAGE).min(len))
    }

    fn on_rating_key(&mut self, key: KeyEvent) {
        let Some(dialog) = self.rating else { return };
        let saved = self
            .anime(dialog.idx)
            .map(|a| self.profile.contains(a.mal_id))
            .unwrap_or(false);
        let rows = Rating::ALL.len() + usize::from(saved);
        match key.code {
            KeyCode::Esc => self.rating = None,
            KeyCode::Up | KeyCode::Char('k') => {
                let sel = dialog.sel.saturating_sub(1);
                self.rating = Some(RatingDialog { sel, ..dialog });
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let sel = (dialog.sel + 1).min(rows.saturating_sub(1));
                self.rating = Some(RatingDialog { sel, ..dialog });
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if dialog.sel < Rating::ALL.len() {
                    self.pick_rating(dialog.idx, Rating::ALL[dialog.sel]);
                } else {
                    self.remove_pick(dialog.idx);
                }
            }
            KeyCode::Char('a') => self.pick_rating(dialog.idx, Rating::Amazing),
            KeyCode::Char('g') => self.pick_rating(dialog.idx, Rating::Good),
            KeyCode::Char('m') => self.pick_rating(dialog.idx, Rating::Meh),
            KeyCode::Char('b') => self.pick_rating(dialog.idx, Rating::Bad),
            KeyCode::Char('x') if saved => self.remove_pick(dialog.idx),
            _ => {}
        }
    }

    fn rebuild_recs(&mut self) {
        let Some(catalog) = &self.catalog else { return };
        self.recs = catalog::recommend(catalog, &self.profile, 120);
        self.pick_sel = self.pick_sel.min(self.recs.len().saturating_sub(1));
    }

    fn go(&mut self, screen: Screen) {
        if screen != Screen::Details {
            self.prev = self.screen;
        }
        self.menu_focus = None;
        self.screen = screen;
        if screen == Screen::Vibes && self.vibe_picks.is_empty() {
            self.load_vibe(false);
        }
        if screen == Screen::Picks {
            self.rebuild_recs();
        }
    }

    fn load_vibe(&mut self, more: bool) {
        let Some(catalog) = &self.catalog else { return };
        if !more {
            self.vibe_shown.clear();
        }
        self.vibe_picks = catalog::vibe_picks(catalog, self.vibe_sel, &self.vibe_shown, 3);
        for idx in &self.vibe_picks {
            if let Some(a) = catalog.get(*idx) {
                self.vibe_shown.push(a.title.clone());
            }
        }
        self.pick_sel = 0;
    }

    fn open_details(&mut self, idx: usize) {
        let Some(anime) = self.anime(idx).cloned() else { return };
        if self.screen != Screen::Details {
            self.prev = self.screen;
        }
        self.detail_idx = Some(idx);
        self.screen = Screen::Details;
        self.scroll = 0;

        let key = catalog::details_key(&anime);
        if let Some(details) = catalog::cached_details(&key) {
            self.detail = DetailState::Ready(details);
            self.rx = None;
            return;
        }
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let fetched = catalog::fetch_details(&anime);
            let _ = tx.send((idx, fetched));
        });
        self.rx = Some(rx);
        self.detail = DetailState::Loading;
    }

    fn poll_details(&mut self) {
        let received = self.rx.as_ref().map(|rx| rx.try_recv());
        let Some(received) = received else { return };
        match received {
            Ok((idx, Some(details))) => {
                if let Some(anime) = self.catalog.as_ref().and_then(|c| c.get(idx)) {
                    catalog::store_details(&catalog::details_key(anime), &details);
                }
                self.detail = DetailState::Ready(details);
                self.rx = None;
            }
            Ok((_, None)) => {
                self.detail = DetailState::Failed;
                self.rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                if matches!(self.detail, DetailState::Loading) {
                    self.detail = DetailState::Failed;
                }
                self.rx = None;
            }
        }
    }


     //sTARTING ALL THE KEYBINDS STUFFS need them  i got an idea
    fn on_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('c') => {
                    self.quit = true;
                    return;
                }
                KeyCode::Char('o') => {
                    if let Some(idx) = self.current_idx() {
                        self.open_details(idx);
                    }
                    return;
                }
                _ => {}
            }
        }

        if let KeyCode::F(n) = key.code {
            if (1..=5).contains(&n) {
                self.go(Screen::from_index((n - 1) as usize));
                return;
            }
        }
        if self.rating.is_some() {
            self.on_rating_key(key);
            return;
        }
        if key.code == KeyCode::Tab {
            self.go(Screen::from_index((self.screen.index() + 1) % 5));
            return;
        }
        if key.code == KeyCode::Char('?') && self.screen != Screen::Search {
            if self.screen == Screen::Help {
                self.screen = self.prev;
            } else {
                self.go(Screen::Help);
            }
            return;
        }
        
        if self.screen != Screen::Search {
            if let KeyCode::Char(c @ '1'..='5') = key.code {
                self.go(Screen::from_index(c as usize - '1' as usize));
                return;
            }
        }
        //Setting up all the keybinds :(
        match self.screen {
            Screen::Search => match key.code {
                KeyCode::Esc => {
                    if self.menu_focus.take().is_none() {
                        self.query.clear();
                        self.hits.clear();
                        self.sel = 0;
                    }
                }
                KeyCode::Enter => {
                    if let Some(i) = self.menu_focus {
                        self.go(MENU[i]);
                    } else if let Some(idx) = self.current_idx() {
                        self.open_rating(idx);
                    }
                }
                KeyCode::Down => {
                    if self.query.is_empty() {
                        self.menu_focus = Some(match self.menu_focus {
                            None => 0,
                            Some(i) => (i + 1).min(MENU.len() - 1),
                        });
                    } else {
                        self.sel = (self.sel + 1).min(self.hits.len().saturating_sub(1));
                    }
                }
                KeyCode::Up => {
                    if self.query.is_empty() {
                        self.menu_focus = match self.menu_focus {
                            None | Some(0) => None,
                            Some(i) => Some(i - 1),
                        };
                    } else {
                        self.sel = self.sel.saturating_sub(1);
                    }
                }
                KeyCode::Left => self.sel = self.sel.saturating_sub(PAGE),
                KeyCode::Right => {
                    if !self.hits.is_empty() {
                        self.sel = (self.sel + PAGE).min(self.hits.len() - 1);
                    }
                }
                KeyCode::Backspace => {
                    self.menu_focus = None;
                    self.query.pop();
                    self.sel = 0;
                    self.refresh_hits();
                }
                KeyCode::Char(c) if !c.is_control() => {
                    self.menu_focus = None;
                    self.query.push(c);
                    self.sel = 0;
                    self.refresh_hits();
                }
                _ => {}
            },
            Screen::Vibes => match key.code {
                KeyCode::Down => {
                    self.pick_sel = (self.pick_sel + 1).min(self.vibe_picks.len().saturating_sub(1));
                }
                KeyCode::Up => self.pick_sel = self.pick_sel.saturating_sub(1),
                KeyCode::Left => {
                    self.vibe_sel = self.vibe_sel.saturating_sub(1);
                    self.load_vibe(false);
                }
                KeyCode::Right => {
                    self.vibe_sel = (self.vibe_sel + 1).min(catalog::VIBES.len() - 1);
                    self.load_vibe(false);
                }
                KeyCode::Enter => {
                    if let Some(idx) = self.current_idx() {
                        self.open_rating(idx);
                    }
                }
                KeyCode::Char('n') => self.load_vibe(true),
                KeyCode::Char('d') => {
                    if let Some(idx) = self.current_idx() {
                        self.open_details(idx);
                    }
                }
                KeyCode::Esc => self.go(Screen::Search),
                _ => {}
            },
            Screen::Picks => match key.code {
                KeyCode::Down => {
                    self.pick_sel = (self.pick_sel + 1).min(self.recs.len().saturating_sub(1));
                }
                KeyCode::Up => self.pick_sel = self.pick_sel.saturating_sub(1),
                KeyCode::Left => self.pick_sel = self.pick_sel.saturating_sub(PAGE),
                KeyCode::Right => {
                    if !self.recs.is_empty() {
                        self.pick_sel = (self.pick_sel + PAGE).min(self.recs.len() - 1);
                    }
                }
                KeyCode::Enter => {
                    if let Some(idx) = self.current_idx() {
                        self.open_rating(idx);
                    }
                }
                KeyCode::Char('r') => self.rebuild_recs(),
                KeyCode::Char('x') => {
                    self.profile.favorites.clear();
                    self.profile.save(&self.profile_path);
                    self.rebuild_recs();
                }
                KeyCode::Char('d') => {
                    if let Some(idx) = self.current_idx() {
                        self.open_details(idx);
                    }
                }
                KeyCode::Esc => self.go(Screen::Search),
                _ => {}
            },
            Screen::Details => match key.code {
                KeyCode::Esc => {
                    let back = self.prev;
                    self.screen = back;
                }
                KeyCode::Char('s') => {
                    if let Some(idx) = self.detail_idx {
                        self.open_rating(idx);
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(idx) = self.detail_idx {
                        self.open_details(idx);
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => self.scroll = self.scroll.saturating_add(1),
                KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
                _ => {}
            },
            Screen::Help => match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    let back = self.prev;
                    self.screen = back;
                }
                _ => {}
            },
        }
    }



    fn draw(&mut self, f: &mut Frame) {
        let area = f.area();
        f.render_widget(
            Block::default().style(Style::default().bg(tui::BG).fg(tui::FG)),
            area,
        );

        let col = tui::column(area);
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(6),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(col);

        self.draw_tabs(f, rows[0]);
        match self.screen {
            Screen::Search => self.draw_search(f, rows[2]),
            Screen::Vibes => self.draw_vibes(f, rows[2]),
            Screen::Picks => self.draw_picks(f, rows[2]),
            Screen::Details => self.draw_details(f, rows[2]),
            Screen::Help => self.draw_help(f, rows[2]),
        }
        if let Some(dialog) = self.rating {
            self.draw_rating_dialog(f, rows[2], dialog);
        }
        self.draw_hints(f, rows[3]);
        self.draw_footer(f, rows[4]);
    }

    fn draw_rating_dialog(&self, f: &mut Frame, area: Rect, dialog: RatingDialog) {
        let Some(anime) = self.anime(dialog.idx) else { return };
        let current = self.profile.rating_of(anime.mal_id);
        let saved = current.is_some();
        let rows = Rating::ALL.len() + usize::from(saved);
        let width = area.width.min(48);
        let height = rows as u16 + 2;
        if width < 24 || area.height < height {
            return;
        }
        let rect = Rect {
            x: area.x + (area.width - width) / 2,
            y: area.y + (area.height - height) / 2,
            width,
            height,
        };
        f.render_widget(Clear, rect);

        let mut lines: Vec<Line> = Vec::new();
        for (i, rating) in Rating::ALL.iter().enumerate() {
            let selected = i == dialog.sel;
            let mut spans = if selected {
                vec![Span::styled(
                    "▎",
                    Style::default().bg(tui::SEL_BG).fg(tui::BRAND),
                )]
            } else {
                vec![Span::raw(" ")]
            };
            let (label, note) = if selected {
                (
                    Style::default()
                        .bg(tui::SEL_BG)
                        .fg(tui::FG)
                        .add_modifier(Modifier::BOLD),
                    Style::default().bg(tui::SEL_BG).fg(tui::MUTED),
                )
            } else {
                (tui::text(), tui::faint())
            };
            spans.push(Span::styled(tui::pad(rating.label(), 11), label));
            let mark = if current == Some(*rating) { "current" } else { "" };
            spans.push(Span::styled(
                format!("{}   {}", rating_key(*rating), mark),
                note,
            ));
            lines.push(Line::from(spans));
        }
        if saved {
            let selected = dialog.sel == Rating::ALL.len();
            let base = if selected {
                Style::default().bg(tui::SEL_BG).fg(tui::FG)
            } else {
                tui::faint()
            };
            lines.push(Line::from(vec![
                Span::styled(
                    if selected { "▎" } else { " " },
                    if selected {
                        Style::default().bg(tui::SEL_BG).fg(tui::BRAND)
                    } else {
                        Style::default()
                    },
                ),
                Span::styled("remove pick", base),
                Span::styled(if selected { "   x" } else { "" }, base),
            ]));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(tui::brand2())
            .style(Style::default().bg(tui::BG))
            .title(Span::styled(
                format!(
                    " {} ",
                    tui::fit(&anime.title, (width as usize).saturating_sub(4))
                ),
                tui::brand(),
            ));
        f.render_widget(Paragraph::new(lines).block(block), rect);
    }

    fn draw_tabs(&self, f: &mut Frame, area: Rect) {
        let mut spans: Vec<Span> = Vec::new();
        for (i, name) in tui::TABS.iter().enumerate() {
            let active = i == self.screen.index();
            let text = format!("{} {}", i + 1, name);
            if active {
                spans.push(Span::styled(
                    format!(" {text} "),
                    Style::default()
                        .bg(tui::BRAND)
                        .fg(tui::BG)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(text, tui::dim()));
            }
            if i + 1 < tui::TABS.len() {
                spans.push(Span::raw("   "));
            }
        }
        f.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    fn draw_search(&self, f: &mut Frame, area: Rect) {
        let field_title = "search a title, or a genre like isekai";

        if self.query.is_empty() {
            let art = tui::bolt_art();
            let art_rows = art.len() as u16;
            let compact_total = 17u16;
            let full_total = compact_total + art_rows + 1;
            let full = area.height >= full_total;
            let total = if full { full_total } else { compact_total };
            let mut y = area
                .y
                .saturating_add(area.height.saturating_sub(total) / 2);

            f.render_widget(
                Paragraph::new(Line::from(Span::styled(tui::greeting(), tui::dim())))
                    .alignment(Alignment::Center),
                tui::band(area, y, 1),
            );
            y = y.saturating_add(2);

            if full {
                f.render_widget(
                    Paragraph::new(art).alignment(Alignment::Center),
                    tui::band(area, y, art_rows),
                );
                y = y.saturating_add(art_rows + 1);
            }

            f.render_widget(
                Paragraph::new(tui::wordmark()).alignment(Alignment::Center),
                tui::band(area, y, tui::WORDMARK_ROWS),
            );
            y = y.saturating_add(tui::WORDMARK_ROWS + 1);

        
            // A little more tui stuff this my catcher lol
            f.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    "you name it, the bolt fetches it",
                    tui::dim(),
                )))
                .alignment(Alignment::Center),
                tui::band(area, y, 1),
            );
            y = y.saturating_add(2);

            self.draw_field(f, tui::band(area, y, 3), field_title, true);
            y = y.saturating_add(4);

            self.draw_menu(f, tui::band(area, y, 4));
            return;
        }

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Min(1)])
            .split(area);

        self.draw_field(f, rows[0], field_title, true);
        self.draw_results(f, rows[2]);
    }

    // Menu bar stuff to start ima js cal it draw instead of select
    fn draw_menu(&self, f: &mut Frame, area: Rect) {
        let saved = self.profile.favorites.len();
        let rows: [(&str, String); 4] = [
            ("vibes", "pick a mood · three hand-picked starters".to_string()),
            (
                "picks",
                format!("{saved} saved · {} picks from your taste", self.recs.len()),
            ),
            ("details", "synopsis · streaming links from AniList".to_string()),
            ("help", "keys and about".to_string()),
        ];
        let lines: Vec<Line> = rows
            .iter()
            .enumerate()
            .map(|(i, (name, blurb))| {
                let selected = self.menu_focus == Some(i);
                let mut spans = Vec::new();
                if selected {
                    spans.push(Span::styled(
                        "▎ ",
                        Style::default().bg(tui::SEL_BG).fg(tui::BRAND),
                    ));
                } else {
                    spans.push(Span::raw("  "));
                }
                let (label, note) = if selected {
                    (
                        Style::default()
                            .bg(tui::SEL_BG)
                            .fg(tui::FG)
                            .add_modifier(Modifier::BOLD),
                        Style::default().bg(tui::SEL_BG).fg(tui::FG),
                    )
                } else {
                    (tui::text(), tui::dim())
                };
                spans.push(Span::styled(tui::pad(name, 12), label));
                spans.push(Span::styled(blurb.clone(), note));
                Line::from(spans)
            })
            .collect();
        f.render_widget(Paragraph::new(lines), area);
    }

    fn draw_field(&self, f: &mut Frame, area: Rect, title: &str, focused: bool) {
        if area.width < 10 || area.height < 3 {
            return;
        }
        let w = area.width as usize;
        let inner = w - 2;
        let label = format!(" {title} ");

        let mut top = vec![Span::styled("┌", tui::border()), Span::styled("╌", tui::border())];
        let lw = tui::width(&label);
        if lw + 1 < inner {
            top.push(Span::styled(label, tui::faint()));
            top.push(Span::styled("╌".repeat(inner - 1 - lw), tui::border()));
        } else {
            top.push(Span::styled("╌".repeat(inner - 1), tui::border()));
        }
        top.push(Span::styled("┐", tui::border()));
        f.render_widget(Paragraph::new(Line::from(top)), tui::band(area, area.y, 1));

        let mut mid = vec![
            Span::styled("╎", tui::border()),
            Span::raw(" "),
            Span::styled("> ", tui::brand()),
        ];
        let room = inner.saturating_sub(3);
        let cursor = focused || self.query.is_empty();
        let content;
        if self.query.is_empty() {
            // the border title already says what to type; keep the field bare
            content = 1;
            if cursor {
                mid.push(Span::styled("▏", tui::brand()));
            }
        } else {
            let shown = tui::tail(&self.query, room.saturating_sub(1));
            content = tui::width(&shown) + usize::from(cursor);
            mid.push(Span::styled(shown, tui::text()));
            if cursor {
                mid.push(Span::styled("▏", tui::brand()));
            }
        }
        let fill = inner.saturating_sub(3).saturating_sub(content);
        mid.push(Span::raw(" ".repeat(fill)));
        mid.push(Span::styled("╎", tui::border()));
        f.render_widget(
            Paragraph::new(Line::from(mid)),
            tui::band(area, area.y.saturating_add(1), 1),
        );

        let bottom = vec![
            Span::styled("└", tui::border()),
            Span::styled("╌".repeat(inner), tui::border()),
            Span::styled("┘", tui::border()),
        ];
        f.render_widget(
            Paragraph::new(Line::from(bottom)),
            tui::band(area, area.y.saturating_add(2), 1),
        );
    }

    fn result_widths(w: usize) -> [usize; 5] {
        let kind = 6;
        let eps = 4;
        let year = 5;
        let gaps = 8;
        // the leading cell is the selection gutter
        let rest = w.saturating_sub(1 + kind + eps + year + gaps);
        let title = (rest * 56 / 100).max(12).min(rest.saturating_sub(8));
        let genres = rest.saturating_sub(title);
        [title, kind, eps, year, genres]
    }

    fn table_row(
        cells: Vec<(String, Style)>,
        widths: &[usize],
        selected: bool,
    ) -> Line<'static> {
        let mut spans: Vec<Span> = Vec::new();
        if selected {
            spans.push(Span::styled(
                "▎",
                Style::default().bg(tui::SEL_BG).fg(tui::BRAND),
            ));
        } else {
            spans.push(Span::raw(" "));
        }
        let gap_style = if selected {
            Style::default().bg(tui::SEL_BG)
        } else {
            Style::default()
        };
        for (i, (cell, style)) in cells.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or(0);
            let mut st = *style;
            if selected {
                st = st.bg(tui::SEL_BG).fg(tui::FG).add_modifier(Modifier::BOLD);
            }
            spans.push(Span::styled(tui::pad(cell, width), st));
            if i + 1 < widths.len() {
                spans.push(Span::styled("  ", gap_style));
            }
        }
        Line::from(spans)
    }
    //I will be doing all the tabs here
    fn draw_results(&self, f: &mut Frame, area: Rect) {
        let widths = Self::result_widths(area.width as usize);
        let header = Self::table_row(
            vec![
                ("title".to_string(), tui::header()),
                ("type".to_string(), tui::header()),
                ("eps".to_string(), tui::header()),
                ("year".to_string(), tui::header()),
                ("genres".to_string(), tui::header()),
            ],
            &widths,
            false,
        );

        let mut lines = vec![header];
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            tui::border(),
        )));

        let (start, end) = Self::page_bounds(self.hits.len(), self.sel);
        let room = (area.height as usize).saturating_sub(3).max(1);
        if self.hits.is_empty() {
            lines.push(Line::from(Span::styled(
                "  no matches · try another title, or a genre like isekai",
                tui::faint(),
            )));
        } else {
            for (i, idx) in self
                .hits
                .iter()
                .enumerate()
                .skip(start)
                .take((end - start).min(room))
            {
                let Some(a) = self.anime(*idx) else { continue };
                let rating = self.profile.rating_of(a.mal_id);
                let title = if rating.is_some() {
                    format!("★ {}", a.title)
                } else {
                    a.title.clone()
                };
                lines.push(Self::table_row(
                    vec![
                        (title, star_style(rating)),
                        (a.kind.clone(), tui::dim()),
                        (
                            a.episodes
                                .map(|e| e.to_string())
                                .unwrap_or_else(|| "—".into()),
                            tui::dim(),
                        ),
                        (
                            a.year
                                .map(|y| y.to_string())
                                .unwrap_or_else(|| "—".into()),
                            tui::dim(),
                        ),
                        (a.genres.iter().take(3).cloned().collect::<Vec<_>>().join(", "), tui::faint()),
                    ],
                    &widths,
                    i == self.sel,
                ));
            }
        }
        f.render_widget(Paragraph::new(lines), area);
    }
    //Doing picks tab :sob 
    fn draw_vibes(&self, f: &mut Frame, area: Rect) {
        let mut lines: Vec<Line> = Vec::new();

        let mut chips: Vec<Span> = vec![Span::styled("  mood  ", tui::faint())];
        for (i, (name, _)) in catalog::VIBES.iter().enumerate() {
            if i > 0 {
                chips.push(Span::styled("  ·  ", tui::faint()));
            }
            if i == self.vibe_sel {
                chips.push(Span::styled(
                    format!(" {name} "),
                    Style::default()
                        .bg(tui::BRAND)
                        .fg(tui::BG)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                chips.push(Span::styled(*name, tui::dim()));
            }
        }
        lines.push(Line::from(chips));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  starters  ", tui::faint()),
            Span::styled("three at a time · n for more", tui::faint()),
        ]));

        let widths = Self::result_widths(area.width as usize);
        if self.vibe_picks.is_empty() {
            lines.push(Line::from(Span::styled(
                "  nothing matched this mood in the catalog",
                tui::faint(),
            )));
        }
        for (i, idx) in self.vibe_picks.iter().enumerate() {
            let Some(a) = self.anime(*idx) else { continue };
            let rating = self.profile.rating_of(a.mal_id);
            lines.push(Self::table_row(
                vec![
                    (
                        if rating.is_some() {
                            format!("★ {}", a.title)
                        } else {
                            a.title.clone()
                        },
                        star_style(rating),
                    ),
                    (a.kind.clone(), tui::dim()),
                    (
                        a.episodes
                            .map(|e| e.to_string())
                            .unwrap_or_else(|| "—".into()),
                        tui::dim(),
                    ),
                    (
                        a.year
                            .map(|y| y.to_string())
                            .unwrap_or_else(|| "—".into()),
                        tui::dim(),
                    ),
                    (a.genres.iter().take(3).cloned().collect::<Vec<_>>().join(", "), tui::faint()),
                ],
                &widths,
                i == self.pick_sel,
            ));
        }
        f.render_widget(Paragraph::new(lines), area);
    }
    //Doing picks tab
    fn draw_picks(&self, f: &mut Frame, area: Rect) {
        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(vec![
            Span::styled("  picks  ", tui::header()),
            Span::styled(
                format!(
                    "{} saved · {} from your taste",
                    self.profile.favorites.len(),
                    self.recs.len()
                ),
                tui::dim(),
            ),
        ]));
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            tui::border(),
        )));

        if self.recs.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  no picks yet · save a few favourites from search, then press r",
                tui::faint(),
            )));
            f.render_widget(Paragraph::new(lines), area);
            return;
        }

        let w = area.width as usize;
        let score_w = 6;
        let kind_w = 6;
        let rest = w.saturating_sub(1 + score_w + kind_w + 8);
        let title_w = (rest * 46 / 100).max(12);
        let why_w = rest.saturating_sub(title_w);
        let widths = [title_w, why_w, kind_w, score_w];

        let header = Self::table_row(
            vec![
                ("title".to_string(), tui::header()),
                ("why it fits".to_string(), tui::header()),
                ("type".to_string(), tui::header()),
                ("score".to_string(), tui::header()),
            ],
            &widths,
            false,
        );
        lines.push(header);

        let (start, end) = Self::page_bounds(self.recs.len(), self.pick_sel);
        let room = (area.height as usize).saturating_sub(4).max(1);
        for (i, rec) in self
            .recs
            .iter()
            .enumerate()
            .skip(start)
            .take((end - start).min(room))
        {
            let Some(a) = self.anime(rec.idx) else { continue };
            let rating = self.profile.rating_of(a.mal_id);
            lines.push(Self::table_row(
                vec![
                    (
                        if rating.is_some() {
                            format!("★ {}", a.title)
                        } else {
                            a.title.clone()
                        },
                        star_style(rating),
                    ),
                    (rec.shared.join(" · "), tui::faint()),
                    (a.kind.clone(), tui::dim()),
                    (format!("{:.1}", rec.score), tui::brand2()),
                ],
                &widths,
                i == self.pick_sel,
            ));
        }
        f.render_widget(Paragraph::new(lines), area);
    }
    //Doing details tabs
    fn draw_details(&self, f: &mut Frame, area: Rect) {
        let Some(anime) = self.detail_idx.and_then(|i| self.anime(i)) else {
            f.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    "  open a title from search, vibes or picks to see it here",
                    tui::faint(),
                ))),
                area,
            );
            return;
        };

        let mut lines: Vec<Line> = Vec::new();
        lines.push(Line::from(Span::styled(
            format!("  {}", anime.title),
            tui::brand(),
        )));

        let mut meta = vec![Span::raw("  "), Span::styled(anime.kind.clone(), tui::dim())];
        if let Some(year) = anime.year {
            meta.push(Span::styled(format!("  ·  {year}"), tui::dim()));
        }
        if let Some(eps) = anime.episodes {
            meta.push(Span::styled(format!("  ·  {eps} eps"), tui::dim()));
        }
        meta.push(Span::styled(format!("  ·  mal {}", anime.mal_id), tui::faint()));
        if let Some(id) = anime.anilist_id {
            meta.push(Span::styled(format!("  ·  anilist {id}"), tui::faint()));
        }
        lines.push(Line::from(meta));

        if !anime.genres.is_empty() {
            let shown: Vec<&str> = anime.genres.iter().take(8).map(|g| g.as_str()).collect();
            let extra = anime.genres.len().saturating_sub(shown.len());
            let mut tags = shown.join(" · ");
            if extra > 0 {
                tags.push_str(&format!(" · +{extra} more"));
            }
            lines.push(Line::from(vec![
                Span::styled("  genres  ", tui::faint()),
                Span::styled(tags, tui::dim()),
            ]));
        }
        let rating = self.profile.rating_of(anime.mal_id);
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                match rating {
                    Some(r) => format!("you rated this {} · s changes it", r.label()),
                    None => "not rated yet · s rates it".to_string(),
                },
                if rating.is_some() {
                    star_style(rating)
                } else {
                    tui::faint()
                },
            ),
        ]));
        lines.push(Line::from(""));

        match &self.detail {
            DetailState::Loading => {
                lines.push(Line::from(Span::styled(
                    "  fetching synopsis and streams from AniList…",
                    tui::faint(),
                )));
            }
            DetailState::Failed => {
                lines.push(Line::from(Span::styled(
                    "  could not reach AniList · r retries",
                    tui::dim(),
                )));
            }
            DetailState::Idle => {
                lines.push(Line::from(Span::styled(
                    "  press r to fetch the synopsis",
                    tui::faint(),
                )));
            }
            DetailState::Ready(details) => {
                lines.push(Line::from(Span::styled("  synopsis", tui::header())));
                if details.synopsis.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "  no synopsis on AniList for this title",
                        tui::faint(),
                    )));
                } else {
                    for row in details.synopsis.lines() {
                        lines.push(Line::from(Span::styled(
                            format!("  {}", row.trim()),
                            tui::dim(),
                        )));
                    }
                }
                if !details.streams.is_empty() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled("  streams", tui::header())));
                    for (site, url) in &details.streams {
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(tui::pad(site, 16), tui::text()),
                            Span::styled(url.clone(), tui::faint()),
                        ]));
                    }
                }
            }
        }

        f.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((self.scroll, 0)),
            area,
        );
    }

    // FINALLY THE EHLP TAB IM SO DONEEE
    fn draw_help(&self, f: &mut Frame, area: Rect) {
        let sections: [(&str, &[(&str, &str)]); 5] = [
            (
                "global",
                &[
                    ("1-5", "switch screen"),
                    ("tab", "next screen"),
                    ("?", "help"),
                    ("ctrl+c", "quit"),
                ],
            ),
            (
                "search",
                &[
                    ("type", "filter the catalog"),
                    ("up / down", "move"),
                    ("enter", "save or remove a pick"),
                    ("ctrl+o", "details"),
                    ("esc", "clear"),
                ],
            ),
            (
                "vibes",
                &[
                    ("left / right", "change mood"),
                    ("up / down", "choose a starter"),
                    ("n", "three more"),
                    ("enter", "save a pick"),
                    ("d", "details"),
                ],
            ),
            (
                "picks",
                &[
                    ("r", "rebuild from your taste"),
                    ("enter", "save one"),
                    ("x", "clear your picks"),
                    ("d", "details"),
                ],
            ),
            (
                "details",
                &[
                    ("s", "save or remove"),
                    ("r", "refetch from AniList"),
                    ("j / k", "scroll"),
                    ("esc", "back"),
                ],
            ),
        ];

        let mut lines: Vec<Line> = Vec::new();
        for (i, (name, rows)) in sections.iter().enumerate() {
            if i > 0 {
                lines.push(Line::from(""));
            }
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(*name, tui::header()),
            ]));
            for (key, label) in rows.iter() {
                lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled(tui::pad(key, 12), tui::brand2()),
                    Span::styled(*label, tui::dim()),
                ]));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("aroe", tui::brand2()),
            Span::styled(" · a cozy terminal anime recommender · your picks stay local", tui::faint()),
        ]));

        f.render_widget(Paragraph::new(lines), area);
    }
    //Ima make some hints for the reviewers

    fn draw_hints(&self, f: &mut Frame, area: Rect) {
        let parts: Vec<(&str, &str)> = match self.screen {
            Screen::Search => vec![
                ("type", "search"),
                ("↓", "menu"),
                ("←→", "page"),
                ("enter", "rate"),
                ("ctrl+o", "details"),
            ],
            Screen::Vibes => vec![
                ("←→", "mood"),
                ("n", "more"),
                ("enter", "rate"),
                ("d", "details"),
            ],
            Screen::Picks => vec![
                ("←→", "page"),
                ("r", "rebuild"),
                ("enter", "rate"),
                ("x", "clear"),
                ("d", "details"),
            ],
            Screen::Details => vec![
                ("s", "rate"),
                ("r", "refetch"),
                ("j/k", "scroll"),
                ("esc", "back"),
            ],
            Screen::Help => vec![("esc", "back"), ("tab", "screens"), ("ctrl+c", "quit")],
        };

        let mut spans: Vec<Span> = vec![Span::raw(" ")];
        for (i, (key, label)) in parts.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled("   ", tui::faint()));
            }
            spans.push(Span::styled(*key, tui::brand2()));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(*label, tui::faint()));
        }
        f.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    fn draw_footer(&self, f: &mut Frame, area: Rect) {
        let (label, value) = match self.screen {
            Screen::Search | Screen::Help => (
                "catalog",
                match &self.catalog {
                    Some(c) => format!(
                        "{} · {} titles",
                        short_path(&self.catalog_path),
                        c.len()
                    ),
                    None => format!(
                        "{} · catalog missing, set AROE_DATA",
                        short_path(&self.catalog_path)
                    ),
                },
            ),
            _ => (
                "profile",
                format!(
                    "{} · {} saved",
                    short_path(&self.profile_path),
                    self.profile.favorites.len()
                ),
            ),
        };
        let mut spans = vec![
            Span::raw(" "),
            Span::styled(label, tui::label()),
            Span::raw("   "),
            Span::styled(value.clone(), tui::dim()),
        ];
        if let Some(pager) = self.pager_text() {
            let used = 1 + tui::width(label) + 3 + tui::width(&value) + tui::width(&pager) + 1;
            let gap = (area.width as usize).saturating_sub(used);
            spans.push(Span::raw(" ".repeat(gap)));
            spans.push(Span::styled(pager, tui::faint()));
        }
        f.render_widget(Paragraph::new(Line::from(spans)), area);
    }

    /// "11-20 of 240 · page 2/24" for the screens that page ten at a time.
    fn pager_text(&self) -> Option<String> {
        let (len, sel) = match self.screen {
            Screen::Search if !self.hits.is_empty() => (self.hits.len(), self.sel),
            Screen::Picks if !self.recs.is_empty() => (self.recs.len(), self.pick_sel),
            _ => return None,
        };
        let (start, end) = Self::page_bounds(len, sel);
        Some(format!(
            "{}-{} of {}  ·  page {}/{}",
            start + 1,
            end,
            len,
            start / PAGE + 1,
            len.div_ceil(PAGE)
        ))
    }
}

fn short_path(path: &std::path::Path) -> String {
    let text = path.display().to_string();
    match std::env::var("HOME") {
        Ok(home) if !home.is_empty() => text.replace(&home, "~"),
        _ => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn row_text(buf: &ratatui::buffer::Buffer, y: u16) -> String {
        (0..buf.area.width)
            .map(|x| buf[(x, y)].symbol().to_string())
            .collect()
    }

    #[test]
    fn active_tab_and_banner_are_painted() {
        let mut app = App::new();
        let mut terminal = Terminal::new(TestBackend::new(120, 50)).unwrap();
        terminal.draw(|f| app.draw(f)).unwrap();
        let buf = terminal.backend().buffer().clone();

        let tabs = row_text(&buf, 0);
        assert!(tabs.contains("1 search"), "tab bar: {tabs:?}");
        assert!(tabs.contains("5 help"), "tab bar: {tabs:?}");
        let x = tabs.find("1 search").expect("search tab") as u16;
        assert_eq!(buf[(x, 0)].style().bg, Some(tui::BRAND));
        assert_eq!(buf[(x, 0)].style().fg, Some(tui::BG));

        let mut found_art = false;
        let mut found_letters = false;
        for y in 0..buf.area.height {
            let line = row_text(&buf, y);
            if line.contains("=*********#=") {
                found_art = true;
            }
            if line.contains("▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▄ █▀▀▀▀") {
                found_letters = true;
            }
        }
        assert!(found_art, "the bolt art should be drawn");
        assert!(found_letters, "the wordmark should render intact");
    }

    #[test]
    fn pages_hold_ten_rows() {
        assert_eq!(App::page_bounds(0, 0), (0, 0));
        assert_eq!(App::page_bounds(240, 0), (0, 10));
        assert_eq!(App::page_bounds(240, 13), (10, 20));
        assert_eq!(App::page_bounds(23, 22), (20, 23));
    }

    #[test]
    fn rating_dialog_saves_the_choice() {
        let mut app = App::new();
        app.profile_path = std::env::temp_dir().join("aroe-test-profile.json");
        let _ = std::fs::remove_file(&app.profile_path);
        app.profile = catalog::Profile::default();
        app.query = "frieren".to_string();
        app.refresh_hits();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.rating.is_some(), "enter should open the rating chooser");

        app.on_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(app.rating.is_none(), "rating closes the chooser");

        let idx = app.hits[0];
        let mal_id = app.catalog.as_ref().unwrap()[idx].mal_id;
        assert_eq!(app.profile.rating_of(mal_id), Some(catalog::Rating::Amazing));

        // rating again from the chooser replaces it, and the file round-trips
        app.open_rating(idx);
        assert_eq!(app.rating.map(|d| d.sel), Some(0));
        app.on_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        assert_eq!(app.profile.rating_of(mal_id), Some(catalog::Rating::Bad));
        let saved = catalog::Profile::load(&app.profile_path);
        assert_eq!(saved.rating_of(mal_id), Some(catalog::Rating::Bad));

        let _ = std::fs::remove_file(&app.profile_path);
    }

    #[test]
    fn down_walks_the_menu_and_enter_opens_it() {
        let mut app = App::new();
        let down = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        let up = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let enter = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);

        app.on_key(down);
        assert_eq!(app.menu_focus, Some(0));
        app.on_key(down);
        assert_eq!(app.menu_focus, Some(1));
        app.on_key(enter);
        assert_eq!(app.screen, Screen::Picks);

        // back on search, the menu starts unfocused and returns there on up
        app.screen = Screen::Search;
        app.menu_focus = None;
        app.on_key(down);
        app.on_key(up);
        assert_eq!(app.menu_focus, None);
        assert_eq!(app.query, "");
    }
}

fn main() -> anyhow::Result<()> {
    let arg = std::env::args().nth(1).unwrap_or_default();
    match arg.as_str() {
        "-V" | "--version" | "version" => {
            println!("aroe {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        "stats" => {
            let path = catalog::default_catalog_path();
            let list = catalog::load(&path)?;
            println!("catalog {} · {} titles", path.display(), list.len());
            return Ok(());
        }
        "details" => {
            let Some(title) = std::env::args().nth(2) else {
                println!("usage: aroe details <title>");
                return Ok(());
            };
            let path = catalog::default_catalog_path();
            let list = catalog::load(&path)?;
            let Some(idx) = catalog::find(&list, &title) else {
                println!("no match for {title:?} in the catalog");
                return Ok(());
            };
            let anime = &list[idx];
            println!("{} · {} · {}", anime.title, anime.kind, anime.year.map(|y| y.to_string()).unwrap_or_else(|| "year?".into()));
            match catalog::cached_details(&catalog::details_key(anime)).or_else(|| catalog::fetch_details(anime)) {
                Some(details) => {
                    catalog::store_details(&catalog::details_key(anime), &details);
                    if !details.synopsis.is_empty() {
                        println!("\n{}", details.synopsis);
                    }
                    if !details.streams.is_empty() {
                        println!();
                        for (site, url) in &details.streams {
                            println!("{site:<16}{url}");
                        }
                    }
                }
                None => println!("\ncould not reach AniList for this title"),
            }
            return Ok(());
        }
        "-h" | "--help" | "help" => {
            println!(
                "aroe — a cozy terminal anime recommender\n\n\
                 usage: aroe                  open the tui\n\
                 \x20      aroe stats            catalog path + title count\n\
                 \x20      aroe details <title>  synopsis + streams for one title\n\n\
                 in the tui: 1-5 or tab for screens, ? for help, ctrl+c to quit\n"
            );
            return Ok(());
        }
        _ => {}
    }

    let mut app = App::new();
    let mut terminal = ratatui::init();
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
