use super::run_program;                                     //#A

#[test]
fn run_program_filter_by_test() {
  assert_eq!(
    "Product { id: 1, name: \"Awesome Gaget\", label: \"awesome\" }".to_string(),
    run_program("filter by awesome".to_string())
  );
}
// #A Uses super to look up one module and import the run program function
