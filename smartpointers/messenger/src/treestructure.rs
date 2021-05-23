use std::cell::RefCell;
use sdt::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
	value: i32,
	children: RefCell<Weak<Node>>>,
	parent: RefCell<Vec<Rc<Node>>>,
}

fn tree() {
	let leaf = Rc::new(Node {
		value: 3,
		children: RefCell::new(vec![]),
		parent: RefCell::new(Weak::new()),
	});

	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

	let branch = RefCell::new(Node {
		value: 5,
		children: RefCell::new(vec![Rc::clone(&leaf)]),
		parent: RefCell::new(Weak::new()),
	});

	*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}