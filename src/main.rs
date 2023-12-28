use std::fs;
use std::io;
use std::path::PathBuf;
use clap::Parser;
use file_header;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory where the files to update are stored. All sub-directories will also be updated automatically.
    dir: PathBuf,
    /// Unix style pattern of the files to update
    pattern: String,
    /// File header to use
    header: PathBuf,
    /// Number of blank lines to insert between the end of the header and the rest of the code
    #[arg(long, default_value_t = 1)]
    blank_lines: usize,
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    let error_msg = format!("Failed to read {:?}", args.dir);
    let dir_entries = fs::read_dir(args.dir).expect(&error_msg);


    let files_to_update = file_header::select_files_matching_pattern(dir_entries, &args.pattern);
    for file in files_to_update {
        dbg!(file);
    }
    Ok(())
}
