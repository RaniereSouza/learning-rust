use crate::fibonacci::fibonacci_rec;

#[test]
fn should_return_one() {
  // Arrange
  let input1 = 0; let input2 = 1;
  // Act
  let result1 = fibonacci_rec(input1);
  let result2 = fibonacci_rec(input2);
  // Assert
  assert_eq!(result1, Ok(1));
  assert_eq!(result2, Ok(1));
}

#[test]
fn should_correctly_compute_result() {
  // Arrange
  let input1 = 2; let input2 = 5; let input3 = 9; let input4 = 13;
  // Act
  let result1 = fibonacci_rec(input1);
  let result2 = fibonacci_rec(input2);
  let result3 = fibonacci_rec(input3);
  let result4 = fibonacci_rec(input4);
  // Assert
  assert_eq!(result1, Ok(2));
  assert_eq!(result2, Ok(8));
  assert_eq!(result3, Ok(55));
  assert_eq!(result4, Ok(377));
}

#[test]
fn should_not_accept_negative_input() {
  // Arrange
  let input1 = -5;
  // Act
  let result1 = fibonacci_rec(input1);
  // Assert
  assert_eq!(result1, Err(
    "[ERROR] argument for Fibonacci should be a positive integer, received -5".to_string()
  ));
}