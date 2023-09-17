use std::io;

fn main() {
    println!("Enter first number: ");

    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    println!("Enter an operator (+, -, *, /): ");

    let mut operator = String::new();
    io::stdin().read_line(&mut operator).unwrap();
    let operator = operator.trim();

    println!("Enter second number: ");

    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    match operator {
        "+" => println!("Result: {}", num1 + num2),
        "-" => println!("Result: {}", num1 - num2),
        "*" => println!("Result: {}", num1 * num2),
        "/" => println!("Result: {}", num1 / num2),
        _ => println!("Invalid operator"),
    }
}
