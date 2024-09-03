pub mod logger;

use std::process::Command;

pub use logger::*;

pub fn clear_terminal_screen() {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).spawn()
    } else {
        // "clear" or "tput reset"
        Command::new("tput").arg("reset").spawn()
    };

    // Alternative solution:
    if result.is_err() {
        print!("{esc}c", esc = 27 as char);
    }
}