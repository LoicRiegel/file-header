use std::fs::ReadDir;
use std::fs::{DirEntry, File};
use std::io::{Read, Result, Write};
use std::iter::Iterator;

pub fn select_files_matching_pattern<'a> (
    dir_entries: &'a mut ReadDir,
    pattern: &'a str,
) -> impl Iterator<Item = Result<DirEntry>> + 'a {
    dir_entries.filter(move |entry| {
        match entry {
            Ok(entry) => entry.path().is_file() && entry.file_name().to_string_lossy().contains(pattern),
            Err(_) => true
        }
    })
}

/// Update the header of a file
pub fn update_header(file: &mut File, current_header: &str, new_header: &str) -> Result<()> {
    // Read original content
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to open a file");

    // Return early if nothing should be done
    if content.starts_with(new_header) {
        return Ok(());
    }

    // Remove current header
    remove_prefix_in_place(&mut content, current_header);

    // Add new header
    content.push_str(new_header);

    file.write_all(content.as_bytes())
        .expect("Failed to write to a file");
    Ok(())
}

/// Remove a preffix from a String in place
fn remove_prefix_in_place(input: &mut String, prefix: &str) {
    if !input.starts_with(prefix) {
        return;
    }
    input.drain(prefix.len()..);
}
