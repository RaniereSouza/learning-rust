// use std::{process::ExitCode, io::{stdin, BufReader}};

// mod helpers; use helpers::{read_line_from, text_to_number};
// mod fibonacci; use fibonacci::fibonacci_memo;

// fn main() -> ExitCode {
//   println!("insert an integer n to get the nth Fibonacci number: ");

//   let mut stdin_bufreader = BufReader::new(stdin());
//   let mut input: Option<i32> = None;
//   let result =
//     read_line_from(&mut stdin_bufreader)
//     .and_then(text_to_number)
//     .and_then(|parsed_number| {
//       input = Some(parsed_number); fibonacci_memo(parsed_number)
//     });

//   if result.is_err() {
//     println!("{}", result.err().unwrap());
//     return ExitCode::FAILURE;
//   };

//   println!("the #{} Fibonacci number is {}", input.unwrap(), result.unwrap());
//   return ExitCode::SUCCESS;
// }

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
  let ui = AppWindow::new()?;

  let mut count = 0;
  ui.on_count_button_clicked({
    let ui_handle = ui.as_weak();
    move || {
      let ui = ui_handle.unwrap();
      println!("DEBUG: count button clicked");
      count = count + 1; ui.set_count(count);
    }
  });
  ui.on_text_input_enter_pressed(move |text| {
    println!("DEBUG: text input content is \"{}\"", text);
  });

  ui.run()
}
