# Ajouter un jour

## 1. Créer le fichier

Créer `src/days/day_XX.rs` :
```rust
pub const TITRE: &str = "Titre du jour";

pub fn partie1(input: &str) -> String {
    // Code ici
    "resultat".to_string()
}

pub fn partie2(input: &str) -> String {
    // Code ici
    "resultat".to_string()
}
```

## 2. Créer l'input

Créer `inputs/day_XX.txt` avec les données du puzzle.

## 3. Enregistrer dans mod.rs

Dans `src/days/mod.rs` :
```rust
pub mod day_XX;  // Ajouter en haut
```

Puis ajouter dans les 3 fonctions :
```rust
// liste_jours_implementes()
vec![1, XX]

// get_titre()
XX => day_XX::TITRE.to_string(),

// executer()
XX => Ok((day_XX::partie1(&input), day_XX::partie2(&input))),
```

## Exemple complet

`day_02.rs` :
```rust
pub const TITRE: &str = "Cube Conundrum";

pub fn partie1(input: &str) -> String {
    let resultat = input.lines().count();
    resultat.to_string()
}

pub fn partie2(input: &str) -> String {
    let resultat = input.len();
    resultat.to_string()
}
```

`mod.rs` :
```rust
pub mod day_01;
pub mod day_02;

pub fn liste_jours_implementes() -> Vec<u8> {
    vec![1, 2]
}

pub fn get_titre(jour: u8) -> String {
    match jour {
        1 => day_01::TITRE.to_string(),
        2 => day_02::TITRE.to_string(),
        _ => "Non implemente".to_string(),
    }
}

pub fn executer(jour: u8) -> Result<(String, String), String> {
    let input = charger_input(jour)?;
    
    match jour {
        1 => Ok((day_01::partie1(&input), day_01::partie2(&input))),
        2 => Ok((day_02::partie1(&input), day_02::partie2(&input))),
        _ => Err(format!("Jour {} non implemente", jour)),
    }
}

fn charger_input(jour: u8) -> Result<String, String> {
    let chemin = format!("inputs/day_{:02}.txt", jour);
    
    match std::fs::read_to_string(&chemin) {
        Ok(contenu) => Ok(contenu),
        Err(_) => Err(format!("Fichier non trouve: {}", chemin)),
    }
}
```

## Lancer
```bash
cargo run
# Menu > numero du jour
```

## Tests (optionnel)

Ajouter à la fin de `day_XX.rs` :
```rust
#[cfg(test)]
mod tests {
    use super::*;

    const EXEMPLE: &str = "donnees test";

    #[test]
    fn test_partie1() {
        assert_eq!(partie1(EXEMPLE), "attendu");
    }

    #[test]
    fn test_partie2() {
        assert_eq!(partie2(EXEMPLE), "attendu");
    }
}
```

Lancer les tests :
```bash
cargo test
```