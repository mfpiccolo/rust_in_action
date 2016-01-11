extern crate rand;                                    //#A

use rand::distributions::{IndependentSample, Range};  //#B

fn lotto_numbers(times: i32) -> Vec<i32> {
  let mut winning_numbers = Vec::new();
  let mut rng = rand::thread_rng();                   //#C
  let between = Range::new(1, 101);

  for n in 0..times {
    winning_numbers.push(between.ind_sample(&mut rng)); //#D
  }
  winning_numbers
}
//#A Importing the external crate
//#B Importing the desired random modules
//#C Setting up the random generator
//#D Pushing the randomly generated number into the list
