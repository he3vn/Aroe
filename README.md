# aroe

<p align="center">
  <img src="logo.jpeg" alt="aroe" width="100" />
</p>

<p align="center">
  <a href="https://github.com/he3vn/Aroe">github</a> · <a href="#install">install</a>
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

> aroe is early. catalog search works today; the recommender is being built in the open.

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

## thanks

assets were made by pinterest users  
and scraping was done by [https://github.com/manami-project/anime-offline-database](https://github.com/manami-project/anime-offline-database)

## license

aroe is licensed under the [MIT License](LICENSE).
