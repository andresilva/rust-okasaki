use std::rc::Rc;

use tree::Tree;
use tree::Tree::{Node, Tip};

pub trait Set<T> {
    fn empty() -> Self;
    fn insert(&self, T) -> Self;
    fn member(&self, T) -> bool;
}

impl<T: Ord + Clone> Set<T> for Tree<T> {
    fn empty() -> Tree<T> {
        Tip
    }

    fn insert(&self, x: T) -> Tree<T> {
        match *self {
            Tip => Node(Rc::new(Tip), x, Rc::new(Tip)),
            Node(ref l, ref v, ref r) if x < *v => Node(Rc::new(l.insert(x)), v.clone(), r.clone()),
            Node(ref l, ref v, ref r) if x > *v => Node(l.clone(), v.clone(), Rc::new(r.insert(x))),
            _ => (*self).clone()
        }
    }

    fn member(&self, x: T) -> bool {
        match *self {
            Tip => false,
            Node(ref l, ref v, _) if x < *v => l.member(x),
            Node(_, ref v, ref r) if x > *v => r.member(x),
            _ => true
        }
    }
}

#[test]
fn treeset() {
    let t: Tree<usize> = Set::empty();
    let t2 = t.insert(6).insert(8).insert(9).insert(7)
        .insert(4).insert(5).insert(1);

    assert_eq!(t2,
            Node(
                Rc::new(Node(
                    Rc::new(Node(
                        Rc::new(Tip),
                        1,
                        Rc::new(Tip))),
                    4,
                    Rc::new(Node(
                        Rc::new(Tip),
                        5,
                        Rc::new(Tip))))),
                6,
                Rc::new(Node(
                    Rc::new(Node(
                        Rc::new(Tip),
                        7,
                        Rc::new(Tip))),
                    8,
                    Rc::new(Node(
                        Rc::new(Tip),
                        9,
                        Rc::new(Tip)))))));

    assert!(t2.member(1));
    assert!(t2.member(9));
    assert!(t2.member(6));

    assert!(!t2.member(0));
    assert!(!t2.member(10));

    assert_eq!(t2.member(1), t2.member2(1));
    assert_eq!(t2.member(0), t2.member2(0));
}
