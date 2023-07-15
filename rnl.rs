//Remove New Lines
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let new_contents = contents.replace("\n", "");

    println!("{}", new_contents);

    Ok(())
}
