pub const NOM: &str = "Exercice 1: FizzBuzz";

pub fn run() -> String {
    let mut resultat = Vec::new();  
    let mut resultat_match = Vec::new(); 
    
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

    for i in 0..=150 {
        let s = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        };
        resultat_match.push(s);
    }
    
    format!(
        "If/Else: {}\n\nMatch: {}",
        resultat.join(", "),
        resultat_match.join(", ")
    )
}