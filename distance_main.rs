mod helper;
use helper::*;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let data = read_file("cal.cedge");

    let n = 21048;

    let mut neighbor = vec![vec![];n];
    let mut edges = HashMap::new();

    for (u, v, d) in data.iter() {
        neighbor[*u].push(*v);
        neighbor[*v].push(*u);

        edges.insert((*u, *v), *d);
        edges.insert((*v, *u), *d);
    }

    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    let start = 8517;
    let mut end = 0;
    (dist, prev) = dijkstra(start, n, &neighbor, &edges);

    // let mut path = vec![end];
    // while end != start {
    //     end = prev[&end].unwrap();
    //     path.push(end);
    // }
    
    let mut res: HashMap<usize, isize> = HashMap::new();
    for (k, v) in prev.iter() {

        res.insert(*k, *v);


    }
    println!("{:?}", dist);
    println!("{:?}", res);
    
}

fn dijkstra(start: usize, n: usize, neighbor: &Vec<Vec<usize>>, edges: &HashMap<(usize, usize), f64>) -> (HashMap<usize, f64>, HashMap<usize, isize>) {
    let mut dist = HashMap::new();
    let mut prev: HashMap<usize, isize> = HashMap::new();

    let mut queue = VecDeque::new();

    for i in 0..n {
        dist.insert(i, std::f64::INFINITY);
        prev.insert(i, -1);
        queue.push_back(i);
    }
    let entry = dist.entry(start).or_insert(-1.0);
    *entry = 0.0;

    while queue.len() != 0 {
        println!("{:?}", queue.len());
        let mut min_dist = std::f64::INFINITY;
        let mut u: usize = 0;
        for node in queue.iter() {
            if *dist.get(node).unwrap() < min_dist {
                min_dist = *dist.get(node).unwrap();
                u = *node;
            }
        }

        for (i, node) in queue.iter().enumerate() {
            if *node == u {
                queue.remove(i);
                break;
            }
        }

        for v in neighbor[u].iter() {
            if queue.contains(v) {
                let alt = *dist.get(&u).unwrap() + edges.get(&(u, *v)).unwrap();
                if alt < *dist.get(v).unwrap() {
                    let entry = dist.entry(*v).or_insert(0.0);
                    *entry = alt;

                    let entry = prev.entry(*v).or_insert(0);
                    *entry = u as isize;
                }
            }
        }
    }
    (dist, prev)
}
