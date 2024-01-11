use std::{ops::Add, str::FromStr, any::type_name, io::stdin};

/// Custom trait to basically represent *any numeric type*:
/// * Can be passed onto the add operation (`+`)
/// * Can be parsed from a string (needed for the `core::str::parse` function)
pub trait Numeric: Add + FromStr {}
impl<T: Add + FromStr> Numeric for T {}

pub fn text_to_number<T: Numeric>(text: String) -> Option<T> {
  let clean_text = text.trim();
  return match clean_text.parse::<T>() {
    Ok(number) => Some(number),
    Err(_) => {
      println!("[ERROR] could not parse value \"{clean_text}\" into a {} (NaN)", type_name::<T>());
      return None;
    },
  };
}

pub fn read_line() -> Option<String> {
  let mut buffer = String::new();
  return match stdin().read_line(&mut buffer) {
    Ok(_) => Some(buffer),
    Err(error) => {
      println!("[ERROR] {error}");
      return None;
    },
  };
}
