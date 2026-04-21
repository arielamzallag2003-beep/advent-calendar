# Ajouter un exercice

## 1. Créer le fichier

Créer `src/exercices/exo_XX.rs` :
```rust
pub const NOM: &str = "Nom de l'exercice";

pub fn run() -> String {
    // Code ici
    "resultat".to_string()
}
```

## 2. Enregistrer dans mod.rs

Dans `src/exercices/mod.rs` :
```rust
pub mod exo_XX;  // Ajouter en haut
```

Puis ajouter dans les 3 fonctions :
```rust
// liste_exercices()
(XX, exo_XX::NOM.to_string()),

// get_nom()
XX => exo_XX::NOM.to_string(),

// executer()
XX => Ok(exo_XX::run()),
```

## Exemple complet

`exo_02.rs` :
```rust
pub const NOM: &str = "Somme 1 a 100";

pub fn run() -> String {
    let somme: u32 = (1..=100).sum();
    somme.to_string()
}
```

`mod.rs` :
```rust
pub mod exo_01;
pub mod exo_02;

pub fn liste_exercices() -> Vec<(u8, String)> {
    vec![
        (1, exo_01::NOM.to_string()),
        (2, exo_02::NOM.to_string()),
    ]
}

pub fn get_nom(id: u8) -> String {
    match id {
        1 => exo_01::NOM.to_string(),
        2 => exo_02::NOM.to_string(),
        _ => "Inconnu".to_string(),
    }
}

pub fn executer(id: u8) -> Result<String, String> {
    match id {
        1 => Ok(exo_01::run()),
        2 => Ok(exo_02::run()),
        _ => Err(format!("Exercice {} non trouve", id)),
    }
}
```

## Lancer
```bash
cargo run
# Menu > e > numero
```