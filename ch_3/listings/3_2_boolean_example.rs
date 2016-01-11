fn main() {
    let yes = true;            //#A
    let no: bool = false;      //#B

    println!("yes is {:?}", yes);
    println!("no is {:?}", no);
}
// #A Compiler infers bool type
// #B Explicit bool type declaration
