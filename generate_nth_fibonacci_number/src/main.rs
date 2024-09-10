
use std::io;

fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

fn main() {
    println!("Enter number to generate nth Fibonacci number: ");
    let mut nth_number = String::new();
    io::stdin().read_line(&mut nth_number).expect("Failed to read input.");
    let nth_number: u32 = nth_number.trim().parse().expect("Please enter a valid number.");

    println!("The {}th Fibonacci number is {}.", nth_number, fibonacci(nth_number));
}
