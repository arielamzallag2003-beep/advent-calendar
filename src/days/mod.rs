pub mod day_01;

pub trait Day {
    fn titre(&self) -> &str;
    fn partie1(&self, input: &str) -> String;
    fn partie2(&self, input: &str) -> String;
}

pub fn liste_jours_implementes() -> Vec<u8> {
    vec![1]
}

pub fn get_titre(jour: u8) -> String {
    match jour {
        1 => day_01::Day01.titre().to_string(),
        _ => "Non implemente".to_string(),
    }
}

pub fn executer(jour: u8) -> Result<(String, String), String> {
    let input = charger_input(jour)?;
    
    match jour {
        1 => {
            let d = day_01::Day01;
            Ok((d.partie1(&input), d.partie2(&input)))
        }
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