mod msg {
    pub fn trim(msg:&str)->&str {
        msg.trim()
    }
}
mod math{
    pub fn add(lhs:isize,rhs:isize)->isize {
        lhs+rhs
    } 
}




fn main(){
 let result ={
    let twoplustwo = math::add(2,2);
 };
  {
    use msg::{trim};

    let hello={
        let msg = "hello ";
        let msg = trim(msg);
    };
  }
 
}



//  {}
// []
// _
// ""
//-
   