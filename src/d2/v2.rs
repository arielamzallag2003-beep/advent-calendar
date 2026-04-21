fn is_invalid_p1(id: i64) -> bool {
    let s = id.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let first = &s[..s.len()/2];
    let second = &s[s.len()/2..];
    first == second
}

fn is_invalid_p2(id: i64) -> bool {
    let s = id.to_string();
    let total_len = s.len();

    for pattern_len in 1..=total_len / 2 {

        if total_len % pattern_len != 0 {
            continue;
        }

        let pattern = s[..pattern_len].to_string();
        let mut all_match = true;

        let mut i = pattern_len;
        while i + pattern_len< total_len {
            let chunk = s[i..i + pattern_len].to_string();
            if chunk != pattern {
                all_match = false;
                break;
            }
            i += pattern_len;
        }

        if all_match {
            return true;
        }
    }

    return false;
}

pub fn partie1(input: &str) -> String {
    let mut total: i64 = 0;

    for range in input.trim().split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].trim().parse::<i64>().unwrap();
        let end = parts[1].trim().parse::<i64>().unwrap();
        for id in start..=end {
            if is_invalid_p1(id) {
                total += id;
            }
        }
    }

    total.to_string()
}

pub fn partie2(input: &str) -> String {
    let mut total: i64 = 0;

    for range in input.trim().split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].trim().parse::<i64>().unwrap();
        let end = parts[1].trim().parse::<i64>().unwrap();
        for id in start..=end {
            if is_invalid_p2(id) {
                total += id;
            }
        }
    }

    total.to_string()
}