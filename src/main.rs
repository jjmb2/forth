use std::io::{self, Write};

fn main() {
    let mut stack = Vec::new();
    loop {
        let mut input = String::new();

        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        if input.trim() == "q" {
            break;
        }

        let tokens = input.trim().split(' ');

        for token in tokens {
            match token.parse::<f64>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    match token {
                        "+" => stack.push(a + b),
                        "-" => stack.push(a - b),
                        "*" => stack.push(a * b),
                        "/" => stack.push(a / b),
                        "%" => stack.push(a % b),
                        _ => println!("invalid operator"),
                    }
                }
            }
        }
        println!("Stack: {:?}", stack);
    }
}
