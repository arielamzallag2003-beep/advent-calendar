pub mod exo_01;

pub fn liste_exercices() -> Vec<(u8, String)> {
    vec![
        (1, exo_01::NOM.to_string()),
    ]
}

pub fn get_nom(id: u8) -> String {
    match id {
        1 => exo_01::NOM.to_string(),
        _ => "Inconnu".to_string(),
    }
}

pub fn executer(id: u8) -> Result<String, String> {
    match id {
        1 => Ok(exo_01::run()),
        _ => Err(format!("Exercice {} non trouve", id)),
    }
}