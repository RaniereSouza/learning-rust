use crate::helpers::text_to_number;

#[test]
fn should_correctly_convert_numeric_string() {
  // Arrange
  let input1 = format!("42"); let input2 = format!("-13");
  let input3 = format!("-001000"); let input4 = format!("3.1416");
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
