extern crate okasaki;

use std::rc::Rc;

use okasaki::map::*;
use okasaki::set::*;
use okasaki::stack::*;
use okasaki::stack::List::*;
use okasaki::tree::Tree;

fn list() {
    let l: List<usize> = Stack::empty();
    let l2 = l.cons(3).cons(2).cons(1);

    println!("{:?}", l2);
    println!("{:?}", l2.tail());

    // Exercise 2.1:
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

fn tree() {
    let t: Tree<usize> = Set::empty();
    let t2 = t.insert(6).insert(8).insert(9).insert(7)
        .insert(4).insert(5).insert(1);

    println!("{:?}", t2);
    println!("{}", t2);
}

fn map() {
    let m: Tree<(String, usize)> = Map::empty();
    let m2 = m.bind("hello".to_string(), 0)
        .bind("world".to_string(), 1)
        .bind("foo".to_string(), 2)
        .bind("bar".to_string(), 3);

    println!("{:?}", m2);
}

fn main() {
    list();
    tree();
    map();
}
