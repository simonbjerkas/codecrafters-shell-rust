use super::ShellError;

use std::{
    fs::{File, OpenOptions},
    io::Write,
};

pub fn write_file(path: &str, content: &str, append: &bool) -> Result<(), ShellError> {
    let content = format!("{}\n", content.trim());
    let mut file = create_file(path, append)?;

    file.write_all(content.as_bytes())
        .map_err(|_| ShellError::WriteFile(file))?;

    Ok(())
}

pub fn create_file(path: &str, append: &bool) -> Result<File, ShellError> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(*append)
        .open(path)
        .map_err(|_| ShellError::CreateFile(path.to_string()))?;

    Ok(file)
}
