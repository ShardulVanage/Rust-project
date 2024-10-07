use std::collections::HashMap;

 fn main(){
   let mut furniture = HashMap::new();
   furniture.insert("chair",5);
   furniture.insert("bed",3);
   furniture.insert("Table",2);
   furniture.insert("Couch",0);

   let mut totalstock =0;
   
for(item,qty) in furniture.iter() {
    totalstock =totalstock +qty;
    let stockcount = if qty == &0 {
        "out of stock".to_owned()
    }else{
        format!("{:?}",qty)
    };

    println!("item :{:?}, stock :{:?}",item,stockcount);
}
println!("total stick = {:?}",totalstock);
 }


//  {}
// []
// _
//""
//-




// #[derive(Debug)]
// struct Content {
//     content:String,
// }

// fn main() {
//     let mut lockers = HashMap::new();
//     lockers.insert(1,Content {content:"stuff".to_owned()});
//     lockers.insert(2,Content {content:"shirt".to_owned()});
//     lockers.insert(3,Content {content:"cloth".to_owned()});

//     for (lockernumber,x) in lockers.iter() {
//     println!("number: {:?}, content: {:?}",lockernumber,x.content);
//     }
// }