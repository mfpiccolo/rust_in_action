fn main() {
  let s = "My First String";       #A
  println!("{}", s.len());         #B
  println!("{:?}", s.as_bytes());  #C
}
// #A Creating an &’static str with literal syntax
// #B Printing out the length of the string
// #C Printing out the slice of numeric values for each character
