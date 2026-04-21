pub const NOM: &str = "Exercice 3: FizzBuzz N-ieme";

#[must_use] 
pub fn run() -> String {
    let a = 3;
    let b = 5;
    let c = 15;
    let d = 17;
    
    format!(
        "FizzBuzz({}) = {}\nFizzBuzz({}) = {}\nFizzBuzz({}) = {}\nFizzBuzz({}) = {}",
        a, fizzbuzz(a),
        b, fizzbuzz(b),
        c, fizzbuzz(c),
        d, fizzbuzz(d),
    )
}

fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}