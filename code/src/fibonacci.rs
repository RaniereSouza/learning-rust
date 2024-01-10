pub fn fibonacci_rec(n: i32) -> i32 {
  return match n {
    0 | 1 => 1,
    _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
  };
}
