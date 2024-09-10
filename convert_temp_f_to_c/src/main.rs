use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {

    println!("Enter the temperature scale to convert from (C/F): ");
    let mut temp_type = String::new();
    io::stdin().read_line(&mut temp_type).expect("Failed to read input");
    let temp_type = temp_type.trim().to_uppercase();

    println!("Enter the temperature value: ");
    let mut temp_value = String::new();
    io::stdin().read_line(&mut temp_value).expect("Failed to read input");
    let temp_value: f64 = temp_value.trim().parse().expect("Please enter a valid number");

    if temp_type == "C" {
        println!("{}°C is equal to {}°F", temp_value, celsius_to_fahrenheit(temp_value));
    } else if temp_type == "F" {
        println!("{}°F is equla to {}°C", temp_value, fahrenheit_to_celsius(temp_value));
    } else {
        println!("Invalid temperature scale. Please enter 'C' for Celsius or 'F' for Fahrenheit.");
    }
}