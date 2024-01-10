use ferris_says::say as ferris_say;
use std::io::{stdout, BufWriter};

fn main() {
  let stdout = stdout();
  let message = String::from("Hello, fellow Rustaceans!");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  ferris_say(&message, width, &mut writer).unwrap();
}
