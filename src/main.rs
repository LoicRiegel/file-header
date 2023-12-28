use std::fs;
use std::path::Path;
use clap::Parser;


/// Add a header to selected files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory
    #[arg(short, long)]
    dir: String,
}


fn main() {
    let args = Args::parse();

    let path= Path::new(&args.dir);
    match fs::read_dir(path) {
        Err(_) => println!("error"),
        Ok(entries) => entries.
            flatten().
            filter(|file| file.file_name().to_string_lossy().contains(".py"))
            .for_each(|file| { 
                let content = fs::read_to_string(file.path());
                if let Ok(content) = content {
                    println!("{:?}", content);
                }
            }
            ),
    }
}
