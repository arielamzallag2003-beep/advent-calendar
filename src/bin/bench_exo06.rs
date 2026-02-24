const N: usize = 300_000_000;
const CAPACITE_OCTETS: usize = N * 4;

fn main() {
    let mut s = String::with_capacity(CAPACITE_OCTETS);

    for i in 0..N {
        let c = match i % 3 {
            0 => '🦀',
            1 => '🦁',
            _ => '🦂',
        };
        s.push(c);
    }

    let apres_push = s.chars().count();

    for _ in 0..N {
        s.pop();
    }

    println!("String emoji push x{N} => {apres_push}  |  pop x{N} => {}", s.len());
}
