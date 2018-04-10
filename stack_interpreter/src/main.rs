extern crate image as im;

use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::iter::Enumerate;
use std::vec::IntoIter;

mod stack_graphics;
mod commands;

use stack_graphics::StackWindow;
use stack_graphics::Point;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("Running stack interpreter prompt.");
    run_interpreter(true, Vec::new());
  } else if args.len() == 2 {
    let script_file = &args[1];

    let mut f = File::open(script_file).expect("file not found");

    let mut program = String::new();
    f.read_to_string(&mut program)
        .expect("something went wrong reading the file");
    
    let program = program.split("\n").map(|s|s.to_string()).collect();

    run_interpreter(false, program);
  }/* else if args.len() == 3 {
    let script_file = &args[1];
    // let options = &args[2]; // no options available yet

    let mut f = File::open(script_file).expect("file not found");

    let mut program = String::new();
    f.read_to_string(&mut program)
      .expect("something went wrong reading the file");
    
    let program: Vec<String> = program.split("\n").map(|s|s.to_string()).collect();

    run_interpreter(false, program);
  }*/ else {
    println!("Please use \"stack <file>\" for program execution or \"stack\" for prompt execution.")
  }
}

fn run_interpreter(prompt_input: bool, mut program: Vec<String>) {

  let mut prompt = prompt_input;
  let labels = get_labels(&program); 
  let mut program_stack = program.clone().into_iter().enumerate();
  let mut stack: Vec<String> = Vec::new();
  let mut input = String::new();
  let mut previous_line = 0;

  let window =
    StackWindow::new(String::from("Graphics"), 1024, 768);

  while input.trim() != "end" {

    if prompt {

      if stack.is_empty() {
        println!("Nothing on the stack");
      } else {
        let default = String::from("");
        let val = stack.last().unwrap_or(&default);
        println!("Top element of stack: {}", val);
      }

      println!("Input value or command:");
      input.clear();
      io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    } else {

      input = match program_stack.next() {
        Some(v) => {
          let (line, command) = v;
          previous_line = line;
          command.to_string()
        },
        None    => {
          println!("Next line not found after line: {}", previous_line + 1);
          prompt = true;
          continue;
        }
      }
    }
    
    program_stack = match_input(&input, &mut program, program_stack, &labels, &mut stack, &window);
  }

  if stack.is_empty() {
    println!("Nothing on the stack");
  } else {
    let default = String::from("");
    stack.pop();
    let val = stack.last().unwrap_or(&default);
    println!("Final top element of stack: {}", val);
  }
  if prompt_input {
    println!("Exiting interpreter loop");
  }
}

fn match_input(input: &String,
  program: &mut Vec<String>,
  program_stack: Enumerate<IntoIter<String>>,
  labels: &HashMap<String, usize>,
  stack: &mut Vec<String>,
  window: &StackWindow) -> Enumerate<IntoIter<String>> {

    if input.starts_with("LABEL ") { return program_stack; }

  match input.trim() {
    "+" => {
      commands::addition(stack);
    }

    "-" => {
      commands::subtraction(stack)
    }

    "*" => {
      commands::multiplication(stack)
    }

    "/" => {
      commands::division(stack)
    }

    "&&" => {
      commands::and(stack)
    }

    "||" => {
      commands::or(stack)
    }

    "!" => {
      commands::not(stack)
    }

    "dup" => {
      commands::duplicate(stack)
    }

    "rot" => {
      commands::rotate(stack)
    }

    "pop" => {
      commands::pop(stack)
    }

    "swp" => {
      commands::swap(stack)
    }

    "nip" => {
      commands::nip(stack)
    }

    "tuck" => {
      commands::swap(stack);
      commands::over(stack);
    }

    "ovr" => {
      commands::over(stack)
    }

    "==" => {
      commands::equals(stack)
    }

    "!=" => {
      commands::not_equal(stack)
    }

    ">" => {
      commands::larger_than(stack)
    }

    "<" => {
      commands::smaller_than(stack)
    }

    "jump" => {
      return commands::jump(program, labels, stack);
    }

    "jump_if" => {
      return commands::jump_if(program, program_stack, labels, stack);
    }

    "windowInit" => {
      window.init();
    }

    "drawLine" => {
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
        panic!(" \"/\"-operation not allowed for given parameters");
    }
      
      
    }

    "drawCircle" => {
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
      use commands::parsable;
    if parsable::<u32>(val1.trim()) 
        & parsable::<u32>(val2.trim())
        & parsable::<u32>(val3.trim())
        & parsable::<u8>(val4.trim())
        & parsable::<u8>(val5.trim())
        & parsable::<u8>(val6.trim()) {
        // put values from stack to variables
        let val1 = val1.trim().parse::<u32>().unwrap();
        let val2 = val2.trim().parse::<u32>().unwrap();
        let val3 = val3.trim().parse::<u32>().unwrap();
        let val4 = val4.trim().parse::<u8>().unwrap();
        let val5 = val5.trim().parse::<u8>().unwrap();
        let val6 = val6.trim().parse::<u8>().unwrap();

        window.draw_circle(Point{ x: val1, y: val2 }, val3,
        Rgba([val4,val5,val6,255]));
        
    } else {
        panic!(" \"/\"-operation not allowed for given parameters");
    }
    }

    "drawTriangle" => {
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
        let val8 = match stack.pop().ok_or("Not enough values on stack!") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
        };
        let val9 = match stack.pop().ok_or("Not enough values on stack!") {
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
        & parsable::<u32>(val5.trim())
        & parsable::<u32>(val6.trim())
        & parsable::<u8>(val7.trim())
        & parsable::<u8>(val8.trim())
        & parsable::<u8>(val9.trim()) {
        // put values from stack to variables
        let val1 = val1.trim().parse::<u32>().unwrap();
        let val2 = val2.trim().parse::<u32>().unwrap();
        let val3 = val3.trim().parse::<u32>().unwrap();
        let val4 = val4.trim().parse::<u32>().unwrap();
        let val5 = val5.trim().parse::<u32>().unwrap();
        let val6 = val6.trim().parse::<u32>().unwrap();
        let val7 = val7.trim().parse::<u8>().unwrap();
        let val8 = val8.trim().parse::<u8>().unwrap();
        let val9 = val9.trim().parse::<u8>().unwrap();

      
      window.draw_triangle(
        Point{ x: val1, y: val2 }, 
        Point{ x: val3, y: val4 }, 
        Point{ x: val5, y: val6 },
        Rgba([val7,val8,val9,255]));
        
        }  else {
        panic!(" \"/\"-operation not allowed for given parameters");
    
        }
    }
    
    

    "drawText" => {
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

    "windowClear" => {
      window.clear_canvas();
    }



    _ => {
      stack.push(String::from(input.clone()));
    }
  }

  program_stack
}

fn get_labels(program: & Vec<String>) -> HashMap<String, usize> {
  let mut labels = HashMap::new();
  for (linenumber, line) in program.clone().into_iter().enumerate() {
    if line.starts_with("LABEL ") {
      labels.insert(line, linenumber);
    }

  }
  labels
}
