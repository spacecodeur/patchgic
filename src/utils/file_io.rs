use std::error::Error;
use std::fs;
use std::path::Path;

pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn write_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    // Crée les répertoires parents si nécessaire
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(file_path, content)?;
    Ok(())
}

pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
