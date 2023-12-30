use std::fs::{DirEntry, OpenOptions, ReadDir};
use std::io::{Read, Result, Write};
use std::iter::Iterator;
use std::path::PathBuf;

/// Return an iterator containing only files matching the pattern
pub fn select_files_matching_pattern<'a>(
    dir_entries: &'a mut ReadDir,
    pattern: &'a str,
) -> impl Iterator<Item = Result<DirEntry>> + 'a {
    dir_entries.filter(move |entry| match entry {
        Ok(entry) => {
            entry.path().is_file() && entry.file_name().to_string_lossy().contains(pattern)
        }
        Err(_) => true,
    })
}

/// Update the header of a file
pub fn update_header(
    file_path: &PathBuf,
    current_header: &str,
    new_header: &str,
    blank_lines_after_header: usize,
) -> Result<()> {
    // Trim blank lines
    let current_header = current_header
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .to_string();
    let new_header = new_header
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .to_string();

    // Read original content
    let mut file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .expect("Error when opening file to read");
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    drop(file);
    content = content.trim_start_matches('\n').to_string();

    // Return early if nothing should be done
    if content.starts_with(&new_header) {
        return Ok(());
    }

    // Remove current header
    remove_prefix_in_place(&mut content, &current_header);

    // Add new header
    content.insert_str(0, &"\n".repeat(blank_lines_after_header + 1));
    content.insert_str(0, &new_header);

    let mut file = OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .open(file_path)
        .expect("Error when opening file to write");
    file.write_all(content.as_bytes())?;
    drop(file);

    Ok(())
}

/// Remove a preffix from a String in place
fn remove_prefix_in_place(input: &mut String, prefix: &str) {
    if prefix.is_empty() || !input.starts_with(prefix) {
        return;
    }
    println!("draining");
    input.drain(prefix.len()..);
}
