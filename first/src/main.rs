use std::io;
fn main() {
    let mut input = String::new();
    println!("Say something");
    match io::stdin().read_line(&mut input) {
        Ok(_)=>{
            println!("Here is {}",input);
        }
        Err(e)=>{
            println!("Error occured {}",e);
        }
    }

}
