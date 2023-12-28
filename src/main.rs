use std::fs;
use std::path::Path;

const CURRENT_DIR: &str = "./tmp/testdir";

fn main() {
    let path= Path::new(CURRENT_DIR);
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
