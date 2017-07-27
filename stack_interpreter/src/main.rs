

use std::io;
use std::collections::LinkedList;
use std::str::FromStr;

fn main() {
    println!("Running 32-bit integer stack interpreter.");
    run_interpreter(true);
}


fn run_interpreter(cmdInput:bool) {

    let mut stack: Vec<String> = Vec::new();

    if cmdInput {

        let mut input = String::new();

        while input.trim() != "end" {
            if stack.is_empty() {
                println!("Nothing on the stack");
            } else {
                let default: i32 = 0;
                let val = stack.front().unwrap_or(&default);
                println!("Top element of stack: {}", val);
            }

            println!("Input value or command:");
            input.clear();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line!");

            match input.trim() {
                "+" => {

                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };
                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    if (val1.trim().find(".") || val2.trim().find(".")) && (parsable::<f64>(val1.trim()) && parsable::<f64>(val2.trim())) {
                        // one or more is float --> result should be float
                    } else if parsable::<i64>(val1.trim()) && parsable::<i64>(val2.trim()) {
                        // both are integers
                    } else if (val1 == "true" || val1 == "false") && (val2 == "true" || val2 == "false") {
                        // both are boolean
                    } else {
                        // treat both as string
                    }

                    stack.push((val1 + val2).to_string());
                },

                "-" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val1 = if parsable::<f64>(val1.trim()) { val1.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val1 = if parsable::<i64>(val1.trim()) { val1.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = if parsable::<f64>(val2.trim()) { val2.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val2 = if parsable::<i64>(val2.trim()) { val2.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    stack.push(val2 - val1);
                },

                "*" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val1 = if parsable::<f64>(val1.trim()) { val1.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val1 = if parsable::<i64>(val1.trim()) { val1.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = if parsable::<f64>(val2.trim()) { val2.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val2 = if parsable::<i64>(val2.trim()) { val2.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    stack.push(val1 * val2);
                },

                "/" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val1 = if parsable::<f64>(val1.trim()) { val1.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val1 = if parsable::<i64>(val1.trim()) { val1.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = if parsable::<f64>(val2.trim()) { val2.trim().parse::<f64>().ok().unwrap() } else { 0 as f64 };
                    let val2 = if parsable::<i64>(val2.trim()) { val2.trim().parse::<i64>().ok().unwrap() } else { 0 as i64 };

                    stack.push(val2 / val1);
                },

                "dup" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    stack.push(val1.clone());
                    stack.push(val1.clone());
                },

                "rot" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    stack.insert(0, val1.clone());
                },
                
                _ => {
                    stack.push(input);
                },
            }


        }
    } else {

    }

    println!("Exiting interpreter loop");
}

fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}
