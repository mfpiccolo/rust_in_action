fn remove_consecutive_dups(list: Vec<i32>) -> Vec<i32> {
  let mut new_list = vec![];
  let mut previous_number = 0;
  for number in list {
    if previous_number != number {
  new_list.push(number)
      previous_number = number;
    }
  }
  new_list
}
