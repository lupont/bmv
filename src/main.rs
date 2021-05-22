use std::io::Write;
use std::{
    fs::{read_to_string, OpenOptions},
    process::{exit, Command},
};

use chrono::Utc;

fn main() {
    match run() {
        Ok(path) => match std::fs::remove_file(path) {
            Ok(_) => {}
            Err(_) => exit(1),
        },

        Err(e) => {
            eprintln!("{}", e.to_string());
            exit(1);
        }
    }
}

fn run() -> Result<String, std::io::Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let datetime = Utc::now().naive_utc();
    let temp_file_path = format!("/tmp/bmv-{}", datetime.format("%Y%m%d%H%M%S"));

    if args.len() == 0 {
        println!("No files specified, exiting.");
        exit(0);
    }

    let editor = match std::env::var("EDITOR") {
        Ok(editor) => editor,

        // Get default editor from config file?
        Err(_) => "vim".into(),
    };

    let mut temp_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_file_path)?;

    let args_str = args.join("\n");

    temp_file.write_all(args_str.as_bytes())?;

    let _ = Command::new(&editor).arg(&temp_file_path).status()?;

    let contents = read_to_string(&temp_file_path)?;
    let contents = contents
        .trim_matches(|c| c == '\n')
        .split("\n")
        .collect::<Vec<_>>();

    let old_len = args.len();
    let new_len = contents.len();

    if old_len == new_len {
        for (old, new) in args.iter().zip(contents.iter()) {
            println!("{} -> {}", old, new);
            std::fs::rename(old, new)?;
        }
    }

    // std::fs::remove_file(&temp_file_path)
    Ok(temp_file_path)
}
