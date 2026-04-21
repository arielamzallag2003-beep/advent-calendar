use std::collections::HashSet;

pub fn partie1(input: &str) -> String {
    //create boxes list
    let mut boxes: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() { continue; }
        let mut parts: Vec<i64> = Vec::new();
        for x in line.split(',') {
            parts.push(x.trim().parse::<i64>().unwrap());
        }
        boxes.push((parts[0], parts[1], parts[2]));
    }

    let n = boxes.len();
    
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut used: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..1000 {
        let mut min_dist: i64 = i64::MAX;
        let mut best = (0usize, 1usize);
        for i in 0..n {
            for j in (i + 1)..n {
                if used.contains(&(i, j)) { continue; }
                let dx = boxes[i].0 - boxes[j].0;
                let dy = boxes[i].1 - boxes[j].1;
                let dz = boxes[i].2 - boxes[j].2;
                let dist = dx * dx + dy * dy + dz * dz;
                if dist < min_dist {
                    min_dist = dist;
                    best = (i, j);
                }
            }
        }
        used.insert(best);
        pairs.push(best);
    }

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b) in &pairs {
        adj[*a].push(*b);
        adj[*b].push(*a);
    }

    let mut remaining: Vec<usize> = Vec::new();
    for i in 0..n {
        remaining.push(i);
    }
    let mut circuit_sizes: Vec<usize> = Vec::new();

    while !remaining.is_empty() {
        let start = remaining.remove(0);
        let mut circuit_size = 1;

        let mut connected: Vec<usize> = vec![start];
        loop {
            let prev_len = connected.len();
            for i in 0..connected.len() {
                let node = connected[i];
                for &neighbor in &adj[node] {
                    if !connected.contains(&neighbor) {
                        connected.push(neighbor);
                    }
                }
            }
            if connected.len() == prev_len { break; }
        }

        for &node in &connected {
            if node != start {
                if let Some(pos) = remaining.iter().position(|&x| x == node) {
                    remaining.remove(pos);
                    circuit_size += 1;
                }
            }
        }

        circuit_sizes.push(circuit_size);
    }

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
    return result.to_string();
}

pub fn partie2(input: &str) -> String {
    let mut boxes: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() { continue; }
        let mut parts: Vec<i64> = Vec::new();
        for x in line.split(',') {
            parts.push(x.trim().parse::<i64>().unwrap());
        }
        boxes.push((parts[0], parts[1], parts[2]));
    }

    let n = boxes.len();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut used: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut last_x_product: i64 = 0;

    loop {
        let mut connected: Vec<usize> = vec![0];
        loop {
            let prev_len = connected.len();
            for i in 0..connected.len() {
                let node = connected[i];
                for &neighbor in &adj[node] {
                    if !connected.contains(&neighbor) {
                        connected.push(neighbor);
                    }
                }
            }
            if connected.len() == prev_len { break; }
        }
        if connected.len() == n { break; }

        let mut min_dist: i64 = i64::MAX;
        let mut best = (0usize, 1usize);
        for i in 0..n {
            for j in (i + 1)..n {
                if used.contains(&(i, j)) { continue; }
                let dx = boxes[i].0 - boxes[j].0;
                let dy = boxes[i].1 - boxes[j].1;
                let dz = boxes[i].2 - boxes[j].2;
                let dist = dx * dx + dy * dy + dz * dz;
                if dist < min_dist {
                    min_dist = dist;
                    best = (i, j);
                }
            }
        }

        used.insert(best);
        adj[best.0].push(best.1);
        adj[best.1].push(best.0);
        last_x_product = boxes[best.0].0 * boxes[best.1].0;
    }

    return last_x_product.to_string();
}