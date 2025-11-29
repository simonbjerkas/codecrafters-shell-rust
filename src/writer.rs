use super::ShellError;

use std::{fs::File, io::Write};

pub fn write_file(path: &str, content: &str) -> Result<(), ShellError> {
    let content = format!("{}\n", content.trim());
    let mut file = File::create(path).map_err(|_| ShellError::CreateFile(path.to_string()))?;
    file.write_all(content.as_bytes())
        .map_err(|_| ShellError::WriteFile(file))?;

    Ok(())
}
