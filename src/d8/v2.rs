use std::collections::HashSet;

pub fn partie1(input: &str) -> String {
    // Parse 3D coordinates
    let mut boxes: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() { continue; }
        let parts: Vec<i64> = line.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        boxes.push((parts[0], parts[1], parts[2]));
    }

    let n = boxes.len();

    // Find 1000 closest pairs, one by one with a simple loop
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

    // Build adjacency list from all pairs
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b) in &pairs {
        adj[*a].push(*b);
        adj[*b].push(*a);
    }

    // Find circuits: take a box, visit all connected boxes, remove them from the list
    let mut remaining: Vec<usize> = (0..n).collect();
    let mut circuit_sizes: Vec<usize> = Vec::new();

    while !remaining.is_empty() {
        let start = remaining.remove(0);
        let mut circuit_size = 1;

        // Queue of boxes to explore in this circuit
        let mut to_visit: Vec<usize> = adj[start]
            .iter()
            .filter(|&&x| remaining.contains(&x))
            .cloned()
            .collect();

        while !to_visit.is_empty() {
            let current = to_visit.remove(0);
            if let Some(pos) = remaining.iter().position(|&x| x == current) {
                remaining.remove(pos);
                circuit_size += 1;
                for &neighbor in &adj[current] {
                    if remaining.contains(&neighbor) && !to_visit.contains(&neighbor) {
                        to_visit.push(neighbor);
                    }
                }
            }
        }
        circuit_sizes.push(circuit_size);
    }

    // Multiply the 3 largest circuits together
    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
    let result = circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
    return result.to_string();
}

pub fn partie2(input: &str) -> String {
    // Parse 3D coordinates
    let mut boxes: Vec<(i64, i64, i64)> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() { continue; }
        let parts: Vec<i64> = line.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect();
        boxes.push((parts[0], parts[1], parts[2]));
    }

    let n = boxes.len();

    // Build adjacency list progressively, same pair-finding loop as part 1
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut used: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();
    let mut last_x_product: i64 = 0;

    loop {
        // Check if all boxes are already in one circuit
        let mut remaining: Vec<usize> = (0..n).collect();
        let start = remaining.remove(0);
        let mut to_visit: Vec<usize> = adj[start]
            .iter()
            .filter(|&&x| remaining.contains(&x))
            .cloned()
            .collect();
        while !to_visit.is_empty() {
            let current = to_visit.remove(0);
            if let Some(pos) = remaining.iter().position(|&x| x == current) {
                remaining.remove(pos);
                for &neighbor in &adj[current] {
                    if remaining.contains(&neighbor) && !to_visit.contains(&neighbor) {
                        to_visit.push(neighbor);
                    }
                }
            }
        }
        if remaining.is_empty() { break; }

        // Find the next closest unused pair
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

        // Connect the pair and save the X product in case this is the last one
        used.insert(best);
        adj[best.0].push(best.1);
        adj[best.1].push(best.0);
        last_x_product = boxes[best.0].0 * boxes[best.1].0;
    }

    return last_x_product.to_string();
}