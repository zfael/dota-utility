use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> io::Result<()> {

    println!("Initiating in 5 seconds...");
    // Wait for 5 seconds
    sleep(Duration::from_secs(5));

    // Specify the file path
    let path = "codes.txt";

    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Create a clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // Iterate over each line in the file
    for line in reader.lines() {
        let code = line?;
        
        // Wait for 4 seconds
        sleep(Duration::from_secs(4));


        // Set the clipboard content to the current code
        let mut enigo = enigo::Enigo::new("argument");
        enigo.key_sequence_parse(&code);
    }

    Ok(())
}