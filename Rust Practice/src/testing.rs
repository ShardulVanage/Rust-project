fn all_caps(words:&str)->String{
    words.to_uppercase()
}
fn main(){

}


#[cfg(test)]
mod test{
    use crate::*;
 
    #[test]
    fn check_all_caps(){
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result,expected,"string should be all uppercase");
    }
}
//  {}
// []
// _
// ""
//-