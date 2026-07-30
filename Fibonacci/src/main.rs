use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
fn main() {
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
