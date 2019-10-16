use std::cell::RefCell;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum ListMemoryLeak<T> {
    Cons(T, RefCell<Rc<ListMemoryLeak<T>>>),
    Nil,
}

impl<T> ListMemoryLeak<T> {
    fn tail(&self) -> Option<&RefCell<Rc<ListMemoryLeak<T>>>> {
        match self {
            ListMemoryLeak::Cons(_, item) => Some(item),
            ListMemoryLeak::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*, List::Cons, List::Nil};

    #[test]
    fn it_works() {
        assert_eq!(2, 2);

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, a.clone());
        let c = Cons(4, Rc::clone(&a));
    }

    #[test]
    fn it_works2() {
        use super::ListMemoryLeak::{Cons, Nil};
        assert_eq!(2, 2);

        let a = Rc::new(Cons(77, RefCell::new(Rc::new(Nil))));
        let b = Rc::new(Cons(88, RefCell::new(Rc::clone(&a))));

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // 取消如下行的注释来观察引用循环；
        // 这会导致栈溢出
        //println!("a next item = {:?}", a.tail());
    }
}