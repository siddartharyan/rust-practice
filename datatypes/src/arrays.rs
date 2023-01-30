pub fn first_array() -> &'static str {
    return "Siddartha";
}

pub fn second_array(){
    let arr = [2,3,4,3,5,6];
    for number in arr.iter(){
        println!("value is :{}",number);
    }

    let mut arr1 = [20;10];
    arr1[1]=0;
    for num in arr1.iter(){
        println!("The value is: {}",num);
    }
}