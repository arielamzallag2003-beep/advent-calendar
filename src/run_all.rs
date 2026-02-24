#![allow(dead_code)]

use advent_calendar::days;

use std::time::Instant;

fn main() {
    let jours = days::liste_jours_implementes();

    println!();
    println!("  +=============================================+");
    println!("  |       ADVENT OF CODE — TOUS LES JOURS      |");
    println!("  +=============================================+");
    println!();

    let debut_total = Instant::now();

    for jour in &jours {
        let titre = days::get_titre(*jour);

        println!("  +---------------------------------------------+");
        println!("  | Jour {jour:02}: {titre:34} |");
        println!("  +---------------------------------------------+");

        let debut = Instant::now();
        match days::executer(*jour) {
            Ok((p1, p2)) => {
                let dur = debut.elapsed();
                println!("    Partie 1 : {p1}");
                println!("    Partie 2 : {p2}");
                println!("    Temps    : {:.3}ms", dur.as_secs_f64() * 1000.0);
            }
            Err(e) => println!("    Erreur   : {e}"),
        }
        println!();
    }

    let total = debut_total.elapsed();
    println!("  =============================================");
    println!("  {} jour(s) | Temps total : {:.3}ms", jours.len(), total.as_secs_f64() * 1000.0);
    println!("  =============================================");
    println!();
}
