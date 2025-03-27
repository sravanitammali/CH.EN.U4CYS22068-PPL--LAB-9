fn main() {
    let n = 10; // Number of Fibonacci terms
    let mut fib = vec![0, 1];

    for i in 2..n {
        let next = fib[i - 1] + fib[i - 2];
        fib.push(next);
    }

    println!("Fibonacci sequence: {:?}", fib);
}
