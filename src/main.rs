use std::fs;
use std::io::{self, Write};
use std::process;

use clap::{crate_version, AppSettings, Arg};

const NAME: &str = "bmv: Bulk Move";
const ABOUT: &str = "This tool accepts any amount of file names, opens them in your $EDITOR and renames the ones you changed.";
const DEFAULT_EDITOR: &str = "vim";
const TMP_FILE_PATH: &str = "/tmp/bmvfile";

fn main() {
    let options = clap::App::new(NAME)
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

    if let Err(e) = run(files) {
        eprintln!("{}", e.to_string());
        process::exit(1);
    }

    if fs::remove_file(TMP_FILE_PATH).is_err() {
        process::exit(1);
    }
}

fn run(file_names: Vec<&str>) -> io::Result<()> {
    for &file_name in &file_names {
        fs::metadata(file_name)?;
    }

    let editor = std::env::var("EDITOR").unwrap_or(DEFAULT_EDITOR.into());

    let mut temp_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(TMP_FILE_PATH)?;

    temp_file.write_all(file_names.join("\n").as_bytes())?;

    let _ = process::Command::new(&editor).arg(TMP_FILE_PATH).status()?;

    let temp_file_contents = fs::read_to_string(TMP_FILE_PATH)?;
    let new_file_names = temp_file_contents
        .trim_matches(|c| c == '\n')
        .split('\n')
        .collect::<Vec<_>>();

    let (prev_len, new_len) = (file_names.len(), new_file_names.len());
    if prev_len == new_len {
        let both = file_names
            .iter()
            .zip(&new_file_names)
            .filter(|(p, n)| p == n);

        for (&prev, &new) in both {
            println!("{} -> {}", prev, new);
            fs::rename(prev, new)?;
        }
    } else {
        eprintln!("Mismatched number of files: {} -> {}", prev_len, new_len);
    }

    Ok(())
}
