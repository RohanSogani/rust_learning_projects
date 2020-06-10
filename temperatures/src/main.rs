use std::io;

fn main() {
    loop {
        println!("Enter a temperature in either C or F");
        println!("Example: 107F or 36C");
        println!("\nType quit to exit!\n");
        println!("Enter the temperature");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Some error occured while taking input");

        let trimmed = input.trim();

        if trimmed == "quit" {
            return;
        }

        let (temp, scale) = trimmed.split_at(trimmed.len() - 1);

        let temp: f64 = match temp.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let scale: char = match scale.parse() {
            Ok(num) => num,
            Err(_) =>  continue,
        };

        if scale == 'C' || scale == 'c' {
            println!("{}C = {}F", temp, convert_to_farenheit(temp));
        } else if scale == 'F' || scale == 'f' {
            println!("{}F = {}C", temp, convert_to_celcius(temp));
        }
    }
}

fn convert_to_celcius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0/9.0)
}

fn convert_to_farenheit(temp: f64) -> f64 {
    ((temp * 9.0)/5.0) + 32.0
}
