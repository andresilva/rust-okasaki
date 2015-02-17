use std::fmt::{Display, Error, Formatter};
use std::rc::Rc;

pub trait Stack<T> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;

    fn append(&self, y: &Self) -> Self where Self: Clone + Sized {
        if self.is_empty() {
            (*y).clone()
        } else {
            self.tail().append(y).cons(self.head())
        }
    }

    fn update(&self, i: usize, x: T) -> Self where Self: Sized {
        if self.is_empty() {
            panic!("index out of bounds");
        }

        if i == 0 {
            self.tail().cons(x)
        } else {
            self.tail().update(i - 1, x).cons(self.head())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>)
}

use stack::List::{Cons, Nil};

impl<T: Clone> Stack<T> for List<T> {
    fn empty() -> List<T> {
        Nil
    }

    fn is_empty(&self) -> bool {
        match *self {
            Nil => true,
            _ => false
        }
    }

    fn cons(&self, x: T) -> List<T> {
        match *self {
            Cons(ref h, ref t) => Cons(x, Rc::new(Cons(h.clone(), t.clone()))),
            Nil => Cons(x, Rc::new(Nil))
        }
    }

    fn head(&self) -> T {
        match *self {
            Cons(ref h, _) => h.clone(),
            Nil => panic!("head of empty list")
        }
    }

    fn tail(&self) -> List<T> {
        match *self {
            Nil => panic!("tail of empty list"),
            Cons(_, ref t) => (**t).clone()
        }
    }
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "["));

        fn _loop<T: Display>(f: &mut Formatter, l: &List<T>, first: bool) -> Result<(), Error> {
            match *l {
                Cons(ref h, ref t) => {
                    if first { try!(write!(f, "{}", *h)); }
                    else { try!(write!(f, ", {}", *h)); }

                    _loop(f, t, false)
                },
                Nil => Result::Ok(())
            }
        }

        try!(_loop(f, self, true));

        write!(f, "]")
    }
}

#[test]
fn list() {
    let l1: List<usize> = Stack::empty();
    let l2 = l1.cons(3).cons(2).cons(1);
    let l3 = l1.cons(5).cons(4);

    assert!(l1.is_empty());
    assert!(!l2.is_empty());
    assert_eq!(l2, Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    assert_eq!(l2.head(), 1);
    assert_eq!(l2.tail(), Cons(2, Rc::new(Cons(3, Rc::new(Nil)))));

    assert_eq!(l1.append(&l2), l2);
    assert_eq!(l2.append(&l1), l2);

    assert_eq!(l2.append(&l3),
               Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::new(Nil)))))))))));

    assert_eq!(l3.append(&l2),
               Cons(4, Rc::new(Cons(5, Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))))))));

    assert_eq!(l3.update(0, 0),
               Cons(0, Rc::new(Cons(5, Rc::new(Nil)))));

    assert_eq!(l3.update(1, 0),
               Cons(4, Rc::new(Cons(0, Rc::new(Nil)))));
}
