struct Number {
    n: i32
}

impl Number {
    fn plus_one(&mut self)   -> &mut Self { self.n += 1; self } //#A
    fn plus_two(&mut self)   -> &mut Self { self.n += 2; self } //#A
    fn plus_three(&mut self) -> &mut Self { self.n += 3; self } //#A
}

fn main() {
    let mut j = Number {n: 0};
    j.plus_one().plus_two().plus_three();                       //#B
    println!("{}", j.n)
}
// #A Chainable method because it returns self
// #B Method chain
