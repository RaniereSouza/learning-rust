use std::process::ExitCode;

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn main() -> ExitCode {
  println!("insert an integer n to get the nth Fibonacci number: ");
  return match read_line() {
    Some(input) => {
      return match text_to_number::<i32>(input) {
        Some(number) => {
          let result = fibonacci_rec(number.clone());
          println!("the #{} Fibonacci number is {}", number, result);
          return ExitCode::SUCCESS;
        },
        None => ExitCode::FAILURE,
      };
    },
    None => ExitCode::FAILURE,
  };
}
