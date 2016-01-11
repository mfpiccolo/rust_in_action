    // …
    } else if l.starts_with("filter by") {
      Product::filter(l, products)
    }
  }
}

impl Product {
  fn filter(l: String, products: &Vec<Self>) {
    let label = l.split(' ').collect::<Vec<&str>>()[2]; #B
    for product in products {
      if product.label == label {
        println!("{:?}", product);
      }
    }
  }
}
#B Method type annotation for collection
