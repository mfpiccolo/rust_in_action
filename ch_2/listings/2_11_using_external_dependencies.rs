extern crate print_hello_world;             //#A
use print_hello_world::print_hello_world;   //#B

fn main() {
   print_hello_world();                     //#C
}
// #A Import an external crate into a file
// #B The use keyword brings names into the local scope.
// #C Invoke the external print function
