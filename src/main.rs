mod days;
mod exercices;

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
                    Ok(jour) if jour >= 1 && jour <= 12 => {
                        executer_jour(jour);
                        attendre_entree();
                    }
                    Ok(jour) => {
                        println!("\n  Jour {} invalide (1-12)", jour);
                        attendre_entree();
                    }
                    Err(_) => {
                        println!("\n  Commande inconnue: {}", choix);
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
            print!("[{:02}] ", jour);
        } else {
            print!(" {:02}  ", jour);
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
                println!("    [{}] {}", id, nom);
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
        
        match choix.parse::<u8>() {
            Ok(id) => {
                executer_exercice(id);
                attendre_entree();
            }
            Err(_) => {
                println!("\n  Commande inconnue: {}", choix);
                attendre_entree();
            }
        }
    }
}

fn executer_exercice(id: u8) {
    println!();
    
    let nom = exercices::get_nom(id);
    println!("  +---------------------------------------------+");
    println!("  | Exercice {}: {:32} |", id, nom);
    println!("  +---------------------------------------------+");
    
    let debut = Instant::now();
    
    match exercices::executer(id) {
        Ok(resultat) => {
            let duree = debut.elapsed();
            println!("    Resultat: {}", resultat);
            println!("    Temps: {:.3}ms", duree.as_secs_f64() * 1000.0);
        }
        Err(erreur) => {
            println!("    Erreur: {}", erreur);
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

fn executer_jour(jour: u8) {
    println!();
    println!("  +---------------------------------------------+");
    
    let titre = days::get_titre(jour);
    println!("  | Jour {:02}: {:34} |", jour, titre);
    
    println!("  +---------------------------------------------+");
    
    let debut = Instant::now();
    
    match days::executer(jour) {
        Ok((partie1, partie2)) => {
            let duree = debut.elapsed();
            
            println!("    Partie 1: {}", partie1);
            println!("    Partie 2: {}", partie2);
            println!("    Temps: {:.3}ms", duree.as_secs_f64() * 1000.0);
        }
        Err(erreur) => {
            println!("    Erreur: {}", erreur);
        }
    }
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
        
        println!("  {} Jour {:02}: {}", statut, jour, titre);
    }
    
    println!();
    println!("  Total: {}/12 jours", jours_faits.len());
}