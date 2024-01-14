use std::{ops::Add, str::FromStr, any::type_name, io::BufRead, fmt::Debug};

/// Custom trait to basically represent *any numeric type*:
/// * Can be passed onto the add operation (`+`)
/// * Can be parsed from a string (needed for the `core::str::parse` function)
pub trait Numeric: Add + FromStr {}
impl<T: Add + FromStr> Numeric for T {}

pub fn text_to_number<T>(text: String) -> Result<T, String>
  where T: Numeric, <T as FromStr>::Err: Debug,
{
  return match text.parse::<T>() {
    Ok(number) => Ok(number),
    Err(error) => Err(format!(
      "[ERROR] could not parse value \"{text}\" into a {} (NaN):\n{:?}",
      type_name::<T>(), error,
    )),
  };
}

pub fn read_line_from<R: BufRead>(mut stream: R) -> Result<String, String> {
  let mut buffer = String::new();
  return match stream.read_line(&mut buffer) {
    Ok(_) => Ok(format!("{}", buffer.trim())),
    Err(error) => Err(format!(
      "[ERROR] could not read input content:\n{:?}", error,
    )),
  };
}

#[cfg(test)]
mod tests;
