
struct Customer {
    age: Option<i32>,
    email:String,
}
fn main() {
let mark =Customer{
    age:Some(22),email:"myz@gmail.com".to_owned(),
};

let becky =Customer{
    age:None,email:"bsz@gmail.com".to_owned(),
};

match becky.age {
    Some(age)=>println!("Customer is {} year old",age),
    None=> println!("customer age not provided"),
}
}
//  {}
// []
// _
//""
