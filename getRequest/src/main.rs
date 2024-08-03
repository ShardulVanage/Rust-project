use error_chain::error_chain;//This line imports the error_chain macro from the error_chain crate. This macro helps in creating custom error types and handling errors more easily.
use std::io::Read;//This imports the Read trait from Rust's standard library. We'll use this to read the response body into a string.

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
//^
//This block uses the error_chain! macro to define custom error types. 
//It's saying that our error type can wrap around std::io::Error (for I/O operations) and reqwest::Error (for HTTP request errors).

fn main() -> Result<()> { //This is the main function of our program. It returns a Result type, which is defined by the error_chain! macro. This allows us to use the ? operator for error handling.
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;//This line sends a GET request to "http://httpbin.org/get" using reqwest's blocking client. The ? at the end will return an error if the request fails.    
  
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    //^
    //These lines create an empty string called body, then read the entire response body into this string. Again, ? is used for error handling.

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
//These lines print out the status code of the response, all the headers (in a debug format), and the body of the response.

    Ok(())
    //This returns a successful result with no value (represented by ()). It indicates that our main function completed successfully.
}


// {}
// []
// _
// ""