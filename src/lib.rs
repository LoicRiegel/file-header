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
/// TODO: use unix-shell pattern matching
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
fn update_header_in_content(
    original_content: &str,
    current_header: &Header,
    new_header: &Header,
    blank_lines_after_header: usize,
) -> String {
    // Trip blank blank at the start
    let mut content = original_content.trim_start_matches('\n').to_string();

    // Remove current header
    dbg!(&content);
    dbg!(&current_header.0);
    if let Some(content_without_prefix) = content.strip_prefix(&current_header.0) {
        content = content_without_prefix.trim_start_matches('\n').to_string();
    }

    // Add new header
    content.insert_str(0, &"\n".repeat(blank_lines_after_header + 1));
    content.insert_str(0, new_header);

    content
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
    let mut original_content = String::new();
    file.read_to_string(&mut original_content)?;
    drop(file);

    let updated_content = update_header_in_content(
        &original_content,
        current_header,
        new_header,
        blank_lines_after_header,
    );

    let mut file = OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .open(file_path)
        .expect("Error when opening file to write");
    file.write_all(updated_content.as_bytes())?;
    drop(file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_header_in_content() {
        let original_content = "Code";
        let current_header = Header::new("");
        let new_header = Header::new("HEADER");
        let blank_lines_after_header: usize = 1;

        let expected = String::from("HEADER\n\nCode");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_header_in_content_zero_blank_lines() {
        let original_content = "Code";
        let current_header = Header::new("");
        let new_header = Header::new("HEADER");
        let blank_lines_after_header: usize = 0;

        let expected = String::from("HEADER\nCode");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_header_with_current_header() {
        let original_content = "HEADER\nCode";
        let current_header = Header::new("HEADER");
        let new_header = Header::new("NEW HEADER");
        let blank_lines_after_header: usize = 1;

        let expected = String::from("NEW HEADER\n\nCode");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_header_with_current_header_zero_blank_lines() {
        let original_content = "HEADER\nCode";
        let current_header = Header::new("HEADER");
        let new_header = Header::new("NEW HEADER");
        let blank_lines_after_header: usize = 0;

        let expected = String::from("NEW HEADER\nCode");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_header_multi_lines() {
        let original_content = "---
HEADER
---

Code";
        let current_header = Header::new("---
HEADER
---");
        let new_header = Header::new("---
NEW HEADER
---");
        let blank_lines_after_header: usize = 1;

        let expected = String::from("---
NEW HEADER
---

Code");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_header_multi_lines_blank_lines_in_headers() {
        let original_content = "---
HEADER
---

Code";
        let current_header = Header::new("---
HEADER
---


");
        let new_header = Header::new("---
NEW HEADER
---

");
        let blank_lines_after_header: usize = 1;

        let expected = String::from("---
NEW HEADER
---

Code");
        let result = update_header_in_content(original_content, &current_header, &new_header, blank_lines_after_header);
        
        assert_eq!(result, expected);
    }
}