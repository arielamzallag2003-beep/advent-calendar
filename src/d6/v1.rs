pub fn partie1(input: &str) -> String {
    // Collect all lines into a vector
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }

    let total_lines = lines.len();
    let operator_line = lines[total_lines - 1];

    // Find the maximum line length
    let mut max_len = 0;
    for line in &lines {
        if line.len() > max_len {
            max_len = line.len();
        }
    }

    // find separator
    let mut is_separator: Vec<bool> = Vec::new();
    for col in 0..max_len {
        let mut all_spaces = true;
        for line in &lines {
            let ch = line.as_bytes().get(col).copied().unwrap_or(b' ');
            if ch != b' ' {
                all_spaces = false;
                break;
            }
        }
        is_separator.push(all_spaces);
    }

        // assign problems index
    let mut col_to_problem: Vec<Option<usize>> = vec![None; max_len];
    let mut current_problem = 0;
    let mut inside_problem = false;

    for col in 0..max_len {
        if !is_separator[col] {
            inside_problem = true;
            col_to_problem[col] = Some(current_problem);
        } else if inside_problem {
            current_problem += 1;
            inside_problem = false;
        }
    }

    let num_problems = if inside_problem { current_problem + 1 } else { current_problem };

    // add problem operator
    let mut operators: Vec<char> = Vec::new();
    for _ in 0..num_problems {
        operators.push('+');
    }

    for col in 0..operator_line.len() {
        let ch = operator_line.as_bytes()[col];
        if ch == b'*' {
            if let Some(problem_index) = col_to_problem[col] {
                operators[problem_index] = '*';
            }
        }
    }

    let mut results: Vec<i64> = Vec::new();
    for op in &operators {
        if *op == '*' {
            results.push(1);
        } else {
            results.push(0);
        }
    }

    // Part 1 specific
    for line in &lines[..total_lines - 1] {
        let bytes = line.as_bytes();
        let mut col = 0;

        while col < bytes.len() {
            if bytes[col].is_ascii_digit() {
                let start_col = col;
                while col < bytes.len() && bytes[col].is_ascii_digit() {
                    col += 1;
                }
                let number: i64 = line[start_col..col].parse().unwrap();

                if let Some(problem_index) = col_to_problem[start_col] {
                    if operators[problem_index] == '*' {
                        results[problem_index] *= number;
                    } else {
                        results[problem_index] += number;
                    }
                }
            } else {
                col += 1;
            }
        }
    }

    let mut grand_total: i64 = 0;
    for result in &results {
        grand_total += result;
    }

    grand_total.to_string()
}


pub fn partie2(input: &str) -> String {
    // Collect all lines into a vector
    let mut lines: Vec<&str> = Vec::new();
    for line in input.lines() {
        lines.push(line);
    }

    let total_lines = lines.len();
    let operator_line = lines[total_lines - 1];

    // Find the maximum line length
    let mut max_len = 0;
    for line in &lines {
        if line.len() > max_len {
            max_len = line.len();
        }
    }

    // find separator
    let mut is_separator: Vec<bool> = Vec::new();
    for col in 0..max_len {
        let mut all_spaces = true;
        for line in &lines {
            let ch = line.as_bytes().get(col).copied().unwrap_or(b' ');
            if ch != b' ' {
                all_spaces = false;
                break;
            }
        }
        is_separator.push(all_spaces);
    }

    // assign problems index
    let mut col_to_problem: Vec<Option<usize>> = vec![None; max_len];
    let mut current_problem = 0;
    let mut inside_problem = false;

    for col in 0..max_len {
        if !is_separator[col] {
            inside_problem = true;
            col_to_problem[col] = Some(current_problem);
        } else if inside_problem {
            current_problem += 1;
            inside_problem = false;
        }
    }

    let num_problems = if inside_problem { current_problem + 1 } else { current_problem };

    // add problem operator
    let mut operators: Vec<char> = Vec::new();
    for _ in 0..num_problems {
        operators.push('+');
    }

    for col in 0..operator_line.len() {
        let ch = operator_line.as_bytes()[col];
        if ch == b'*' {
            if let Some(problem_index) = col_to_problem[col] {
                operators[problem_index] = '*';
            }
        }
    }

    let mut results: Vec<i64> = Vec::new();
    for op in &operators {
        if *op == '*' {
            results.push(1);
        } else {
            results.push(0);
        }
    }

    // Part 2 specific
    let mut col = max_len;
    while col > 0 {
        col -= 1;

        if let Some(problem_index) = col_to_problem[col] {
            let mut digit_string = String::new();
            for row in 0..total_lines - 1 {
                let ch = lines[row].as_bytes().get(col).copied().unwrap_or(b' ');
                if ch.is_ascii_digit() {
                    digit_string.push(ch as char);
                }
            }

            if !digit_string.is_empty() {
                let number: i64 = digit_string.parse().unwrap();
                if operators[problem_index] == '*' {
                    results[problem_index] *= number;
                } else {
                    results[problem_index] += number;
                }
            }
        }
    }

    let mut grand_total: i64 = 0;
    for result in &results {
        grand_total += result;
    }

    grand_total.to_string()
}