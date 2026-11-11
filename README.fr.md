# aroe

<p align="center">
  <img src="logo.jpeg" alt="aroe" width="100" />
</p>

<p align="center">
  <a href="https://github.com/he3vn/Aroe">github</a> · <a href="#installation">installation</a>
</p>

<p align="center">
  <a href="README.md">English</a> · Français
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-666666?labelColor=333333" alt="licence MIT" /></a>
  <a href="https://github.com/he3vn/Aroe"><img src="https://img.shields.io/github/downloads/he3vn/Aroe/total?labelColor=333333&color=666666" alt="téléchargements GitHub" /></a>
  <a href="https://github.com/he3vn/Aroe/stargazers"><img src="https://img.shields.io/github/stars/he3vn/Aroe?labelColor=333333&color=666666&logo=github" alt="étoiles GitHub" /></a>
  <a href="https://github.com/he3vn/Aroe/releases/latest"><img src="https://img.shields.io/github/v/release/he3vn/Aroe?label=release&labelColor=333333&color=666666" alt="dernière version stable" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/built%20with-Rust-000000?logo=rust&labelColor=333333&color=666666" alt="écrit en Rust" /></a>
</p>

---

<p align="center">
  <img src="https://picsum.photos/seed/aroe/1200/500" alt="une image au hasard" />
</p>

**le recommandeur d'anime qui vit dans ton terminal.**

- **pars de tes goûts** — dis-lui ce que tu as regardé, avec une note sur 10 si tu veux. aroe en fait un profil de goût : genres et thèmes préférés, époque, série ou film, et à quel point tu notes les choses.
- **chaque choix a une raison** — les recommandations s'expliquent : genres en commun, année proche ou note similaire. pas de boîte noire.
- **nouveau dans l'anime ?** — pas de quiz. choisis une ambiance (action, romance, quelque chose de court…) et reçois une poignée de séries accessibles au lieu d'une liste de mille lignes.
- **élimine ce que tu as déjà vu** — les titres déjà regardés sont filtrés, ainsi que tout ce qui sort des limites d'année, de type ou d'épisodes que tu as fixées.
- **du cache, pas du spam** — les données du catalogue sont sauvegardées en local, donc les relances sont instantanées et les API publiques ne sont pas martelées.
- **un seul binaire rust, pas d'electron** — tourne dans le terminal que tu utilises déjà.

> aroe en est à ses débuts. la recherche dans le catalogue fonctionne aujourd'hui ; le recommandeur se construit en public.

---

## installation

```bash
git clone https://github.com/he3vn/Aroe
cd Aroe
cargo build --release
```

puis lance-le :

```bash
cargo run --release
```

## merci

les assets ont été réalisés par des utilisateurs de Pinterest  
et le scraping a été fait par [https://github.com/manami-project/anime-offline-database](https://github.com/manami-project/anime-offline-database)

## licence

aroe est sous [licence MIT](LICENSE).
