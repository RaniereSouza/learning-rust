use std::{process::ExitCode, io::{stdin, BufReader}};

mod helpers; use helpers::{read_line_from, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");

  let stdin_with_readline = BufReader::new(stdin());
  let mut input: Option<i32> = None;
  let result =
    read_line_from(stdin_with_readline)
    .and_then(text_to_number)
    .and_then(|parsed_number| {
      input = Some(parsed_number); fibonacci_rec(parsed_number)
    });

  if result.is_err() {
    println!("{}", result.err().unwrap());
    return ExitCode::FAILURE;
  };

  println!("the #{} Fibonacci number is {}", input.unwrap(), result.unwrap());
  return ExitCode::SUCCESS;
}
