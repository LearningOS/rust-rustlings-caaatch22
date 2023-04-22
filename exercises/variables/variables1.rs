// variables1.rs
// Make me compile!
// Execute `rustlings hint variables1` or use the `hint` watch subcommand for a hint.


enum Mes0<T> {
    Some(T),
    Quit,
}

enum Mes00 {
    Move(i32, i32),
}

enum Mes1 {
    Quit,
    Move(i32, i32),
}

enum Mes2 {
    Mess(String),
}

enum Mes3 {
    Quit,
    Move(i32, i32),
    Mess(String),
}

use std::mem;

fn main() {
    let x = 5;
    println!("x has the value {}", x);
    println!("{}", mem::size_of::<Option<i32>>());
    println!("{}", mem::size_of::<Mes00>());
    println!("{}", mem::size_of::<Mes2>());
    println!("{}", mem::size_of::<Mes1>());
    println!("{}", mem::size_of::<Mes3>());

    let s = "你a好";
    let mut it = s.chars();
    assert_eq!(it.next(), Some('你'));
    assert_eq!(it.next(), Some('a'));

    println!("{}", capitalize_first("nicdsca"));
    let a = [1, 2, 3];

    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);
    let sum2 = a.into_iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);

}


pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}
