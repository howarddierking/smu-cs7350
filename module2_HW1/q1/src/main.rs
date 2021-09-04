// use std::env;
use std::time::{Duration, Instant};
// NOTE: Rust doesn't have a built-in random number generator so I did need to use a package for that
use rand::Rng;
use rand::distributions::{Distribution, Uniform};

fn main() {
    // TODO: replace fixed array of sizes with a vector generated by min, max, and step values per the example
    let sizes = [10, 50, 100, 500, 1000, 5000, 10_000, 50_000, 100_000, 500_000];
    let mut durations = [Duration::default(); 10];

    //
    // question 1
    //
    // for i in 0..sizes.len() {
    //     durations[i] = hello_n(sizes[i]);
    // }

    // println!("Running times for hello_n");
    // for (i, size) in sizes.iter().enumerate() {
    //     println!("duration for size {}: {:?}", size, durations[i]);
    // }

    //
    // question 2
    //

    build_ordered_list(sizes[1]);

}

fn hello_n(n: i32) -> Duration {
    let start = Instant::now();

    for _i in 1..n {
        println!("Hello, world!");    
    }

    return start.elapsed();
}

fn build_ordered_list(n: i32) -> Duration {
    let n = 5;
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
            println!("idx_insertion={}, will need to do {} move(s) right", idx_insertion, i-idx_insertion);
            let start = idx_insertion;
            let end = idx_insertion+(i-idx_insertion);
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
    

    


    return (start.elapsed());
}

// fn how_many_randos(n: i32) -> Duration{
//     println!("n = {}", n);
//     let start = Instant::now();

//     // do the work and write output to stdout

//     return (start.elapsed());
// }

// fn sort_rando_array(n: i32) -> Duration{
//     println!("n = {}", n);
//     let start = Instant::now();

//     // do the work and write output to stdout

//     return (start.elapsed());
// }