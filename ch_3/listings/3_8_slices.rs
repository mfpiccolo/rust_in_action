fn main() {
  let array: [&str; 4] = ["one", "two", "three", "four"];   //#A
  let one_two = &array[0..2];                               //#B
  let three_four = &array[2..];                             //#B
  let first_three = &array[..3];                            //#B
  let whole = &array[..];                                   //#B

  println!("{:?} ", one_two);                               //#C
  println!("{:?} ", three_four);                            //#C
  println!("{:?} ", first_three);                           //#C
  println!("{:?} ", whole);                                 //#C
}
// #A Creating an array of &str’s
// #B Taking slices of the array
// #C Print statement using the display format
