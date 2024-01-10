use std::{io::stdin, str::FromStr};

pub fn text_to_number<T: FromStr>(text: String) -> Option<T> {
  return match text.trim().parse::<T>() {
    Ok(number) => Some(number),
    Err(_) => {
      println!("[ERROR] could not parse to a number (NaN)");
      return None;
    },
  };
}

pub fn read_line() -> Option<String> {
  let mut buffer = String::new();
  return match stdin().read_line(&mut buffer) {
    Ok(_) => Some(buffer),
    Err(error) => {
      println!("[ERROR] {}", error.to_string());
      return None;
    },
  };
}
