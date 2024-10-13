use std::fs;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Mode to use
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

fn main() {
    let args = Cli::parse();



    // Check the mode
    match args.mode.as_str() {
        "move_photos" => move_photos(),
        "select_raw" => println!("select_raw"),
        _ => println!("unknown mode"),
    }
}