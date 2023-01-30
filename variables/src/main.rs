fn main() {
    let name = "Alex";
    let mut age = 32;
    println!("The name is {} and age is {}",name,age);
    age = 31;
    println!("The name is {} and age is {}",name,age);
    //shadowing is allowed in rust
    let age = 96;
    println!("the age is {}",age);

    //string slices are immutable in rust
    let cat:&str = "Fluffy";
    println!("{}",cat);

    let cat: &'static str = "Maxy";
    println!("{}",cat);

    say_hello();

    let mut name = "Sai";
    change_name(&mut name);
    println!("changed name is {}",name);
}

fn say_hello(){
    println!("Hello world");
}

fn change_name(name: &mut &str) {
    *name = "Siddartha";
    println!("Hello {}",name);
}
