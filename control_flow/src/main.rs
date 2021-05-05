fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    }else {
        println!("condition was false");
    }

    let array = ["hola", "agur", "hasta luego"];

    for element in array.iter() {
        println!("the content of the array is {}", element);
    }

    for num in (0..4).rev() {
        println!("{}", num);
    }
}
