use std::collections::{HashSet,VecDeque};
use itertools::Itertools;

fn solve(mut g: Vec<Vec<u8>>, insts: &str) -> usize {
    let (mut r, mut c) = (0..g.len()).cartesian_product(0..g[0].len())
        .find(|&(r, c)| g[r][c] == b'@')
        .unwrap();
    'outer: for i in insts.bytes() {
        let (dr, dc) = match i {
            b'^' => (-1,  0),
            b'>' => ( 0,  1),
            b'v' => ( 1,  0),
            b'<' => ( 0, -1),
            _ => continue,
        };
        let mut q = VecDeque::from([(r, c)]);
        let mut seen = HashSet::new();
        while let Some((rr, cc)) = q.pop_front() {
            if !seen.insert((rr, cc)) {
                continue;
            }
            let (r2, c2) = ((rr as isize + dr) as usize, (cc as isize + dc) as usize);
            match g[r2][c2] {
                b'#' => continue 'outer,
                b'O' => q.push_back((r2, c2)),
                b'[' => q.extend([(r2, c2), (r2, c2 + 1)]),
                b']' => q.extend([(r2, c2), (r2, c2 - 1)]),
                _ => continue,
            }
        }
        let boxes = seen.iter()
            .sorted_by_key(|&&(rr, cc)| (c.abs_diff(cc), r.abs_diff(rr)))
            .rev();
        for &(rr, cc) in boxes {
            let (r2, c2) = ((rr as isize + dr) as usize, (cc as isize + dc) as usize);
            g[r2][c2] = g[rr][cc];
            g[rr][cc] = b'.';
        }
        (r, c) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
    }
    (0..g.len()).cartesian_product(0..g[0].len())
        .filter(|&(r, c)| matches!(g[r][c], b'O' | b'['))
        .map(|(r, c)| r * 100 + c)
        .sum()
}

fn main() {
    let path = "data/data.txt";
    let s = std::fs::read_to_string(path).unwrap();
    let (a, insts) = s.as_str().split_once("\r\n\r\n").unwrap();
    let g1 = a.lines().map(|l| l.as_bytes().to_vec()).collect();
    let g2 = a.lines().map(|l| l.bytes().flat_map(|b| match b {
        b'#' => b"##",
        b'O' => b"[]",
        b'.' => b"..",
        b'@' => b"@.",
        _ => unreachable!(),
    }).copied().collect()).collect();
    let (a,b) = (solve(g1, insts), solve(g2, insts));
    println!("{a}\n{b}");
}