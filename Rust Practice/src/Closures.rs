
//example of function

fn addfb(a:i32,b:i32)->i32 {
    a+b
}// we dont have to write return bcz rust is smart to do that

main(){
    //this is closures
    let add = |a:i32,b:i32|-> {
        a+b
    } 
    //this is also closures
    let add = |a,b|a+b;
    
    let sum = add(1,1);
}
//  {}
// []
// _
//""
//-


