// src/main.rs
use std::env;
use std::process::Command;

use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use rustyline::DefaultEditor;
use serde_json::{json, Value};

fn main() -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let api_key = match env::var("GOOGLE_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            println!("\n[INFO] Gemini API key (GOOGLE_API_KEY) is missing. Please enter your Gemini API key:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let key = input.trim().to_string();
            if key.is_empty() {
                eprintln!("\n[ERROR] No Gemini API key provided. Exiting.\n");
                std::process::exit(1);
            }
            unsafe { env::set_var("GOOGLE_API_KEY", &key) };
            key
        }
    };
    let client = Client::new();

    println!("Skript AI Shell - Type 'exit' to quit");

    loop {
        let line = rl.readline("skript> ");

        let prompt = match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
                    break;
                }
                if trimmed.is_empty() {
                    continue;
                }
                rl.add_history_entry(trimmed)?;
                trimmed.to_owned()
            }
            Err(_) => break,
        };

        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": format!("You are a Windows PowerShell expert. Provide only the PowerShell command (using built-in modules unless otherwise explicitly specified) for this prompt: '{}'. If the command contains quotes, ensure only single quotes are used. The command itself should never be enclosed in quotes. No explanation.", prompt)
                }]
            }]
        });

        let response = client
            .post(&format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send();

        let text = match response {
            Ok(resp) => parse_response(resp.text().unwrap_or_default()),
            Err(e) => {
                eprintln!("\n[ERROR] Failed to contact Gemini API: {}\n", e);
                continue;
            }
        };
        print!("Suggested command: {}", text);
        run_command(&text);
    }

    Ok(())
}

fn parse_response(response: String) -> String {
    let parsed: Value = serde_json::from_str(&response).unwrap_or_default();

    parsed["candidates"]
        .get(0)
        .and_then(|c| c["content"]["parts"].get(0))
        .and_then(|p| p["text"].as_str())
        .map(|s| s.trim().to_string())
        .unwrap_or("<No command returned>".to_string())
}

fn run_command(command: &str) {
    let sanitized = command.trim();
    
    if sanitized.is_empty() {
        println!("\nNo valid command to execute.\n");
        return;
    }

    println!("\nExecuting: {}\n", sanitized);

    let output = Command::new("powershell")
       .arg("-Command")
       .arg(sanitized)
       .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stdout.trim().is_empty() {
                println!("[stdout]\n{}", stdout);
            }
            if !stderr.trim().is_empty() {
                eprintln!("[stderr]\n{}", stderr);
            }
            println!("\nProcess exited with status: {}", output.status);
        },
        Err(e) => eprintln!("\n[ERROR] Failed to execute command: {}\n", e),
    }
}







