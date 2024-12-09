use std::process::Command;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    set_cmd_title("Launcher");

    let path = "config.txt";
    let app_paths = match read_config(path) {
        Ok(paths) => paths,
        Err(_) => {
            println!("Config file not found. Please check the file path.");
            return;
        }
    };

    println!("What to start?:");
    println!("1 | School (Teams, VS code, Chrome)");
    println!("2 | Coding (VS code, Chrome)");
    println!("3 | Just chilling");
    println!("4 | Blender + Godot");
    println!("0 | Exit");
    print!("Choose: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read the input!");

    let input = input.trim();

    match input {
        "1" => {
            println!("School");
            start_program(&app_paths[0]); // VS Code
            start_program(&app_paths[1]); // Teams
            start_program(&app_paths[2]); // Chrome
            thread::sleep(Duration::from_secs(3));
        }
        "2" => {
            println!("Coding");
            start_program(&app_paths[0]); // VS Code
            start_program(&app_paths[2]); // Chrome
            thread::sleep(Duration::from_secs(3));
        }
        "3" => {
            println!("Just chilling");
            start_program(&app_paths[2]); // Chrome
            thread::sleep(Duration::from_secs(3));
        }
        "4" => {
            println!("Game coding");
            start_program(&app_paths[3]); // Blender
            start_program(&app_paths[4]); // Godot
            thread::sleep(Duration::from_secs(3));
        }
        "0" => {
            println!("Exiting");
            println!("Closing in 3 seconds...");
            thread::sleep(Duration::from_secs(3));
        }
        _ => {
            println!("Unknown option! Try again.");
        }
    }
}

fn start_program(program: &str) {
    match Command::new(program).spawn() {
        Ok(_) => println!("{} started successfully!", program),
        Err(e) => eprintln!("Error starting {}: {}", program, e),
    }
}

fn set_cmd_title(title: &str) {
    let _ = Command::new("cmd")
        .args(["/c", "title", title])
        .status()
        .expect("Failed to set CMD title");
}

fn read_config(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            lines.push(line);
        }
    }
    Ok(lines)
}
