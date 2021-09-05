use std::time::{Duration, Instant};
// NOTE: Rust doesn't have a built-in random number generator so I did need to use a package for that
use rand::distributions::{Distribution, Uniform};

fn main() {
    let min = 1_000_000;
    let max = 10_000_000;
    let step = 1_000_000;

    let mut durations = Vec::new();

    for n in (min..=max).step_by(step) {
        durations.push(build_and_sort_randos(n));
    }

    println!("\nRunning times for build_and_count_randos");
    for d in durations {
        println!("n={}, time (ns)={:?}", d.0, d.1.as_nanos());
    }
}
// TODO: get a better understanding of Rust's owndership feature (references and borrowing)
// and then rewrite this function to accept the array as a parameter
fn build_and_sort_randos(n: i32) -> (i32, Duration) {
    // build
    let mut rng = rand::thread_rng();
    // Assumption: range for uniform distribution intended to be inclusive
    let dist = Uniform::from(1..=3);

    let mut randos = vec![0; n as usize];
    for i in 0..n {
        let number = dist.sample(&mut rng);
        randos[i as usize] = number;
    }

    // sort
    let start = Instant::now();

    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;

    for i in randos.iter() {
        match i {
            1 => ones = ones + 1,
            2 => twos = twos + 1,
            3 => threes = threes + 1,
            _ => panic!("this isn't the value I'm looking for"),
        }
    }

    // write `ones` 1s
    for i in 0..ones {
        randos[i] = 1;
    }

    // write `twos` 2s
    for j in ones..(ones + twos) {
        randos[j] = 2;
    }

    // write `threes` 3s
    for k in (ones + twos)..(ones + twos + threes) {
        randos[k] = 3;
    }

    println!("{:?}", randos);

    return (n, start.elapsed());
}
