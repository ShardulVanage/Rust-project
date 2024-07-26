use  serde::{Deserialize, Serialize};//This line imports the Deserialize and Serialize traits from the serde crate. These traits are used for converting Rust structs to and from JSON.

//This defines a Paragraph struct with a single name field. The #[derive(...)] attribute automatically implements the Serialize and Deserialize traits for this struct.
#[derive(Serialize,Deserialize)]
struct Paragraph{
    name: String
}
//This defines an Article struct with fields for article, author, and a vector of Paragraphs. It also derives Serialize and Deserialize.
#[derive(Serialize,Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph:Vec<Paragraph>
} 


fn main() {
    //This is the start of the main function. It defines a raw string literal (r#"..."#) containing a JSON object.
     let json = r#" 
{
  "article": "how to work with json in Rust",
  "author": "akhil",
  "paragraph": [
    {
      "name": "starting sentences"
    },
    {
      "name": "body of the paragraph"
    },
    {
      "name": "end of the paragraph"
    }
  ]
}
"#;
//This line calls the read_json_typed function with the JSON string and assigns the result to parsed.
let parsed: Article = read_json_typed(json);


for paragraph in &parsed.paragraph {
    println!("\n\n {}", paragraph.name);
}


println!("\n\n The name of the first paragraph is: {}",parsed.paragraph[0].name);
}

//This function takes a JSON string, uses serde_json::from_str to parse it into an Article struct, and returns the result. The unwrap() call will panic if parsing fails.
fn read_json_typed(raw_json: &str)-> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed
}
//  {}
// []
// _




// Certainly. Let's break down the #[derive(Serialize,Deserialize)] line in detail:

// #[...] is an attribute in Rust. Attributes are used to add metadata to items in Rust code.
// derive is a special built-in attribute in Rust that automatically implements traits for a struct or enum.
// Serialize and Deserialize are traits from the serde library:

// Serialize allows a Rust data structure to be converted into a serialized format (like JSON).
// Deserialize allows a serialized format to be converted back into a Rust data structure.


// By using #[derive(Serialize,Deserialize)], you're telling the Rust compiler to automatically generate implementations of these traits for your struct.
// This automatic implementation saves you from having to manually write the code to convert your struct to and from JSON.
// The derive macro expands at compile time to generate the necessary code for serialization and deserialization.
// This approach is declarative - you're describing what you want (serialization and deserialization capabilities) rather than how to do it.
// It's important to note that for this to work, all fields in the struct must also implement Serialize and Deserialize. Built-in types like String and Vec already implement these traits.