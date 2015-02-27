extern crate okasaki;

use std::rc::Rc;

use okasaki::heap::*;
use okasaki::map::*;
use okasaki::set::*;
use okasaki::stack::*;
use okasaki::stack::List::*;
use okasaki::tree::Tree;
use okasaki::trie::PatriciaTrie;

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

fn heap() {
    let h: LeftistHeap<usize> = Heap::empty();
    let h2: LeftistHeap<usize> = h.insert(10).insert(9).insert(8).insert(11).insert(1).insert(4).insert(12);

    println!("{:?}", h2);
    println!("{}", h2);
}

fn trie() {
    let t: PatriciaTrie<usize> = Map::empty();
    let t2 = t.bind("test".to_string(), 0)
        .bind("slow".to_string(), 1)
        .bind("water".to_string(), 2)
        .bind("slower".to_string(), 3)
        .bind("tester".to_string(), 4)
        .bind("te".to_string(), 5);

    println!("{:?}", t2);
    println!("{}", t2);

    let t3 = t.bind("test".to_string(), 0)
        .bind("team".to_string(), 1);

    println!("{:?}", t3);
    println!("{}", t3);

    let t4 = t3.bind("toast".to_string(), 3);

    println!("{:?}", t4);
    println!("{}", t4);

    let t5 = t2.bind("toast".to_string(), 6)
        .bind("toad".to_string(), 7);

    println!("{:?}", t5);
    println!("{}", t5);
}

fn main() {
    list();
    tree();
    map();
    heap();
    trie();
}
