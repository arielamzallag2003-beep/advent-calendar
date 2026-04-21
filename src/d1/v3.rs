pub fn partie1(input: &str) -> String {
    // Input reading
    let instructions: Vec<(char, i32)> = input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse::<i32>().unwrap()))
        .collect();

    let mut counter = 0;
    let mut arrow = 50;
    let mut turn_number;
    for (dir, num) in &instructions {
        turn_number = *num;
        if *dir == 'L' {
            arrow = (arrow + (100-turn_number%100)) % 100;
        }
        else {
            arrow = (arrow + turn_number % 100) % 100;
        }
        if arrow == 0 {
            counter+=1;
        }
    }
    return counter.to_string();
}

pub fn partie2(input: &str) -> String {
    // Input reading
    let instructions: Vec<(char, i32)> = input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse::<i32>().unwrap()))
        .collect();

    let mut counter = 0;
    let mut arrow = 50;
    let mut turn_number;
    for (dir, num) in &instructions {
        turn_number = *num;
        if *dir == 'L' {
            for _ in 0..turn_number {
                arrow = (arrow - 1 + 100) % 100;
                if arrow == 0 { counter += 1; }
            }
        }
        else {
            for _ in 0..turn_number {
                arrow = (arrow + 1) % 100;
                if arrow == 0 { counter += 1; }
            }
        }
    }
    return counter.to_string();
}