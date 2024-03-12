#![allow(unused)]

mod drop_trait;
mod refcell;
mod refence_counted;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    #[test]
    fn using_box() {
        let b = Box::new(5);
        assert_eq!(5, *b);
    }

    #[test]
    fn recursive_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn test_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn test_deref_with_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn teste_deref_with_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn teste_deref_coercions() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}
