#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: &'static str,
    pub label: &'static str,
}

impl Product {
  pub fn filter(l: &String, products: &Vec<Self>) -> String {
    let label = l.split(' ').collect::<Vec<&str>>()[2];
    let mut products_string = String::new();
    for product in products {
      if product.label == label {
        products_string = format!("{:?}", product);
      }
    }
    products_string
  }
}
