type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x, y);

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}

use sdt::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[i8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[i8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

// The never types that never returns
fn bar() -> ! {
    // --snip--
}

let guess: u32 = match guess.trim().parse() {
    Ok(_) = 5,
    Err(_) = "hello",
}