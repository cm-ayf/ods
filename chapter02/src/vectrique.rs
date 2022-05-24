use std::collections::VecDeque;

use chapter01::interface::List;

#[derive(Default)]
pub struct VecTrique<T> {
  left: VecDeque<T>,
  right: VecDeque<T>,
}

impl<T> VecTrique<T> {
  fn balance(&mut self) {
    let len = self.left.len() + self.right.len();
    if len <= 1 {
      return;
    }

    loop {
      match self.left.len().cmp(&(len / 2)) {
        std::cmp::Ordering::Equal => break,
        std::cmp::Ordering::Greater => {
          if let Some(value) = self.left.pop_back() {
            self.right.push_front(value);
          };
        }
        std::cmp::Ordering::Less => {
          if let Some(value) = self.right.pop_front() {
            self.left.push_back(value);
          };
        }
      }
    }
  }
}

impl<T: Clone> List<T> for VecTrique<T> {
  fn get(&self, index: usize) -> Option<T> {
    if index < self.left.len() {
      return self.left.get(index).cloned();
    }

    if index < self.left.len() + self.right.len() {
      return self.right.get(index - self.left.len()).cloned();
    }

    None
  }

  fn set(&mut self, index: usize, value: T) -> Option<T> {
    if index < self.left.len() {
      self.left.push_back(value);
      self.left.swap_remove_back(index)
    } else if index < self.left.len() + self.right.len() {
      self.left.push_back(value);
      self.left.swap_remove_back(index - self.left.len())
    } else {
      None
    }
  }

  fn add(&mut self, index: usize, value: T) {
    if index < self.left.len() {
      self.left.insert(index, value);
    } else if index < self.left.len() + self.right.len() {
      self.right.insert(index - self.left.len(), value);
    } else if index == self.left.len() + self.right.len() {
      self.right.push_back(value);
    }

    self.balance();
  }

  fn remove(&mut self, index: usize) -> Option<T> {
    let r = if index < self.left.len() {
      self.left.remove(index)
    } else if index < self.left.len() + self.right.len() {
      self.right.remove(index - self.left.len())
    } else {
      None
    };

    self.balance();
    r
  }

  fn size(&self) -> usize {
    self.left.len() + self.right.len()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_vec_trique() {
    let mut tq: VecTrique<_> = Default::default();
    tq.add(0, 1);
    tq.add(1, 2);
    tq.add(0, 3);
    tq.add(3, 4);
    tq.add(1, 2);
    tq.set(1, 5);
    assert_eq!(Some(1), tq.remove(2));

    tq.add(2, 6);

    assert_eq!(Some(3), tq.get(0));
    assert_eq!(Some(5), tq.get(1));
    assert_eq!(Some(6), tq.get(2));
    assert_eq!(Some(2), tq.get(3));
    assert_eq!(Some(4), tq.get(4));
  }
}