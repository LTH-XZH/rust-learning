use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 2..n+1 {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

fn main() {
    println!("Enter a number to calculate its Fibonacci:");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
