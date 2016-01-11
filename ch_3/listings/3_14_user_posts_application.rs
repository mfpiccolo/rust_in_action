#[derive(Debug)]                           //#A
struct User {
  id: i32,
  first_name: String,
  last_name: String,
  posts_count: i32
}

#[derive(Debug)]                          //#A
struct Post {
  user_id: i32,
  text: String
}

fn main() {
  let mut me = User {                     //#B
    id: 1,
    first_name: "Mike".to_string(),
    last_name: "Piccolo".to_string(),
    posts_count: 0
  };

  let post = create_post(&mut me);

  println!("{:?}", me);
  println!("{:?}", post);
}

fn create_post(u: &mut User) -> Post {
  let p = Post {                          //#B
    user_id: u.id,
    text: "How to Rust".to_string()
  };

  u.posts_count += 1;
  p
}
// #A Attribute to tell the compiler to automatically allow debugging of this struct
// #B Instantiating a struct with fields
