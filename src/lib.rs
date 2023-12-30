use std::fs::{DirEntry, OpenOptions, ReadDir};
use std::io::{Read, Result, Write};
use std::iter::Iterator;
use std::ops::Deref;
use std::path::PathBuf;

pub struct Header(String);

impl Header {
    pub fn new(string: &str) -> Self {
        Self(
            string
                .trim_start_matches('\n')
                .trim_end_matches('\n')
                .to_string(),
        )
    }
}

impl Deref for Header {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    current_header: &Header,
    new_header: &Header,
    blank_lines_after_header: usize,
) -> Result<()> {
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
    if content.starts_with(&new_header.0) {
        return Ok(());
    }

    // Remove current header
    if let Some(content_without_prefix) = content.strip_prefix(&current_header.0) {
        content = content_without_prefix.trim_start_matches("\n").to_string();
    }

    // Add new header
    content.insert_str(0, &"\n".repeat(blank_lines_after_header + 1));
    content.insert_str(0, new_header);

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
