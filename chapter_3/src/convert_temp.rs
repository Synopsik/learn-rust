use std::io;


fn main() {

    loop {
        println!("Please enter a temp type to convert Fahrenheit (F) or Celsius (C): ");

        let mut temp_type = String::new();
        let mut temp = String::new();

        io::stdin().read_line(&mut temp_type).expect("Please enter C or F");

        println!("Please enter a temp: ");

        io::stdin().read_line(&mut temp).expect("Please enter a valid temperature");

        let temp_type: char = match temp_type.trim().parse() {
            Ok(temp) => temp,
            Err(_) => continue,
        };

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        if temp_type == 'C' {

            println!("Celsius: {}", temp * 1.8 + 32.0);

        } else if temp_type == 'F' {

            println!("Celsius: {}", (temp - 32.0) * (5.0 / 9.0));

        } else {
            println!("Please enter C or F");
            continue;
        }
    }
}