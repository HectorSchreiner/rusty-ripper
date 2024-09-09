use std::{error::Error, path::Path};

mod components;
mod http_attack;
mod cli;

use components::{Log, LogType, Logger};
use http_attack::http_post_attack;
use cli::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::new();
    println!("{:?}", &cli);
    run(&cli).await?;
    Ok(())
}

async fn run(cli: &Cli) -> Result<(), Box<dyn Error>> {
    match &cli.mode {
        Mode::HTTP => {
            http_post_attack(&cli.address, Path::new(&cli.password_wordlist.to_string()), &cli.username).await?;
        }
        _ => { 
            let mut logger = Logger::new();
            logger.log(Log::new("Error".to_string(), LogType::Error, true))
        }
    }
    Ok(())
}

