use std::process::exit;

mod helpers; use helpers::{read_line, text_to_number};
mod fibonacci; use fibonacci::fibonacci_rec;

fn main() {
  println!("insert an integer n to get the nth Fibonacci number: ");
  match read_line() {
    Some(input) => {
      match text_to_number::<i32>(input) {
        Some(number) => {
          let result = fibonacci_rec(number.clone());
          println!("the #{} Fibonacci number is {}", number, result);
          exit(0);
        },
        None => exit(1),
      };
    },
    None => exit(1),
  };
}
