# aroe

<p align="center">
  <img src="logo.jpeg" alt="aroe" width="100" />
</p>

<p align="center">
  <a href="https://github.com/he3vn/Aroe">github</a> · <a href="#install">install</a> · <a href="#usage">usage</a> · <a href="#how-it-works">how it works</a>
</p>

<p align="center">
  English · <a href="README.fr.md">Français</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-666666?labelColor=333333" alt="MIT license" /></a>
  <a href="https://github.com/he3vn/Aroe"><img src="https://img.shields.io/github/downloads/he3vn/Aroe/total?labelColor=333333&color=666666" alt="total GitHub release downloads" /></a>
  <a href="https://github.com/he3vn/Aroe/stargazers"><img src="https://img.shields.io/github/stars/he3vn/Aroe?labelColor=333333&color=666666&logo=github" alt="GitHub stars" /></a>
  <a href="https://github.com/he3vn/Aroe/releases/latest"><img src="https://img.shields.io/github/v/release/he3vn/Aroe?label=release&labelColor=333333&color=666666" alt="latest stable release" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/built%20with-Rust-000000?logo=rust&labelColor=333333&color=666666" alt="built with Rust" /></a>
</p>

---

<p align="center">
  <img src="https://picsum.photos/seed/aroe/1200/500" alt="a random placeholder image" />
</p>

**the anime recommender that lives in your terminal.**

- **start from your taste** — tell it what you've watched, optionally with a score out of 10. aroe turns that into a taste profile: favourite genres and themes, the era you like, TV or movie, and how highly you rate things.
- **every pick has a reason** — recommendations explain themselves with shared genres, a similar year, or a similar rating. no black box.
- **new to anime?** — no quiz. pick a vibe (action, romance, something short…) and get a handful of beginner-friendly shows instead of a thousand-row list.
- **drops what you've seen** — already watched titles are filtered out, along with anything outside the year, type or episode limits you set.
- **cache, not spam** — catalog data is saved locally, so reruns are instant and the public APIs don't get hammered.
- **one rust binary, no electron** — runs in whatever terminal you already use.

> aroe is early. catalog search works today; the recommender is being built in the open. see [roadmap](#roadmap).

---

## install

```bash
git clone https://github.com/he3vn/Aroe
cd Aroe
cargo build --release
```

then run it:

```bash
cargo run --release
```

## usage

aroe asks two things:

- **are you new to anime?** if yes, pick a vibe and it suggests a starting point.
- **if not**, start typing a title and pick from the catalog. it shows the type, episodes, year and tags of what you chose.

## how it works

1. **lookup** — turn a typed title into a real catalog entry: id, genres, year, episode count, score, popularity.
2. **cache** — save what it fetched into `data/` so the next run is instant.
3. **profile** — turn your watched list into weighted preferences. a show you rated 9/10 counts more than one you rated 6/10.
4. **score** — compare every candidate against that profile. shared genres and themes carry the most weight; year and format break ties.
5. **filter** — remove anything already watched, plus anything outside the limits you set.
6. **present** — top N results, formatted for a terminal, each with a reason.

## where the data comes from

| source | what it's for | key needed |
|---|---|---|
| MyAnimeList (via Jikan v4) | main catalog: titles, genres, year, score | no |
| AniList (GraphQL) | second catalog, better bulk queries, its own similar lists | no |

both are free and public. neither is first-party for this app, so requests are rate-limited — which is the whole reason the cache exists.

## roadmap

- [x] search a title from the catalog
- [x] "i'm new" mode with starter picks
- [ ] cache lookups so reruns don't refetch
- [ ] score two anime against each other
- [ ] recommend from your watched list
- [ ] show the reason behind each recommendation
- [ ] add AniList as a second source
- [ ] save your watched list between runs
- [ ] GUI, if the terminal version earns it

## not planned for now

- logging into your MAL/AniList account. reading a real user list needs OAuth, which is a separate project.
- scraping MyAnimeList's website directly. the APIs exist; use them.
- a web or desktop UI before the terminal version works.

## development

```bash
git clone https://github.com/he3vn/Aroe
cd Aroe
cargo build
cargo run
```

## license

aroe is licensed under the [MIT License](LICENSE).
