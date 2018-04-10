use std::str::FromStr;
use std::iter::Enumerate;
use std::vec::IntoIter;
use std::collections::HashMap;

pub fn addition(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
        // both are integers
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();
        stack.push((val1 + val2).to_string());
    } else {
        // both are boolean
        panic!(" \"+\"-operation not allowed for given parameters");
    }
}

pub fn subtraction(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
        // both are integers
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();
        stack.push((val2 - val1).to_string());
    } else {
        panic!(" \"-\"-operation not allowed for given parameters");
    }
}

pub fn multiplication(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
        // both are integers
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();
        stack.push((val2 * val1).to_string());
    } else {
        panic!(" \"*\"-operation not allowed for given parameters");
    }
}

pub fn division(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val2.trim()) {
        // both are integers
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();
        stack.push((val2 - val1).to_string());
    } else {
        panic!(" \"/\"-operation not allowed for given parameters");
    }
}

pub fn and(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<bool>(val1.trim()) & parsable::<bool>(val2.trim()) {
        // both are boolean
        let val1 = val1.trim().parse::<bool>().unwrap();
        let val2 = val2.trim().parse::<bool>().unwrap();
        stack.push((val2 & val1).to_string());
    } else {
        panic!(" \"&&\"-operation not allowed for given parameters");
    }
}

pub fn or(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<bool>(val1.trim()) & parsable::<bool>(val2.trim()) {
        // both are boolean
        let val1 = val1.trim().parse::<bool>().unwrap();
        let val2 = val2.trim().parse::<bool>().unwrap();
        stack.push((val2 || val1).to_string());
    } else {
        panic!(" \"||\"-operation not allowed for given parameters");
    }
}

pub fn not(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if parsable::<bool>(val1.trim()) {
        //value is boolean
        let val1 = val1.trim().parse::<bool>().unwrap();
        stack.push((!val1).to_string());
    } else {
        panic!(" \"!\"-operation not allowed for given parameters");
    }
}

pub fn duplicate(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    stack.push(val1.clone());
    stack.push(val1.clone());
}

pub fn rotate(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    stack.insert(0, val1.clone());
}

pub fn pop(stack: &mut Vec<String>) {
    let _val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

pub fn swap(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", e);
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", e);
        }
    };

    stack.push(val1);
    stack.push(val2);
}

pub fn nip(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", e);
        }
    };

    {
        let _val2 = match stack.pop().ok_or("Not enough values on stack!") {
            Ok(v) => v,
            Err(e) => {
                panic!("{}", e);
            }
        };
    }

    stack.push(val1);
}

pub fn over(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    stack.push(val2.clone());
    stack.push(val1.clone());
    stack.push(val2.clone());
}

pub fn equals(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val3;

    if val1 == val2 {
        val3 = true
    } else {
        val3 = false
    }

    stack.push(val3.to_string());
}

pub fn not_equal(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val3;

    if !(val1 == val2) {
        val3 = true
    } else {
        val3 = false
    }

    stack.push(val3.to_string());
}

pub fn larger_than(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val3;

    if val1 > val2 {
        val3 = true
    } else {
        val3 = false
    }

    stack.push(val3.to_string());
}

pub fn smaller_than(stack: &mut Vec<String>) {
    let val1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let val3;

    if val1 < val2 {
        val3 = false
    } else {
        val3 = true
    }

    stack.push(val3.to_string());
}

pub fn jump(
    program: &Vec<String>,
    program_stack: &Enumerate<IntoIter<String>>,
    labels: HashMap<usize, String>,
    stack: &mut Vec<String>,
) {
    let val = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
}

pub fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}
