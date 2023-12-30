use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Directory where the files to update are stored. All sub-directories will also be updated automatically.
    dir: PathBuf,
    /// Unix style pattern of the files to update
    pattern: String,
    /// File header currently present in the files (optional)
    #[arg(short, long)]
    current_header: Option<PathBuf>,
    /// File header to apply
    #[arg(short, long)]
    new_header: PathBuf,
    /// Number of blank lines to insert between the end of the header and the rest of the code
    #[arg(long, default_value_t = 1)]
    blank_lines: usize,
}

fn main() -> Result<(), ()> {
    let mut exit_code: Result<(), ()> = Ok(());
    let args = Args::parse();
    let current_header_content = match args.current_header {
        Some(pathbuf) => fs::read_to_string(pathbuf).expect("Could not read header file"),
        None => String::new(),
    };
    let new_header_content = fs::read_to_string(args.new_header).expect("Could not read header file");

    let error_msg = format!("Failed to read {:?}", args.dir);
    let mut dir_entries = fs::read_dir(args.dir).expect(&error_msg);

    let files_to_update =
        file_header::select_files_matching_pattern(&mut dir_entries, &args.pattern);
    for file_entry in files_to_update.filter_map(|result| result.ok()) {
        if let Err(err) = file_header::update_header(&file_entry.path(), &current_header_content, &new_header_content) {
            eprintln!("Could not update file {:?} {}", file_entry.file_name(), err);
            exit_code = Err(());
        }
    }
    return exit_code
}
