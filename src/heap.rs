use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

pub trait Heap<T: Ord> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn merge(&self, &Self) -> Self;
    fn insert(&self, T) -> Self;

    fn find_min(&self) -> T;
    fn delete_min(&self) -> Self;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeftistHeap<T> {
    Tip,
    Node(usize, T, Rc<LeftistHeap<T>>, Rc<LeftistHeap<T>>),
}

use heap::LeftistHeap::{Tip, Node};

impl<T: Ord + Clone> Heap<T> for LeftistHeap<T> {
    fn empty() -> LeftistHeap<T> {
        Tip
    }

    fn is_empty(&self) -> bool {
        match *self {
            Tip => true,
            _ => false
        }
    }

    fn merge(&self, h: &LeftistHeap<T>) -> LeftistHeap<T> {
        fn rank<T>(h: &LeftistHeap<T>) -> usize {
            match *h {
                Tip => 0,
                Node(r, _, _, _) => r
            }
        }

        fn make_node<T: Clone>(x: T, l: Rc<LeftistHeap<T>>, r: Rc<LeftistHeap<T>>) -> LeftistHeap<T> {
            if rank(&l) >= rank(&r) { Node(rank(&r) + 1, x.clone(), l.clone(), r.clone()) }
            else { Node(rank(&l) + 1, x.clone(), r.clone(), l.clone()) }
        }

        match (self, h) {
            (e, &Tip) => e.clone(),
            (&Tip, e) => e.clone(),
            (&Node(_, ref x, ref l1, ref r1), &Node(_, ref y, ref l2, ref r2)) => {
                if *x <= *y {
                    make_node(x.clone(), l1.clone(), Rc::new(r1.merge(h)))
                } else {
                    make_node(y.clone(), l2.clone(), Rc::new(self.merge(r2)))
                }
            }
        }
    }

    fn insert(&self, x: T) -> LeftistHeap<T> {
        let h = Node(1, x, Rc::new(Tip), Rc::new(Tip));
        h.merge(self)
    }

    fn find_min(&self) -> T {
        match *self {
            Tip => panic!("empty heap"),
            Node(_, ref x, _, _) => x.clone()
        }
    }

    fn delete_min(&self) -> LeftistHeap<T> {
        match *self {
            Tip => panic!("empty heap"),
            Node(_, _, ref l, ref r) => l.merge(r)
        }
    }
}

impl<T: Display> Display for LeftistHeap<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn aux<T: Display>(f: &mut Formatter, t: &LeftistHeap<T>, right: bool, indent: &str) -> Result<(), Error> {
            match *t {
                Node(ref rank, ref x, ref l, ref r) => {
                    try!(aux(f, r, true, &(indent.to_string() + if right { "        " } else { " |      " })));

                    try!(write!(f, "{}", indent));
                    try!(if right { write!(f, "{}", " /") } else { write!(f, "{}", " \\") });
                    try!(write!(f, "{}", "----- "));

                    try!(writeln!(f, "(#{}, {})", rank, x));

                    aux(f, l, false, &(indent.to_string() + if right { " |      " } else { "        " }))
                },
                Tip => {
                    try!(write!(f, "{}", indent));
                    try!(if right { write!(f, "{}", " /") } else { write!(f, "{}", " \\") });
                    try!(write!(f, "{}", "----- "));

                    writeln!(f, "{}", "()")
                }
            }
        }

        match *self {
            Node(ref rank, ref x, ref l, ref r) => {
                try!(aux(f, r, true, ""));
                try!(writeln!(f, "(#{}, {})", rank, x));
                aux(f, l, false, "")
            },
            Tip => Result::Ok(())
        }
    }
}

#[test]
fn leftistheap() {
    let h: LeftistHeap<usize> = Heap::empty();
    let h2: LeftistHeap<usize> = h.insert(10).insert(9).insert(8).insert(11).insert(1).insert(4);

    assert!(h.is_empty());
    assert!(!h2.is_empty());

    assert_eq!(h2.find_min(), 1);
    assert_eq!(h2.delete_min(), h.insert(10).insert(9).insert(8).insert(11).insert(4));
}
