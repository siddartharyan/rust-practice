pub fn switch_statement(parameter: Suit){
    match parameter {
        Suit::Diamond => {println!("Diamond")}
        Suit::Club => {println!("Club")}
        Suit::Heart => {println!("Heart")}
        Suit::Spade => {println!("Spade")}
    }
}


pub enum Suit{
    Diamond,
    Heart,
    Spade,
    Club
}