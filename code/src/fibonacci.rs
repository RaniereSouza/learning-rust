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
