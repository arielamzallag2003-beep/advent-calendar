use advent_calendar::days;
use advent_calendar::exercices;

use std::io;
use std::io::Write;
use std::time::Instant;

fn main() {
    loop {
        afficher_menu();
        
        let choix = lire_entree();
        
        match choix.as_str() {
            "q" | "quit" => {
                println!("\n  Au revoir!\n");
                break;
            }
            "a" | "all" => {
                executer_tous_les_jours();
                attendre_entree();
            }
            "l" | "list" => {
                afficher_liste();
                attendre_entree();
            }
            "e" | "exo" => {
                menu_exercices();
            }
            _ => {
                match choix.parse::<u8>() {
                    Ok(jour) if (1..=12).contains(&jour) => {
                        menu_jour(jour);
                    }
                    Ok(jour) => {
                        println!("\n  Jour {jour} invalide (1-12)");
                        attendre_entree();
                    }
                    Err(_) => {
                        println!("\n  Commande inconnue: {choix}");
                        attendre_entree();
                    }
                }
            }
        }
    }
}

fn afficher_menu() {
    effacer_ecran();
    
    println!();
    println!("  +=============================================+");
    println!("  |       ADVENT OF CODE - CALENDRIER RUST      |");
    println!("  +=============================================+");
    println!();
    
    // Grille des jours
    println!("  Jours:");
    print!("  ");
    
    let jours_faits = days::liste_jours_implementes();
    
    for jour in 1..=12 {
        if jours_faits.contains(&jour) {
            print!("[{jour:02}] ");
        } else {
            print!(" {jour:02}  ");
        }
        
        if jour == 6 {
            println!();
            print!("  ");
        }
    }
    println!();
    println!();
    println!("  [ ] = non fait   [XX] = fait");
    println!();
    println!("  ---------------------------------------------");
    println!("  Commandes:");
    println!("    1-12    Executer un jour");
    println!("    a       Executer tous les jours");
    println!("    l       Liste detaillee");
    println!("    e       Exercices pratiques");
    println!("    q       Quitter");
    println!("  ---------------------------------------------");
}

fn menu_exercices() {
    loop {
        effacer_ecran();
        
        println!();
        println!("  +=============================================+");
        println!("  |            EXERCICES PRATIQUES              |");
        println!("  +=============================================+");
        println!();
        
        let exos = exercices::liste_exercices();
        
        if exos.is_empty() {
            println!("  Aucun exercice disponible.");
        } else {
            for (id, nom) in &exos {
                println!("    [{id}] {nom}");
            }
        }
        
        println!();
        println!("  ---------------------------------------------");
        println!("  Commandes:");
        println!("    <id>    Executer un exercice");
        println!("    b       Retour au menu principal");
        println!("  ---------------------------------------------");
        
        let choix = lire_entree();
        
        if choix == "b" || choix == "back" || choix == "q" {
            break;
        }
        
        if let Ok(id) = choix.parse::<u8>() {
            executer_exercice(id);
            attendre_entree();
        } else {
            println!("\n  Commande inconnue: {choix}");
            attendre_entree();
        }
    }
}

fn executer_exercice(id: u8) {
    println!();
    
    let nom = exercices::get_nom(id);
    println!("  +---------------------------------------------+");
    println!("  | Exercice {id}: {nom:32} |");
    println!("  +---------------------------------------------+");
    
    let debut = Instant::now();
    
    match exercices::executer(id) {
        Ok(resultat) => {
            let duree = debut.elapsed();
            println!("    Resultat: {resultat}");
            println!("    Temps: {:.3}ms", duree.as_secs_f64() * 1000.0);
        }
        Err(erreur) => {
            println!("    Erreur: {erreur}");
        }
    }
}

fn effacer_ecran() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn lire_entree() -> String {
    print!("\n  Choix: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_lowercase()
}

fn attendre_entree() {
    println!("\n  Appuyez sur Entree...");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

/// Sub-menu shown when the user selects a specific day.
fn menu_jour(jour: u8) {
    loop {
        effacer_ecran();

        let titre = days::get_titre(jour);
        let test_dispo = days::test_input_existe(jour);

        println!();
        println!("  +=============================================+");
        println!("  | Jour {jour:02}: {titre:34} |");
        println!("  +=============================================+");
        println!();
        println!("  Commandes:");
        println!("    r    Input reel  (inputs/day_{jour:02}.txt)");
        if test_dispo {
            println!("    t    Input test (inputs/day_{jour:02}_test.txt)");
        } else {
            println!("    t    Input test [non disponible]");
        }
        println!("    b    Retour");
        println!("  ---------------------------------------------");

        let choix = lire_entree();

        match choix.as_str() {
            "b" | "back" | "q" => break,
            "r" | "" => {
                afficher_resultat_jour(jour, false);
                attendre_entree();
            }
            "t" => {
                if test_dispo {
                    afficher_resultat_jour(jour, true);
                } else {
                    println!("\n  Cree d'abord inputs/day_{jour:02}_test.txt");
                }
                attendre_entree();
            }
            _ => {
                println!("\n  Commande inconnue: {choix}");
                attendre_entree();
            }
        }
    }
}

/// Run a day and print its results. `test` selects the test input file.
fn afficher_resultat_jour(jour: u8, test: bool) {
    println!();
    let label = if test { "TEST" } else { "REEL" };
    println!("  --- Input {label} ---");

    let debut = Instant::now();
    let resultat = if test {
        days::executer_test(jour)
    } else {
        days::executer(jour)
    };

    match resultat {
        Ok((partie1, partie2)) => {
            let duree = debut.elapsed();
            println!("    Partie 1: {partie1}");
            println!("    Partie 2: {partie2}");
            println!("    Temps: {:.3}ms", duree.as_secs_f64() * 1000.0);
        }
        Err(erreur) => {
            println!("    Erreur: {erreur}");
        }
    }
}

/// Used by "run all days" — always uses real input.
fn executer_jour(jour: u8) {
    println!();
    println!("  +---------------------------------------------+");
    let titre = days::get_titre(jour);
    println!("  | Jour {jour:02}: {titre:34} |");
    println!("  +---------------------------------------------+");
    afficher_resultat_jour(jour, false);
}

fn executer_tous_les_jours() {
    println!("\n  Execution de tous les jours...\n");
    
    let jours = days::liste_jours_implementes();
    let debut_total = Instant::now();
    
    for jour in jours {
        executer_jour(jour);
        println!();
    }
    
    let duree_totale = debut_total.elapsed();
    
    println!("  =============================================");
    println!("  Temps total: {:.3}ms", duree_totale.as_secs_f64() * 1000.0);
    println!("  =============================================");
}

fn afficher_liste() {
    println!("\n  Liste des jours:\n");
    
    let jours_faits = days::liste_jours_implementes();
    
    for jour in 1..=12 {
        let statut = if jours_faits.contains(&jour) { "[x]" } else { "[ ]" };
        let titre = days::get_titre(jour);
        
        println!("  {statut} Jour {jour:02}: {titre}");
    }
    
    println!();
    println!("  Total: {}/12 jours", jours_faits.len());
}