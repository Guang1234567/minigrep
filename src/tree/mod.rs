use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    value: T,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 2);

        let leaf = Rc::new(Node { value: 77, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]) });

        println!("leaf.parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 66,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

            println!("leaf.parent = {:?}", leaf.parent.borrow().upgrade());
        }

        println!("leaf.parent = {:?}", leaf.parent.borrow().upgrade());
    }
}