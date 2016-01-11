struct User {                                           //#A
  id: i32,
  name: &'static str,
  email: &'static str,
}

fn main() {
  let user = User {                                     //#B
    id: 1,
    name: "Joe",
    email: "not@real.com",
  };

  send_welcome_email(user.name, user.email);            //#C
}

fn send_welcome_email(name: &str, email: &str) {
  // in real life there would be logic here to send an email
    println!("Email sent to {} at {}", name, email);
}
// #A Defining a User Struct
// #B Instantiating a User Struct instance
// #C Accessing the User Struct’s fields
