use std::fs::{self, File};
use std::io::{self, BufRead, Read};
use std::collections::HashMap;

struct Manual<'a> {
    rules: Vec<(&'a str, &'a str)>,
    pages: Vec<Vec<&'a str>>
}

impl<'a> Manual<'a> {
    pub fn new(input: &'a str) -> Self {
        let (a,b) = input.split_once("\r\n\r\n").expect("DNE blank line");
        let rules: Vec<(&'a str, &'a str)>= a.lines().map(|l| l.split_once('|').expect("DNE |")).collect();
        let pages: Vec<Vec<&'a str>> = b.lines().map(|r| r.split(',').collect()).collect();
        Self { rules, pages }
    }

    pub fn get_failed(&self) -> Vec<bool> {
        let n = self.pages.len();
        let mut failed = vec![false; n];

        for (f,s) in self.rules.iter() {
            for p in 0..n {
                if failed[p] {
                    continue;
                }

                if let Some(si) = self.pages[p].iter().position(|x| x == s) {
                    if self.pages[p][si..].contains(f) {
                        failed[p] = true;
                    }
                }
            }
        }
        failed
    }
}

fn partone(m: &Manual) -> usize {
    let f = m.get_failed();

    m.pages.iter().enumerate().filter_map(|pt| {
        if f[pt.0] {
            return None;
        }
        let middle = pt.1[pt.1.len() / 2];
        Some(middle.parse::<usize>().expect("NaN"))
    }).sum()
}

fn parttwo(m: &Manual) -> usize {
    let f = m.get_failed();

    let mut failed_pages: Vec<_> = m.pages.iter().enumerate().filter_map(|p| {
        if f[p.0] {
            return Some(p.1.clone());
        }
        None
    }).collect();

    let mut sum = 0;

    for mut page in failed_pages {
        loop {
            let mut swapped = false;

            'outer: for &(first,second) in m.rules.iter() {
                for i in 0..page.len() {
                    if page[i] == first {
                        continue 'outer;
                    }
                    if page[i] == second {
                        for j in i..page.len() {
                            if page[j] == first {
                                page.swap(i, j);
                                swapped = true;
                                continue 'outer;
                            }
                        }
                    }
                }
            }

            if !swapped {
                sum += page[page.len() / 2].parse::<usize>().expect("it to be a number");
                break;
            }
        }
    }
    sum
}

fn main() {
    let file_path = "data/data.txt";
    let s = fs::read_to_string(file_path).unwrap();

    let m = Manual::new(&s);
    let p1 = partone(&m);

    println!("{p1}");

    let p2 = parttwo(&m);

    println!("{p2}");
}