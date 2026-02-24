# 🎄 Advent of Code — Calendrier Rust

Projet d'apprentissage Rust structuré autour des puzzles [Advent of Code](https://adventofcode.com/).

---

## Structure du projet

```
src/
  main.rs          → menu interactif
  run_all.rs       → exécuter tous les jours d'un coup
  lib.rs           → expose les modules (utilisé par les benchmarks)
  days/
    mod.rs         → dispatcher (relie les puzzles au menu)
    GUIDE_DAYS.md  → guide pour ajouter un jour
  d1/mod.rs        → solution du jour 1
  d2/mod.rs        → solution du jour 2 
  ...
  d12/mod.rs       → solution du jour 12
  exercices/
    mod.rs
    exo_01.rs … exo_07.rs
  bin/
    bench_exo04.rs … bench_exo07.rs   → benchmarks manuels

inputs/
  day_01.txt           → input réel du jour 1 (non commité)
  day_01_test.txt      → exemple du puzzle (non commité)
  ...

benches/
  d01.rs … d12.rs     → benchmarks Criterion par jour
```

---

## Commandes

### Menu interactif
```bash
cargo run --release
```
Depuis le menu :
- `1`–`12` → ouvre le sous-menu du jour (input réel ou test)
- `a` → exécute tous les jours implémentés
- `e` → menu des exercices pratiques
- `l` → liste des jours avec leur statut
- `q` → quitter

### Exécuter tous les jours (sans menu)
```bash
cargo run --release --bin run-all
```

### Benchmarks Criterion
```bash
cargo bench --bench d01          # benchmarker le jour 01
cargo bench                      # tous les jours
```

Sauvegarder/comparer une baseline (le nom est libre, tu choisis) :
```bash
# Avant d'optimiser → sauvegarder sous le nom "brute_force"
cargo bench --bench d01 -- --save-baseline brute_force

# Après avoir réécrit la solution → comparer à "brute_force"
cargo bench --bench d01 -- --baseline brute_force
```
Criterion affiche l'écart en % et indique si c'est une régression ou une amélioration.

Rapport HTML généré : `target/criterion/Jour 01/report/index.html`

### Benchmarks manuels (exercices)
```bash
cargo run --release --bin bench_exo07
```

### Tests unitaires
```bash
cargo test
```

---

## Ajouter un jour

> Voir aussi `src/days/GUIDE_DAYS.md`

**1.** Ouvrir `src/dN/mod.rs` et implémenter la solution :

```rust
use crate::days::Day;

pub struct DayN;

impl Day for DayN {
    fn titre(&self) -> &str { "Nom du puzzle" }

    fn partie1(&self, input: &str) -> String {
        // ta solution ici
        "resultat".to_string()
    }

    fn partie2(&self, input: &str) -> String {
        // ta solution ici
        "resultat".to_string()
    }
}
```

**2.** Déclarer le jour comme implémenté dans `src/days/mod.rs` :

```rust
pub fn liste_jours_implementes() -> Vec<u8> {
    vec![1, N]  // ← ajouter N
}
```

**3.** Créer les fichiers d'input :
- `inputs/day_0N.txt` → ton input personnel (sur le site AoC)
- `inputs/day_0N_test.txt` → l'exemple donné dans l'énoncé

C'est tout. Le menu, `run-all` et les benchmarks fonctionnent automatiquement.

---

## Dépendances

| Crate | Usage |
|---|---|
| [`criterion`](https://docs.rs/criterion) | Benchmarks statistiques avec rapports HTML |

---

## Notes

- Les fichiers `inputs/` ne sont **pas commités** (`.gitignore`) — chaque personne utilise son propre input AoC.
- Les stubs (jours non implémentés) retournent `"Non implemente"` et ne font pas crasher le programme.
- Les benchmarks Criterion sauvegardent les baselines dans `target/criterion/` — faire `cargo bench -- --save-baseline <nom>` avant d'optimiser pour pouvoir comparer ensuite.
