/*
external_antinode_pair_locs = (antenna_1 + slope, antenna_2 - slope) where,
                                                                slope = |antenna_1 - antenna_2|
                                                              & antenna_1.y > antenna_2.y
                                                                (i.e. antenna_1 is below antenna_2 in the 2d map)

internal_antinode_pair_locs = if slope % 3 { return (antenna_2 + slope/3, antenna_2 + 2*slope/3) }
*/
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellStatus {
	Antinode,
	Nothing
}

#[derive(Debug)]
struct Map {
	grid: [[CellStatus; 50]; 50],
	antenna_clusters: HashMap<char, Vec<(isize, isize)>>
}

impl Map {
	pub fn new(input: String) -> Self {
		let grid = [[CellStatus::Nothing; 50]; 50];
		let mut antenna_clusters: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
		let mut y = 0;
		for l in input.as_str().lines() {
			let mut x = 0;
			for c in l.chars() {
				match c {
					'.' => {},
					ant_ty => antenna_clusters.entry(ant_ty).or_insert(Vec::new()).push((y,x)),
				}
				x+=1;
			}
			y+=1;
		}
		Self { grid, antenna_clusters }
	}

	pub fn partone(&mut self) -> usize {
		for (_, loc_t) in &self.antenna_clusters {
			for i in 0..loc_t.len() {
				for j in 0..loc_t.len() {
					if i==j { continue; }
					let my = loc_t[i].0 - loc_t[j].0;
					let mx = loc_t[i].1 - loc_t[j].1;
					let antinode1y = loc_t[i].0 + my;
					let antinode1x = loc_t[i].1 + mx;
					if (0..50).contains(&antinode1x) && (0..50).contains(&antinode1y) {
						self.grid[antinode1y as usize][antinode1x as usize] = CellStatus::Antinode;
					}
					let antinode2y = loc_t[j].0 - my;
					let antinode2x = loc_t[j].1 - mx;
					if (0..50).contains(&antinode2x) && (0..50).contains(&antinode2y) {
						self.grid[antinode2y as usize][antinode2x as usize] = CellStatus::Antinode;
					}
				}
			}
		}
		let mut antinodes = 0;
		for row in self.grid {
			for col in row {
				if CellStatus::Antinode == col { print!("#"); antinodes+=1; }
				else { print!("."); }
			}
			println!();
		}
		antinodes
	}
}

fn main() {
	let file_path = "data/data.txt";
	let s = std::fs::read_to_string(file_path).unwrap();

	let mut map = Map::new(s);
	let x = map.partone();
	println!("{x}");
}