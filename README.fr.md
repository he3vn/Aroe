# aroe

<p align="center">
  <img src="logo.jpeg" alt="aroe" width="100" />
</p>

<p align="center">
  <a href="https://github.com/he3vn/Aroe">github</a> · <a href="#installation">installation</a> · <a href="#utilisation">utilisation</a> · <a href="#comment-ça-marche">comment ça marche</a>
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

> aroe en est à ses débuts. la recherche dans le catalogue fonctionne aujourd'hui ; le recommandeur se construit en public. voir la [feuille de route](#feuille-de-route).

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

## utilisation

aroe pose deux questions :

- **tu débutes dans l'anime ?** si oui, choisis une ambiance et il te propose un point de départ.
- **sinon**, commence à taper un titre et choisis dans le catalogue. il affiche le type, le nombre d'épisodes, l'année et les tags de ce que tu as choisi.

## comment ça marche

1. **lookup** — transformer un titre tapé en vraie entrée de catalogue : id, genres, année, nombre d'épisodes, note, popularité.
2. **cache** — sauvegarder ce qui a été récupéré dans `data/` pour que la prochaine exécution soit instantanée.
3. **profil** — transformer ta liste en préférences pondérées. une série notée 9/10 compte plus qu'une série notée 6/10.
4. **score** — comparer chaque candidat à ce profil. les genres et thèmes partagés pèsent le plus ; l'année et le format départagent.
5. **filtre** — retirer tout ce qui a déjà été vu, plus tout ce qui sort des limites fixées.
6. **présentation** — les N meilleurs résultats, formatés pour le terminal, chacun avec une raison.

## d'où viennent les données

| source | à quoi ça sert | clé nécessaire |
|---|---|---|
| MyAnimeList (via Jikan v4) | catalogue principal : titres, genres, année, note | non |
| AniList (GraphQL) | second catalogue, meilleures requêtes en masse, ses propres listes de similaires | non |

les deux sont gratuits et publics. aucun n'est officiel pour cette app, donc les requêtes sont limitées — c'est toute la raison d'être du cache.

## feuille de route

- [x] rechercher un titre dans le catalogue
- [x] mode « je débute » avec des suggestions de départ
- [ ] mettre les recherches en cache pour éviter de tout recharger
- [ ] comparer deux anime entre eux
- [ ] recommander à partir de ta liste
- [ ] afficher la raison derrière chaque recommandation
- [ ] ajouter AniList comme seconde source
- [ ] sauvegarder ta liste entre les exécutions
- [ ] GUI, si la version terminal le mérite

## pas prévu pour l'instant

- se connecter à ton compte MAL/AniList. lire une vraie liste utilisateur demande OAuth, ce qui est un projet à part.
- scraper le site de MyAnimeList directement. les API existent ; utilise-les.
- une interface web ou bureau avant que la version terminal ne fonctionne.

## développement

```bash
git clone https://github.com/he3vn/Aroe
cd Aroe
cargo build
cargo run
```

## licence

aroe est sous [licence MIT](LICENSE).
