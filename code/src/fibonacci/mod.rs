/// Fibonacci with only recursion
pub fn fibonacci_rec(n: i32) -> Result<i32, String> {
  return match n {
    x if x < 0 => Err(format!(
      "[ERROR] argument for Fibonacci should be a positive integer, received {x}"
    )),
    0 | 1 => Ok(1),
    _ => match (fibonacci_rec(n - 1), fibonacci_rec(n - 2)) {
      (a, b) if a.is_err() || b.is_err() => Err(
        "[ERROR] Unknown error in Fibonacci recursion".to_string()
      ),
      (a, b) => Ok(a.unwrap() + b.unwrap()),
    },
  };
}

// /// Fibonacci with memoization
// pub fn fibonacci_memo(n: i32) -> Result<i32, String> { todo!() }

#[cfg(test)]
mod tests;
