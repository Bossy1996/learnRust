fn main() {
    println!("Hello, world!");
    another_function(5);
    let x = five();

    let _x = 5;

    let y = {
        let x = 3;
        x + 1

    };

    println!("the value of y is: {}", y);
    println!("The value of x is {}", x);
}

fn another_function(x: i32){
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}