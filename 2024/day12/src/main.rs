use std::collections::{HashMap, HashSet, BTreeSet};

fn main() {
    let input = std::fs::read_to_string("data/data.txt").expect("Failed to read file");
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid.insert((i as i32, j as i32), c);
        }
    }

    let mut sets: HashMap<(i32, i32), HashSet<(i32, i32)>> = grid
        .keys()
        .map(|&p| (p, HashSet::from([p])))
        .collect();

    for (&p, &c) in &grid {
        let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for &(dx, dy) in &neighbors {
            let neighbor = (p.0 + dx, p.1 + dy);
            if let Some(&neighbor_char) = grid.get(&neighbor) {
                if c == neighbor_char {
                    let union = sets[&p].union(&sets[&neighbor]).cloned().collect::<HashSet<_>>();
                    for &point in &union {
                        sets.insert(point, union.clone());
                    }
                }
            }
        }
    }

    let mut unique_sets: BTreeSet<BTreeSet<(i32, i32)>> = BTreeSet::new();
    for set in sets.values() {
        unique_sets.insert(set.iter().cloned().collect());
    }

    let edge = |ps: &BTreeSet<(i32, i32)>| -> (HashSet<((i32, i32), (i32, i32))>, HashSet<((i32, i32), (i32, i32))>) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut edges = HashSet::new();

        for &p in ps {
            for &(dx, dy) in &directions {
                let neighbor = (p.0 + dx, p.1 + dy);
                if !ps.contains(&neighbor) {
                    edges.insert((p, (dx, dy)));
                }
            }
        }

        let adjusted_edges: HashSet<_> = edges
            .iter()
            .filter(|&&((x, y), (dx, dy))| {
                !edges.contains(&((x + dy, y - dx), (dx, dy)))
            })
            .cloned()
            .collect();

        (edges, adjusted_edges)
    };

    for part in 0..=1 {
        let total: usize = unique_sets
            .iter()
            .map(|s| {
                let component_size = s.len();
                component_size * if part == 0 { edge(s).0.len() } else { edge(s).1.len() }
            })
            .sum();

        println!("{}", total);
    }
}
