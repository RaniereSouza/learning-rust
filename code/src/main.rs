use std::{io::stdin, str::FromStr, process::exit};

mod fibonacci; use fibonacci::fibonacci_rec;

fn text_to_number<T: FromStr>(text: String) -> Option<T> {
  return match text.trim().parse::<T>() {
    Ok(number) => Some(number),
    Err(_) => {
      println!("[ERROR] could not parse to a number (NaN)");
      return None;
    },
  };
}

fn read_line() -> Option<String> {
  let mut buffer = String::new();
  return match stdin().read_line(&mut buffer) {
    Ok(_) => Some(buffer),
    Err(error) => {
      println!("[ERROR] {}", error.to_string());
      return None;
    },
  };
}

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
