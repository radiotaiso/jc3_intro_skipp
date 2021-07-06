use std::path::Path;
use std::process::Command;

fn main() {
    // let default_system_path = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Just Cause 3"; //  Steam default C drive path to games folder
    let default_system_path = "D:\\pruebaz\\JC3"; // TEST

    // let mut user_path = "ayylmao";
    println!("welcome, perrazo!");

    if Path::new(&default_system_path).exists() {
        println!("Ahhhperrazo si existe");
        hacky_patch(&default_system_path)
    } else {
        println!("shut up, nerd");
    };
}

pub fn hacky_patch(game_dir: &str) {

    let hacky_path: &str = "\\dropzone\\videos";
    let hacky_file: &str = "corona_video.bikc";

    Command::new("cmd").args(&["/C", format!("cd {}", game_dir)] );
    Command::new("cmd").args(&["/C", format!("mkdir {}", hacky_path)]);
    Command::new("cmd").args(&["/C", format!("type nul >> {}", hacky_file)]);
}