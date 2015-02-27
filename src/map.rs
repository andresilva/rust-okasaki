use std::rc::Rc;

use tree::Tree;
use tree::Tree::{Node, Tip};

pub trait Map<K, V> {
    fn empty() -> Self;
    fn bind(&self, K, V) -> Self;
    fn lookup(&self, K) -> V;
}

impl<K: Ord + Clone, V: Clone> Map<K, V> for Tree<(K, V)> {
    fn empty() -> Tree<(K, V)> {
        Tip
    }

    fn bind(&self, k: K, v: V) -> Self {
        match *self {
            Tip =>
                Node(Rc::new(Tip), (k, v), Rc::new(Tip)),
            Node(ref l, (ref k1, ref v1), ref r) if k < *k1 =>
                Node(Rc::new(l.bind(k, v)), (k1.clone(), v1.clone()), r.clone()),
            Node(ref l, (ref k1, ref v1), ref r) if k > *k1 =>
                Node(l.clone(), (k1.clone(), v1.clone()), Rc::new(r.bind(k, v))),
            _ =>
                self.clone()
        }
    }

    fn lookup(&self, x: K) -> V {
        match *self {
            Tip => panic!("element does not exist"),
            Node(ref l, (ref k, _), _) if x < *k => l.lookup(x),
            Node(_, (ref k, _), ref r) if x > *k => r.lookup(x),
            Node(_, (_, ref v), _) => v.clone()
        }
    }
}

#[test]
fn treemap() {
    let m: Tree<(&str, usize)> = Map::empty();
    let m2 = m.bind("hello", 0)
        .bind("world", 1)
        .bind("foo", 2)
        .bind("bar", 3);

    assert_eq!(m2.lookup("hello"), 0);
    assert_eq!(m2.lookup("world"), 1);
    assert_eq!(m2.lookup("foo"), 2);
    assert_eq!(m2.lookup("bar"), 3);
}
