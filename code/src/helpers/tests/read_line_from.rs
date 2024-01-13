use std::io::Cursor;

use crate::helpers::read_line_from;

#[test]
fn should_correctly_extract_from_input() {
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