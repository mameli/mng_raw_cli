use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Mode to use: move_photos, select_raw
    mode: String
}

fn move_photos () {
    let jpg_dir = Path::new("jpg/selected/");
    let raw_dir = Path::new("raw/selected/");

    // Create directories if they do not exist
    if let Err(e) = fs::create_dir_all(jpg_dir) {
        eprintln!("Failed to create directory jpg/: {}", e);
    }
    if let Err(e) = fs::create_dir_all(raw_dir) {
        eprintln!("Failed to create directory raw/: {}", e);
    }

    let jpg_dir = Path::new("jpg/");
    let raw_dir = Path::new("raw/");

    let pb = indicatif::ProgressBar::new(100);

    // Iterate over files in the current directory
    for entry in fs::read_dir(".").expect("Failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                if extension == "JPG" {
                    println!(
                        "move {} to jpg/{}",
                        path.display(),
                        path.file_name().unwrap().to_str().unwrap()
                    );
                    fs::rename(&path, jpg_dir.join(path.file_name().unwrap()))
                        .expect("Failed to move JPG file");
                } else if extension == "RAF" {
                    println!(
                        "move {} to raw/{}",
                        path.display(),
                        path.file_name().unwrap().to_str().unwrap()
                    );
                    fs::rename(&path, raw_dir.join(path.file_name().unwrap()))
                        .expect("Failed to move RAF file");
                } else {
                    println!("{} in root folder", path.display());
                }
            }
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn select_raw () {
    let raw_selected = Path::new("raw/selected/");

    // Create raw/selected directory if it does not exist
    if let Err(e) = fs::create_dir_all(raw_selected) {
        eprintln!("Failed to create directory raw/selected/: {}", e);
    }

    // Iterate over files in jpg/selected directory
    if let Ok(entries) = fs::read_dir("jpg/selected/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(filename) = path.file_name() {
                    if let Some(filename_str) = filename.to_str() {
                        let selected_raf = filename_str.replace(".JPG", ".RAF");
                        let raw_file_path = format!("raw/{}", selected_raf);
                        let raw_file = Path::new(&raw_file_path);
                        if raw_file.is_file() {
                            println!("move raw/{} to raw/selected/{}", selected_raf, selected_raf);
                            let destination = raw_selected.join(&selected_raf);
                            if let Err(e) = fs::rename(raw_file, destination) {
                                eprintln!("Failed to move file: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let args = Cli::parse();

    // Check the mode
    match args.mode.as_str() {
        "move_photos" => move_photos(),
        "select_raw" => select_raw(),
        _ => println!("unknown mode"),
    }
}