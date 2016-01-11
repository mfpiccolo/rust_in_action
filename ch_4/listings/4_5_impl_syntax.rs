struct Job {
  id: i32,
  priority: i32,                                                     //#A
  queue_id: i32,
  runnable: bool
}

impl Job {
  fn new(id: i32, priority: i32, queue_id: i32) -> Job {             //#B
    Job {id: id,
      priority: priority,
      queue_id: queue_id,
      runnable: false
    }
  }

  fn check_queue(&mut self, queue_id: i32, priority: i32) -> &Self {  //#C
    if self.queue_id == queue_id && self.priority <= priority {
      self.runnable = true;
    }
    self
  }

  fn run(&self) {
    if self.runnable {
       println!("Running job #{}", self.id);
    } else {
      println!("Unable to run job #{}", self.id);
    }
  }
}

fn main() {
  let current_queue_id = 4;
  let current_priority = 2;
  let mut j = Job::new(5, 1, 4);                                  //#D
  j.check_queue(current_queue_id, current_priority).run();        //#E
}
// #A A Lower number in the queue will have higher priority
// #B Associated function to instantiate a new job struct
// #C Chainable instance function borrowing mutable self
// #D  Building a struct with associated function
// #E Using chained functions to run instance of job struct
