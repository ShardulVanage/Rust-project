extern crate flate2; //This line declares that the program will use the external flate2 crate, which provides compression and decompression functionalities.
use flate2::write::GzEncoder;//
use flate2::Compression;//These lines import specific items from the flate2 crate: GzEncoder for GZIP compression and Compression for specifying compression levels.
use std::env::args;//args for accessing command-line arguments
use std::fs::File;//File for file I/O operations
use std::io::copy;//copy for copying data between readers and writers
use std::io::BufReader;//BufReader for buffered reading
use std::time::Instant; // Instant for measuring elapsed time

fn main(){
    //This block checks if the number of command-line arguments is exactly 3
    //  (program name, source file, target file). If not, it prints a usage message to stderr and exits the program.
    if args().len()!=3{
        eprintln!("usage:`source` `target`");
        return;
    }
    //This line opens the source file (second command-line argument) and wraps it in a BufReader for efficient reading.
    let mut input = BufReader::new (File::open(args().nth(1).unwrap()).unwrap());
    //This line creates the target file (third command-line argument) for writing.
    let output = File::create(args().nth(2).unwrap()).unwrap(); 
    //This line creates a new GzEncoder with default compression level, which will write compressed data to the output file.
    let mut encoder = GzEncoder::new(output,Compression::default());
   //This line records the start time of the compression process.
    let start = Instant::now();
    //his line copies all data from the input file to the encoder, compressing it in the process.
    copy(&mut input,&mut encoder).unwrap();
    //This line finishes the compression and returns the underlying writer (the output file).
    let output = encoder.finish().unwrap();
    //This line prints the length of the source file.
    println!(
        "source len:{:?}",
        input.get_ref().metadata().unwrap().len()
    );
    //This line prints the length of the compressed (target) file.
    println!("Target len{:?}",output.metadata().unwrap().len());
    //This line prints the time taken for the compression process.
    println!("Elapsed:{:?}",start.elapsed());

}

//how to run the code
//cargo run xyz.pdf comp_xyz.pdf
