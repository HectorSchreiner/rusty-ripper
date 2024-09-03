use std::{error::Error, path::Path};

mod components;
mod http_attack;

use http_attack::http_post_attack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("/home/hector/Programming/Rust/rusty-ripper//src/wordlist.txt");
    http_post_attack("https://google.com", path, "username").await?;    
    Ok(())
}
