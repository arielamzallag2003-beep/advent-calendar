pub const NOM: &str = "Exercice 1: FizzBuzz";

pub fn run() -> String {
    let mut resultat = Vec::new();  
    
    for i in 0..=150 {
        if i % 15 == 0 {
            resultat.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            resultat.push("Fizz".to_string());
        } else if i % 5 == 0 {
            resultat.push("Buzz".to_string());
        } else {
            resultat.push(i.to_string());
        }
    }
    
    resultat.join(", ")
}	