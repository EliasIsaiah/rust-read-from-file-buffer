use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // let line = line.unwrap();
        let my_line = String::from(line?);
        if my_line.contains(&args.pattern) {
            println!("{}", my_line);
        }
        // println!("{}", line);
    }
    Ok(())
}
