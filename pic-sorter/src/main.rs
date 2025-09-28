use clap::Parser;
use std::path::Path;
use std::{fs, error::Error};
use chrono::{DateTime, Utc, Datelike};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    directory: String,
}

fn is_image(file: &fs::DirEntry) -> bool {
    let path = file.path();
    let extension = path.extension().unwrap_or_default().to_str().unwrap_or_default();
    let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "heic", "heif"];

    image_extensions.contains(&extension)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    // We don't check if the directory exists, read_dir() will return an error if it doesn't exist.
    let directory = Path::new(&args.directory);
    let mut seen_years: Vec<String> = Vec::new();
    let files = directory.read_dir()?;
    for file in files {
        let file = file?;
        if is_image(&file) {
            let attr = fs::metadata(&file.path())?;
            let created: DateTime<Utc> = attr.created()?.into();
            let year = created.year().to_string();
            
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