// Already defined User struct from listing 4.1
// Implemention of send_welcome_email on User struct
fn main() {
  let user = User {id: 1, name: "Joe", email: "not@real.com"}
  user.send_welcome_email();      //#A
}
// #A Calling a method on a user instance
