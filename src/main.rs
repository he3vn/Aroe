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

    fn page_bounds(len:usize,sel:usize) -> (usize,usize) {
        let start=(sel/PAGE) *PAGE;
        (start, (start+PAGE).min(len))
