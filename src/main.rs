use std::io::Write;
use std::{fs, process};

use chrono::Utc;
use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    let options = App::new("bmv: Bulk Move")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::new("args").multiple(true))
        .version(crate_version!())
        .get_matches();

    let args = options.values_of("args").unwrap_or_default().collect();

    match run(args) {
        Ok(path) => match fs::remove_file(path) {
            Ok(_) => {}
            Err(_) => process::exit(1),
        },

        Err(e) => {
            eprintln!("{}", e.to_string());
            process::exit(1);
        }
    }
}

fn run(file_names: Vec<&str>) -> Result<String, std::io::Error> {
    let datetime = Utc::now().naive_utc();
    let temp_file_path = format!("/tmp/bmv-{}", datetime.format("%Y%m%d%H%M%S"));
    let default_editor = "vim".to_string();

    if file_names.len() == 0 {
        process::exit(0);
    }

    // Get default editor from config file? Flag?
    let editor = std::env::var("EDITOR").unwrap_or(default_editor);

    let mut temp_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&temp_file_path)?;

    let args_str = file_names.join("\n");

    temp_file.write_all(args_str.as_bytes())?;

    let _ = process::Command::new(&editor)
        .arg(&temp_file_path)
        .status()?;

    let temp_file_contents = fs::read_to_string(&temp_file_path)?;
    let new_file_names = temp_file_contents
        .trim_matches(|c| c == '\n')
        .split("\n")
        .collect::<Vec<_>>();

    let old_len = file_names.len();
    let new_len = new_file_names.len();

    if old_len == new_len {
        for (old, new) in file_names.iter().zip(new_file_names.iter()) {
            println!("{} -> {}", old, new);
            fs::rename(old, new)?;
        }
    }

    Ok(temp_file_path)
}
