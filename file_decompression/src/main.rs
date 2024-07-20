//These lines import the fs (filesystem) and io (input/output) modules from the standard library.
use std::fs;
use std::io;

fn main() {
    
    //process the real program in the real_main function
    //for clean end and exit use this in the main function
    std::process::exit(real_main());
}

fn real_main() -> i32 {

    let args: Vec<_> = std::env::args().collect(); //This collects command-line arguments into a vector.

    //This checks if there's at least one argument (the filename). If not, it prints usage instructions and returns 1 (indicating an error).
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*args[1]);//This creates a Path object from the first argument (the filename).

    let file = fs::File::open(&fname).unwrap();//This opens the file and unwraps the result (panics if there's an error).

    let mut archive = zip::ZipArchive::new(file).unwrap();//This creates a ZipArchive from the file.

    //This loops through each file in the archive, getting a mutable reference to each.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
      //This gets the enclosed name of the file, skipping to the next iteration if there isn't one.
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            //This prints the file's comment if it exists.
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        //This checks if the file is a directory. If so, it creates the directory.
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();

        } else {

            //If it's a file, this prints information about it, creates parent directories if needed, creates the output file, and copies the contents.
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

       //On Unix systems, this sets the file permissions to match those in the zip file.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    0
}