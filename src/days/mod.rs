pub mod day_01;

/// Trait que chaque jour doit implémenter
pub trait Day {
    fn day_number(&self) -> u8;
    fn title(&self) -> &str;
    fn solve_part1(&self, input: &str) -> String;
    fn solve_part2(&self, input: &str) -> String;
}

/// Liste des jours disponibles
pub fn available_days() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(day_01::Day01),
    ]
}

/// Exécute un jour spécifique
pub fn run_day(day: u8) -> Result<(), String> {
    let days = available_days();

    let day_impl = days.iter()
        .find(|d| d.day_number() == day)
        .ok_or_else(|| format!("Jour {} non implémenté", day))?;

    let input = load_input(day)?;

    println!("\n═══════════════════════════════════════");
    println!("  Jour {:02}: {}", day, day_impl.title());
    println!("═══════════════════════════════════════\n");

    println!("Partie 1: {}", day_impl.solve_part1(&input));
    println!("Partie 2: {}", day_impl.solve_part2(&input));

    Ok(())
}

/// Charge l'input pour un jour donné
fn load_input(day: u8) -> Result<String, String> {
    let path = format!("inputs/day_{:02}.txt", day);
    std::fs::read_to_string(&path)
        .map_err(|_| format!("Fichier input non trouvé: {}", path))
}
