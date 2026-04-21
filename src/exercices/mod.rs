pub mod exo_01;
pub mod exo_02;
pub mod exo_03;
pub mod exo_04;
pub mod exo_05;
pub mod exo_06;
pub mod exo_07;


#[must_use] 
pub fn liste_exercices() -> Vec<(u8, String)> {
    vec![
        (1, exo_01::NOM.to_string()),
        (2, exo_02::NOM.to_string()),
        (3, exo_03::NOM.to_string()),
        (4, exo_04::NOM.to_string()),
        (5, exo_05::NOM.to_string()),
        (6, exo_06::NOM.to_string()),
        (7, exo_07::NOM.to_string()),
    ]
}

#[must_use] 
pub fn get_nom(id: u8) -> String {
    match id {
        1 => exo_01::NOM.to_string(),
        2 => exo_02::NOM.to_string(),
        3 => exo_03::NOM.to_string(),
        4 => exo_04::NOM.to_string(),
        5 => exo_05::NOM.to_string(),
        6 => exo_06::NOM.to_string(),
        7 => exo_07::NOM.to_string(),
        _ => "Inconnu".to_string(),
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn executer(id: u8) -> Result<String, String> {
    match id {
        1 => Ok(exo_01::run()),
        2 => Ok(exo_02::run()),
        3 => Ok(exo_03::run()),
        4 => Ok(exo_04::run()),
        5 => Ok(exo_05::run()),
        6 => Ok(exo_06::run()),
        7 => Ok(exo_07::run()),
        _ => Err(format!("Exercice {id} non trouve")),
    }
}