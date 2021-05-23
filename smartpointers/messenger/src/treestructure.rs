use std::cell::RefCell;
use sdt::rc::Rc;

#[derive(Debug)]
struct Node {
	value: i32,
	children: RefCell<Vec<Rc<Node>>>,
}