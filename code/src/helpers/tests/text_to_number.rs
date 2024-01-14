use crate::helpers::text_to_number;

#[test]
fn should_correctly_convert_numeric_strings() {
  // Arrange
  let input1 = format!("42");         let input2 = format!("-13");
  let input3 = format!("-001000");    let input4 = format!("3.1416");
  let input5 = format!("3000000000"); let input6 = format!("06.0022140760");
  // Act
  let result1 = text_to_number::<u8>(input1);
  let result2 = text_to_number::<i8>(input2);
  let result3 = text_to_number::<i16>(input3);
  let result4 = text_to_number::<f32>(input4);
  let result5 = text_to_number::<u64>(input5);
  let result6 = text_to_number::<f64>(input6);
  // Assert
  assert_eq!(result1, Ok(42u8));
  assert_eq!(result2, Ok(-13i8));
  assert_eq!(result3, Ok(-1000i16));
  assert_eq!(result4, Ok(3.1416f32));
  assert_eq!(result5, Ok(3000000000u64));
  assert_eq!(result6, Ok(6.0022140760f64));
}

fn expected_parse_error_string(input: String, type_name: String) -> String {
  return format!(
    "[ERROR] could not parse value \"{input}\" into a {type_name} (NaN)"
  );
}

fn is_error_string_as_expected<T>(result:  Result<T, String>, expected: String) -> bool {
  return result.err().unwrap().contains(expected.as_str());
}

#[test]
fn should_not_convert_non_numeric_strings() {
  // Arrange
  let input1 = format!("bomboclat"); let input2 = format!("");
  // Act
  let result1 = text_to_number::<u8>(input1.clone());
  let result2 = text_to_number::<i8>(input2.clone());
  // Assert
  assert!(is_error_string_as_expected(
    result1, expected_parse_error_string(input1, "u8".to_string()),
  ));
  assert!(is_error_string_as_expected(
    result2, expected_parse_error_string(input2, "i8".to_string()),
  ));
}

#[test]
fn should_not_convert_strings_unfit_to_parse_type() {
  // Arrange
  let input1 = format!("-5");
  let input2 = format!("3000000000");
  let input3= format!("3.1416");
  // Act
  let result1 = text_to_number::<u8>(input1.clone());
  let result2 = text_to_number::<i16>(input2.clone());
  let result3 = text_to_number::<u64>(input3.clone());
  // Assert
  assert!(is_error_string_as_expected(
    result1, expected_parse_error_string(input1, "u8".to_string()),
  ));
  assert!(is_error_string_as_expected(
    result2, expected_parse_error_string(input2, "i16".to_string()),
  ));
  assert!(is_error_string_as_expected(
    result3, expected_parse_error_string(input3, "u64".to_string()),
  ));
}
