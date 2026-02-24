pub trait Day {
    fn titre(&self) -> &'static str;
    fn partie1(&self, input: &str) -> String;
    fn partie2(&self, input: &str) -> String;
}

#[must_use] 
pub fn liste_jours_implementes() -> Vec<u8> {
    vec![1, 2, 3]
}

#[must_use] 
pub fn get_titre(jour: u8) -> String {
    match jour {
        1  => crate::d1::Day01.titre().to_string(),
        2  => crate::d2::Day02.titre().to_string(),
        3  => crate::d3::Day03.titre().to_string(),
        4  => crate::d4::Day04.titre().to_string(),
        5  => crate::d5::Day05.titre().to_string(),
        6  => crate::d6::Day06.titre().to_string(),
        7  => crate::d7::Day07.titre().to_string(),
        8  => crate::d8::Day08.titre().to_string(),
        9  => crate::d9::Day09.titre().to_string(),
        10 => crate::d10::Day10.titre().to_string(),
        11 => crate::d11::Day11.titre().to_string(),
        12 => crate::d12::Day12.titre().to_string(),
        _  => "Non implemente".to_string(),
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn executer(jour: u8) -> Result<(String, String), String> {
    let input = charger_input(jour)?;
    executer_sur(jour, &input)
}

#[allow(clippy::missing_errors_doc)]
pub fn executer_test(jour: u8) -> Result<(String, String), String> {
    let input = charger_test_input(jour)?;
    executer_sur(jour, &input)
}

#[must_use]
pub fn test_input_existe(jour: u8) -> bool {
    let chemin = format!("inputs/day_{jour:02}_test.txt");
    std::path::Path::new(&chemin).exists()
}

fn executer_sur(jour: u8, input: &str) -> Result<(String, String), String> {
    macro_rules! run_day {
        ($struct:expr) => {{
            let d = $struct;
            Ok((d.partie1(input), d.partie2(input)))
        }};
    }

    match jour {
        1  => run_day!(crate::d1::Day01),
        2  => run_day!(crate::d2::Day02),
        3  => run_day!(crate::d3::Day03),
        4  => run_day!(crate::d4::Day04),
        5  => run_day!(crate::d5::Day05),
        6  => run_day!(crate::d6::Day06),
        7  => run_day!(crate::d7::Day07),
        8  => run_day!(crate::d8::Day08),
        9  => run_day!(crate::d9::Day09),
        10 => run_day!(crate::d10::Day10),
        11 => run_day!(crate::d11::Day11),
        12 => run_day!(crate::d12::Day12),
        _  => Err(format!("Jour {jour} non implemente")),
    }
}

// Retourne toujours Ok (fichier manquant → chaîne vide, pas une erreur fatale)
#[allow(clippy::unnecessary_wraps)]
fn charger_input(jour: u8) -> Result<String, String> {
    let chemin = format!("inputs/day_{jour:02}.txt");
    match std::fs::read_to_string(&chemin) {
        Ok(contenu) => Ok(contenu),
        Err(_) => Ok(String::new()),
    }
}

fn charger_test_input(jour: u8) -> Result<String, String> {
    let chemin = format!("inputs/day_{jour:02}_test.txt");
    match std::fs::read_to_string(&chemin) {
        Ok(contenu) => Ok(contenu),
        Err(_) => Err(format!("Fichier test non trouve: {chemin}")),
    }
}
