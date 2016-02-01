#[test]
fn append_current_count_test() {
  let mut encoded = "a".to_string();
  let mut current_count = 15;
  append_current_count(&mut encoded, current_count);   //#A
  assert_eq!("a15".to_string(), encoded);              //#B
}
// #A Call function that we are testing to mutate encoded variable
// #B Assertion that will panic if function did not mutate the string properly
