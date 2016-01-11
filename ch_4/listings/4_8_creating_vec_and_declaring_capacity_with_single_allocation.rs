fn main () {
    let mut list: Vec<i32> = Vec::new();
    list.reserve(100);                                        //#A
    println!("Initial Capacity: {}\n", list.capacity());
    list.push(1);
    println!("Length: {} \nCapacity: {}\n", list.len(), list.capacity());
    for x in 0..100 {
        list.push(x);
        println!("Length: {} \nCapacity: {}\n", list.len(), list.capacity());
    }
}
// #A Use reserve() method to increase initial capacity
