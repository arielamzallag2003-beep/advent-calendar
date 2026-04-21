pub const NOM: &str = "Exercice 4: String push/pop (300M fois)";

const N: usize = 300_000_000;

#[must_use] 
pub fn run() -> String {
    let mut s = String::with_capacity(N);

    for i in 0..N {
        let c = match i % 3 {
            0 => 'a',
            1 => 'b',
            _ => 'c',
        };
        s.push(c);
    }

    let longueur_apres_push = s.len();

    for _ in 0..N {
        s.pop();
    }

    let longueur_apres_pop = s.len();

    format!(
        "String::push() x{N} => longueur = {longueur_apres_push}\n\
         String::pop()  x{N} => longueur = {longueur_apres_pop}"
    )
}
