#![feature(custom_derive)]
mod ops;                                                           //#A
mod models;                                                        //#B

use models::product::Product;                                      //#C
use models::label::Label;
use ops::run_cli;                                                  //#D

fn main() {
  // same contents of the main fn from listing 4.9
}
// #A Declare ops module for run_cli() function.
// #B Declare models module.  We will use this to declare product and label module.
// #C Import Product and Label.  Notice that they are nested inside another module
// #D Import ops module.
