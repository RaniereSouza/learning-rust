use std::process::ExitCode;

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");
  let Some(input) = read_line() else { return ExitCode::FAILURE };
  let Some(number) = text_to_number::<i32>(input) else { return ExitCode::FAILURE };
  match fibonacci_rec(number) {
    Ok(result) => {
      println!("the #{number} Fibonacci number is {}", result);
      return ExitCode::SUCCESS;
    },
    Err(error_message) => {
      println!("{error_message}");
      return ExitCode::FAILURE;
    },
  };
}
