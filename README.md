# AnimeRO

An anime recommender. Tell it what you've watched, and it tells you what to
watch next based on your current tastes. If you haven't watched anything yet, it
gives you a few good starting points instead.

Terminal app, written in Rust. GUI maybe later, once the terminal version is
actually fun to use.

---

## What it will do

**If you've watched some anime**

- You type in titles you've seen, optionally with a score out of 10.
- It looks each one up in an anime catalog and remembers it locally.
- It builds a rough taste profile from those shows: which genres and themes you
  lean toward, what era you like, TV vs movie, how highly you rated things.
- It scores the rest of the catalog against that profile, drops anything you've
  already seen, and prints the top matches.
- Every result says *why* it showed up — shared genres, similar year, similar
  rating — so the list isn't a black box.

**If you're new**

- No quiz. It gives you a handful of widely liked, beginner-friendly shows
  instead of a thousand-row list.
- Optionally you pick a vibe first (action, romance, something short, etc.).

## How it works, in plain English

1. **Lookup** — turn a typed title into a real catalog entry: id, genres, year,
   episode count, score, popularity.
2. **Cache** — save what it fetched into `data/` so the next run is instant and
   the APIs don't get hammered.
3. **Profile** — turn your watched list into weighted preferences. A show you
   rated 9/10 counts more than one you rated 6/10.
4. **Score** — compare every candidate against that profile. Shared genres and
   themes carry the most weight; year and format break ties.
5. **Filter** — remove anything already watched, plus anything outside the
   limits you set (year, type, episode count).
6. **Present** — top N results, formatted for a terminal, each with a reason.

## Where the data comes from

| Source | What it's for | Key needed |
|---|---|---|
| MyAnimeList (via Jikan v4) | main catalog: titles, genres, year, score | No |
| AniList (GraphQL) | second catalog, better bulk queries, its own similar lists | No |

Both are free and public. Neither is first-party for this app, so requests are
rate-limited — which is the whole reason the cache exists.

Endpoint details and gotchas live in `docs/API-NOTES.md`.

## Planned, roughly in order

1. Search one title and print the raw result. Nothing else.
2. Cache results so reruns don't refetch.
3. Format a single result so it's readable.
4. Score two anime against each other and sanity-check the number.
5. Recommend from a list of known anime.
6. "I'm new" mode with starter picks.
7. Show the reason behind each recommendation.
8. Add AniList as a second source.
9. Save your watched list between runs.
10. GUI, if the terminal version earns it.

## Not planned for now

- Logging into your MAL/AniList account. Reading a real user list needs OAuth,
  which is a separate project.
- Scraping MyAnimeList's website directly. The APIs exist; use them.
- A web or desktop UI before the terminal version works.


