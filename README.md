# Anime Recommender

A terminal app that recommends anime based on what you've already watched.
You write all the JavaScript. This repo is just the skeleton: folders, empty
files with a one-line note about what belongs in them, and a bit of reference
material.

GUI later (undecided). The `ui/` folder is where that swap will happen.

---

## Folder map

```
anime-recommender/
├── app/            # entry point + app flow (start here)
│   └── main.js
├── api/            # talking to the outside world
│   ├── jikan.js    # MyAnimeList data (Jikan v4, no key needed)
│   ├── anilist.js  # AniList data (GraphQL, no key needed)
│   └── cache.js    # save/load downloaded data so you don't refetch
├── js/             # pure logic, no network, no printing
│   ├── recommender.js  # the actual recommendation engine
│   ├── scoring.js      # genre/score/year similarity math
│   └── filters.js      # "only 2010+, only TV, not already watched"
├── ui/             # everything the user sees
│   ├── terminal.js # read input, print menus/results
│   └── format.js   # turn an anime object into a pretty line of text
├── assets/         # static stuff (ascii art, sample data, icons later)
├── data/           # cached JSON from the APIs (gitignored)
└── docs/
    └── API-NOTES.md
```

## Suggested build order

1. `api/jikan.js` — fetch one anime by name. Print the raw JSON. That's it.
2. `api/cache.js` — dump results into `data/anime.json` so reruns are instant.
3. `api/anilist.js` — same idea, second source (optional at first).
4. `ui/format.js` + `ui/terminal.js` — make a result look readable.
5. `js/scoring.js` — score anime A vs anime B (shared genres, year gap, rating).
6. `js/recommender.js` — take your list, score candidates, sort, return top N.
7. `app/main.js` — glue: ask for titles → recommend → print → loop.

## Running

```
node app/main.js
```

(`package.json` has `"type": "module"`, so use `import`, not `require`. Delete
that line if you'd rather use `require()`.)

## Data sources

| Source | API | Key needed | Notes |
|---|---|---|---|
| MyAnimeList | Jikan v4 `https://api.jikan.moe/v4` | No | ~3 req/sec, ~60 req/min. Be nice. |
| AniList | `https://graphql.anilist.co` | No | GraphQL. One request can pull a lot. |

Details, endpoints and gotchas: `docs/API-NOTES.md`.

## Reference

The recommendation style we're aiming for:
https://anime.ameo.dev/interactive-recommender

That one builds a weighted profile from your scores rather than just matching
genres. Good thing to read after the basic version works.

## Progress checklist

- [ ] Fetch anime from Jikan
- [ ] Cache results locally
- [ ] Show a readable result
- [ ] Score two anime against each other
- [ ] Recommend from a list of known anime
- [ ] "I'm new" mode → a few starter picks
- [ ] Add AniList as a second source
- [ ] Decide on GUI
