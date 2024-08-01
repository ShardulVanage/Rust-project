use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct Article  {
    artical:String,
    author:String,
    paragraph:Vec<Paragraph>
}

#[derive(Serialize,Deserialize)]
 struct Paragraph{
    name:String,
 }


fn main() {
    let artical:Article =Article{
        artical:String::from("how to work with Jsonin Rust"),
        author:String::from("Shardul"),
        paragraph:vec![
            Paragraph{
                name:String::from("something in paragraph is this")
            },
            Paragraph{
                name:String::from("something in paragraph 2 is this")
            },
            Paragraph{
                name:String::from("something in paragraph 3 is this")
            }
        ]
    };
    let json= serde_json::to_string(&artical).unwrap();
    println!("this is json:{} ",json)
}
//  {}
// []
// _
//""
