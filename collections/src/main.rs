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

fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_score = vec![10, 50];

    // HashMap <_, _> is needed here because it's possible to collect into many different data structures
    // and Rust doesn't know which you want unless you specify.
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_score.iter()).collect();

    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Overwritting a value
    // scores.insert(String::from("Blue"), 25);

    //Only inserting a Value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}