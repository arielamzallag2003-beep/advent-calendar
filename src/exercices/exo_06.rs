pub const NOM: &str = "Exercice 6: String push/pop emojis (300M fois)";

const N: usize = 300_000_000;

const CAPACITE_OCTETS: usize = N * 4;

#[must_use] 
pub fn run() -> String {
    let mut s = String::with_capacity(CAPACITE_OCTETS);

    for i in 0..N {
        let c = match i % 3 {
            0 => '🦀',
            1 => '🦁',
            _ => '🦂',
        };
        s.push(c);
    }

    let longueur_apres_push = s.chars().count();

    for _ in 0..N {
        s.pop();
    }

    let longueur_apres_pop = s.len();

    format!(
        "String emoji push() x{N} => {longueur_apres_push} chars\n\
         String emoji pop()  x{N} => {longueur_apres_pop} octets restants"
    )
}
