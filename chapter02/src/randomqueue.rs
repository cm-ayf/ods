//! Information Mathematics 2 - assignment 2
//! - Exercise 2..2: implemented [`RandomQueue`]
//! - Exercise 2..3: implemented [`VecTrique`]
//! - Exercise 2..5: implemented [`List::rotate`] in trait [`List`].
//! for checking, [`List`] is implemented also for [`VecDeque`].
//! 
//! execute `cargo test --bin report2` to run all tests
//! 

use rand::Rng;

use chapter01::interface::Queue;

#[derive(Default, Clone)]
struct RandomQueue<T> {
  queue: Vec<T>,
  rng: rand::rngs::ThreadRng,
}

impl<T> Queue<T> for RandomQueue<T> {
  fn add(&mut self, value: T) {
    self.queue.push(value);
  }

  fn remove(&mut self) -> Option<T> {
    let len = self.queue.len();
    if len == 0 {
      return None;
    }

    let i = self.rng.gen_range(0, len);
    Some(self.queue.swap_remove(i))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn test_random_queue() {
    let mut qs = Vec::new();

    // generates empty queue
    let mut q: RandomQueue<_> = Default::default();
    for i in 0..10 {
      q.add(i);
    }
    
    for _ in 0..10 {
      // note that RandomQueue::clone() is implemented
      // in such way that creates new ThreadRng on each clone
      qs.push(q.clone());
    }

    let mut stacks = HashSet::new();
    // insert original queue into set
    stacks.insert(q.queue);

    while let Some(mut q) = qs.pop() {
      let mut stack = Vec::new();
      while let Some(item) = q.remove() {
        stack.push(item);
      }

      println!("{:?}", stack);
      if !stacks.insert(stack) {
        panic!("same stack");
      }
    }
  }
}

