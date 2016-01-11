fn main() {
    let p1 = Product {id: 1, name: "Awesome Gaget", label: "awesome"};
    let p2 = Product {id: 2, name: "Cool Widget", label: "cool"};
    let p3 = Product {id: 3, name: "Rad Gizmo", label: "cool"};
    let p4 = Product {id: 4, name: "Lame Thingy", label: "lame"};
    let products = vec![p1, p2, p3, p4];

    let l1 = Label {id: 1, text: "cool"};
    let l2 = Label {id: 2, text: "awesome"};
    let l3 = Label {id: 3, text: "lame"};
    let labels = vec![l1, l2, l3];

    run_cli(&products, &labels);
}

/// run_cli() function defined here
