
pub const NOM: &str = "Exercice 5: Vec<char> push/pop (300M fois)";

const N: usize = 300_000_000;

#[must_use] 
pub fn run() -> String {
    let mut v: Vec<char> = Vec::with_capacity(N);

    // --- Phase push ---
    for i in 0..N {
        let c = match i % 3 {
            0 => 'a',
            1 => 'b',
            _ => 'c',
        };
        v.push(c);
    }

    let longueur_apres_push = v.len();

    for _ in 0..N {
        v.pop();
    }

    let longueur_apres_pop = v.len();

    format!(
        "Vec<char>::push() x{N} => longueur = {longueur_apres_push}\n\
         Vec<char>::pop()  x{N} => longueur = {longueur_apres_pop}"
    )
}
