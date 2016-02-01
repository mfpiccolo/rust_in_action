#[test]
fn remove_consecutive_dups_test() {
  assert!(vec![1,2,3,4] == remove_consecutive_dups(vec![1,2,2,3,4,4,4]));
}
