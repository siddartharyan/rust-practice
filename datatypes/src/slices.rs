pub fn first_slice(){
    let mut arr1 = ["a","b","c","d","e"];
    solve(&mut arr1[1..4]);
    println!("{:?}",arr1);
}

pub fn solve(slice: &mut [&str]){
    slice[1]="s";
}