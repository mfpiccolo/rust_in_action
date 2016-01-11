struct User {                                        //#A
  first_name: String,
  last_name: String,
  posts_count: i32
}

fn main() {
  let me = User {                                    //#B
    first_name: "Mike".to_string(),
    last_name: "Piccolo".to_string(),
    posts_count: 0
  };

  println!(
    "My name is ({} {} and I have {} posts)",
    me.first_name, me.last_name, me.posts_count      //#C
  );
}
// #A Defining a User struct and its attributes with type annotation
// #B Instantiating a user with fields
// #C Accessing the u struct’s fileds
