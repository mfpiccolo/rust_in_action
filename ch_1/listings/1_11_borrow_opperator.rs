fn main() {
  let two = 2;
  let eight = 8;
  print_sum(&two, &eight) // #A
}

fn print_sum(first: &i32, second: &i32) {  // #B
  println!(“{}”, first + second); // #C
  println!(“{:p}”, first); // D
}

//#A We pass the variables with the borrow operator
//#B The function is expecting borrowed i32 integers
//#C Prints out the sum of the two borrowed values
//#D Prints out the address that references the borrowed value
