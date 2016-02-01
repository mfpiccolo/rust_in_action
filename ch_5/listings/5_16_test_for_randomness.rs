#[test]
#[should_panic]
fn lotto_numbers_random_test() {
  assert_eq!(lotto_numbers(5), lotto_numbers(5))
}
