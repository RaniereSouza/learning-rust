use std::process::ExitCode;

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");
  let Some(input) = read_line() else { return ExitCode::FAILURE };
  let Some(number) = text_to_number::<i32>(input) else { return ExitCode::FAILURE };
  let result = fibonacci_rec(number);
  if result.is_err() { println!("{}", result.err().unwrap()); return ExitCode::FAILURE };
  println!("the #{number} Fibonacci number is {}", result.unwrap());
  return ExitCode::SUCCESS;
}
