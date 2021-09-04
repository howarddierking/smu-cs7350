use std::time::{Duration, Instant};

fn main() {
    let min = 100_000;
    let max = 1_000_000;
    let step = 100_000;

    let mut durations = Vec::new();
    
    for n in (min..=max).step_by(step){
        durations.push(hello_n(n));
    }

    println!("\nRunning times for hello_n");
    for d in durations{
        println!("n={}, time (ns)={:?}", d.0, d.1.as_nanos());
    }
}

fn hello_n(n: i32) -> (i32, Duration) {
    let start = Instant::now();

    for _i in 1..n {
        println!("Hello, world!");    
    }

    return (n, start.elapsed());
}