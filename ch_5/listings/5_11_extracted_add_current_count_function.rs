fn appent_current_count(encoded: &mut String, current_count: i32) {
  for i_char in current_count.to_string().chars() {
    encoded.push(i_char);
  }
}
