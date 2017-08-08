use std::os;
use std::io::File;

fn main() {
    // Extract words and file path from command line args
    let args = os::args();
    if args.len() != 4 {
        println!("Wrong number of arguments");
        return;
    }

    let mut args = args.into_iter().skip(1);

    let word_from = args.next().unwrap();
    // If source word is empty then there is nothing to replace
    if word_from.is_empty() { return; }  

    let word_to = args.next().unwrap();

    let file = Path::new(args.next().unwrap());

    // Open and read the file entirely
    let mut src = File::open(&file).unwrap();
    let data = src.read_to_string().unwrap();
    drop(src);  // Close the file early

    // Run replace operation in memory
    let new_data = data.replace(&*word_from, &*word_to);

    // Recreate the file and dump the processed contents to it
    let mut dst = File::create(&file).unwrap();
    dst.write_str(&*new_data).unwrap();

    println!("done");
}
