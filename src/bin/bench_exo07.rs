const N: usize = 300_000_000;

fn main() {
    let mut v: Vec<char> = Vec::with_capacity(N);

    for i in 0..N {
        let c = match i % 3 {
            0 => '🦀',
            1 => '🦁',
            _ => '🦂',
        };
        v.push(c);
    }

    let apres_push = v.len();

    for _ in 0..N {
        v.pop();
    }

    println!("Vec<char> emoji push x{N} => {apres_push}  |  pop x{N} => {}", v.len());
}
