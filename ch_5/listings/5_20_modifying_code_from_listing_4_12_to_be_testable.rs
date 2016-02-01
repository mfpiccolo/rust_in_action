use std::io;
use std::io::prelude::*;

pub fn run_cli(products: &Vec<Product>, labels: &Vec<Label>, command: &String)                             #A
-> String {
  let mut std_out = String::new();
  if command == "ls products" {
    std_out = format!("{:?}", products)
  } else if command== "ls labels" {
    std_out = format!("{:?}", labels);
  } else if command.starts_with("filter by") {
    std_out = Product::filter(&command, products)  #B
  }
  std_out
}
#A The run_cli() function now accepts and returns a string (to mimic user input)
#B Passing the command to the filter function.
