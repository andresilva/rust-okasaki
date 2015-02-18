use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree<T> {
    Tip,
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
}

use tree::Tree::{Node, Tip};

impl<T: Display> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn _loop<T: Display>(f: &mut Formatter, t: &Tree<T>, right: bool, indent: &str) -> Result<(), Error> {
            match *t {
                Node(ref l, ref v, ref r) => {
                    try!(_loop(f, r, true, &(indent.to_string() + if right { "        " } else { " |      " })));

                    try!(write!(f, "{}", indent));
                    try!(if right { write!(f, "{}", " /") } else { write!(f, "{}", " \\") });
                    try!(write!(f, "{}", "----- "));

                    try!(writeln!(f, "{}", *v));

                    _loop(f, l, false, &(indent.to_string() + if right { " |      " } else { "        " }))
                },
                Tip => Result::Ok(())
            }
        }

        match *self {
            Node(ref l, ref v, ref r) => {
                try!(_loop(f, r, true, ""));
                try!(writeln!(f, "{}", *v));
                _loop(f, l, false, "")
            }
            Tip => Result::Ok(())
        }
    }
}

impl<T: Ord> Tree<T> {
    // Exercise 2.2:
    // only performs at most d + 1 comparisons, where d is the depth of the tree
    pub fn member2(&self, x: T) -> bool {
        match *self {
            Tip => false,
            Node(_, ref v, _) => {
                fn member_aux<T: Ord>(t: &Tree<T>, x: T, c: &T) -> bool {
                    match *t {
                        Tip => x == *c,
                        Node(ref l, ref v, ref r) =>
                            if x < *v {
                                member_aux(l, x, c)
                            } else {
                                member_aux(r, x, v)
                            }
                    }
                }

                member_aux(self, x, v)
            },
        }
    }
}
