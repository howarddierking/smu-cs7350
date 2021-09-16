use itertools::Itertools;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::distributions::{Distribution, Uniform};

fn main() {
    let min_verticies = 100;
    let max_verticies = 1000;
    let step_verticies = 100;

    let min_edges = 100;
    let max_edges = 1000;
    let step_edges = 100;

    let mut durations = Vec::new();

    for v in (min_verticies..=max_verticies).step_by(step_verticies) {
        for e in (min_edges..=max_edges).step_by(step_edges){
            let r = create_random_graph(v, e);
            // println!("{:?}", r.0);
            report_adjacency_list(r.0);
            durations.push((r.1, r.2, r.3));
        }
    }

    for d in durations {
        println!("vertices={}, edges={}, time (ns)={:?}", d.0, d.1, d.2.as_nanos());
    }
}

fn create_random_graph(verticies: u32, edges: u32) -> (HashMap<u32, Vec<u32>>, u32, u32, Duration) {
    let start = Instant::now();

    // random number generator
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(1..=verticies);

    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();

    // add all vertices to the adjacency list along with an empty vector (space can be optimized later)
    for v in 1..=verticies {
        adj_list.insert(v, Vec::new());
    }

    // create edges between random verticies
    for _ in 0..edges {
        let left:u32 = dist.sample(&mut rng);
        let right:u32 = dist.sample(&mut rng);

        // the problem doesn't call for it, but if need to ensure E_i must be unique, we would need
        // to add a search to the vector at this point in the adjacency list. In a naive approach, 
        // this would add a multiplication step, though nothing approaching either V or E
        let neighbors = adj_list.get_mut(&left).unwrap();
        neighbors.push(right);
    }

    (adj_list, verticies, edges, start.elapsed())
}

fn report_adjacency_list(g: HashMap<u32, Vec<u32>>) {
    // need to sort
    let sorted_keys = g.keys().sorted();

    println!("{}\t# number of vertices", g.len());

    let mut last_line = 1 + sorted_keys.len();

    for (i, k) in sorted_keys.enumerate() {
        let val = g.get(k).unwrap();

        println!("{}\t# starting value for vertex {}", last_line, i + 1);
        last_line = last_line + val.len();
    }

    // Good lord this is ugly. TODO: figure out how I can get sorted keys without
    // using an iterator that consumes the collection, thus moving the pointer
    let sorted_keys_2 = g.keys().sorted();

    for k in sorted_keys_2 {
        let val = g.get(k).unwrap();
        for z in val {
            println!("{0} 1\t# Vertex {1} is adjacent to vertex {0}", z, k);
        }
    }
}
