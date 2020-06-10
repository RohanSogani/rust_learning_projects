use std::io;
// Learning about constants
const MILLION: u32 = 1000000;
fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x/M is {}", x/MILLION);

    // Shadowing, 1st variable is shadowed by second variable
    let x = 5;
    println!("The value of x before is {}", x);
    // x = x * 25; // gives immutable variable error, use shadowing
    let x = x + 1;
    println!("The value of x after 1st op is {}", x);
    let x = x * 3;
    println!("The value of x after 2nd op is {}", x);

    let mut spaces = String::new();
    println!("How many spaces do you want?");

    io::stdin()
        .read_line(&mut spaces)
        .expect("Failed to read line");

    let spaces = spaces.len();
    println!("Spaces set to {}", spaces - 1);

    let tup: (i32, f64, u32) = (-777, 77.77, 77);

    let (x, _, _) = tup;

    println!("The value of first place in tuple is {}", x);
    println!("Another way to access our tuple");
    println!("1st value {}", tup.0);
    println!("2nd value {}", tup.1);
    println!("3rd value {}", tup.2);

    let a = [1, 2, 4, 8, 16];
    let b: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let init_a = [2; 5];
}
