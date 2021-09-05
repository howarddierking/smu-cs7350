use std::time::{Duration, Instant};
// NOTE: Rust doesn't have a built-in random number generator so I did need to use a package for that
use rand::distributions::{Distribution, Uniform};

fn main() {
    let min = 100_000;
    let max = 1_000_000;
    let step = 100_000;

    let mut durations = Vec::new();
    
    for n in (min..=max).step_by(step){
        durations.push(build_and_count_randos(n));
    }

    println!("\nRunning times for build_and_count_randos");
    for d in durations{
        println!("n={}, time (ns)={:?}", d.0, d.1.as_nanos());
    }
}

fn build_and_count_randos(n: i32) -> (i32, Duration) {
    let start = Instant::now();

    let mut rng = rand::thread_rng();
    // Assumption: range for uniform distribution intended to be inclusive
    let dist = Uniform::from(1..=3);

    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;

    let mut randos = vec![0; n as usize];
    for i in 0..n {
        let number = dist.sample(&mut rng);
        match number {
            1 => ones = ones + 1,
            2 => twos = twos + 1,
            3 => threes = threes + 1,
            _ => panic!("this isn't the value I'm looking for")

        }
        randos[i as usize] = number;
    }

    println!("Ones={}, Twos={}, Threes={}", ones, twos, threes);

    return (n, start.elapsed());
}