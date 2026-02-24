const N: usize = 300_000_000;

fn main() {
    let mut s = String::with_capacity(N);

    for i in 0..N {
        let c = match i % 3 {
            0 => 'a',
            1 => 'b',
            _ => 'c',
        };
        s.push(c);
    }

    let apres_push = s.len();

    for _ in 0..N {
        s.pop();
    }

    println!("String push x{N} => {apres_push}  |  pop x{N} => {}", s.len());
}
