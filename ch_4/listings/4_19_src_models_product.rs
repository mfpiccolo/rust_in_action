#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: &'static str,
    pub label: &'static str,
}

impl Product {
  pub fn filter(l: String, products: &Vec<Self>) {
    let label = l.split(' ').collect::<Vec<&str>>()[2];
    for product in products {
      if product.label == label {
        println!("{:?}", product);
      }
    }
  }
}
