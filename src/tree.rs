use std::cmp::max;
use std::fmt::{Display, Error, Formatter};
use std::num::Int;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree<T> {
    Tip,
    Node(Rc<Tree<T>>, T, Rc<Tree<T>>),
}

use tree::Tree::{Node, Tip};

impl<T: Display + PartialEq + Clone> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        fn depth<T>(t: &Tree<T>) -> usize {
            match *t {
                Tip => 0,
                Node(ref l, _, ref r) => 1 + max(depth(l), depth(r))
            }
        }

        fn print_whitespace(n: usize, f: &mut Formatter) -> Result<(), Error> {
            for _ in (0..n) {
                try!(write!(f, " "));
            }
            Result::Ok(())
        }

        fn aux<T: Clone + Display + PartialEq>(nodes: Vec<Tree<T>>, level: usize, max_level: usize, f: &mut Formatter) -> Result<(), Error> {
            if nodes.iter().all(|n| *n == Tip) { return Result::Ok(()) }

            let floor = max_level - level;
            let edge_lines = 2.pow(max(floor - 1, 0)) as usize;
            let first_spaces = 2.pow(floor) - 1 as usize;
            let between_spaces = 2.pow(floor + 1) - 1 as usize;

            try!(print_whitespace(first_spaces, f));

            let new_nodes =
                nodes.iter().fold(vec![], |mut acc, n| {
                    let ret =
                        match *n {
                            Node(ref l, ref v, ref r) => {
                                write!(f, "{}", v);
                                acc.push((**l).clone());
                                acc.push((**r).clone());
                                acc
                            },
                            Tip => {
                                write!(f, " ");
                                acc.push(Tip);
                                acc.push(Tip);
                                acc
                            }
                        };

                    print_whitespace(between_spaces, f);

                    ret
                });

            try!(writeln!(f, ""));

            for i in (1..(edge_lines + 1)) {
                for node in nodes.iter() {
                    try!(print_whitespace(first_spaces - i, f));
                    match *node {
                        Tip => { try!(print_whitespace(edge_lines + edge_lines + i + 1, f)); },
                        Node(ref l, _, ref r) => {
                            if **l != Tip { try!(write!(f, "/")); }
                            else { try!(write!(f, " ")); }

                            try!(print_whitespace(i + i - 1, f));

                            if **r != Tip { try!(write!(f, "\\")); }
                            else { try!(write!(f, " ")); }

                            try!(print_whitespace(edge_lines + edge_lines - 1, f));
                        }
                    }
                }

                try!(writeln!(f, ""));
            }

            aux(new_nodes, level + 1, max_level, f)
        }

        aux(vec![(*self).clone()], 1, depth(self), f)
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
