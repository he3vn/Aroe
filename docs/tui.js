/* ==========================================================================
   aroe — Authentic Ratatui TUI Web Emulator
   Faithfully reproduces the Rust terminal UI from main.rs & tui.rs.
   Pure Vanilla JS, 0 dependencies, 60fps, ultra-lightweight.
   ========================================================================== */

(function () {
  'use strict';

  // --- Real Anime Dataset from Aroe ---
  const CATALOG = [
    { mal_id: 52991, title: 'Sousou no Frieren', kind: 'TV', episodes: 28, year: 2023, score: 9.35, genres: ['Adventure', 'Drama', 'Fantasy'], desc: 'The adventure is over, but life goes on for an elf mage just beginning to learn what living is all about.' },
    { mal_id: 1, title: 'Cowboy Bebop', kind: 'TV', episodes: 26, year: 1998, score: 8.92, genres: ['Action', 'Drama', 'Sci-Fi'], desc: 'The easygoing bounty hunter Spike Spiegel and his partner Jet Black cruise the solar system.' },
    { mal_id: 9253, title: 'Steins;Gate', kind: 'TV', episodes: 24, year: 2011, score: 9.07, genres: ['Drama', 'Sci-Fi', 'Suspense'], desc: 'Self-proclaimed mad scientist Rintarou Okabe accidentally discovers the ability to send text messages to the past.' },
    { mal_id: 16498, title: 'Attack on Titan', kind: 'TV', episodes: 25, year: 2013, score: 8.54, genres: ['Action', 'Award Winning', 'Drama', 'Fantasy'], desc: 'After his hometown is destroyed, Eren Jaeger vows to cleanse the earth of the giant humanoid Titans.' },
    { mal_id: 51009, title: 'Jujutsu Kaisen Season 2', kind: 'TV', episodes: 23, year: 2023, score: 8.79, genres: ['Action', 'Fantasy'], desc: 'Satoru Gojou and Suguru Getou are tasked with escorting the Star Plasma Vessel.' },
    { mal_id: 40748, title: 'Jujutsu Kaisen', kind: 'TV', episodes: 24, year: 2020, score: 8.62, genres: ['Action', 'Fantasy'], desc: 'Yuuji Itadori swallows a cursed talisman to save his friends and becomes host to Sukuna.' },
    { mal_id: 37521, title: 'Vinland Saga', kind: 'TV', episodes: 24, year: 2019, score: 8.73, genres: ['Action', 'Adventure', 'Drama'], desc: 'Young Thorfinn grows up listening to stories of old sailors and seeks revenge against Askeladd.' },
    { mal_id: 48583, title: 'Attack on Titan: The Final Season', kind: 'TV', episodes: 16, year: 2021, score: 8.78, genres: ['Action', 'Drama'], desc: 'The struggle between Eldia and Marley reaches its climax as Eren executes his ultimate plan.' },
    { mal_id: 50709, title: 'Bocchi the Rock!', kind: 'TV', episodes: 12, year: 2022, score: 8.81, genres: ['Comedy', 'Music', 'Slice of Life'], desc: 'Socially anxious guitarist Hitori Gotou joins a band called Kessoku Band.' },
    { mal_id: 50172, title: 'Mob Psycho 100 III', kind: 'TV', episodes: 12, year: 2022, score: 8.94, genres: ['Action', 'Comedy', 'Supernatural'], desc: 'Mob continues his daily life while deciding on his high school career and future path.' },
    { mal_id: 32182, title: 'Mob Psycho 100', kind: 'TV', episodes: 12, year: 2016, score: 8.48, genres: ['Action', 'Comedy', 'Supernatural'], desc: 'Shigeo Kageyama, an 8th grader with immense psychic powers, strives to live a normal life.' },
    { mal_id: 50602, title: 'Chainsaw Man', kind: 'TV', episodes: 12, year: 2022, score: 8.52, genres: ['Action', 'Fantasy'], desc: 'Denji makes a pact with a devil dog named Pochita to become Chainsaw Man.' },
    { mal_id: 50265, title: 'Spy x Family', kind: 'TV', episodes: 12, year: 2022, score: 8.51, genres: ['Action', 'Comedy'], desc: 'A spy must build a fake family to execute a top secret mission, unaware his wife is an assassin and daughter a telepath.' },
    { mal_id: 38000, title: 'Demon Slayer: Kimetsu no Yaiba', kind: 'TV', episodes: 26, year: 2019, score: 8.48, genres: ['Action', 'Fantasy'], desc: 'Tanjiro sets out on a perilous quest to cure his sister Nezuko and avenge his family.' },
    { mal_id: 39535, title: 'Mushoku Tensei: Jobless Reincarnation', kind: 'TV', episodes: 11, year: 2021, score: 8.36, genres: ['Adventure', 'Drama', 'Fantasy', 'Isekai'], desc: 'A 34-year-old shut-in is reborn in a magical world as Rudeus Greyrat with memories intact.' },
    { mal_id: 48569, title: 'Odd Taxi', kind: 'TV', episodes: 13, year: 2021, score: 8.68, genres: ['Drama', 'Mystery'], desc: 'Odokawa is an eccentric walrus taxi driver who gets caught up in the mystery of a missing high school girl.' },
    { mal_id: 32281, title: 'Your Name.', kind: 'Movie', episodes: 1, year: 2016, score: 8.84, genres: ['Award Winning', 'Drama', 'Supernatural'], desc: 'High school students Mitsuha and Taki mysteriously begin swapping bodies across time.' },
    { mal_id: 28851, title: 'A Silent Voice', kind: 'Movie', episodes: 1, year: 2016, score: 8.93, genres: ['Award Winning', 'Drama'], desc: 'A former bully seeks redemption and reconciliation with the deaf girl he tormented in elementary school.' },
    { mal_id: 33352, title: 'Violet Evergarden', kind: 'TV', episodes: 13, year: 2018, score: 8.67, genres: ['Drama', 'Fantasy'], desc: 'A former soldier girl seeks to understand the meaning of "I love you" by working as an Auto Memory Doll.' },
    { mal_id: 30, title: 'Neon Genesis Evangelion', kind: 'TV', episodes: 26, year: 1995, score: 8.35, genres: ['Action', 'Avant Garde', 'Drama', 'Sci-Fi'], desc: 'Shinji Ikari is drafted by his estranged father to pilot a giant bio-machine called an Evangelion.' },
  ];

  // Curated Vibes matching catalog.rs lines 435-520
  const VIBES = [
    { name: 'action', titles: ['Jujutsu Kaisen', 'Sousou no Frieren', 'Attack on Titan', 'Vinland Saga', 'Chainsaw Man'] },
    { name: 'chill', titles: ['Bocchi the Rock!', 'Spy x Family', 'Violet Evergarden', 'Odd Taxi'] },
    { name: 'deep', titles: ['Steins;Gate', 'Neon Genesis Evangelion', 'Cowboy Bebop', 'Vinland Saga'] },
    { name: 'isekai', titles: ['Mushoku Tensei: Jobless Reincarnation', 'Sousou no Frieren'] },
    { name: 'beginner', titles: ['Spy x Family', 'Demon Slayer: Kimetsu no Yaiba', 'Your Name.', 'Attack on Titan'] },
    { name: 'short', titles: ['Bocchi the Rock!', 'Chainsaw Man', 'Odd Taxi', 'Violet Evergarden'] },
  ];

  // Bolt art from tui.rs line 55
  const BOLT_ART = [
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

  // Wordmark from tui.rs line 77
  const WORDMARK = [
    "▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▄ █▀▀▀▀",
    "█   █ █▄▄▄▀ █   █ █▄▄▄ ",
    "█▀▀▀█ █  ▀▄ █   █ █    ",
    "█   █ █   █ ▀▄▄▄▀ █▄▄▄▄",
  ];

  // Dynamic greeting from tui.rs line 147
  function getGreeting() {
    const hour = new Date().getHours();
    if (hour < 5) return 'still up?';
    if (hour < 12) return 'good morning';
    if (hour < 18) return 'good afternoon';
    if (hour < 22) return 'good evening';
    return 'good night';
  }

  // --- TUI Application State (matching struct App in main.rs) ---
  const state = {
    screen: 0, // 0: Search, 1: Vibes, 2: Picks, 3: Details, 4: Help
    query: '',
    hits: [],
    sel: 0, // Table selection
    menuFocus: null, // Menu row focus on Search screen
    vibeSel: 0, // Selected vibe
    selectedAnimeIdx: 0,
    ratings: {
      52991: 'amazing', // Frieren rated amazing by default
      1: 'good',        // Cowboy Bebop rated good
    },
    ratingDialog: null, // { anime, sel: 0 }
  };

  const container = document.getElementById('tui-container');
  if (!container) return;

  // --- Search function matching search() in catalog.rs ---
  function performSearch(q) {
    if (!q.trim()) return [];
    const query = q.toLowerCase().trim();
    return CATALOG.filter(a =>
      a.title.toLowerCase().includes(query) ||
      a.genres.some(g => g.toLowerCase().includes(query)) ||
      (a.year && String(a.year).includes(query))
    );
  }

  // --- Render Functions (matching draw() in main.rs) ---
  function render() {
    state.hits = performSearch(state.query);
    if (state.sel >= state.hits.length) {
      state.sel = Math.max(0, state.hits.length - 1);
    }

    let html = '';

    // 1. Tabs bar (draw_tabs in main.rs)
    const tabs = ['search', 'vibes', 'picks', 'details', 'help'];
    html += '<div class="tui-tabs-bar">';
    tabs.forEach((name, idx) => {
      const active = state.screen === idx ? 'active' : '';
      html += `<div class="tui-tab ${active}" data-tab="${idx}">${idx + 1} ${name}</div>`;
    });
    html += '</div>';

    // 2. Main content per screen
    switch (state.screen) {
      case 0:
        html += renderSearchScreen();
        break;
      case 1:
        html += renderVibesScreen();
        break;
      case 2:
        html += renderPicksScreen();
        break;
      case 3:
        html += renderDetailsScreen();
        break;
      case 4:
        html += renderHelpScreen();
        break;
    }

    // 3. Rating Dialog popup if active
    if (state.ratingDialog) {
      html += renderRatingDialog();
    }

    // 4. Status Bar Hints (draw_hints in main.rs)
    html += `
      <div class="tui-status-bar">
        <span>tab switch screen · / search · ↑↓ navigate · enter select · ? help</span>
        <span>${Object.keys(state.ratings).length} saved · Rust CLI</span>
      </div>
    `;

    container.innerHTML = html;
    attachListeners();
  }

  // --- Render Search Screen (draw_search in main.rs) ---
  function renderSearchScreen() {
    let out = '';

    // Search input box
    out += `
      <div class="tui-box-wrap">
        <div class="tui-box">
          <span class="tui-box-prompt">&gt;</span>
          <input type="text" id="tui-search-input" class="tui-box-input" value="${escapeHtml(state.query)}" placeholder="search a title, or a genre like isekai..." autofocus spellcheck="false" autocomplete="off"/>
        </div>
      </div>
    `;

    if (!state.query.trim()) {
      // Idle search screen: Greeting + Bolt Art + Wordmark + Menu
      out += `
        <div class="tui-center-area">
          <div class="tui-greeting">${getGreeting()}</div>
          <div class="tui-bolt-art">${BOLT_ART.join('\n')}</div>
          <div class="tui-wordmark">${WORDMARK.join('\n')}</div>
          <div class="tui-tagline">you name it, the bolt fetches it</div>

          <div class="tui-menu">
            <div class="tui-menu-item ${state.menuFocus === 0 ? 'selected' : ''}" data-nav-screen="1">
              <span class="tui-menu-gutter">${state.menuFocus === 0 ? '▎' : ' '}</span>
              <span class="tui-menu-name">vibes</span>
              <span class="tui-menu-desc">pick a mood · three hand-picked starters</span>
            </div>
            <div class="tui-menu-item ${state.menuFocus === 1 ? 'selected' : ''}" data-nav-screen="2">
              <span class="tui-menu-gutter">${state.menuFocus === 1 ? '▎' : ' '}</span>
              <span class="tui-menu-name">picks</span>
              <span class="tui-menu-desc">${Object.keys(state.ratings).length} saved · recommendations from your taste</span>
            </div>
            <div class="tui-menu-item ${state.menuFocus === 2 ? 'selected' : ''}" data-nav-screen="3">
              <span class="tui-menu-gutter">${state.menuFocus === 2 ? '▎' : ' '}</span>
              <span class="tui-menu-name">details</span>
              <span class="tui-menu-desc">synopsis · streaming links from AniList</span>
            </div>
            <div class="tui-menu-item ${state.menuFocus === 3 ? 'selected' : ''}" data-nav-screen="4">
              <span class="tui-menu-gutter">${state.menuFocus === 3 ? '▎' : ' '}</span>
              <span class="tui-menu-name">help</span>
              <span class="tui-menu-desc">keys and about</span>
            </div>
          </div>
        </div>
      `;
    } else {
      // Results Table (draw_results in main.rs)
      if (state.hits.length === 0) {
        out += `<div style="padding: 24px 0; color: #6e684e; text-align: center;">no matches · try another title, or a genre like isekai</div>`;
      } else {
        out += `
          <table class="tui-results-table">
            <thead>
              <tr>
                <th style="width: 44%;">title</th>
                <th style="width: 10%;">type</th>
                <th style="width: 10%;">eps</th>
                <th style="width: 10%;">year</th>
                <th style="width: 26%;">genres</th>
              </tr>
            </thead>
            <tbody>
        `;
        state.hits.forEach((item, idx) => {
          const isSelected = idx === state.sel;
          const rated = state.ratings[item.mal_id];
          const star = rated ? `<span style="color:#f3d54a;">★</span> ` : '';
          out += `
            <tr class="${isSelected ? 'selected' : ''}" data-row-idx="${idx}">
              <td>${isSelected ? '▎ ' : '&nbsp; '}${star}${escapeHtml(item.title)}</td>
              <td>${escapeHtml(item.kind)}</td>
              <td>${item.episodes || '?'}</td>
              <td>${item.year || '?'}</td>
              <td class="genres-cell">${escapeHtml(item.genres.slice(0, 3).join(', '))}</td>
            </tr>
          `;
        });
        out += `</tbody></table>`;
      }
    }
    return out;
  }

  // --- Render Vibes Screen (draw_vibes in main.rs) ---
  function renderVibesScreen() {
    let out = '<div style="padding: 10px 0;">';
    out += '<div style="color: #928b6e; font-size: 12px; margin-bottom: 12px;">pick a vibe (use ←/→ or click):</div>';
    out += '<div style="display: flex; gap: 10px; margin-bottom: 20px; flex-wrap: wrap;">';
    VIBES.forEach((v, idx) => {
      const active = idx === state.vibeSel;
      out += `
        <div class="tui-tab ${active ? 'active' : ''}" data-vibe-idx="${idx}" style="cursor: pointer;">
          ${active ? '▸ ' : ''}${v.name}
        </div>
      `;
    });
    out += '</div>';

    const currentVibe = VIBES[state.vibeSel];
    out += `<div style="color: #f3d54a; font-weight: 700; margin-bottom: 14px;">curated ${currentVibe.name} starters:</div>`;
    out += '<table class="tui-results-table"><tbody>';
    currentVibe.titles.forEach((title, idx) => {
      const item = CATALOG.find(a => a.title.toLowerCase().includes(title.toLowerCase()));
      if (item) {
        out += `
          <tr data-view-detail="${item.mal_id}">
            <td style="width: 50%;"><span style="color: #f3d54a;">★</span> ${escapeHtml(item.title)}</td>
            <td style="width: 15%;">${item.kind} · ${item.year}</td>
            <td style="width: 15%; color: #f3d54a;">★ ${item.score}</td>
            <td style="width: 20%; color: #b2ba7a;">${item.genres.slice(0, 2).join(', ')}</td>
          </tr>
        `;
      }
    });
    out += '</tbody></table></div>';
    return out;
  }

  // --- Render Picks Screen (draw_picks in main.rs) ---
  function renderPicksScreen() {
    let out = '<div style="padding: 10px 0;">';
    out += '<div style="color: #f3d54a; font-weight: 700; font-size: 14px; margin-bottom: 8px;">personalized recommendations</div>';
    out += '<div style="color: #928b6e; font-size: 12px; margin-bottom: 16px;">every pick has a reason. derived from your taste profile:</div>';

    const recs = [
      { anime: CATALOG[2], reason: 'shared genres: Sci-Fi, Thriller · matching era: 2011' },
      { anime: CATALOG[6], reason: 'shared genres: Adventure, Action · high score: 8.73' },
      { anime: CATALOG[9], reason: 'shared genres: Action, Comedy · highly rated: 8.94' },
      { anime: CATALOG[8], reason: 'shared genres: Slice of Life, Comedy · recent: 2022' },
    ];

    out += '<table class="tui-results-table"><tbody>';
    recs.forEach(r => {
      out += `
        <tr data-view-detail="${r.anime.mal_id}">
          <td style="width: 35%;"><span style="color: #f3d54a;">★</span> <strong>${escapeHtml(r.anime.title)}</strong></td>
          <td style="width: 15%; color: #f3d54a;">★ ${r.anime.score}</td>
          <td style="width: 50%; color: #b2ba7a;">→ ${escapeHtml(r.reason)}</td>
        </tr>
      `;
    });
    out += '</tbody></table></div>';
    return out;
  }

  // --- Render Details Screen (draw_details in main.rs) ---
  function renderDetailsScreen() {
    const item = CATALOG[state.selectedAnimeIdx] || CATALOG[0];
    return `
      <div style="padding: 10px 0;">
        <div style="border: 1px solid #6e684e; border-radius: 6px; padding: 16px; background: #2a2920;">
          <div style="color: #f3d54a; font-size: 16px; font-weight: 700; margin-bottom: 10px;">${escapeHtml(item.title)}</div>
          <div style="display: flex; gap: 20px; color: #928b6e; font-size: 12px; margin-bottom: 14px;">
            <span>type: <strong style="color: #dfdbbf;">${item.kind}</strong></span>
            <span>year: <strong style="color: #dfdbbf;">${item.year}</strong></span>
            <span>episodes: <strong style="color: #dfdbbf;">${item.episodes}</strong></span>
            <span>score: <strong style="color: #f3d54a;">★ ${item.score}</strong></span>
          </div>
          <div style="color: #b2ba7a; font-size: 12px; margin-bottom: 14px;">
            genres: ${item.genres.join(' · ')}
          </div>
          <div style="color: #dfdbbf; font-size: 13px; line-height: 1.6; max-width: 700px;">
            ${escapeHtml(item.desc)}
          </div>
        </div>
      </div>
    `;
  }

  // --- Render Help Screen (draw_help in main.rs) ---
  function renderHelpScreen() {
    return `
      <div style="padding: 10px 0; font-size: 12.5px;">
        <div style="color: #f3d54a; font-weight: 700; font-size: 14px; margin-bottom: 10px;">keys and about</div>
        <div style="color: #928b6e; margin-bottom: 16px;">aroe 1.0.0 · the anime recommender that lives in your terminal</div>

        <div style="display: grid; grid-template-columns: 140px 1fr; gap: 8px; max-width: 500px; color: #dfdbbf;">
          <span style="color: #f3d54a;">1 - 5</span><span>switch tabs (search, vibes, picks, details, help)</span>
          <span style="color: #f3d54a;">Tab</span><span>cycle through tabs</span>
          <span style="color: #f3d54a;">↑ / ↓</span><span>navigate results or menu</span>
          <span style="color: #f3d54a;">Enter</span><span>rate selected anime or open details</span>
          <span style="color: #f3d54a;">Esc</span><span>clear search / return to home</span>
          <span style="color: #f3d54a;">?</span><span>toggle this help screen</span>
        </div>
      </div>
    `;
  }

  // --- Render Rating Dialog (draw_rating_dialog in main.rs) ---
  function renderRatingDialog() {
    const { anime, sel } = state.ratingDialog;
    const options = [
      { key: 'a', label: 'amazing', weight: '+2.0' },
      { key: 'g', label: 'good',    weight: '+1.0' },
      { key: 'm', label: 'meh',     weight: '+0.2' },
      { key: 'b', label: 'bad',     weight: '-1.5' },
    ];
    let rowsHtml = '';
    options.forEach((opt, idx) => {
      const isSel = idx === sel;
      const current = state.ratings[anime.mal_id] === opt.label ? ' <span style="color:#b2ba7a;">(current)</span>' : '';
      rowsHtml += `
        <div class="tui-dialog-row ${isSel ? 'selected' : ''}" data-rating-choice="${opt.label}">
          <span class="tui-dialog-label">${isSel ? '▎ ' : '&nbsp; '}${opt.label}${current}</span>
          <span class="tui-dialog-key">${opt.key} (${opt.weight})</span>
        </div>
      `;
    });

    return `
      <div class="tui-dialog-overlay">
        <div class="tui-dialog-title">┌─ Rate: ${escapeHtml(anime.title)} ─┐</div>
        ${rowsHtml}
        <div style="margin-top: 10px; font-size: 11px; color: #6e684e; text-align: center;">press a/g/m/b or click to save · esc cancel</div>
      </div>
    `;
  }

  // --- Attach Event Handlers ---
  function attachListeners() {
    // 1. Tab buttons
    container.querySelectorAll('.tui-tab[data-tab]').forEach(tab => {
      tab.addEventListener('click', (e) => {
        state.screen = parseInt(e.currentTarget.getAttribute('data-tab'), 10);
        render();
      });
    });

    // 2. Search Input
    const input = document.getElementById('tui-search-input');
    if (input) {
      input.focus();
      // Keep cursor at end
      input.setSelectionRange(input.value.length, input.value.length);

      input.addEventListener('input', (e) => {
        state.query = e.target.value;
        state.sel = 0;
        render();
      });

      input.addEventListener('keydown', (e) => {
        if (e.key === 'ArrowDown') {
          e.preventDefault();
          if (state.hits.length > 0) {
            state.sel = Math.min(state.hits.length - 1, state.sel + 1);
            render();
          } else {
            state.menuFocus = state.menuFocus === null ? 0 : Math.min(3, state.menuFocus + 1);
            render();
          }
        } else if (e.key === 'ArrowUp') {
          e.preventDefault();
          if (state.hits.length > 0) {
            state.sel = Math.max(0, state.sel - 1);
            render();
          } else {
            state.menuFocus = state.menuFocus === null ? null : Math.max(0, state.menuFocus - 1);
            render();
          }
        } else if (e.key === 'Enter') {
          e.preventDefault();
          if (state.hits.length > 0) {
            const selectedAnime = state.hits[state.sel];
            state.ratingDialog = { anime: selectedAnime, sel: 0 };
            render();
          } else if (state.menuFocus !== null) {
            state.screen = state.menuFocus + 1;
            state.menuFocus = null;
            render();
          }
        } else if (e.key === 'Escape') {
          state.query = '';
          state.menuFocus = null;
          render();
        }
      });
    }

    // 3. Menu items click
    container.querySelectorAll('.tui-menu-item[data-nav-screen]').forEach(item => {
      item.addEventListener('click', (e) => {
        state.screen = parseInt(e.currentTarget.getAttribute('data-nav-screen'), 10);
        render();
      });
    });

    // 4. Vibe items click
    container.querySelectorAll('.tui-tab[data-vibe-idx]').forEach(tab => {
      tab.addEventListener('click', (e) => {
        state.vibeSel = parseInt(e.currentTarget.getAttribute('data-vibe-idx'), 10);
        render();
      });
    });

    // 5. Table rows click to rate
    container.querySelectorAll('.tui-results-table tr[data-row-idx]').forEach(row => {
      row.addEventListener('click', (e) => {
        const idx = parseInt(e.currentTarget.getAttribute('data-row-idx'), 10);
        state.sel = idx;
        const selectedAnime = state.hits[idx];
        if (selectedAnime) {
          state.ratingDialog = { anime: selectedAnime, sel: 0 };
          render();
        }
      });
    });

    // 6. View detail clicks
    container.querySelectorAll('[data-view-detail]').forEach(el => {
      el.addEventListener('click', (e) => {
        const id = parseInt(e.currentTarget.getAttribute('data-view-detail'), 10);
        const idx = CATALOG.findIndex(a => a.mal_id === id);
        if (idx !== -1) {
          state.selectedAnimeIdx = idx;
          state.screen = 3; // details
          render();
        }
      });
    });

    // 7. Rating Dialog click
    container.querySelectorAll('.tui-dialog-row[data-rating-choice]').forEach(row => {
      row.addEventListener('click', (e) => {
        const choice = e.currentTarget.getAttribute('data-rating-choice');
        if (state.ratingDialog) {
          state.ratings[state.ratingDialog.anime.mal_id] = choice;
          state.ratingDialog = null;
          render();
        }
      });
    });
  }

  // --- Global Keyboard Navigation (matching on_key in main.rs) ---
  window.addEventListener('keydown', (e) => {
    // Only capture keys if not typing in external form elements
    if (document.activeElement && document.activeElement.tagName === 'INPUT' && document.activeElement.id !== 'tui-search-input') {
      return;
    }

    // If rating dialog is open
    if (state.ratingDialog) {
      if (e.key === 'Escape') {
        state.ratingDialog = null;
        render();
      } else if (e.key === 'a') {
        state.ratings[state.ratingDialog.anime.mal_id] = 'amazing';
        state.ratingDialog = null;
        render();
      } else if (e.key === 'g') {
        state.ratings[state.ratingDialog.anime.mal_id] = 'good';
        state.ratingDialog = null;
        render();
      } else if (e.key === 'm') {
        state.ratings[state.ratingDialog.anime.mal_id] = 'meh';
        state.ratingDialog = null;
        render();
      } else if (e.key === 'b') {
        state.ratings[state.ratingDialog.anime.mal_id] = 'bad';
        state.ratingDialog = null;
        render();
      } else if (e.key === 'Enter') {
        const choices = ['amazing', 'good', 'meh', 'bad'];
        state.ratings[state.ratingDialog.anime.mal_id] = choices[state.ratingDialog.sel];
        state.ratingDialog = null;
        render();
      } else if (e.key === 'ArrowDown') {
        state.ratingDialog.sel = Math.min(3, state.ratingDialog.sel + 1);
        render();
      } else if (e.key === 'ArrowUp') {
        state.ratingDialog.sel = Math.max(0, state.ratingDialog.sel - 1);
        render();
      }
      return;
    }

    // Screen switching keys (1-5, Tab, ?)
    if (e.key === 'Tab') {
      e.preventDefault();
      state.screen = (state.screen + 1) % 5;
      render();
    } else if (e.key === '?' && state.screen !== 0) {
      state.screen = state.screen === 4 ? 0 : 4;
      render();
    } else if (!state.query && ['1', '2', '3', '4', '5'].includes(e.key)) {
      state.screen = parseInt(e.key, 10) - 1;
      render();
    }
  });

  // --- Copy button functionality ---
  const copyBtn = document.getElementById('copy-btn');
  if (copyBtn) {
    copyBtn.addEventListener('click', () => {
      const code = 'git clone https://github.com/he3vn/Aroe\ncd Aroe\ncargo run --release';
      navigator.clipboard.writeText(code).then(() => {
        copyBtn.innerHTML = '<span>✓ Copied!</span>';
        setTimeout(() => {
          copyBtn.innerHTML = `
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>
            <span>Copy</span>
          `;
        }, 1500);
      });
    });
  }

  function escapeHtml(str) {
    if (!str) return '';
    return String(str)
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  // Initial boot
  render();
})();


/* ==========================================================================
   SCROLL REVEAL — IntersectionObserver for section animations
   Matches yogurt.so's sections.mjs reveal behavior.
   ========================================================================== */
(function () {
  'use strict';

  const revealTargets = [
    '.steps-section',
    '.step',
    '.privacy-section',
    '.local-section',
    '.faq-story',
    '.install-section',
    '.site-footer',
  ];

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible');
          observer.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.15, rootMargin: '0px 0px -40px 0px' }
  );

  // Observe all reveal targets once DOM is ready
  revealTargets.forEach((selector) => {
    document.querySelectorAll(selector).forEach((el) => {
      observer.observe(el);
    });
  });
})();


/* ==========================================================================
   INSTALL TABS SWITCHER
   ========================================================================== */
(function () {
  'use strict';
  const tabs = document.querySelectorAll('.install-tab[data-install-target]');
  const paneCurl = document.getElementById('install-pane-curl');
  const paneBinary = document.getElementById('install-pane-binary');
  const paneCargo = document.getElementById('install-pane-cargo');
  const copyBtn = document.getElementById('copy-btn');

  tabs.forEach(tab => {
    tab.addEventListener('click', () => {
      tabs.forEach(t => t.classList.remove('active'));
      tab.classList.add('active');
      const target = tab.getAttribute('data-install-target');

      if (paneCurl) paneCurl.style.display = target === 'curl' ? 'block' : 'none';
      if (paneBinary) paneBinary.style.display = target === 'binary' ? 'block' : 'none';
      if (paneCargo) paneCargo.style.display = target === 'cargo' ? 'block' : 'none';
      if (copyBtn) copyBtn.style.display = target === 'binary' ? 'none' : 'flex';
    });
  });

  // Update copy button to copy whatever command is active
  if (copyBtn) {
    copyBtn.addEventListener('click', () => {
      const activeTab = document.querySelector('.install-tab.active');
      const target = activeTab ? activeTab.getAttribute('data-install-target') : 'curl';
      let code = 'curl -fsSL https://raw.githubusercontent.com/he3vn/Aroe/main/install.sh | bash';
      if (target === 'cargo') {
        code = 'git clone https://github.com/he3vn/Aroe\ncd Aroe\ncargo run --release';
      }
      navigator.clipboard.writeText(code).then(() => {
        copyBtn.innerHTML = '<span>✓ Copied!</span>';
        setTimeout(() => {
          copyBtn.innerHTML = `
            <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>
            <span>Copy</span>
          `;
        }, 1500);
      });
    });
  }
})();
