use count_down::count_down;

fn main() {
    // should be run with n1 n2 n3 n4 n5 n6 n7 where
    // n1 to n4 are one digit numbers
    // n5 is 10, 15, or 20
    // n6 is 25, 50, 75 or 100
    // n7 is the number to arrive at
    // example: using 3 4 7 2 15 25 303 should yield
    // 3 * (7 * 15 - 4) as the best result
    let mut numbers: Vec<count_down::Int> = std::env::args().skip(1)
        .map(|arg| arg.parse::<count_down::Int>().unwrap()).collect();
    let goal = numbers.pop().unwrap();
    let results = count_down::solutions(numbers, goal);
    let top = std::cmp::min(results.len(), 5);
    for result in &results[..top] {
        println!("{:?}", result);
    }
}
