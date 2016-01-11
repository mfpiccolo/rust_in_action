fn main() {
  let one_two_three: (i32, &str, char) = (1, "two", '3');    //#A
  let (one, two, three) = one_two_three;                     //#B

  println!("Tuple: {:?}", one_two_three);
  println!("one: {:?}", one);
  println!("two: {:?}", two);
  println!("three: {:?}", three);
  println!("3rd index: {:?}", one_two_three.2);              //#C
}
// #A Creating a tuple with type annotated
// #B Destructuring example
// #C Accessing index of tuple
