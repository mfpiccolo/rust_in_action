fn main () {
    let mut list: Vec<i32> = Vec::new();                      //#A
    println!("Initial Capacity: {}\n", list.capacity());      //#B
    list.push(1);                                             //#C
    println!("Length: {} \nCapacity: {}\n", list.len(), list.capacity());
    for x in 0..100 {                                         //#D
        list.push(x);
        println!("Length: {} \nCapacity: {}\n", list.len(), list.capacity());
    }
}
// #A Creating a Vec using explicit syntax
// #B Checks the initial capacity (Hint: it is zero)
// #C Using push() method to add an element to the list
// #D A loop that will push the integers from 0 to 99 into the list
