use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| {
    	for i in 1..10 {
    		println!("hi number {} from the spawned thread", i);
    		thread::sleep(Duration::from_millis(1));
    	}
    });

    handle.join().unwrap();
    
    for i in 1..5 {
    	println!("hi numnber {} from the main thread", i);
    	thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || { // moves the ownership of v to de closure
    	println!("Here's a vector: {:?}", v);
    });

    handle2.join().unwrap();

    // Using message passing to transfer data between threads

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
    	let val = String::from("hi");
    	tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Channels and Ownership transference

    

}
