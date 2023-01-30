fn main() {
    println!("Hello, world!");

    let p1: Point<i32> = Point {x:21,y:32};
    println!("{:?}",p1);
    let p2: Point<f32> = Point { x: 21.7882, y: 526.82992};
    println!("{:?}",p2);
    println!("{:?}",Color::Red(10));
    println!("{:?}",Color::Green(20));
    let p3: Point2<i32,f32> = Point2 { x: 100, y: 100.8992};
    println!("{:?}",p3);
}

#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T
}


#[derive(Debug)]
enum Color<T>{
    Red(T),
    Green(T),
    Blue(T)
}

#[derive(Debug)]
struct Point2<T,V>{
    x:T,
    y:V
}