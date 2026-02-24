pub const NOM: &str = "Exercice 7: Vec<char> push/pop emojis (300M fois)";

const N: usize = 300_000_000;

#[must_use] 
pub fn run() -> String {
    let mut v: Vec<char> = Vec::with_capacity(N);

    for i in 0..N {
        let c = match i % 3 {
            0 => '🦀',
            1 => '🦁',
            _ => '🦂',
        };
        v.push(c);
    }

    let longueur_apres_push = v.len();

    for _ in 0..N {
        v.pop();
    }

    let longueur_apres_pop = v.len();

    format!(
        "Vec<char> emoji push() x{N} => {longueur_apres_push} chars\n\
         Vec<char> emoji pop()  x{N} => {longueur_apres_pop} chars restants"
    )
}
