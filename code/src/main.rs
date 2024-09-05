slint::include_modules!();

mod helpers; use helpers::text_to_number;
mod fibonacci; use fibonacci::fibonacci_memo;

fn main() -> Result<(), slint::PlatformError> {
  let ui = AppWindow::new()?;

  ui.on_calculate_fibonacci({
    let ui_handle = ui.as_weak();
    move |text| {
      let ui = ui_handle.unwrap();
      let mut input: Option<i32> = None;
      let result =
        text_to_number(text.into())
        .and_then(|parsed_number| {
          input = Some(parsed_number); fibonacci_memo(parsed_number)
        });

      if result.is_err() {
        println!("{}", result.err().unwrap());
        ui.set_result_error("Calculation failed! See logs for more details.".into());
      }
      else {
        ui.set_result_success(format!(
          "the #{} Fibonacci number is {}",
          input.unwrap(), result.unwrap()
        ).into());
      }
    }
  });

  ui.run()
}
