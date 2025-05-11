use serde_json::{self, Value};
use std::io::Write;
use std::fs::File;
use std::{env, process};

#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>>{

    let args:Vec<String> = env::args().collect();

    if args.len() < 2{
        eprintln!("Usage {} github-username", args[0]);
        process::exit(1);
    }

    let username = &args[1];
    let url = format!("https://api.github.com/users/{}/events/public", username);

    let client = reqwest::Client::builder()
        .user_agent("github-activity/0.1.0")
        .build()?;

    let events:Vec<Value> = client.get(&url)
        .send()
        .await?
        .json()
        .await?;

    let output = serde_json::to_string_pretty(&events)?;
    let mut file = File::create("ghevents.json")?;
    file.write_all(output.as_bytes())?;

    println!("Successfully wrote everyting ....");

    Ok(())

}