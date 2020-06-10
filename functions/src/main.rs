fn main() {
    println!("Hello, world!");
    another_function(4, 5);
    let x = ret_val();
    println!("Returned Value = {}", x);
    let x = plus_one(x);
    println!("Attempting to increment {}", x);
    println!("Value of x is {}", x);
}

fn ret_val() -> i32 {
    println!("Entered ret_val");
    10
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(a: i32, b: i32) {
    println!("Entered Another Function with arguments {} and {}", a, b);
}
