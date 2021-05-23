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
    // The code below won't compile because the val ownership has been transfered
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);


    // Sending multiple values and seeing the reciever wait

    let (request, response) = mpsc::channel();

    thread::spawn(move || {
    	let vals = vec![
    		String::from("hi"),
    		String::from("from"),
    		String::from("the"),
    		String::from("thread"),
    	];

    	for val in vals {
    		request.send(val).unwrap();
    		thread::sleep(Duration::from_secs(1));
    	}
    });

    for received in response {
    	println!("Got {}", received);
    }

    // Creating multiple producers by cloning the transmitter

    let (request2, response2) = mpsc::channel();

    let request2_1 = request2.clone();
    thread::spawn(move || {
    	let vals = vec![
    		String::from("hi"),
    		String::from("from"),
    		String::from("the"),
    		String::from("thread"),
    	];

    	for val in vals {
    		request2.send(val).unwrap();
    		thread::sleep(Duration::from_secs(1));
    	}
    });

    thread::spawn(move || {
    	let vals = vec![
    		String::from("more"),
    		String::from("messages"),
    		String::from("for"),
    		String::from("you"),
    	];

    	for val in vals {
    		request2.send(val).unwrap();
    		thread::sleep(Duration::from_secs(1));
    	}
    });

    for received in response2 {
    	println!("Got: {}", received);
    }
}
