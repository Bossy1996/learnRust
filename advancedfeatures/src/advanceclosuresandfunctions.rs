fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

let list_of_numbers = vec![1, 2, 3, 4, 5];
let list_of_string = Vec<String> = list_of_numbers.iter().map(ToString::to_string).colllect();