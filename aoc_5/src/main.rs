use std::fs;

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();

    let first = s.lines();

    let mut unique_values = Vec::new();

    for line in first {
        for value in line.split('|') {
            let value = value.trim();
            if !unique_values.contains(&value.to_string()) {
                unique_values.push(value.to_string());
            }
        }
    }

    let n = unique_values.len();
    let mut table = vec![vec![0; n]; n];

    for line in s.lines() {
        let values: Vec<&str> = line.split('|').map(|v| v.trim()).collect();
        let i = unique_values.iter().position(|v| v == values[0]).expect("value not found");
        let j = unique_values.iter().position(|v| v == values[1]).expect("value not found");
        table[i][j] = -1;
        table[j][i] = 1;
    }

    let mut row_sums: Vec<(usize, i32)> = table
        .iter()
        .enumerate()
        .map(|(index, row)| (index, row.iter().sum()))
        .collect();

    row_sums.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Unique Values: {:?}", unique_values);
    for row in &table {
        for x in row{
            print!("{:>4}",x);
        }
        println!();
    }
    println!("Row Sums (Index, Sum): {:?}", row_sums);
    let mut sorted = Vec::new();
    for (i,_) in row_sums {
        sorted.push(&unique_values[i]);
    }
    println!("{:?}", sorted);
}
