use colored::*;
use std::process::Command;
use which::which;

fn main() {
    let header: String = "Ollama Update Checker".to_string();
    let seperator: String = "=".repeat(header.len());
    println!("{}", header.blue());
    println!("{}", seperator.blue());    

    // Check if ollama is installed
    if !is_command_available("ollama") {
        eprintln!("{}", "Error: ollama is not installed or not in PATH".red());
        std::process::exit(1);
    }

    // Get list of models
    let models = get_ollama_models();

    // Update each model
    for model in models {
        update_model(&model);
    }
}

fn is_command_available(command: &str) -> bool {
    which(command).is_ok()
}

fn get_ollama_models() -> Vec<String> {
    let output = Command::new("ollama")
        .arg("list")
        .output()
        .expect("Failed to execute ollama list");

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .map(|line| line.split_whitespace().next().unwrap().to_string())
        .collect()
}

fn update_model(model: &str) {
    let model_header = format!("== Updating {} ==", model.green());
    let seperator = "=".repeat(model_header.len());

    println!("{}", model_header.blue());
    println!("{}", seperator.blue());

    let status = Command::new("ollama")
        .arg("pull")
        .arg(model)
        .status()
        .expect("Failed to execute ollama pull");

    if status.success() {
        println!("{}", "== Update successful ==".green());
    } else {
        eprintln!("{}", "== Update failed ==".red());
    }
    println!();
}
