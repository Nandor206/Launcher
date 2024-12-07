use std::process::Command;
use std::io::{self,Write};
use std::thread;
use std::time::Duration;

fn main() {
    set_cmd_title("Launcher");


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
            start_program(r"c:\Users\stark\AppData\Local\Programs\Microsoft VS Code\Code.exe");
            start_program(r"c:\Program Files\WindowsApps\MSTeams_24295.605.3225.8804_x64__8wekyb3d8bbwe\ms-teams.exe");
            start_program(r"C:\Program Files\Google\Chrome\Application\chrome.exe");
            thread::sleep(Duration::from_secs(3));
        }
        "2" => {
            println!("Coding");
            start_program(r"c:\Users\stark\AppData\Local\Programs\Microsoft VS Code\Code.exe");
            start_program(r"C:\Program Files\Google\Chrome\Application\chrome.exe");
            thread::sleep(Duration::from_secs(3));
        }
        "3" => {
            println!("Just chilling");
            start_program(r"C:\Program Files\Google\Chrome\Application\chrome.exe");
            thread::sleep(Duration::from_secs(3));
        }
        "4" => {
            println!("Game coding");
            start_program(r"C:\Program Files\Blender Foundation\Blender 4.3\blender.exe");
            start_program(r"C:\Users\stark\Documents\Godot_v4.3-stable_mono_win64\Godot_v4.3-stable_mono_win64.exe");
            thread::sleep(Duration::from_secs(3));
        }
        "0" => {
            println!("Exiting");
            println!("Closing in 3 seconds...");
            thread::sleep(Duration::from_secs(3));
        }
        _ => {
            println!("Ismeretlen opció! Próbáld újra.");
        }
    }
}

fn start_program(program: &str) {
    match Command::new(program).spawn() {
        Ok(_) => println!("{} sikeresen elindítva!", program),
        Err(e) => eprintln!("Hiba történt a(z) {} indításakor: {}", program, e),
    }
}

fn set_cmd_title(title: &str) {
    let _ = Command::new("cmd")
        .args(["/c", "title", title])
        .status()
        .expect("Failed to set CMD title");
}