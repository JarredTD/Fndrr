use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::{fs, path::Path};

#[derive(Parser)]
struct Args {
    file_name: String,
    #[clap(default_value = ".")]
    start_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let mut stream = BufWriter::new(std::io::stdout());

    let entry = &args.start_path;
    let file_name = &args.file_name;

    search_file(&entry, &file_name, &mut stream).unwrap_or_else(|err| eprintln!("{}", err));
    stream.flush().unwrap_or_else(|err| eprintln!("{}", err));
}

fn search_file(dir: &Path, in_file_name: &str, stream: &mut dyn Write) -> Result<()> {
    let mut err_msgs = Vec::new();

    for entry in
        fs::read_dir(dir).with_context(|| format!("Failed to access: {}", dir.display()))?
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                err_msgs.push(format!("Failed to read directory entry: {}", err));
                continue;
            }
        };
        let path = entry.path();
        let full_path = match path.canonicalize() {
            Ok(full_path) => full_path,
            Err(err) => {
                err_msgs.push(format!("Failed to canonicalize file {}", err));
                continue;
            }
        };

        if path.is_dir() {
            if let Err(_e) = search_file(&path, in_file_name, stream) {
                err_msgs.push(format!("Failed to read directory {}", full_path.display()))
            }
        }
        if let Some(file_name_in_path) = path.file_name().and_then(|name| name.to_str()) {
            if file_name_in_path
                .to_lowercase()
                .contains(&in_file_name.to_lowercase())
            {
                if let Err(_e) = writeln!(stream, "{}", full_path.display()) {
                    err_msgs.push(format!(
                        "Failed to write file {} to stream",
                        full_path.display()
                    ));
                }
            }
        }
    }
    if !err_msgs.is_empty() {
        eprintln!("Encountered Errors:");
        for msg in err_msgs {
            eprintln!("\t {}", msg);
        }
    }

    Ok(())
}
