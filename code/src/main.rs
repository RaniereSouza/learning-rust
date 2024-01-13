use std::{process::ExitCode, fmt::Display};

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn log_err_return_failure<T, E: Display>(res: Result<T, E>) -> ExitCode {
  println!("{}", res.err().unwrap()); ExitCode::FAILURE
}

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");

  let mut input: Option<i32> = None;
  let result = read_line()
    .and_then(text_to_number)
    .and_then(|parsed_number| { input = Some(parsed_number); fibonacci_rec(parsed_number) });
  if result.is_err() { return log_err_return_failure(result) };

  println!("the #{} Fibonacci number is {}", input.unwrap(), result.unwrap());
  return ExitCode::SUCCESS;
}
