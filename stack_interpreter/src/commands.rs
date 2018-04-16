use std::io;
use std::str::FromStr;
use std::iter::Enumerate;
use std::vec::IntoIter;
use std::collections::HashMap;

use stack_graphics::{ StackWindow, Point };

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

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val1.trim()) {
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();

        val3 = val2 == val1;
        
    } else {
        panic!(" \"==\"-operation not allowed for given parameters: {} and {}", val1, val2);
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

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val1.trim()) {
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();

        val3 = val2 != val1;
        
    } else {
        panic!(" \"!=\"-operation not allowed for given parameters: {} and {}", val1, val2);
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

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val1.trim()) {
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();

        val3 = val1 > val2;
        
    } else {
        panic!(" \">\"-operation not allowed for given parameters: {} and {}", val1, val2);
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

    if parsable::<i64>(val1.trim()) & parsable::<i64>(val1.trim()) {
        let val1 = val1.trim().parse::<i64>().unwrap();
        let val2 = val2.trim().parse::<i64>().unwrap();

        val3 = val1 < val2;
        
    } else {
        panic!(" \"<\"-operation not allowed for given parameters: {} and {}", val1, val2);
    }

    stack.push(val3.to_string());
}

pub fn jump(
    program: &Vec<String>,
    labels: &HashMap<String, usize>,
    input: &String,
) -> Enumerate<IntoIter<String>> {

    let val: Vec<String> = input.split(":").map(|s|s.to_string()).collect();

    if val.len() > 2 || val[1].trim().len() == 0 {
        panic!("Invalid label on jump!");
    }

    let val = val[1].trim();
    let label_index = labels.get(val).unwrap();

    let new_stack: Vec<String> = program
                        .clone()
                        .into_iter()
                        .skip(*label_index)
                        .collect();
    new_stack.into_iter().enumerate()
}

pub fn jump_if(
    program: &Vec<String>,
    program_stack: Enumerate<IntoIter<String>>,
    labels: &HashMap<String, usize>,
    stack: &mut Vec<String>,
    input: &String
) -> Enumerate<IntoIter<String>> {
    let val = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", e);
        }
    };
    if parsable::<bool>(val.trim()){
        if val.trim().parse::<bool>().unwrap() {
            return jump(program, labels, input);
        }
    }
    program_stack  
}

pub fn draw_line(window: &StackWindow, stack: &mut Vec<String>){
    use im::Rgba;
    
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
    let val3 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val4 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val5 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val6 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let val7 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    use commands::parsable;
    if parsable::<u32>(val1.trim()) 
        & parsable::<u32>(val2.trim())
        & parsable::<u32>(val3.trim())
        & parsable::<u32>(val4.trim())
        & parsable::<u8>(val5.trim())
        & parsable::<u8>(val6.trim())
        & parsable::<u8>(val7.trim()) {
        // put values from stack to variables
        let val1 = val1.trim().parse::<u32>().unwrap();
        let val2 = val2.trim().parse::<u32>().unwrap();
        let val3 = val3.trim().parse::<u32>().unwrap();
        let val4 = val4.trim().parse::<u32>().unwrap();
        let val5 = val5.trim().parse::<u8>().unwrap();
        let val6 = val6.trim().parse::<u8>().unwrap();
        let val7 = val7.trim().parse::<u8>().unwrap();

        window.draw_line(
        Point{ x: val1, y: val2 }, Point{ x: val3, y: val4 },
        Rgba([val5,val6,val7,255]));
        
    } 
    else {
        panic!("Values were not of expected types: from top down expected: uint, uint, uint, uint, ushort, ushort, ushort!");
    }      
}

pub fn draw_circle(window: &StackWindow, stack: &mut Vec<String>) {
    use im::Rgba;
      
    let r = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let x = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let y = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let red = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let green = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let blue = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    use commands::parsable;
    if parsable::<u32>(x.trim()) 
        & parsable::<u32>(y.trim())
        & parsable::<u32>(r.trim())
        & parsable::<u8>(red.trim())
        & parsable::<u8>(green.trim())
        & parsable::<u8>(blue.trim()) {
        // put values from stack to variables
        let x = x.trim().parse::<u32>().unwrap();
        let y = y.trim().parse::<u32>().unwrap();
        let r = r.trim().parse::<u32>().unwrap();
        let red = red.trim().parse::<u8>().unwrap();
        let green = green.trim().parse::<u8>().unwrap();
        let blue = blue.trim().parse::<u8>().unwrap();

        window.draw_circle(Point{ x: x, y: y }, r,
        Rgba([red,green,blue,255]));
        
    } else {
        panic!("Values were not of expected types: from top down expected: uint, uint, uint, ushort, ushort, ushort!");
    }
}

pub fn draw_triangle(window: &StackWindow, stack: &mut Vec<String>) {
    use im::Rgba;

    let x0 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let y0 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let x1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let y1 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let x2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let y2 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let red = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let green = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let blue = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    use commands::parsable;
    if parsable::<u32>(x0.trim()) 
        & parsable::<u32>(y0.trim())
        & parsable::<u32>(x1.trim())
        & parsable::<u32>(y1.trim())
        & parsable::<u32>(x2.trim())
        & parsable::<u32>(y2.trim())
        & parsable::<u8>(red.trim())
        & parsable::<u8>(green.trim())
        & parsable::<u8>(blue.trim()) 
        {
        // put values from stack to variables
        let x0 = x0.trim().parse::<u32>().unwrap();
        let y0 = y0.trim().parse::<u32>().unwrap();
        let x1 = x1.trim().parse::<u32>().unwrap();
        let y1 = y1.trim().parse::<u32>().unwrap();
        let x2 = x2.trim().parse::<u32>().unwrap();
        let y2 = y2.trim().parse::<u32>().unwrap();
        let red = red.trim().parse::<u8>().unwrap();
        let green = green.trim().parse::<u8>().unwrap();
        let blue = blue.trim().parse::<u8>().unwrap();

      
        window.draw_triangle(
            Point{ x: x0, y: y0 }, 
            Point{ x: x1, y: y1 }, 
            Point{ x: x2, y: y2 },
            Rgba([red,green,blue,255]));
        
    }  else {
        panic!("Values were not of expected types: from top down expected: uint, uint, uint, uint, uint, uint, ushort, ushort, ushort!");
    
    }
}
    
    

pub fn draw_text(window: &StackWindow, stack: &mut Vec<String>) {
    use im::Rgba;

    let text = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let x = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let y = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let red = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let green = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    let blue = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };

    use commands::parsable;

    if parsable::<u32>(x.trim()) 
        & parsable::<u32>(y.trim())
        & parsable::<u8>(red.trim())
        & parsable::<u8>(green.trim())
        & parsable::<u8>(blue.trim()) {
        
        let x = x.trim().parse::<u32>().unwrap();
        let y = y.trim().parse::<u32>().unwrap();
        let red = red.trim().parse::<u8>().unwrap();
        let green = green.trim().parse::<u8>().unwrap();
        let blue = blue.trim().parse::<u8>().unwrap();

        window.draw_text(text.trim().to_string(), (x , y), (red, green, blue));
    } else {
        println!("{0}, {1}, {2}, {3}, {4}, {5}", text, x, y, red, green, blue);
        panic!("Values were not of expected types: from top down expected: String, uint, uint, ushort, ushort, ushort!");
    }

}

pub fn console_in(stack: &mut Vec<String>) {

    let mut input = String::new();
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    stack.push(input);
}

pub fn console_out(stack: &mut Vec<String>) {

    let text = match stack.pop().ok_or("No value on the stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        },
    };
    println!("{}", text);
}

fn parsable<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}
