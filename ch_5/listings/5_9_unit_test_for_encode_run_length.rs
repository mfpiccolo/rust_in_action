#[test]
fn encode_run_length_test() {
  let original_string = "wwwwggzuuuiiiii".to_string();             //#A
  let encoded_string = "w4g2z1u3i5".to_string();                   //#B
  assert_eq!(encoded_string, encode_run_length(original_string));  //#C
}
// #A String that will be encoded
// #B Expected encoded string
// #C Assertion that will panic if function does not properly encode
