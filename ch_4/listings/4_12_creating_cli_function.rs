use std::io;
use std::io::prelude::*;

fn run_cli(products: &Vec<Product>, labels: &Vec<Label>) {
  println!("Check out these awesome products!");
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let l = line.unwrap();
    if l == "ls products" {
      for product in products {
        println!("{:?}", product);
      }
    } else if l == "ls labels" {
      for label in labels {
        println!("{:?}", label);
      }
    // } else if l.starts_with("filter by") {
      // Product::filter(l, products)
    // }
  }
}
