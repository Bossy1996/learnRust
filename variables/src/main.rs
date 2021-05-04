fn main() {
  /*   let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("max points available {}", MAX_POINTS); */
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("the value of x is {}", x);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let sum = 5 + 10; // int

    let difference = 95.5 - 4.3; // Float f32

    let product = 4 * 30; // int u32
    
    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    println!("{}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    let _t = true; // _var to not used variables

    let _f: bool = false; // with explicit type notation.

    // tuples

    let _tup: (u32, f64, u8) = (500, 6.4, 1);

    let (x, y ,z) = _tup;
    
    println!("{}, {}, {}", x, y, z);

    let x: (u32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{},{},{}", one, six_point_four, five_hundred);
}
