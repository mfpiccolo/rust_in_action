fn main() {
  let x = 3;
  let y = Box::new(6);
  print!(“{}”, *y + x); // prints '9'
}
