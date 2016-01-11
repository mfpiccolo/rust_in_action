fn main() {
  let x = 5;                                    //#A
  get_a_string();                               //#B
}

fn get_a_string() -> String {
  let s: String = "Return Me!".to_string()    //#C
}
// #A Declaration Statement
// #B Expression Statement
// #C Expression
