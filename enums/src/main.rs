fn main() {
    println!("{:?}",Color::Red);
    println!("{:?}",Person::Name(String::from("siddarth")));
}

#[derive(Debug)]
enum Color{
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum Person{
    Name(String),
    SurName(String),
    Age(u32)
}