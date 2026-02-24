pub const NOM: &str = "Exercice 2: Fibonacci";

#[must_use] 
pub fn run() -> String {
    let n = 42;
    let m = 51;
    let o = 42;
    let p = 51;
    
    format!(
        "Iteratif:\n  Fibonacci({}) = {}\n  Fibonacci({}) = {}\n\nRecursif:\n  Fibonacci({}) = {}\n  Fibonacci({}) = {}", 
        n, fibonacci(n), 
        m, fibonacci(m),
        o, fibonacci_recursif(o),
        p, fibonacci_recursif(p)
    )
}

// itérative
fn fibonacci(n: u32) -> u64 {
    if n <= 1 { return u64::from(n); }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    b
}

// récursive
fn fibonacci_recursif(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursif(n - 1) + fibonacci_recursif(n - 2),
    }
}