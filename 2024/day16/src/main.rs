use std::collections::{BinaryHeap, HashMap, VecDeque, HashSet};

fn solve(g: Vec<Vec<u8>>) -> (usize, usize) {
    let (mut start, mut end) = ((0, 0), (0, 0));
    for r in 0..g.len() {
        for c in 0..g[0].len() {
            match g[r][c] {
                b'S' => start = (r, c),
                b'E' => end = (r, c),
                _ => {}
            }
        }
    }
    let mut p1 = i64::MAX;
    let mut v = HashMap::new();
    let mut q = BinaryHeap::from([(0, 0, start)]);
    while let Some((score, d, (r, c))) = q.pop() {
        let score = -score;

        if (r, c) == end {
            if score > p1 {
                break;
            }
            p1 = score;
        }

        for d_idx in 0..4 {
            let (dr, dc) = [(1, 0), (0, 1), (-1, 0), (0, -1)][d_idx];
            let (rr, cc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            if g[rr][cc] == b'#' {
                continue;
            }

            let s = score + if d == d_idx {1} else {1001};
            let last_visited = v.get(&(rr, cc, d_idx)).copied().unwrap_or(i64::MAX);
            if s <= last_visited {
                v.insert((rr, cc, d_idx), s);
                q.push((-s, d_idx, (rr, cc)));
            }
        }
    }
    let mut p2 = HashSet::new();
    let mut vv = v.into_iter().collect::<Vec<_>>();
    vv.sort_by_key(|&(_, score)| score);

    let mut current = (end.0, end.1);
    let mut current_score = p1;

    for d in 0..4 {
        if v.get(&(end.0, end.1, d)).copied().unwrap_or(i64::MAX) == p1 {
            q.push_back((end, d, p1));
        }
    }
    while let Some(((r, c), d, s)) = q.pop_front() {
        p2.insert((r, c));
        for d_idx in 0..4 {
            let ss = s - if d == d_idx {1} else {1001};
            let (dr, dc) = [(1, 0), (0, 1), (-1, 0), (0, -1)][d];
            let (rr, cc) = ((r as isize - dr) as usize, (c as isize - dc) as usize);
            if v.get(&(rr, cc, d_idx)).copied().unwrap_or(i64::MAX) == ss {
                q.push_back(((rr, cc), d_idx, ss));
            }
        }
    }

    (p1 as _, v.len() + 1)
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();
    let (p1,p2) = solve(g);
    println!("{p1}\n{p2}");
}
