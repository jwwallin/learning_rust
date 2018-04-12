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

    if input.starts_with("LABEL:") { 
      return program_stack; 
    } else if input.starts_with("jump:") {
      return commands::jump(program, labels, input);
    } else if input.starts_with("jump_if:") {
      let input = input.replace("jump_if:", "jump:");
      return commands::jump_if(program, program_stack, labels, stack, &input);
    }

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

    "windowInit" => {
      window.init();
    }

    "drawLine" => {
      commands::draw_line(window, stack);
    }

    "drawCircle" => {
      commands::draw_circle(window, stack);
    }

    "drawTriangle" => {
      commands::draw_triangle(window, stack);
    }
    
    "drawText" => {
      commands::draw_text(window, stack);
    }

    "windowClear" => {
      window.clear_canvas();
    }

    "consoleIn" => {
      commands::console_in(stack);
    }

    "consoleOut" => {
      commands::console_out(stack);
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
    if line.starts_with("LABEL:") {
      let label: Vec<String> = line.split(":").map(|s|s.to_string()).collect();

      if label.len() > 2 || label[1].trim().len() == 0 {
          panic!("Invalid label on line {}!", linenumber);
      };

      labels.insert(label[1].trim().to_string(), linenumber);
    }

  }
  labels
}
