use std::{ops::Add, str::FromStr, any::type_name, io::stdin};

/// Custom trait to basically represent *any numeric type*:
/// * Can be passed onto the add operation (`+`)
/// * Can be parsed from a string (needed for the `core::str::parse` function)
pub trait Numeric: Add + FromStr {}
impl<T: Add + FromStr> Numeric for T {}

pub fn text_to_number<T: Numeric>(text: String) -> Result<T, String> {
  let clean_text = text.trim();
  return match clean_text.parse::<T>() {
    Ok(number) => Ok(number),
    Err(_) => Err(format!(
      "[ERROR] could not parse value \"{clean_text}\" into a {} (NaN)", type_name::<T>(),
    )),
  };
}

pub fn read_line() -> Result<String, String> {
  let mut buffer = String::new();
  return match stdin().read_line(&mut buffer) {
    Ok(_) => Ok(buffer),
    Err(error) => Err(format!("[ERROR] could not read input from user:\n{error}")),
  };
}
