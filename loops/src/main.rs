use rand::Rng;
mod switch;
#[path = "loops/for_loop.rs"] mod for_loop;

fn main() {
    let mut num = rand::thread_rng();
    let mut number: i32 = num.gen();
    let ans = if number > 5 {true} else {false};
    println!("the answer is {}",ans); 
    switch::switch_statement(switch::Suit::Club);
    switch::switch_statement(switch::Suit::Spade);
    for_loop::forloop();
}


