use std::io::{Read, Write, Result};
use std::fs::ReadDir;
use std::fs::{DirEntry, File};
use std::iter::Iterator;

pub fn select_files_matching_pattern(dir_entries: ReadDir, _: &str) -> impl Iterator<Item = Result<DirEntry>> {
    dir_entries.filter(|entry| entry.is_ok())
}


/// Update the header of a file
pub fn update_header(file: &mut File, current_header: &str, new_header: &str) -> Result<()> {
    // Read original content
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to open a file");

    // Return early if nothing should be done
    if content.starts_with(new_header) {
        return Ok(())
    }

    // Remove current header
    remove_prefix_in_place(&mut content, current_header);

    // Add new header
    content.push_str(new_header);

    file.write(content.as_bytes()).expect("Failed to write to a file");
    Ok(())
}


/// Remove a preffix from a String in place
fn remove_prefix_in_place(input: &mut String, prefix: &str) {
    if !input.starts_with(prefix) {
        return;
    }
    input.drain(prefix.len()..);
}
