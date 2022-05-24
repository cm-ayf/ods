use std::collections::*;
use std::fmt::Display;
use std::io;

use rand::prelude::*;

fn main() {
  match std::env::args().nth(1) {
    Some(arg) => match &arg[..] {
      "1" => e1(),
      "2" => e2(),
      "3" => e3(),
      "4" => e4(),
      "5" => e5(),
      "6" => e6(),
      "7" => e7(),
      "8" => e8(),
      "9" => e9(),
      _ => (),
    },
    None => (),
  }
}

fn e1() {
  let stdin = io::stdin();
  let mut stack = Vec::new();
  
  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }
    stack.push(buf);
  }

  while let Some(line) = stack.pop() {
    print!("{}", line);
  }
}

fn e2() {
  let stdin = io::stdin();
  
  'outer: loop {
    let mut stack = Vec::new();
    let mut break_here = false;
    
    'inner: for _ in 0..50 {
      let mut buf = String::new();
      if stdin.read_line(&mut buf).unwrap() == 0 {
        break_here = true;
        break 'inner;
      }
      stack.push(buf);
    }

    while let Some(line) = stack.pop() {
      print!("{}", line);
    }

    if break_here {
      break 'outer;
    }
  }
}

fn e3() {
  let stdin = io::stdin();
  let mut queue = VecDeque::new();
  
  for _ in 0..42 {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      return;
    }
    queue.push_back(buf);
  }

  loop {
    let mut break_here = false;
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break_here = true;
    }

    let line = queue.pop_front().unwrap();
    if buf.trim().is_empty() {
      print!("{}", line);
    }

    queue.push_back(buf);

    if break_here {
      break;
    }
  }
}

fn e4() {
  let stdin = io::stdin();
  let mut uset = HashSet::new();
  
  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }

    if uset.insert(buf.clone()) {
      print!("{}", buf);
    }
  }
}

fn e5() {
  let stdin = io::stdin();
  let mut uset = HashSet::new();
  
  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }

    if !uset.insert(buf.clone()) {
      print!("{}", buf);
    }
  }
}

/// because [`BTreeSet`] and [`BTreeMap`] uses ordering defined in [`Ord`] trait,
/// and [`std::string::String`] implements [`Ord`] trait with alphabetical order,
/// we need to implement another [`Ord`] trait, thus wrap string with DIY-made struct.
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd)]
struct WrapString(String);

impl Ord for WrapString {
  fn cmp(&self, other: &WrapString) -> std::cmp::Ordering {
    self.0.len().cmp(&other.0.len()).then(self.0.cmp(&other.0))
  }
}

impl Display for WrapString {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

fn e6() {
  let stdin = io::stdin();
  let mut sset = BTreeSet::new();

  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }
    sset.insert(WrapString(buf));
  }

  for l in &sset {
    print!("{}", l);
  }
}

fn e7() {
  let stdin = io::stdin();
  let mut sset = BTreeMap::new();

  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }
    let buf = WrapString(buf);
    match sset.get_mut(&buf) {
      Some(v) => *v += 1,
      None => {
        sset.insert(buf, 1);
      },
    }
  }

  for (l, i) in &sset {
    for _ in 0..*i {
      print!("{}", l);
    }
  }
}

fn e8() {
  let stdin = io::stdin();
  let mut queue_even = VecDeque::new();
  let mut queue_odd = VecDeque::new();

  let mut i = 0;
  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }
    match i % 2 {
      0 => queue_even.push_back(buf),
      _ => queue_odd.push_back(buf),
    }
    i += 1;
  }

  while let Some(line) = queue_even.pop_front() {
    print!("{}", line);
  }

  while let Some(line) = queue_odd.pop_front() {
    print!("{}", line);
  }
}

fn e9() {
  let stdin = io::stdin();
  let mut rng = thread_rng();
  let mut stack = Vec::new();
  
  loop {
    let mut buf = String::new();
    if stdin.read_line(&mut buf).unwrap() == 0 {
      break;
    }
    stack.push(buf);
  }

  stack.shuffle(&mut rng);

  while let Some(line) = stack.pop() {
    print!("{}", line);
  }
}
