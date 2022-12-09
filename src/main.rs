use std::io::Read;
use std::io::stdin;
#[allow(non_snake_case)]

fn main() {
    let mut stack: Vec<f64> = Vec::new(); // result stack
    let mut expression = Vec::new(); // expression buffer
    stdin().read(&mut expression).unwrap(); // read expression 

    let mut i = 0;
    while i < expression.len() {
        let token = expression[i]; // get token
        if token == b' ' { // token is u8, so we need to compare it to byte literal
            continue;
        } else if isdigit(expression[i]) { // check if token is a number
            let token = {
                let mut num = String::new(); // number stack
                while isdigit(expression[i]) { // check if next token is a number
                    num.push(expression[i] as char); // push number to stack
                    i += 1;
                }
                num.parse::<f64>().unwrap() // parse number to f64
            };
            stack.push(token);
        } else {
            let b = stack.pop().unwrap() as f64; // pop two numbers from stack
            let a = stack.pop().unwrap() as f64;
            let result = match token {
                b'+' => a + b,
                b'-' => a - b,
                b'*' => a * b,
                b'/' => a / b,
                _ => panic!("Invalid operator"),
            };
            stack.push(result); // push result to stack
        }
        i += 1;
    }
    println!("{:?}", stack); // debugging stack if more than 1 term
    println!("{}", stack.pop().unwrap()); // print result
}

fn isdigit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}