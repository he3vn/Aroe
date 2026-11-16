/* ============================================
   aroe terminal — lightweight interactive shell
   Simulates the aroe CLI in the browser.
   No dependencies, no frameworks, < 8 KB.
   ============================================ */

(function () {
  'use strict';

  // --- DOM refs ---
  const output   = document.getElementById('terminal-output');
  const input    = document.getElementById('terminal-input');
  const promptEl = document.getElementById('prompt');
  const cursorEl = document.getElementById('cursor-blink');
  const body     = document.getElementById('terminal-body');

  // --- State ---
  let history = [];
  let histIdx = -1;
  let currentPrompt = 'aroe ~>';

  // --- Mini anime catalog (embedded so no fetch needed) ---
  const catalog = [
    { title: 'Frieren: Beyond Journey\'s End', type: 'TV', year: 2023, episodes: 28, genres: ['Adventure','Drama','Fantasy'], score: 9.3 },
    { title: 'Cowboy Bebop', type: 'TV', year: 1998, episodes: 26, genres: ['Action','Drama','Sci-Fi'], score: 8.9 },
    { title: 'Steins;Gate', type: 'TV', year: 2011, episodes: 24, genres: ['Drama','Sci-Fi','Thriller'], score: 9.1 },
    { title: 'Attack on Titan', type: 'TV', year: 2013, episodes: 25, genres: ['Action','Drama','Fantasy'], score: 8.5 },
    { title: 'Spirited Away', type: 'Movie', year: 2001, episodes: 1, genres: ['Adventure','Fantasy','Supernatural'], score: 8.8 },
    { title: 'Your Name.', type: 'Movie', year: 2016, episodes: 1, genres: ['Drama','Romance','Supernatural'], score: 8.9 },
    { title: 'Violet Evergarden', type: 'TV', year: 2018, episodes: 13, genres: ['Drama','Fantasy','Slice of Life'], score: 8.7 },
    { title: 'Mob Psycho 100', type: 'TV', year: 2016, episodes: 12, genres: ['Action','Comedy','Supernatural'], score: 8.6 },
    { title: 'Bocchi the Rock!', type: 'TV', year: 2022, episodes: 12, genres: ['Comedy','Music','Slice of Life'], score: 8.4 },
    { title: 'Neon Genesis Evangelion', type: 'TV', year: 1995, episodes: 26, genres: ['Action','Drama','Sci-Fi'], score: 8.4 },
    { title: 'Chainsaw Man', type: 'TV', year: 2022, episodes: 12, genres: ['Action','Fantasy','Horror'], score: 8.3 },
    { title: 'Spy x Family', type: 'TV', year: 2022, episodes: 12, genres: ['Action','Comedy','Slice of Life'], score: 8.6 },
    { title: 'Jujutsu Kaisen', type: 'TV', year: 2020, episodes: 24, genres: ['Action','Fantasy','Supernatural'], score: 8.7 },
    { title: 'A Silent Voice', type: 'Movie', year: 2016, episodes: 1, genres: ['Drama','Romance','Slice of Life'], score: 8.9 },
    { title: 'Vinland Saga', type: 'TV', year: 2019, episodes: 24, genres: ['Action','Adventure','Drama'], score: 8.7 },
    { title: 'Made in Abyss', type: 'TV', year: 2017, episodes: 13, genres: ['Adventure','Drama','Fantasy'], score: 8.7 },
    { title: 'Demon Slayer', type: 'TV', year: 2019, episodes: 26, genres: ['Action','Fantasy','Supernatural'], score: 8.5 },
    { title: 'March Comes in Like a Lion', type: 'TV', year: 2016, episodes: 22, genres: ['Drama','Slice of Life'], score: 8.6 },
    { title: 'Odd Taxi', type: 'TV', year: 2021, episodes: 13, genres: ['Drama','Mystery','Thriller'], score: 8.5 },
    { title: 'Mushoku Tensei', type: 'TV', year: 2021, episodes: 11, genres: ['Adventure','Drama','Fantasy'], score: 8.3 },
  ];

  // Vibe presets
  const vibes = {
    action:   ['Attack on Titan','Chainsaw Man','Jujutsu Kaisen','Demon Slayer','Mob Psycho 100'],
    romance:  ['Your Name.','A Silent Voice','Violet Evergarden','March Comes in Like a Lion'],
    chill:    ['Bocchi the Rock!','Spy x Family','Violet Evergarden','Odd Taxi'],
    deep:     ['Steins;Gate','Neon Genesis Evangelion','Made in Abyss','Vinland Saga'],
    beginner: ['Spy x Family','Demon Slayer','Your Name.','Bocchi the Rock!','Frieren: Beyond Journey\'s End'],
    short:    ['Bocchi the Rock!','Mob Psycho 100','Spy x Family','Mushoku Tensei','Odd Taxi'],
  };

  // --- Helpers ---
  function span(text, cls) {
    return `<span class="${cls}">${esc(text)}</span>`;
  }

  function esc(s) {
    const d = document.createElement('div');
    d.textContent = s;
    return d.innerHTML;
  }

  function print(html) {
    output.insertAdjacentHTML('beforeend', html + '\n');
    scrollBottom();
  }

  function printPlain(text) {
    print(esc(text));
  }

  function printBrand(text) {
    print(span(text, 't-brand t-bold'));
  }

  function scrollBottom() {
    body.scrollTop = body.scrollHeight;
  }

  // --- Bolt ASCII art (from tui.rs) ---
  const BOLT = [
    '                      =.',
    '                    :%#.',
    '                  =*=*',
    '                ++-:#:',
    '               +=.--@',
    '            :*=.:-=*',
    '          .+=.:---*:',
    '         -+:.:---=*********#=',
    '      .=+:.::--------=====+=.',
    '     :+-.:-----------===**:',
    '     =*********#%----==*:.',
    '               ++--==+=',
    '              :+--=++=',
    '             .+--=+-',
    '             @-=%',
    '            *-+*.',
    '           -**-.',
    '          .%-',
    '          #=',
  ];

  const WORDMARK = [
    '▄▀▀▀▄ █▀▀▀▄ ▄▀▀▀▄ █▀▀▀▀',
    '█   █ █▄▄▄▀ █   █ █▄▄▄ ',
    '█▀▀▀█ █  ▀▄ █   █ █    ',
    '█   █ █   █ ▀▄▄▄▀ █▄▄▄▄',
  ];

  // --- Commands ---
  const commands = {
    help() {
      print(span('available commands:', 't-brand t-bold'));
      print('');
      const cmds = [
        ['help',              'show this message'],
        ['search <query>',    'search the anime catalog'],
        ['vibes',             'pick a vibe, get recs'],
        ['vibe <name>',       'action, romance, chill, deep, beginner, short'],
        ['info <title>',      'show details about an anime'],
        ['recommend',         'get personalized picks (try it!)'],
        ['about',             'what is aroe?'],
        ['clear',             'clear the terminal'],
        ['version',           'print version'],
        ['neofetch',          'system info, aroe style'],
      ];
      for (const [cmd, desc] of cmds) {
        print(`  ${span(cmd.padEnd(20), 't-brand')} ${span(desc, 't-muted')}`);
      }
      print('');
      print(span('tip: try "search frieren" or "vibe chill"', 't-faint'));
    },

    about() {
      print('');
      for (const line of WORDMARK) {
        print(span(line, 't-brand t-bold'));
      }
      print('');
      print(span('the anime recommender that lives in your terminal.', 't-fg'));
      print('');
      print(`${span('version', 't-muted')}   ${span('0.4.0', 't-fg')}`);
      print(`${span('language', 't-muted')}  ${span('rust', 't-brand')}`);
      print(`${span('license', 't-muted')}   ${span('MIT', 't-fg')}`);
      print(`${span('repo', 't-muted')}      ${span('github.com/he3vn/Aroe', 't-label')}`);
      print('');
    },

    version() {
      print(span('aroe 0.4.0', 't-brand') + span(' (built with rust)', 't-muted'));
    },

    clear() {
      output.innerHTML = '';
    },

    neofetch() {
      print('');
      const info = [
        ['', ''],
        ['os', 'your terminal'],
        ['host', 'aroe 0.4.0'],
        ['kernel', 'ratatui + crossterm'],
        ['shell', 'aroe ~>'],
        ['lang', 'rust 🦀'],
        ['catalog', catalog.length + ' titles loaded'],
        ['vibes', Object.keys(vibes).join(', ')],
        ['theme', 'golden amber on dark olive'],
        ['', ''],
      ];
      for (let i = 0; i < Math.max(BOLT.length, info.length); i++) {
        const art = i < BOLT.length ? span(BOLT[i], 't-brand') : ' '.repeat(34);
        if (i < info.length && info[i][0]) {
          print(`${art}  ${span(info[i][0], 't-label')}${span(': ', 't-faint')}${span(info[i][1], 't-fg')}`);
        } else {
          print(art);
        }
      }
      print('');
      // Color blocks
      const colors = ['#ff5f57','#ffbd2e','#28c840','#f3d54a','#bc942d','#b2ba7a','#928b6e','#dfdbbf'];
      let blocks = '  ';
      for (const c of colors) blocks += `<span style="background:${c};color:${c}">███</span>`;
      print(blocks);
      print('');
    },

    search(args) {
      if (!args) {
        print(span('usage: search <query>', 't-error'));
        return;
      }
      const q = args.toLowerCase();
      const results = catalog.filter(a =>
        a.title.toLowerCase().includes(q) ||
        a.genres.some(g => g.toLowerCase().includes(q))
      );
      if (!results.length) {
        print(span(`no results for "${args}"`, 't-muted'));
        print(span('try: search fantasy, search bebop, search 2022', 't-faint'));
        return;
      }
      print(span(`found ${results.length} result${results.length > 1 ? 's' : ''}:`, 't-brand'));
      print('');
      for (const a of results) {
        print(
          `  ${span('★', 't-brand')} ${span(a.title, 't-fg t-bold')}` +
          `  ${span(a.type, 't-label')}` +
          `  ${span(String(a.year), 't-muted')}` +
          `  ${span(a.score.toFixed(1), 't-brand')}`
        );
        print(`    ${span(a.genres.join(', '), 't-faint')}  ${span(a.episodes + ' ep', 't-muted')}`);
      }
      print('');
    },

    vibes() {
      print(span('pick a vibe:', 't-brand t-bold'));
      print('');
      for (const v of Object.keys(vibes)) {
        print(`  ${span('▸', 't-brand')} ${span(v, 't-label')}`);
      }
      print('');
      print(span('type "vibe <name>" to get recs', 't-faint'));
    },

    vibe(args) {
      if (!args) {
        commands.vibes();
        return;
      }
      const name = args.toLowerCase().trim();
      const picks = vibes[name];
      if (!picks) {
        print(span(`unknown vibe "${args}"`, 't-error'));
        print(span('available: ' + Object.keys(vibes).join(', '), 't-faint'));
        return;
      }
      print(span(`vibe: ${name}`, 't-brand t-bold'));
      print('');
      for (const title of picks) {
        const a = catalog.find(x => x.title === title);
        if (a) {
          print(
            `  ${span('★', 't-brand')} ${span(a.title, 't-fg t-bold')}` +
            `  ${span(a.score.toFixed(1), 't-brand')}` +
            `  ${span(a.genres.join(', '), 't-faint')}`
          );
        }
      }
      print('');
    },

    info(args) {
      if (!args) {
        print(span('usage: info <title>', 't-error'));
        return;
      }
      const q = args.toLowerCase();
      const a = catalog.find(x => x.title.toLowerCase().includes(q));
      if (!a) {
        print(span(`not found: "${args}"`, 't-muted'));
        return;
      }
      print('');
      print(span(`┌─ ${a.title}`, 't-brand t-bold'));
      const fields = [
        ['type',     a.type,               't-fg'],
        ['year',     String(a.year),        't-fg'],
        ['episodes', String(a.episodes),    't-fg'],
        ['score',    a.score.toFixed(1),     't-brand'],
        ['genres',   a.genres.join(', '),   't-label'],
      ];
      for (const [label, val, cls] of fields) {
        print(`${span('│', 't-brand')} ${span(label.padEnd(10), 't-muted')} ${span(val, cls)}`);
      }
      print(span('└─', 't-brand'));
      print('');
    },

    recommend() {
      print(span('generating picks...', 't-muted'));
      print('');
      // Pick 4 random ones
      const shuffled = [...catalog].sort(() => Math.random() - 0.5);
      const picks = shuffled.slice(0, 4);
      print(span('recommended for you:', 't-brand t-bold'));
      print('');
      for (const a of picks) {
        const reasons = [];
        if (a.score >= 8.7) reasons.push('highly rated');
        if (a.year >= 2020) reasons.push('recent');
        if (a.genres.includes('Drama')) reasons.push('compelling drama');
        if (a.genres.includes('Action')) reasons.push('great action');
        if (a.genres.includes('Fantasy')) reasons.push('rich world');
        if (a.episodes <= 13) reasons.push('short run');
        const reason = reasons.slice(0, 2).join(', ') || 'worth watching';
        print(
          `  ${span('★', 't-brand')} ${span(a.title, 't-fg t-bold')}` +
          `  ${span(a.score.toFixed(1), 't-brand')}`
        );
        print(`    ${span('→ ' + reason, 't-label')}`);
      }
      print('');
      print(span('every pick has a reason. no black box.', 't-faint'));
      print('');
    },
  };

  // --- Execute ---
  function execute(line) {
    const trimmed = line.trim();
    if (!trimmed) return;

    // Echo the command
    print(`${span(currentPrompt, 't-brand t-bold')} ${esc(trimmed)}`);

    // Parse
    const parts = trimmed.split(/\s+/);
    const cmd = parts[0].toLowerCase();
    const args = parts.slice(1).join(' ');

    if (commands[cmd]) {
      commands[cmd](args);
    } else {
      print(span(`command not found: ${cmd}`, 't-error'));
      print(span('type "help" for available commands', 't-faint'));
    }
  }

  // --- Input handling ---
  input.addEventListener('keydown', function (e) {
    if (e.key === 'Enter') {
      const val = input.value;
      if (val.trim()) {
        history.unshift(val);
        if (history.length > 50) history.pop();
      }
      histIdx = -1;
      execute(val);
      input.value = '';
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (histIdx < history.length - 1) {
        histIdx++;
        input.value = history[histIdx];
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (histIdx > 0) {
        histIdx--;
        input.value = history[histIdx];
      } else {
        histIdx = -1;
        input.value = '';
      }
    } else if (e.key === 'l' && e.ctrlKey) {
      e.preventDefault();
      commands.clear();
    }
  });

  // Keep cursor synced with input
  input.addEventListener('input', function () {
    cursorEl.style.display = '';
  });

  // Focus terminal on click
  body.addEventListener('click', function () {
    input.focus();
  });

  // --- Boot sequence ---
  function boot() {
    for (const line of WORDMARK) {
      print(span(line, 't-brand t-bold'));
    }
    print('');
    print(span('the anime recommender that lives in your terminal.', 't-fg'));
    print(span('v0.4.0 · built with rust · MIT license', 't-muted'));
    print('');
    print(span('type "help" for commands, or try "search frieren"', 't-faint'));
    print('');
    input.focus();
  }

  // --- Copy button ---
  const copyBtn = document.getElementById('copy-btn');
  if (copyBtn) {
    copyBtn.addEventListener('click', function () {
      const code = 'git clone https://github.com/he3vn/Aroe\ncd Aroe\ncargo build --release\ncargo run --release';
      navigator.clipboard.writeText(code).then(() => {
        copyBtn.innerHTML = '✓';
        setTimeout(() => {
          copyBtn.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg>';
        }, 1500);
      });
    });
  }

  boot();
})();
