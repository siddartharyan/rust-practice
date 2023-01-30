mod arrays;
mod vectors;
mod slices;
mod tuples;
fn main() {
    println!("Hello, world!");
    println!("{}",arrays::first_array());
    arrays::second_array();
    vectors::first_vector();
    slices::first_slice();
    tuples::first_tuple();

    let emp = Employee{
        name:String::from("siddartha"),
        age:21,
        company:String::from("dummy")
    };
    println!("{:?}",emp);
    println!("{:?}",emp.fn_detail());
    println!("{:?}",Employee::static_fn_detail());
}

#[derive(Debug)]
struct Employee{
    name: String,
    age: u32,
    company: String
}
impl Employee {
    fn fn_detail(&self) -> String{
        return format!("name : {}, age: {}, company:{}",self.name,self.age,self.company);
    }

    fn static_fn_detail() -> String{
        return String::from("Dummy Details");
    }
}