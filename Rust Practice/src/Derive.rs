#[derive(Debug)]
enum Posotion {
    Manger,
    Supervisor,
    Worker,
}
#[derive(Debug)]
struct Employee {
    postion:Posotion,
    workhours:i64,

}

fn main() {
let me = Employee {
    postion: Posotion::Manger,
    workhours:23,
};
println!("{:?}",me)
}
//  {}
// []
// _
//""
   