use super::ShellError;
use anyhow::Result;

use std::{fs::File, io::Write};

pub fn write_file(path: &str, content: &str, append: &bool) -> Result<()> {
    let content = format!("{}\n", content.trim());
    let mut file = create_file(path, append)?;

    file.write_all(content.as_bytes())
        .map_err(|_| ShellError::WriteFile(file))?;

    Ok(())
}

pub fn create_file(path: &str, append: &bool) -> Result<File, ShellError> {
    let file = match append {
        true => File::options()
            .create(true)
            .append(*append)
            .open(path)
            .map_err(|_| ShellError::CreateFile(path.to_string()))?,

        false => File::create(path).map_err(|_| ShellError::CreateFile(path.to_string()))?,
    };

    Ok(file)
}
