use std::collections::VecDeque;

use chapter01::interface::List;

trait RotateList<T: Clone>: List<T> {
  fn rotate(&mut self, r: usize) {
    let size = self.size();
    let r = r % size;
    if r < size / 2 {
      for _ in 0..r {
        if let Some(v) = self.remove(size - 1) {
          self.add(0, v);
        }
      }
    } else {
      for _ in 0..(size - r) {
        if let Some(v) = self.remove(0) {
          self.add(size - 1, v);
        }
      }
    }
  }
}

#[derive(Default)]
struct WrapVecDeque<T>(VecDeque<T>);

impl<T: Clone> List<T> for WrapVecDeque<T> {
  fn get(&self, index: usize) -> Option<T> {
    VecDeque::get(&self.0, index).cloned()
  }

  fn set(&mut self, index: usize, value: T) -> Option<T> {
    if index >= self.size() {
      return None;
    }

    self.0.push_back(value);
    self.0.swap_remove_back(index)
  }

  fn add(&mut self, index: usize, value: T) {
    if index <= self.size() {
      self.0.insert(index, value);
    }
  }

  fn remove(&mut self, index: usize) -> Option<T> {
    VecDeque::remove(&mut self.0, index)
  }

  fn size(&self) -> usize {
    self.0.len()
  }
}

impl<T: Clone> RotateList<T> for WrapVecDeque<T> {}

#[cfg(test)]
mod test {
  use super::*;
  use crate::vectrique::VecTrique;

  impl<T: Clone> RotateList<T> for VecTrique<T> {}

  #[test]
  fn test_rotate() {
    let mut dq: WrapVecDeque<_> = Default::default();
    let mut tq: VecTrique<_> = Default::default();

    for i in 0..10 {
      dq.add(i, i);
      tq.add(i, i);
    }

    dq.rotate(3);
    tq.rotate(6);

    for i in 0..10 {
      assert_eq!(Some(i), dq.get((i + 3) % 10));
    }

    for i in 0..10 {
      assert_eq!(Some(i), tq.get((i + 6) % 10));
    }
  }
}