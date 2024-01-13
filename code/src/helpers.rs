use std::{ops::Add, str::FromStr, any::type_name, io::BufRead, fmt::Debug};

/// Custom trait to basically represent *any numeric type*:
/// * Can be passed onto the add operation (`+`)
/// * Can be parsed from a string (needed for the `core::str::parse` function)
pub trait Numeric: Add + FromStr {}
impl<T: Add + FromStr> Numeric for T {}

pub fn text_to_number<T>(text: String)  -> Result<T, String> where
  T: Numeric, <T as FromStr>::Err: Debug,
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
mod tests {
  use std::io::Cursor;
  use super::*;

  #[test]
  fn should_correctly_convert_text_to_number() {
    // Arrange
    let input1 = format!("42"); let input2 = format!("-13");
    let input3 = format!("-001000"); let input4 = format!("3.1416");
    let input5 = format!("3000000000"); let input6 = format!("06.0022140760");
    // Act
    let result1 = text_to_number::<u8>(input1);
    let result2 = text_to_number::<i8>(input2);
    let result3 = text_to_number::<i16>(input3);
    let result4 = text_to_number::<f32>(input4);
    let result5 = text_to_number::<u64>(input5);
    let result6 = text_to_number::<f64>(input6);
    // Assert
    assert_eq!(result1, Ok(42u8));
    assert_eq!(result2, Ok(-13i8));
    assert_eq!(result3, Ok(-1000i16));
    assert_eq!(result4, Ok(3.1416f32));
    assert_eq!(result5, Ok(3000000000u64));
    assert_eq!(result6, Ok(6.0022140760f64));
  }

  #[test]
  fn should_correctly_read_line_from_input() {
    // Arrange
    let input1 = Cursor::new("Hello, World!");
    let input2 = Cursor::new("Hello, Rust!\nHello World!\n");
    // Act
    let result1 = read_line_from(input1);
    let result2 = read_line_from(input2);
    // Assert
    assert_eq!(result1, Ok("Hello, World!".to_string()));
    assert_eq!(result2, Ok("Hello, Rust!".to_string()));
  }
}
