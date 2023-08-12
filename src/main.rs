use std::io;

const ADD: i32 = 1;
const SUBTRACT: i32 = 2;
const MULTIPLY: i32 = 3;
const DIVIDE: i32 = 4;
const REMAINDER: i32 = 5;
const QUIT: i32 = 6;

fn main() {
    println!("Operating..");

    println!("Please enter the number one at a time");

    loop {
        println!("Choose the number.");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Remainder");
        println!("6. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input..");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if input == QUIT {
            println!("Program Ended.");
            break;
        }

        println!("Please enter two numbers");

        let num1: i32 = get_number("First");
        let num2: i32 = get_number("Second");

        match input {
            ADD => {
                let result = num1 + num2;
                println!("Result: {}", result);
            }
            SUBTRACT => {
                let result = num1 - num2;
                println!("Result: {}", result);
            }
            MULTIPLY => {
                let result = num1 * num2;
                println!("Result: {}", result);
            }
            DIVIDE => {
                if num2 == 0 {
                    println!("Cannot be divided by zero.");
                } else {
                    let result = num1 / num2;
                    println!("Result: {}", result);
                }
            }
            REMAINDER => {
                if num2 == 0 {
                    println!("Cannot be divided by zero.");
                } else {
                    let result = num1 % num2;
                    println!("Result: {}", result);
                }
            }
            _ => {
                println!("Invalid choice. Please choose again.")
            }
        }
    }
}

fn get_number(prompt: &str) -> i32 {
    loop {
        let mut num_input = String::new();
        println!("{} Number: ", prompt);
        io::stdin().read_line(&mut num_input).expect("Failed to read input..");

        match num_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}
