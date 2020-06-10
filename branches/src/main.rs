fn main() {
    let x = 2;
    if x < 5 {
        println!("{} is less than 5", x);
    } else {
        println!("{} is greater than 5", x);
    }
    loops();
    let a: [i32; 5] = [1, 2, 4, 8, 16];
    print_array(a);
    for_loop(a);
    while_loop();
    count_down(5);
}

fn count_down(val: i32) {
    for i in (1..val).rev() {
        println!("{}", i);
    }
}

fn for_loop(a: [i32; 5]) {
    for element in a.iter() {
        println!("Value is {}", element);
    }
}

fn print_array(a: [i32; 5]){
    let mut i = 0;
    while i < a.len() {
        println!("element at {} is {}", i, a[i]);
        i += 1;
    }
}

fn while_loop() {
    let mut i = 0;
    while i != 3 {
        println!("Value of i is {}", i);
        i += 1
    }
}
fn loops() {
    let mut cnt = 0;
    let res = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };

    println!("Loop returned {}", res);
    // Infy Loop
    /* loop {
        println!("Go Crazy");
    } */
}
