use super::ShellError;

use std::{fs::File, io::Write};

pub fn write_file(path: &str, content: &str, append: &bool) -> Result<(), ShellError> {
    let content = format!("{}\n", content.trim());
    let mut file = match append {
        true => open_file(path)?,
        false => create_file(path)?,
    };

    file.write_all(content.as_bytes())
        .map_err(|_| ShellError::WriteFile(file))?;

    Ok(())
}

pub fn create_file(path: &str) -> Result<File, ShellError> {
    let file = File::create(path).map_err(|_| ShellError::CreateFile(path.to_string()))?;
    Ok(file)
}

fn open_file(path: &str) -> Result<File, ShellError> {
    let file = File::open(path).map_err(|_| ShellError::OpenFile(path.to_string()))?;
    Ok(file)
}
