fn main() {
    let mut changable = "I can".to_string();
    change(&mut changable); // prints out “I can change”
}

fn change(changable: &mut String) {
    changable.push_str(&" change".to_string());
    println!("{}", changable);
}
