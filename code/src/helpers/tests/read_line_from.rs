use std::io::Cursor;

use crate::helpers::read_line_from;

#[test]
fn should_correctly_extract_from_input() {
  // Arrange
  let mut input1 = Cursor::new("Hello, World!");
  let mut input2 = Cursor::new("Hello, Rust!\nHello, Unit Testing!\n");
  // Act
  let result1 = read_line_from(&mut input1);
  let result2 = read_line_from(&mut input2);
  let result3 = read_line_from(&mut input2);
  // Assert
  assert_eq!(result1, Ok("Hello, World!".to_string()));
  assert_eq!(result2, Ok("Hello, Rust!".to_string()));
  assert_eq!(result3, Ok("Hello, Unit Testing!".to_string()));
}

#[test]
fn should_fail_on_empty_buffer() {
  // Arrange
  let mut input1 = Cursor::new("");
  let mut input2 = Cursor::new("Hello, World!");
  // Act
  let result1 = read_line_from(&mut input1);
  let result2 =
    read_line_from(&mut input2)
    .and_then(|_| { read_line_from(&mut input2) });
  // Assert
  assert!(result1.err().unwrap().contains("[ERROR] could not read input content: end of buffer"));
  assert!(result2.err().unwrap().contains("[ERROR] could not read input content: end of buffer"));
}
