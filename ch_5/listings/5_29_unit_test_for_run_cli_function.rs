use ops::run_cli;                                      //#A
use models::product::Product;                          //#A
use models::label::Label;                              //#A

#[test]
fn run_cli_test() {
    let expected_result = format!(                     //#B
      "{:?}",Product {id: 1, name: "a", label: "b"}
    );
    let products = vec![                               //#C
      Product {id: 1, name: "a", label: "b"},
      Product {id: 2, name: "c", label: "d"},
    ];
    let labels = vec![                               //#C
      Label {id: 1, text: "b"},
      Label {id: 2, text: "d"},
    ];

    let result = run_cli(&products, &labels, &"filter by b".to_string());
    assert_eq!(expected_result, result);             //#D
}
// #A Imports the run_cli() function and the two model structs
// #B This sets up our expected result which will be one of the filtered products
// #C Setting up our model collections that we will pass to run_cli()
// #D Assertion that ensures the run_cli() returns the expected result
