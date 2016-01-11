fn encode_run_length(string: String) -> String {
  let mut encoded = String::new();
  let mut c = string.chars().nth(0).unwrap();
  let mut current_count = 0;
  encoded.push(c);

  for s_char in string.chars() {
    if c == s_char {
      current_count += 1;
    } else {
      for i_char in current_count.to_string().chars() {
        encoded.push(i_char);
      }
      encoded.push(s_char);
      c = s_char;
      current_count = 1;
    }
  }

  for i_char in current_count.to_string().chars() {
    encoded.push(i_char);
  }

  encoded
}
