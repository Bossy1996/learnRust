fn main() {
                    // s is not valid here, it's not yet declared    
   let mut s = String::from("hello"); // s is valid from this point forward

   // do stuff with s
   s.push_str(", world!"); // push_str() appends a literal to a string
   println!("{}", s); // this would print `hello, world!`
                    // this scope is now over, and s is no longer valid

    let s1 = String::from("hello"); // s1 come into scope

    takes_ownership(s1); // s1's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterwards

    let something = gives_ownership(); // give_ownership moves its return value into something

    let somthing_s2 = String::from("hello"); // something_s2 comes into scope
    
    let something_s3 = takes_and_gives_back(somthing_s2); // something_s2 is moved into
                                                         // takes_and_gives, which also
                                                         // moves its return value into something_s3
    
    println!("{}, somhting_s2 is moves to s3, {}", something, something_s3);
} // Here, x goes out of the scope, then s1. But beacuse s1's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String { // gives_ownership will move its return value into
                                 // the function that calls it
    let some_string = String::from("hello"); // some string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// takes_and_give_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_ string come into scope
    a_string // a_string is returned and moves out to the calling function
}