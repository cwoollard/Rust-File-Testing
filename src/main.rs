// This code came from Stackoverflow which is covered by an MIT License.
// Written by: Vladimir Matveev https://stackoverflow.com/users/788207/vladimir-matveev
// https://stackoverflow.com/questions/27215396/how-to-replace-a-word-in-a-file-in-a-txt/27218171?noredirect=1#comment78149771_27218171


use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    // Handle errors
    run().unwrap();
}

fn run() -> Result<(), io::Error> {
    // Extract words and file path from the command line args
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 3 {
        println!("Wrong number of arguments");
        return Ok(());
    }

    let word_from = &args[0];
    // If the source word is empty then there is nothing to replace
    if word_from.is_empty() { return Ok(()); }

    let word_to = &args[1];

    let file_name = &args[2];
    let file_path = Path::new(&file_name);

    // Open and read the file entirely
    let mut src = File::open(&file_path)?;
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    drop(src);  // Close the file early

    // Run the replace operation in memory
    let new_data = data.replace(&*word_from, &*word_to);

    // Recreate the file and dump the processed contents to it
    let mut dst = File::create(&file_path)?;
    dst.write(new_data.as_bytes())?;

    println!("done");

    Ok(())
}

