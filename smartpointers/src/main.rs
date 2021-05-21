fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn deref_trait() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
