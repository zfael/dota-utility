use enigo::{Enigo, Key, Keyboard, Settings, Direction::{ Press, Click }};
use tokio::time::{sleep, Duration};
use std::env;
use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The key to validate against the upstream
    #[arg(long)]
    key: String,

    /// The text to type
    #[arg(long)]
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let key_url = "https://gist.githubusercontent.com/zfael/ad6ede1ebce26dc58d887e2a99469c08/raw/mr-key.txt";
    let response = reqwest::get(key_url).await?;

    if !response.status().is_success() {
        println!("Unauthorized");
        return Ok(());
    } 

    let upstream_key = response.text().await?;
    if args.key != upstream_key.trim() {
        println!("Unauthorized");
        return Ok(());
    }

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    // Define the wait sequence
    let mut wait_sequence = vec![
        1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    ];

    loop {
        // Wait for 5 seconds
        println!("Initializing in 5 seconds...");
        sleep(Duration::from_secs(5)).await;

        // Simulate pressing "Enter"
        enigo.key(Key::Return, Click);

        // Wait for 1 second
        sleep(Duration::from_secs(1)).await;

        // Type the string
        enigo.text(&args.text);

        // Wait for 1 second
        sleep(Duration::from_secs(1)).await;

        // Simulate pressing "Enter" twice
        enigo.key(Key::Return, Click);
        sleep(Duration::from_secs(1)).await;
        enigo.key(Key::Return, Click);

        // Get the first value from the wait sequence
        let wait_for = wait_sequence.remove(0);

        println!("Wait for {} minutes...", wait_for);

        // Wait for the specified number of minutes
        sleep(Duration::from_secs(wait_for * 60)).await;

        // Push the value back to the end of the sequence
        wait_sequence.push(wait_for);
    }
}