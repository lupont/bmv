use std::io::{self, Write};
use std::{fs, process};

use chrono::Utc;
use clap::{crate_version, App, AppSettings, Arg};

fn main() {
    let options = App::new("bmv: Bulk Move")
        .about("This tool accepts any amount of file names, opens them in your $EDITOR and renames the ones you changed.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::TrailingVarArg)
        .arg(
            Arg::new("files")
                .multiple(true)
                .about("The names of the file to rename."),
        )
        .version(crate_version!())
        .get_matches();

    let files = options.values_of("files").unwrap_or_default().collect();
    let datetime = Utc::now().naive_utc();
    let temp_file_path = format!("/tmp/bmv-{}", datetime.format("%Y%m%d%H%M%S"));

    let result = run(&temp_file_path, files);

    if let Err(e) = &result {
        eprintln!("{}", e.to_string());
    }

    match fs::remove_file(temp_file_path) {
        Ok(_) if result.is_ok() => {}
        _ => process::exit(1),
    }
}

fn run(temp_file_path: &str, file_names: Vec<&str>) -> io::Result<()> {
    let default_editor = "vim".to_string();

    if file_names.len() == 0 {
        process::exit(0);
    }

    for &file_name in file_names.iter() {
        fs::metadata(file_name)?;
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
            if old != new {
                println!("{} -> {}", old, new);
                fs::rename(old, new)?;
            }
        }
    }

    Ok(())
}
