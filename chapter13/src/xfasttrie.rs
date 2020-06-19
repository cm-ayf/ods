use chapter01::interface::{SSet, USet};
use chapter05::linearhashtable::LinearHashTable;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, Default)]
pub struct BTNode {
    x: RefCell<i32>,
    prefix: RefCell<usize>,
    child: [RefCell<Option<Rc<BTNode>>>; 2], // 0 = left, 1 = right
    jump: RefCell<Option<Rc<BTNode>>>,
    parent: RefCell<Option<Weak<BTNode>>>,
    prev: RefCell<Option<Weak<BTNode>>>, // left
    next: RefCell<Option<Rc<BTNode>>>,   // right
}

impl BTNode {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PartialEq for BTNode {
    fn eq(&self, other: &Self) -> bool {
        *self.prefix.borrow() == *other.prefix.borrow()
    }
}

impl Hash for BTNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prefix.borrow().hash(state);
    }
}

#[derive(Clone, Debug, Default)]
pub struct XFastTrie {
    n: usize,
    r: Rc<BTNode>,
    head: Option<Rc<BTNode>>,   // dummy1
    tail: Option<Weak<BTNode>>, // dummy2
    t: Vec<LinearHashTable<Rc<BTNode>>>,
}

impl XFastTrie {
    const W: usize = 32;
    pub fn new() -> Self {
        let r = Rc::new(BTNode::new());
        let dummy1: Rc<BTNode> = Default::default();
        let dummy2: Rc<BTNode> = Default::default();
        *dummy1.next.borrow_mut() = Some(dummy2.clone());
        *dummy2.prev.borrow_mut() = Some(Rc::downgrade(&dummy1));
        *r.jump.borrow_mut() = Some(dummy2.clone());
        Self {
            r,
            n: 0,
            head: Some(dummy1),
            tail: Some(Rc::downgrade(&dummy2)),
            t: vec![LinearHashTable::new(); XFastTrie::W + 1],
        }
    }
}

impl SSet<i32> for XFastTrie {
    fn size(&self) -> usize {
        self.n
    }

    fn add(&mut self, x: i32) -> bool {
        let mut c = 0;
        let ix = x as usize;
        let mut u = self.r.clone();

        // 1 - search for ix until falling out of the trie
        let mut i = 0;
        let mut next;
        for _ in 0..XFastTrie::W {
            c = (ix >> (XFastTrie::W - i - 1)) & 1;
            match *u.child[c].borrow() {
                Some(ref c) => next = c.clone(),
                None => break,
            }
            u = next;
            i += 1;
        }
        if i == XFastTrie::W {
            return false; // already contains x - abort
        }
        let pred = match c {
            0 => {
                let j = u.jump.borrow_mut().take();
                match j {
                    Some(ref j) => j.prev.borrow().as_ref().and_then(|p| p.upgrade()),
                    None => None,
                }
            }
            _ => u.jump.borrow_mut().take(), // right
        };

        // 2 - add path to ix
        while i < XFastTrie::W {
            c = (ix >> (XFastTrie::W - i - 1)) & 1;
            let n = Rc::new(BTNode::new());
            *n.prefix.borrow_mut() = ix >> (XFastTrie::W - i - 1);
            self.t[i + 1].add(n.clone());
            n.parent.borrow_mut().replace(Rc::downgrade(&u));
            u.child[c].borrow_mut().replace(n);
            let uc = u.child[c].borrow().clone().unwrap();
            u = uc;
            i += 1;
        }
        *u.x.borrow_mut() = x;

        // 3 - add u to linked list
        *u.prev.borrow_mut() = pred.as_ref().map(|p| Rc::downgrade(&p));
        *u.next.borrow_mut() = pred.as_ref().and_then(|p| p.next.borrow().clone());
        u.prev
            .borrow()
            .as_ref()
            .map(|p| p.upgrade().map(|p| p.next.borrow_mut().replace(u.clone())));
        u.next
            .borrow()
            .as_ref()
            .map(|n| n.prev.borrow_mut().replace(Rc::downgrade(&u)));

        // 4 - walk back up, updating jump pointers
        let mut v = u.parent.borrow().as_ref().and_then(|p| p.upgrade());
        while let Some(vi) = v {
            if (vi.child[0].borrow().is_none()
                && (vi.jump.borrow().is_none()
                    || vi
                        .jump
                        .borrow()
                        .as_ref()
                        .filter(|j| (*j.x.borrow() as usize) > ix)
                        .is_some()))
                || (vi.child[1].borrow().is_none()
                    && (vi.jump.borrow().is_none()
                        || vi
                            .jump
                            .borrow()
                            .as_ref()
                            .filter(|j| (*j.x.borrow() as usize) < ix)
                            .is_some()))
            {
                vi.jump.borrow_mut().replace(u.clone());
            }
            v = vi.parent.borrow().as_ref().and_then(|p| p.upgrade());
        }
        self.n += 1;
        true
    }
    fn remove(&mut self, x: &i32) -> Option<i32> {
        let mut c;
        let ix = *x as usize;
        let mut u = self.r.clone();

        // 1 - find leaf, u, containing x
        let mut i = 0;
        let mut next;
        for _ in 0..XFastTrie::W {
            c = (ix >> (XFastTrie::W - i - 1)) & 1;
            match *u.child[c].borrow() {
                Some(ref c) => next = c.clone(),
                None => return None,
            }
            u = next;
            i += 1;
        }

        // 2 - remove u from linked list
        let next = u.next.borrow_mut().take();
        let prev = u.prev.borrow_mut().take();
        next.as_ref().map(|n| *n.prev.borrow_mut() = prev.clone());
        prev.as_ref()
            .map(|p| *p.upgrade().unwrap().next.borrow_mut() = next.clone());
        let mut v = u.clone();

        // 3 - delete nodes on path to u
        for i in (0..=(XFastTrie::W - 1)).rev() {
            c = (ix >> (XFastTrie::W - i - 1)) & 1;
            let vp = v
                .parent
                .borrow()
                .as_ref()
                .and_then(|p| p.upgrade())
                .unwrap();
            v = vp;
            self.t[i + 1].remove(v.child[c].borrow_mut().take().as_ref().unwrap());
            if v.child[1 - c].borrow().is_some() {
                break;
            }
        }

        // 4 - update jump pointers
        c = if v.child[0].borrow().is_none() { 1 } else { 0 };
        *v.jump.borrow_mut() = if c == 0 {
            prev.as_ref().and_then(|p| p.upgrade())
        } else {
            next.clone()
        };
        let mut v = v.parent.borrow().as_ref().and_then(|p| p.upgrade());
        while let Some(vi) = v {
            if vi
                .jump
                .borrow()
                .as_ref()
                .filter(|j| Rc::ptr_eq(j, &u))
                .is_some()
            {
                let c = if vi.child[0].borrow().is_none() { 1 } else { 0 };
                *vi.jump.borrow_mut() = if c == 0 {
                    prev.as_ref().and_then(|p| p.upgrade())
                } else {
                    next.clone()
                };
            }
            v = vi.parent.borrow().as_ref().and_then(|p| p.upgrade());
        }
        self.n -= 1;
        Some(Rc::try_unwrap(u).ok().unwrap().x.into_inner())
    }

    fn find(&self, x: &i32) -> Option<i32> {
        let mut l = 0;
        let mut h = XFastTrie::W + 1;
        let ix = *x as usize;
        let mut u = self.r.clone();
        let q = Rc::new(BTNode::new());
        while h - l > 1 {
            let i = (l + h) / 2;
            *q.prefix.borrow_mut() = ix >> (XFastTrie::W - i);
            match self.t[i].find(&q) {
                None => h = i,
                Some(v) => {
                    u = v;
                    l = i;
                }
            }
        }
        if l == XFastTrie::W {
            return Some(*u.x.borrow());
        }
        let c = (ix >> (XFastTrie::W - l - 1)) & 1;
        let pred = if c == 1 {
            u.jump.borrow().clone()
        } else {
            let j = u.jump.borrow().clone();
            match j {
                Some(ref j) => j.prev.borrow().as_ref().and_then(|p| p.upgrade()),
                None => None,
            }
        };
        match pred {
            Some(pred) => match &*pred.next.borrow() {
                Some(n) if n.next.borrow().is_none() => None,
                Some(n) if n.prev.borrow().is_none() => None,
                _ => pred.next.borrow().as_ref().map(|u| *u.x.borrow()),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chapter01::interface::SSet;
    use chapter09::redblacktree::RedBlackTree;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_xfasttrie() {
        let mut rng = thread_rng();
        let n = 200;
        let mut redblacktree = RedBlackTree::<i32>::new();
        let mut xfasttrie = XFastTrie::new();
        for _ in 0..5 {
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                redblacktree.add(x);
                xfasttrie.add(x);
                assert_eq!(redblacktree.size(), xfasttrie.size());
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = xfasttrie.find(&x);
                assert_eq!(y1, y2);
            }
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let b1 = redblacktree.remove(&x);
                let b2 = xfasttrie.remove(&x);
                assert_eq!(b1, b2);
            }
            assert_eq!(redblacktree.size(), xfasttrie.size());
            for _ in 0..n {
                let x = rng.gen_range(0, 5 * n);
                let y1 = redblacktree.find(&x);
                let y2 = xfasttrie.find(&x);
                assert_eq!(y1, y2);
            }
        }
    }
}