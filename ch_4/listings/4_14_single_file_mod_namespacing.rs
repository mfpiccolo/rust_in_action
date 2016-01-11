mod animal {
  mod safe {
    mod dog {
     fn bark() {println!("Bark!!!")}
    }
    mod cat {
     fn meow() {println!("Meow!!!")}
    }
  }

  mod might_eat_you {
    mod lion {
     fn roar() {println!("Roar!!!")}
    }
  }
}
