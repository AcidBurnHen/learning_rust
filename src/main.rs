// --------------------------- //
// Reference Cycles
// --------------------------- //

// Memory leaks can occur when using Rc and the RefCell smart pointer for mutability

use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}
fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));

    println!("A count: {:?}", Rc::strong_count(&a));

    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::clone(&a)),
    }));
    println!("B is created: \n a count: {:?}", Rc::strong_count(&a));
    println!("B count: {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::clone(&b)),
    }));

    (*a).borrow_mut().next = Some(Rc::clone(&c));
}
