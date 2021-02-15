use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    next: Option<Rc<Node<T>>>,
    prev: RefCell<Option<Weak<Node<T>>>>,
    data: T,
}


impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }
    fn append(&self, data: T) -> Self {
        let new_node = Rc::new(Node {
            next: self.head.clone(),
            prev: RefCell::new(None),
            data,
        });

        match self.head.clone() {
            Some(node) => {
                let mut prev = node.prev.borrow_mut();
                *prev = Some(Rc::downgrade(&new_node));
            }
            None => {}
        }
        Self { head: Some(new_node) }
    }
}

#[test]
fn test() {
    let list_of_nums = LinkedList::new().append(1).append(2).append(3);
    println!("nums: {:?}", list_of_nums);
}