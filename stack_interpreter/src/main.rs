

use std::io;
use std::collections::LinkedList;
use std::str::FromStr;

fn main() {
    println!("Running stack interpreter.");
    run_interpreter(true);
}


fn run_interpreter(cmd_input:bool) {

    let mut stack: Vec<String> = Vec::new();

    if cmd_input {

        let mut input = String::new();

        while input.trim() != "end" {
            if stack.is_empty() {
                println!("Nothing on the stack");
            } else {
                let default = String::from("");
                let val = stack.last().unwrap_or(&default);
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

                    if (val1.trim().contains(".") | val2.trim().contains(".")) & (parsable::<f64>(val1.trim()) & parsable::<f64>(val2.trim())) {
                        // one or more is float --> result should be float
                        let val1 = val1.trim().parse::<f64>().unwrap();
                        let val2 = val2.trim().parse::<f64>().unwrap();
                        stack.push((val1 + val2).to_string());

                    } else if (parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim())) {
                        // both are integers
                        let val1 = val1.trim().parse::<i64>().unwrap();
                        let val2 = val2.trim().parse::<i64>().unwrap();
                        stack.push((val1 + val2).to_string());

                    } else if ((val1.trim() == "true") | (val1.trim() == "false")) & ((val2.trim() == "true") | (val2.trim() == "false")) {
                        // both are boolean
                        panic!(" \"+\"-operation not allowed for boolean types");

                    } else {
                        // treat both as string
                        println!("interpreted as concatenation of strings");
                        stack.push(val2 + val1.as_str());
                    }
                },

                "-" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    if (val1.trim().contains(".") | val2.trim().contains(".")) & (parsable::<f64>(val1.trim()) & parsable::<f64>(val2.trim())) {
                        // one or more is float --> result should be float
                        let val1 = val1.trim().parse::<f64>().unwrap();
                        let val2 = val2.trim().parse::<f64>().unwrap();
                        stack.push((val2 - val1).to_string());

                    } else if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
                        // both are integers
                        let val1 = val1.trim().parse::<i64>().unwrap();
                        let val2 = val2.trim().parse::<i64>().unwrap();
                        stack.push((val2 - val1).to_string());

                    } else if ((val1.trim() == "true") | (val1.trim() == "false")) & ((val2.trim() == "true") | (val2.trim() == "false")) {
                        // both are boolean
                        panic!(" \"-\"-operation not allowed for boolean types");

                    } else {
                        // treat both as string
                        panic!(" \"-\"-operation not allowed for String types");
                    }
                },

                "*" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    if (val1.trim().contains(".") | val2.trim().contains(".")) & (parsable::<f64>(val1.trim()) & parsable::<f64>(val2.trim())) {
                        // one or more is float --> result should be float
                        let val1 = val1.trim().parse::<f64>().unwrap();
                        let val2 = val2.trim().parse::<f64>().unwrap();
                        stack.push((val2 * val1).to_string());

                    } else if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
                        // both are integers
                        let val1 = val1.trim().parse::<i64>().unwrap();
                        let val2 = val2.trim().parse::<i64>().unwrap();
                        stack.push((val2 * val1).to_string());

                    } else if ((val1.trim() == "true") | (val1.trim() == "false")) & ((val2.trim() == "true") | (val2.trim() == "false")) {
                        // both are boolean
                        panic!(" \"*\"-operation not allowed for boolean types");

                    } else {
                        // treat both as string
                        panic!(" \"*\"-operation not allowed for String types");
                    }
                },

                "/" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    if (val1.trim().contains(".") | val2.trim().contains(".")) & (parsable::<f64>(val1.trim()) & parsable::<f64>(val2.trim())) {
                        // one or more is float --> result should be float
                        let val1 = val1.trim().parse::<f64>().unwrap();
                        let val2 = val2.trim().parse::<f64>().unwrap();
                        stack.push((val2 - val1).to_string());

                    } else if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
                        // both are integers
                        let val1 = val1.trim().parse::<i64>().unwrap();
                        let val2 = val2.trim().parse::<i64>().unwrap();
                        stack.push((val2 - val1).to_string());

                    } else if ((val1.trim() == "true") | (val1.trim() == "false")) & ((val2.trim() == "true") | (val2.trim() == "false")) {
                        // both are boolean
                        panic!(" \"/\"-operation not allowed for boolean types");

                    } else {
                        // treat both as string
                        panic!(" \"/\"-operation not allowed for String types");
                    }
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

<<<<<<< Updated upstream
                "pop" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };
                },

                "swp" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {panic!("{}", e);},
                    };
                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {panic!("{}", e);},
                    };

                    stack.push(val1);
                    stack.push(val2);
                },

                "nip" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {panic!("{}", e);},
                    };
                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {panic!("{}", e);},
                    };

                    stack.push(val1);
                },
                
                "ovr" => {
                    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };
                    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
                        Ok(v) => v,
                        Err(e) => {println!("{}", e); continue},
                    };

                    stack.push(val2.clone());
                    stack.push(val1.clone());
                    stack.push(val2.clone());
                },
                
                _ => {
                    stack.push(String::from(input.clone()));
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
