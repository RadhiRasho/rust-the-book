/* Vairables and Mutability
 fn main() {
     let mut x = 5;
     println!("The value of x is: {x}");
     x = 6;
     println!("The value of x is: {x}");
    }
*/

/* Different Scopings and Shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
*/


/* This code errors out because we are trying to change the type of a variable while using Shadowing
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
}
*/

/* Data Types
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
}
*/

/* Scalar Types
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}
*/

/* Numeric Operations
fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
}
*/

/* Boolean Types
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
*/

/* Character Types
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
*/

/* Compound Types
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
*/

/* Destructuring Tuples
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
}
*/

/* Accessing Tuples
fn main() {
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
}
*/

/* Arrays
fn main() {
    let a = [1, 2, 3, 4, 5];
}
*/

/* Accessing Arrays
fn main() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
*/

/* Accessing Arrays with Index Out of Bounds
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
}
*/

/* Functions
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}
*/

/* Statements and Expressions
fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1 // no semicolon here
    };
    println!("The value of y is: {y}");
}
*/

/* Functions with Return Values Part: 1
fn main() {
    let x = five();
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // no semicolon here
}

fn five() -> i32 {
    return 5; // semicolon here
}
*/

/* Functions with Return Values Part: 2
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
*/

/* Comments
// hello, world
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
fn main() {
    let lucky_number = 7; // I'm feeling lucky today
}

fn main() {
    // I'm feeling lucky today
    let lucky_number = 7;
}
*/

/* Control Flow with if Expressions
fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main() {
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }
}
*/

/* handling Multiple Conditions with else if
fn main() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/

/* Using if in a let Statement
fn main() {
    let condition = true;
    let number = if condition { 5 } else {
        6
        // "six" // this will error out because the types are different
    };
    println!("The value of number is: {number}");
}
*/

/* Repetition with Loops
fn main() {
    loop {
        println!("again!");
    }
}
*/

/* Returning Values from Loops
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // this will return the value of counter * 2
        }
    };
    println!("The result is: {result}");
}
*/

/* Loops Labels to Disambiguate between multiple loops
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/

/* Conditional Loops with while
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
*/

/* Looping Through a Collection with for
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 { //? Can use this to loop through an array
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() { // * but it's better to use this
        println!("{element}");
    }
}

fn main() {
    for number in (1..4).rev() { //? Can use this to loop through a range
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
*/


fn main() {}