/// Fibonacci with only recursion
#[allow(dead_code)]
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

fn __fib_memo_rec(n: i32, memo: &mut Vec<i32>) -> i32 {
  let index: usize = n.try_into().unwrap();
  if index >= memo.len() {
    let new_value =
      __fib_memo_rec(n - 1, memo) + __fib_memo_rec(n - 2, memo);
    memo.push(new_value)
  }
  memo[index]
}

/// Fibonacci with memoization
pub fn fibonacci_memo(n: i32) -> Result<i32, String> {
  return match n {
    x if x < 0 => Err(format!(
      "[ERROR] argument for Fibonacci should be a positive integer, received {x}"
    )),
    0 | 1 => Ok(1),
    _ => {
      let mut memo = vec![1, 1];
      Ok(__fib_memo_rec(n, &mut memo))
    },
  }
}

#[cfg(test)]
mod tests;
