use std::fs;

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();
    let mut total = 0;
    for l in s.lines() {
        let (target, n) = l.split_once(':').expect("DNEncounter :");
        let nums: Vec<usize> = n.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        // println!("{target}\n{:?}\n\n", nums);
        if calc_calibration(&nums, target.parse::<usize>().unwrap(), 1, nums[0]) {
            print!("+ {target} ");
            total+=target.parse::<usize>().unwrap();
        }
    }
    println!("\n{total}");
}

fn calc_calibration(nums: &[usize], target: usize, i: usize, intermediate: usize) -> bool {
    if i == nums.len() { return intermediate == target; }

    let add_r = calc_calibration(nums, target, i+1, intermediate+nums[i]);
    let mult_r = calc_calibration(nums, target, i+1, intermediate*nums[i]);

    add_r || mult_r
}
