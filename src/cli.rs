use clap::{Arg, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn cli() {
    let matches = Command::new("Rusty-Ripper")
        .version("1.0")
        .author("Hector Schousbo")
        .about("Command line Bruteforce written in Rust")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .required(true)
                .num_args(1)
                .help("Specify attack url ex. https://www.example.com")
        ).arg(
            Arg::new("username")
                .short('l')
                .long("username")
                .required(true)
                .num_args(1)
                .help("Specify username ex. admin")
        ).arg(
            Arg::new("password-wordlist")
                .short('P')
                .long("password-wordlist")
                .required(true)
                .num_args(1)
                .help("Specify path for wordlist ex. /etc/wordlists/rockyou.txt")
        ).get_matches();

    let url = matches.get_one::<String>("url").unwrap();
    let username = matches.get_one::<String>("username").unwrap();
    let password_wordlist_path = matches.get_one::<String>("password-wordlist").unwrap();
}