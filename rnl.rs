//Remove New Lines
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];
    let output_filename = &args[2];
    
    let mut file = File::open(input_filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_contents = contents.replace("\n", "");

    fs::write(output_filename, new_contents)?;

    Ok(())
}
