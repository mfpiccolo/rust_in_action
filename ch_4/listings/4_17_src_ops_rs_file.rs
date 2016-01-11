use std::io;
use std::io::prelude::*;
use models::product::Product;                                   //#A
use models::label::Label;                                       //#A

pub fn run_cli(products: &Vec<Product>, labels: &Vec<Label>) {
    // same contents of the run_cli() fn from listing 4.10
}
// #A We need to import the Product and Label structs here as well
