use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> Vec<(usize, usize, f64)> {
    let file = File::open(path).expect("Failed to open file");

    let reader = BufReader::new(file);

    let mut edges: Vec<(usize, usize, f64)> = Vec::new();
    

    for line in reader.lines() {
        
        if let Ok(line) = line {
            
            let s: Vec<&str> = line.split(" ").collect();
            let xi = s[1].parse::<usize>().unwrap();
            let yi = s[2].parse::<usize>().unwrap();
            let di = s[3].parse::<f64>().unwrap();
            edges.push((xi, yi, di));


        }
    }

    edges
}