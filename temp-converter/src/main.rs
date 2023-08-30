use std::io;

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    let celsius = (fahrenheit - 32.0) * 5.0/9.0;

    return celsius;
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit = (celsius * 9.0/5.0) + 32.0;

    return fahrenheit;
}

fn main() {
    let mut symbol = String::new();

    println!("Enter the Symbol (F or C): ");
    io::stdin()
    .read_line(&mut symbol)
    .expect("Must be one of two symbols: F or C");

    if symbol.trim().to_uppercase() == "F" {
        println!("Enter the Temperature in Fahrenheit to Convert To Celsius: ");
        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Must be a valid whole number");

        let temp = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => todo!("To Do Later On"),
        };

        let celsius = fahrenheit_to_celsius(temp);
        println!("The Temperature is {celsius} in Celsius");
    }
    else {
            println!("Enter the Temperature in Celsius to Convert To Fahrenheit: ");
            let mut temp = String::new();

            io::stdin()
            .read_line(&mut temp)
            .expect("Must be a valid whole number");

            let temp = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => todo!("To Do Later On"),
            };

            let celsius = celsius_to_fahrenheit(temp);
            println!("The Temperature is {celsius} in Fahrenheit");
    }
}
