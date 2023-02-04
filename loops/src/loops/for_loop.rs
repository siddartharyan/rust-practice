pub fn forloop(){
    for i in 1..32 {
        println!("{0} * {0} is {1}",i,i*i);
    }

    let arr = ["cat","dog","cow","hen"];

    for i in 0..arr.len(){
        if i % 2 == 0{
            println!("{} is {}",i,arr[i]);
        }
    }
    while_loop();
}


pub fn while_loop(){

    let mut index = 1;

    while index <= 32{
        if index %4 == 0{
            println!("{}",index);
        }
        index += 1; 
    }
}