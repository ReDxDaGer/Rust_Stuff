use std::io;

fn main() {
    println!("****************** CALCULATOR ******************");

    println!("Enter the operator (+, -, *, /): ");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read line");
    let op: char = op.trim().chars().next().unwrap();

    println!("Please enter the first number: ");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    println!("Please enter the second number: ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => {
            println!("Please enter the operator (+, -, *, /)");
            return;
        }
    };

    println!("Result: {}", result);

    println!("************************************************");
}
