use clap::{Arg, ArgGroup, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::components::{Log, LogType, Logger};
use crate::http_attack::http_post_attack;

#[derive(Debug)]
pub struct Cli {
    pub address: String,
    pub username: String,
    pub password_wordlist: String,
    pub mode: Mode
}
impl Cli {
    pub fn new() -> Self {
        let matches = Command::new("Rusty-Ripper")
            .version("1.0")
            .author("Hector Schousbo")
            .about("Command line Bruteforce written in Rust")
            .arg(
                Arg::new("address")
                    .short('a')
                    .long("address")
                    .required(true)
                    .num_args(1)
                    .help("Specify attack url or ip ex. https://www.example.com or 10.0.0.0")
            ).arg(
                Arg::new("username")
                    .short('l')
                    .long("username")
                    .num_args(1)
                    .help("Specify username ex. admin")
            ).arg(
                Arg::new("password-wordlist")
                    .short('P')
                    .long("password-wordlist")
                    .required(true)
                    .num_args(1)
                    .help("Specify path for wordlist ex. /etc/wordlists/rockyou.txt")
            ).arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .num_args(1)
                    .help("Specify type of attack mode ex. http")
            ).get_matches();

        let address = matches.get_one::<String>("address").unwrap().to_string();
        let username = matches.get_one::<String>("username").unwrap().to_string();
        let password_wordlist = matches.get_one::<String>("password-wordlist").unwrap().to_string();
        let mode_str = matches.get_one::<String>("mode").unwrap();

        let mode: Mode = match mode_str.as_str() {
            "http" => Mode::HTTP,
            "ssh" => Mode::SSH,
            _ => {
                println!("Unknown attack mode: {}", mode_str);
                std::process::exit(1);
            }
        };

        Cli { address, username, password_wordlist, mode }
    }

}

#[derive(Debug)]
pub enum Mode {
    HTTP,
    SSH,
}