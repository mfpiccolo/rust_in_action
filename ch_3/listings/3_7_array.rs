fn main() {
    let array: [i8; 3] = [1, 2, 3];       //#A
    let mut array_with_defaults = [0; 5]; //#B
    array_with_defaults[3] = 25;          //#C

    for x in &array_with_defaults {       //#D
        println!("{}", x);                //#E
    }
}
// #A Using literal array syntax
// #B Default value array inferring type
// #C Index assignment
// #D Iterating over array values
// #D Print line with string interpolation
