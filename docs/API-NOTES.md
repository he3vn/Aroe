# API notes

Reference only. No code here — you write the requests.

## MyAnimeList via Jikan v4

Base: `https://api.jikan.moe/v4` — public, unofficial, no key.

Useful endpoints:

| Endpoint | What it gives you |
|---|---|
| `GET /anime?q=<title>&limit=5&sfw` | search by name |
| `GET /anime/{mal_id}/full` | one anime, full detail |
| `GET /anime/{mal_id}/recommendations` | MAL's own "similar" list |
| `GET /top/anime?filter=bypopularity` | candidates for "I'm new" mode |
| `GET /genres/anime` | genre id → name list |

Fields worth grabbing: `mal_id`, `title`, `title_english`, `year` (or
`aired.from`), `type`, `episodes`, `status`, `score`, `scored_by`, `members`,
`genres[].name`, `themes[].name`, `demographics[].name`, `synopsis`,
`images.jpg.image_url`.

Gotchas:

- Rate limit is roughly **3 requests/second and 60/minute**. Go over and you get
  a `429` with a `Retry-After` header. Add a delay between calls.
- It's a scraper in front of MAL, so it's sometimes slow or briefly down. Cache.
- `q=` search returns partial matches in its own order — you still need to pick
  the right one (exact title match, or let the user choose from the top 5).

## AniList

Endpoint: `POST https://graphql.anilist.co` with `{ "query": ..., "variables": ... }`.
Public data needs no key. Everything is one endpoint; you ask for the fields you
want.

Handy queries:

- `Media(search: "...", type: ANIME)` → one show.
- `Page(page: 1, perPage: 50) { media(type: ANIME, sort: POPULARITY_DESC) }` → bulk
  candidates in a single request.
- `Media(id: ...) { recommendations { nodes { mediaRecommendation { ... } } } }`
  → AniList's own similar list.

Fields worth grabbing: `id`, `idMal`, `title { romaji english }`, `genres`,
`startDate { year }`, `averageScore` (0–100), `popularity`, `episodes`,
`format`, `status`, `coverImage { large }`.

Gotchas:

- Rate limit is currently about **30 requests/minute** (it was 90; they throttle
  when load is high). Read the `X-RateLimit-Remaining` response header.
- `averageScore` is 0–100, MAL's `score` is 0–10. Normalize before comparing.
- A GraphQL error still comes back as HTTP 200 sometimes — check the `errors`
  array, not just the status code.

## Matching the two sources

AniList has `idMal`, which is the MyAnimeList id. So:

- Use **MAL id as the canonical key** for anything you cache or dedupe.
- Store the AniList id alongside it when you have it.

## Your own list

Reading a user's actual MAL/AniList list requires OAuth (login) — that's a whole
project. For now, "anime you know" means the user types titles in the terminal,
or you load a `assets/my-list.json` you write by hand. Keep that door open.

## Reference

https://anime.ameo.dev/interactive-recommender — the recommendation style to aim
for later: build a weighted taste profile from scores instead of only matching
genres.
