extern crate image as im;

use std::io;
use std::str::Lines;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod stack_graphics;
mod commands;

use stack_graphics::StackWindow;
use stack_graphics::Point;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 {
    println!("Running stack interpreter prompt.");
    run_interpreter(true, String::new().lines());
  } else if args.len() == 2 {
    let script_file = &args[1];

    let mut f = File::open(script_file).expect("file not found");

    let mut program = String::new();
    f.read_to_string(&mut program)
        .expect("something went wrong reading the file");
    
    let program = program.lines();

    run_interpreter(false, program);
  } else if args.len() == 3 {
    let script_file = &args[1];
    // let options = &args[2];

    let mut f = File::open(script_file).expect("file not found");

    let mut program = String::new();
    f.read_to_string(&mut program)
      .expect("something went wrong reading the file");
    
    let program = program.lines();

    run_interpreter(false, program);
  } else {
    println!("Please use \"stack <file> <options>\" for program execution or \"stack\" for prompt execution.")
  }
}

fn run_interpreter(prompt_input: bool, program: Lines) {

  let mut program_stack = program.enumerate();
  let mut stack: Vec<String> = Vec::new();
  let mut input = String::new();
  let mut previous_line = 0;
  
  let window =
    StackWindow::new(String::from("Graphics"), 200, 200);

  while input.trim() != "end" {

    if prompt_input {

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
          println!("Next line not found after line: {}", previous_line);
          break;
        }
      }
    }
    
    match_input(&input, &mut stack, &window);
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
  stack: &mut Vec<String>,
  window: &StackWindow) {

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

    "ovr" => {
      commands::over(stack)
    }

    "==" => {
      commands::equals(stack)
    }

    "initWindow" => {
      window.init();
    }

    "lineDraw" => {
      use im::Rgba;
      
      window.drawLine(
        Point{ x: 10, y: 10 }, Point{ x: 100, y: 100 },
        Rgba([128,128,128,255]));
    }

    _ => {
      stack.push(String::from(input.clone()));
    }
  }
}
