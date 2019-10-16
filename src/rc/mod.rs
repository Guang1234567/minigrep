use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List_ref_version<'a, T> {
    Cons(T, &'a Box<List_ref_version<'a, T>>),
    Nil,
}

#[derive(Debug)]
enum List_rc_version<T> {
    Cons(T, Rc<List_rc_version<T>>),
    Nil,
}

#[derive(Debug)]
enum List_rc_ref_version<T> {
    Cons(Rc<RefCell<T>>, Rc<List_rc_ref_version<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_list_ref_version() {
        use crate::rc::List_ref_version::{Cons, Nil};
        /*
         let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
         let b = Cons(3, Box::new(a));
         let c = Cons(4, Box::new(a));
         */
        let tail = Box::new(Nil);
        let a = Cons(10, &tail);
        let tail = &Box::new(a);
        let b = Cons(3, &tail);

        println!("b = {:?}", b);

        assert_eq!(
            vec!["Rust:", "Trust me."],
            vec!["Rust:", "Trust me."]
        );
    }


    #[test]
    fn test_list_rc_version() {
        use crate::rc::List_rc_version::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        assert_eq!(
            vec!["Rust:", "Trust me."],
            vec!["Rust:", "Trust me."]
        );
    }


    #[test]
    fn test_list_rc_ref_version() {
        use crate::rc::List_rc_ref_version::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(77)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(88)), Rc::clone(&a));

        println!("before b = {:?}", b);
        println!("before c = {:?}", c);

        *value.borrow_mut() += 10;

        println!("after b = {:?}", b);
        println!("after c = {:?}", c);

        assert_eq!(
            vec!["Rust:", "Trust me."],
            vec!["Rust:", "Trust me."]
        );
    }
}