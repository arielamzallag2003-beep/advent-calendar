mod days;

use std::io::{self, Write};

fn main() {
    loop {
        match show_menu() {
            MenuAction::RunDay(day) => {
                if let Err(e) = days::run_day(day) {
                    eprintln!("\nErreur: {}", e);
                }
                pause();
            }
            MenuAction::RunAll => {
                run_all_days();
                pause();
            }
            MenuAction::Quit => {
                println!("\nAu revoir!");
                break;
            }
            MenuAction::Invalid => {
                println!("\nChoix invalide. Veuillez réessayer.");
            }
        }
    }
}

enum MenuAction {
    RunDay(u8),
    RunAll,
    Quit,
    Invalid,
}

fn show_menu() -> MenuAction {
    clear_screen();
    print_header();
    print_available_days();
    print_options();

    print!("\nVotre choix: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return MenuAction::Invalid;
    }

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "q" | "quit" | "exit" => MenuAction::Quit,
        "a" | "all" | "tous" => MenuAction::RunAll,
        _ => {
            if let Ok(day) = input.parse::<u8>() {
                if day >= 1 && day <= 12 {
                    MenuAction::RunDay(day)
                } else {
                    MenuAction::Invalid
                }
            } else {
                MenuAction::Invalid
            }
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn print_header() {
    println!(r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║         🎄  ADVENT OF CODE - CALENDRIER 🎄               ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
    "#);
}

fn print_available_days() {
    let available = days::available_days();

    println!("    Jours disponibles:");
    println!("    ───────────────────");

    for day in &available {
        println!("    [{:02}] {}", day.day_number(), day.title());
    }

    if available.is_empty() {
        println!("    Aucun jour implémenté pour le moment.");
    }

    println!();
}

fn print_options() {
    println!("    ───────────────────────────────────────────────────────");
    println!("    Options:");
    println!("    [1-12]  Lancer un jour spécifique");
    println!("    [a]     Lancer tous les jours");
    println!("    [q]     Quitter");
    println!("    ───────────────────────────────────────────────────────");
}

fn run_all_days() {
    println!("\n    Exécution de tous les jours...\n");

    let available = days::available_days();

    for day in available {
        if let Err(e) = days::run_day(day.day_number()) {
            eprintln!("    Jour {:02}: {}", day.day_number(), e);
        }
        println!();
    }
}

fn pause() {
    println!("\n    Appuyez sur Entrée pour continuer...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
