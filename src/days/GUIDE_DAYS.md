# Ajouter un jour

## Structure

Chaque jour vit dans son propre dossier :
```
src/
  d1/mod.rs   ← jour 1 (implémenté)
  d2/mod.rs   ← jour 2 (stub)
  ...
  d12/mod.rs  ← jour 12 (stub)
```

Les inputs restent dans `inputs/day_XX.txt`.

---

## 1. Implémenter le jour

Ouvrir `src/dXX/mod.rs` et remplacer le stub par la vraie solution :

```rust
use crate::days::Day;

pub struct DayXX;

impl Day for DayXX {
    fn titre(&self) -> &str {
        "Titre du puzzle"
    }

    fn partie1(&self, input: &str) -> String {
        // logique ici
        "resultat".to_string()
    }

    fn partie2(&self, input: &str) -> String {
        // logique ici
        "resultat".to_string()
    }
}
```

## 2. Créer l'input

Créer `inputs/day_XX.txt` avec les données du puzzle.

## 3. Déclarer comme implémenté

Dans `src/days/mod.rs`, ajouter le numéro du jour dans `liste_jours_implementes()` :

```rust
pub fn liste_jours_implementes() -> Vec<u8> {
    vec![1, XX]  // ← ajouter XX ici
}
```

C'est tout ! `get_titre` et `executer` dans `mod.rs` référencent déjà tous les jours automatiquement.

---

## Lancer

```bash
cargo run --release          # menu principal
cargo run --release --bin advent-calendar  # identique
cargo test                   # tests unitaires
```

## Tests (optionnel)

Ajouter à la fin de `src/dXX/mod.rs` :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE: &str = "donnees test";

    #[test]
    fn test_partie1() {
        assert_eq!(DayXX.partie1(EXEMPLE), "attendu");
    }

    #[test]
    fn test_partie2() {
        assert_eq!(DayXX.partie2(EXEMPLE), "attendu");
    }
}
```