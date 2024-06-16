use std::io;
use rand::prelude::*;



fn main()
 { 
    let guess_list = ["apple","banana","grapes","orange"];
    let mut rng = thread_rng(); //it is function which grnerate random number from rand::prelude

    let index = rng.gen_range(0..guess_list.len());
    let random_friut = guess_list[index];

println!("random fruit :{}",random_friut);// hmm

    let mut input =String::new();
    loop
    {
        match io::stdin().read_line(&mut input)
        {
           Ok(_)=>
           {
               let fruit_selected = input.trim().to_lowercase();
               println!("fruit selected :{}",fruit_selected);
               if !guess_list.contains(&&fruit_selected.as_str()) {
                   println!("fruit entered does not match");
                   continue;
               }
               if guess_checker(&fruit_selected,random_friut){
                   println!("you are winner");
                   break;
               }else {
                   println!("Retry")
               }
           }
           Err(err)=>
           {
                   println!("Error:{}",err);
           }
       }
    }
   
}

fn guess_checker(fruit_selc:&str,random_friut_selc:&str)->bool {
    return fruit_selc == random_friut_selc;
}