use models::product::Product;

#[test]
fn filter_test() {
    let expected_result = format!(
      "{:?}",Product {id: 1, name: "a", label: "b"}
    );
    let products = vec![
      Product {id: 1, name: "a", label: "b"},
      Product {id: 2, name: "c", label: "d"},
      Product {id: 2, name: "e", label: "f"},
    ];

    let result = Product::filter(&"filter by b".to_string(), &products);
    assert_eq!(expected_result, result);
}
