pub fn first_vector(){

    let mut first:Vec<i32>= Vec::new();
    first = vec![2,3,4];
    println!("{:?}",first);
    first.push(10);
    println!("{:?}",first);
}