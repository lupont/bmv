use std::io::Write;
use std::{
    fs::{read_to_string, OpenOptions},
    process::{exit, Command},
};

use chrono::Utc;

fn main() {
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

    let mut temp_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_file_path)
    {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not open file at path {}, exiting.", &temp_file_path);
            exit(1);
        }
    };

    let args_str = args.join("\n");

    if let Err(_) = temp_file.write_all(args_str.as_bytes()) {
        eprintln!("Could not write bytes to file, exiting.");
        exit(2);
    }

    if let Err(_) = Command::new(&editor).arg(&temp_file_path).status() {
        eprintln!("Spawning {} failed, exiting.", &editor);
        exit(3);
    }

    let contents = match read_to_string(&temp_file_path) {
        Ok(file_contents) => file_contents,
        Err(_) => {
            eprintln!("Could not read file {}, exiting.", &temp_file_path);
            exit(4);
        }
    };
    let contents = contents
        .trim_matches(|c| c == '\n')
        .split("\n")
        .collect::<Vec<_>>();

    let old_len = args.len();
    let new_len = contents.len();

    if old_len != new_len {
        eprintln!("Wrong number of file names, exiting.");
        exit(5);
    }

    for (old, new) in args.iter().zip(contents.iter()) {
        println!("{} -> {}", old, new);
        std::fs::rename(old, new).unwrap();
    }
}
