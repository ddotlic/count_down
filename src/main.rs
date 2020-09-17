use count_down::count_down;
use std::time::Instant;

fn main() {
    // should be run with n1 n2 n3 n4 n5 n6 n7 where
    // n1 to n4 are one digit numbers
    // n5 is 10, 15, or 20
    // n6 is 25, 50, 75 or 100
    // n7 is the number to arrive at
    // example: using 1 1 4 7 15 50 522 should yield
    // 50 + (1 + 7) * (4 * 15 - 1) as the best result
    let mut numbers: Vec<count_down::Int> = std::env::args().skip(1)
        .map(|arg| arg.parse::<count_down::Int>().unwrap()).collect();
    let goal = numbers.pop().unwrap();
    let time = Instant::now();
    let results = count_down::solutions(numbers, goal);
    let elapsed = time.elapsed().as_millis();
    let total_results = results.len();
    let top = std::cmp::min(total_results, 5);
    println!("Found {} results in {} ms.", total_results, elapsed);
    println!("Top 5 results (or less if there aren't as many)");
    for result in &results[..top] {
        println!("{:?}", result.0);
    }
}
