use std::fs::File;
use std::io::{BufRead, BufReader};
// use rand::{Rng, seq::SliceRandom};

pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let file = File::open(path).expect("Failed to open file");

    let reader = BufReader::new(file);

    let mut edges: Vec<(usize, usize)> = Vec::new();
    

    for line in reader.lines() {
        
        if let Ok(line) = line {
            
            let s: Vec<&str> = line.split(" ").collect();
            let xi = s[1].parse::<usize>().unwrap();
            let yi = s[2].parse::<usize>().unwrap();
            edges.push((xi, yi));


        }
    }

    edges
}