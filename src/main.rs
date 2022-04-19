/*
Tax Calculator
For a given input it will calculate the amount that will add to 
the target with the tax rate. 
Default tax rate is 7% but can be easily changed in the main function.

Written by Matthew Thornton 
April 19, 2022
*/
#![allow(non_snake_case)]

fn get_input() -> String {
    let mut line = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut line).unwrap();
    println!();
    if let Some('\n')=line.chars().next_back() {line.pop();}
    if let Some('\r')=line.chars().next_back() {line.pop();}
    return line;
}

fn find_tax_target(target: f64, rate: f64) -> f64 {
    let mut input = target;
    while (((input+(input*rate))*100.0).round())/100.0 != target {
        input -= 0.01;
        if input < 0.00 {
            println!("Target could not be calculated");
            std::process::exit(1);
        }
    }
    return input;
}

fn main() {
    // tax rate of 7%
    let rate: f64 = 0.07;
    let input = get_input();
    // proceed only if input is a number
    if input.parse::<f64>().is_ok() { 
        let number = input.parse().unwrap();
        if number > 0.0 {
            println!("${:.2}", find_tax_target(number, rate));
        }
        else {
            println!("Only valid with positive numbers.");
        }
    }
    else {
        println!("Input was not a valid number.")
    }
}
