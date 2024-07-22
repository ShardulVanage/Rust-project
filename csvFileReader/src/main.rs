use csv;
use std::error::Error;
use std::env;

fn readfromfile(path:&str)-> Result<(),Box<dyn Error>> {
    let mut reader = csv ::Reader::from_path(path)?;

    for result in reader.records(){
        let record = result?;
        println!("{:?}",record);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_csv_file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    readfromfile(file_path)?;

    Ok(())
}
// -
// fn main() {
//     if let Err(e) = readfromfile("./customers.csv"){
//         eprintln!("{}",e);
//     }
// }