extern crate okasaki;

use okasaki::stack::*;
use okasaki::stack::List::*;

use std::rc::Rc;

fn main() {
    let l: List<usize> = Stack::empty();
    let l2 = l.cons(3).cons(2).cons(1);

    println!("{:?}", l2);
    println!("{:?}", l2.tail());

    // exercise 2.1:
    // O(n) time and space
    fn suffixes(l: List<usize>) -> List<List<usize>> {
        match l {
            Nil => {
                let l: List<List<usize>> = Stack::empty();
                l.cons(Nil)
            }
            Cons(h, ref t) => Cons(Cons(h, (*t).clone()), Rc::new(suffixes((**t).clone())))
        }
    }

    println!("{:?}", suffixes(l.cons(4).cons(3).cons(2).cons(1)));

    println!("{}", suffixes(l.cons(4).cons(3).cons(2).cons(1)));
}
