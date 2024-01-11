pub fn fibonacci_rec(n: i32) -> Option<i32> {
  return match n {
    x if x < 0 => {
      println!("[ERROR] argument for Fibonacci should be a positive integer, received {x}");
      return None;
    },
    0 | 1 => Some(1),
    _ => match (fibonacci_rec(n - 1), fibonacci_rec(n - 2)) {
      (a, b) if a.is_none() || b.is_none() => None,
      (a, b) => Some(a.unwrap() + b.unwrap()),
    },
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_return_one() {
    // Arrange
    let input1 = 0; let input2 = 1;
    // Act
    let result1 = fibonacci_rec(input1);
    let result2 = fibonacci_rec(input2);
    // Assert
    assert_eq!(result1, Some(1));
    assert_eq!(result2, Some(1));
  }

  #[test]
  fn should_correctly_compute_fibonacci() {
    // Arrange
    let input1 = 2; let input2 = 5; let input3 = 9; let input4 = 13;
    // Act
    let result1 = fibonacci_rec(input1);
    let result2 = fibonacci_rec(input2);
    let result3 = fibonacci_rec(input3);
    let result4 = fibonacci_rec(input4);
    // Assert
    assert_eq!(result1, Some(2));
    assert_eq!(result2, Some(8));
    assert_eq!(result3, Some(55));
    assert_eq!(result4, Some(377));
  }

  #[test]
  fn should_not_accept_negative_input() {
    // Arrange
    let input1 = -5;
    // Act
    let result1 = fibonacci_rec(input1);
    // Assert
    assert_eq!(result1, None);
  }
}
