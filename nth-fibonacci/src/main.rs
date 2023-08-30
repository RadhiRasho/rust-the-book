use std::io;

fn fib(nth: i64) -> i64 {
    if nth <= 1 {
        return nth;
    }

    return fib(nth -1) + fib(nth - 2);
}

fn main() {
    let mut nth = String::new();

    println!("Enter the specific fibonacci number placement you want (nth: 10, 40, etc...): ");
    io::stdin()
    .read_line(&mut nth)
    .expect("Must be one of two symbols: F or C");

    let nth: i64 = match nth.trim().parse() {
        Ok(number) => number,
        Err(_) => todo!("To Implement Later On"),
    };

    let fib = fib(nth);
    println!("The {nth}th Fibonacci Number is {fib}!");
}
