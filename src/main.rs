use std::io::{self, Write};
use std::{fs, process};

use clap::{crate_version, App, AppSettings, Arg};

const NAME: &str = "bmv: Bulk Move";
const ABOUT: &str = "This tool accepts any amount of file names, opens them in your $EDITOR and renames the ones you changed.";
const DEFAULT_EDITOR: &str = "vim";
const TMP_FILE_PATH: &str = "/tmp/bmvfile";

fn main() {
    let options = App::new(NAME)
        .about(ABOUT)
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

    let result = run(files);

    if let Err(e) = &result {
        eprintln!("{}", e.to_string());
    }

    match fs::remove_file(TMP_FILE_PATH) {
        Ok(_) if result.is_ok() => {}
        _ => process::exit(1),
    }
}

fn run(file_names: Vec<&str>) -> io::Result<()> {
    if file_names.len() == 0 {
        process::exit(0);
    }

    for &file_name in file_names.iter() {
        fs::metadata(file_name)?;
    }

    // Get default editor from config file? Flag?
    let editor = std::env::var("EDITOR").unwrap_or(DEFAULT_EDITOR.into());

    let mut temp_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&TMP_FILE_PATH)?;

    let args_str = file_names.join("\n");

    temp_file.write_all(args_str.as_bytes())?;

    let _ = process::Command::new(&editor)
        .arg(&TMP_FILE_PATH)
        .status()?;

    let temp_file_contents = fs::read_to_string(&TMP_FILE_PATH)?;
    let new_file_names = temp_file_contents
        .trim_matches(|c| c == '\n')
        .split("\n")
        .collect::<Vec<_>>();

    if file_names.len() == new_file_names.len() {
        for (old, new) in file_names.iter().zip(new_file_names.iter()) {
            if old != new {
                println!("{} -> {}", old, new);
                fs::rename(old, new)?;
            }
        }
    }

    Ok(())
}
