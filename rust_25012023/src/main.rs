use rand::Rng;

use crate::archive::arch::print_file;
mod player;
mod archive;
fn main() {
    println!("Hello, world!");
    say_hi();
    player::play_movie("avengers");
    print_file("a big file");
    let mut rng = rand::thread_rng();
    let number:i32 = rng.gen();
    println!("the random number is: {}",number);
}

fn say_hi(){
    println!("Hello siddartha");
}