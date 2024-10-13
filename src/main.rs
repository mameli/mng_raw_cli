use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Mode to use
    mode: String
}

fn main() {
    let args = Cli::parse();

    // Check the mode
    match args.mode.as_str() {
        "move_photos" => println!("move_photos"),
        "select_raw" => println!("select_raw"),
        _ => println!("unknown mode"),
    }
}