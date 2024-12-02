mod day01;
mod day02;
mod day03;

fn main() {
    println!("Day 01\n======");
    let now = std::time::Instant::now();
    day01::run();
    println!("{}µs\n", now.elapsed().as_micros());

    println!("Day 02\n======");
    let now = std::time::Instant::now();
    day02::run();
    println!("{}µs\n", now.elapsed().as_micros());

    println!("Day 03\n======");
    let now = std::time::Instant::now();
    day03::run();
    println!("{}µs\n", now.elapsed().as_micros());
}
