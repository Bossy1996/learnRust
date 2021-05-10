/* fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let vector = vec!(1, 2, 3);
    println!("{:?}", vector);
} */

fn main() {
/*     let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8); */
    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    println!("{}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let ve = vec![1, 2, 3, 4, 5];
/* 
    let does_not_exist = &ve[100];
    let does_not_exist = v.get(100);
 */
    for i in &ve {
        println!("{}", i);
    }

    let mut vec = vec![1, 2, 3, 4, 5];

    for i in &mut vec {
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}