//Main logic/app
//YH this gon be imports
mod catalog;
mod tui;
use std::iter::Copied;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier,Style};
use ratatui::text::{Line,Span};
use ratatui::widgets::{Block,Borders,Clear,Paragraph,Wrap};
use catalog::{Anime, Details, Profile, Rating, Rec};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
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
const PAGE: usize=10;

#[derive(Clone,Copy)]
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

    // There is going to be a lot of functions i think this is the best way to do it

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

    fn open_rating(&mut self, idx: usize) {
        let Some(anime) = self.anime(idx) else { return };
        let sel = self
            .profile
            .rating_of(anime.mal_id)
            .and_then(|r| Rating::ALL.iter().position(|x| *x == r))
            .unwrap_or(0);
        self.rating=Some(RatingDialog { idx, sel });
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

    n go(&mut self, screen: Screen) {
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
            //Keybinds for the tui forgot to add this before vibes
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
                    if let Some(idx)=self.current_idx() {
                        self.open_rating(idx);
                    }
                }
                KeyCode::Char('r')=> self.rebuild_recs(),
                KeyCode::Char('x')=> {
                    self.profile.favorites.clear();
                    self.profile.save(&self.profile_path);
                    self.rebuild_rec();
                }
                KeyCode::Char('d') => {
                    if let Some(idx)=self.current_idx() {
                        self.open_details(idx);
                    }
                }
                KeyCode::Esc => self.go(Screen::Search),
                _ => {}
            },
            Screen::Details => match key.code {
                KeyCode::Esc => {
                    let back=self.prev;
                    self.screen =back;
                }
                KeyCode::Char('s')  => {
                    if let Some(idx)=self.detail_idx {
                        self.open_rating(idx);
                    }
                }