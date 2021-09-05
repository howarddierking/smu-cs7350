// use std::env;
use std::time::{Duration, Instant};
// NOTE: Rust doesn't have a built-in random number generator so I did need to use a package for that
use rand::distributions::{Distribution, Uniform};

fn main() {
    let min = 100;
    let max = 1_000;
    let step = 100;

    let mut durations = Vec::new();
    
    for n in (min..=max).step_by(step){
        durations.push(build_ordered_list(n));
    }

    println!("\nRunning times for build_ordered_list");
    for d in durations{
        println!("n={}, time (ns)={:?}", d.0, d.1.as_nanos());
    }
}

fn build_ordered_list(n: i32) -> (i32, Duration) {
    // let n = 5;
    let start = Instant::now();

    // create array of size n
    let mut numbers = vec![0; n as usize];

    // generate random number
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(1..n);

      for i in 0..n{        
        let number = dist.sample(&mut rng);
        
        println!("before: i={}, array={:?}, number={}", i, numbers, number);

        // find the insertion point idx_i
        let mut idx_insertion = 0;
        while number >= numbers[idx_insertion] && numbers[idx_insertion] != 0 {
            idx_insertion = idx_insertion + 1;
        }

        // if numbers[idx_insertion] == 0, insert it; else, shift everything from idx_insertion right and insert
        if numbers[idx_insertion] == 0 {
            numbers[idx_insertion] = number;
        } else {
            let moves_right = (i as usize) - idx_insertion;
            println!("idx_insertion={}, will need to do {} move(s) right", idx_insertion, moves_right);
            let start = idx_insertion;
            let end = idx_insertion+(moves_right);
            println!("start={}, end={}", start, end);
            for j in (start..end).rev(){
                println!("j={}", j);
                numbers[j+1] = numbers[j];
            }
            numbers[idx_insertion] = number;
        }
        
        println!("after: i={}, array={:?}\n", i, numbers);
    }

    println!("array {:?}", numbers);
    
    return (n, start.elapsed());
}