fn main() {
  let passable: fn(&str) -> &str = print_twice;                   //#A
  call_f(passable);                                               //#B
}

fn call_f(f: fn(&str) -> &str) {                                  //#C
  println!("You have passed a function. It does the following:");

  let result = f("some string");                                  //#D

  println!("and it returns:");
  println!("{:?}", result);
}

fn print_twice(s: &str) -> &str {
  println!("{:?}", s);
  println!("{:?}", s);

  return s;
}
// #A Assigning a variable with a function type
// #B Calling a function that accepts a function as an argument
// #C Argument type annotation for function
// #D Calling the passed in function
