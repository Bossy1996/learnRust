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

fn string() {
    // let mut s = String::new(); // Creates a new empty String.
    let data = "Initial contents";
    let s = data.to_string();
     
    // Strings are UTF-8 encoded, so we can include any properly encoded data in them.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let s1 = "Hello, ".to_string();
    let s2 = "world!".to_string();
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}