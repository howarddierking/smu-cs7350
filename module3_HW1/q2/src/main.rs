use itertools::Itertools;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn main() {
    let min = 1000;
    let max = 10000;
    let step = 1000;

    let mut durations = Vec::new();

    for n in (min..=max).step_by(step) {
        let r = create_cycle(n);
        report_adjacency_list(r.0);
        durations.push((r.1, r.2));
    }

    for d in durations {
        println!("n={}, time (ns)={:?}", d.0, d.1.as_nanos());
    }
}

fn create_cycle(verticies: u32) -> (HashMap<u32, Vec<u32>>, u32, Duration) {
    let start = Instant::now();

    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    for v in 1..=verticies {
        // create the adjacent verticies
        let left = if v == verticies { 1 } else { v + 1 };
        let right = if v == 1 { verticies } else { v - 1 };

        // create the list item
        adj_list.insert(v, vec![left, right]);
    }

    (adj_list, verticies, start.elapsed())
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
