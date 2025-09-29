use chrono::{DateTime, Datelike, Utc};
use clap::Parser;
use std::path::Path;
use std::{error::Error, fs, fs::Metadata};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    directory: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    // We don't check if the directory exists, read_dir() will return an error if it doesn't exist.
    let directory = Path::new(&args.directory);
    let mut seen_years: Vec<String> = Vec::new();
    let files = directory.read_dir()?;
    for file in files {
        let file = file?;
        if file.path().is_file() {
            let attr: Metadata = fs::metadata(&file.path())?;
            let created: DateTime<Utc> = attr.created()?.into();
            let modified: DateTime<Utc> = attr.modified()?.into();
            // Windows uses created or modified date, so we need to check which one is older.
            let year: String = if created < modified {
                created.year().to_string()
            } else {
                modified.year().to_string()
            };

            let new_path = directory.join(&year);

            println!("Moving {} to {}", file.path().display(), new_path.display());

            // Create the year directory if it doesn't exist
            if !seen_years.contains(&year) {
                fs::create_dir_all(&new_path)?;
                seen_years.push(year.clone());
            }

            // Move the file to the year directory
            fs::rename(&file.path(), &new_path.join(file.file_name()))?;
        }
    }

    Ok(())
}
