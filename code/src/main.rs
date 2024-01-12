use std::process::ExitCode;

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn log_err_return_failure<T>(res: Result<T, String>) -> ExitCode {
  println!("{}", res.err().unwrap()); ExitCode::FAILURE
}

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");

  let input = read_line();
  if input.is_err() { return log_err_return_failure(input) };

  let number = text_to_number::<i32>(input.unwrap());
  if number.is_err() { return log_err_return_failure(number) };

  let result = fibonacci_rec(number.clone().unwrap());
  if result.is_err() { return log_err_return_failure(result) };

  println!("the #{} Fibonacci number is {}", number.unwrap(), result.unwrap());
  return ExitCode::SUCCESS;
}
