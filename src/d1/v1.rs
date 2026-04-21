pub fn partie1(input: &str) -> String {
    println!("V1");

    let mut counter = 0;
    let s = input.lines();
    let mut arrow = 50;
    let mut turn_number = 0;
    for line in s {
        let mut c: std::str::Chars<'_> = line.chars();
        c.next();
        turn_number = c.as_str().parse::<i32>().unwrap();
        if (line.as_bytes()[0] == "L".as_bytes()[0]){
            arrow = (arrow + (100-turn_number%100)) % 100;
        }
        else {
            arrow = (arrow + turn_number % 100) % 100;
        }
        if (arrow == 0){
            counter+=1;
        }
    }
    return counter.to_string();
}

pub fn partie2(input: &str) -> String {
    let mut counter = 0;
    let s = input.lines();
    let mut arrow = 50;
    let mut turn_number = 0;
    for line in s {
        let mut c: std::str::Chars<'_> = line.chars();
        c.next();
        turn_number = c.as_str().parse::<i32>().unwrap();
        if (line.as_bytes()[0] == "L".as_bytes()[0]){
            for _ in 0..turn_number {
                arrow = (arrow - 1 + 100) % 100;
                if (arrow == 0){ counter += 1; }
            }
        }
        else {
            for _ in 0..turn_number {
                arrow = (arrow + 1) % 100;
                if (arrow == 0){ counter += 1; }
            }
        }
    }
    return counter.to_string();
}