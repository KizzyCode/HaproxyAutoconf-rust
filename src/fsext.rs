use std::{
    fs, path::Path,
    io::{ Result, ErrorKind }
};


/// Writes a file atomically
pub fn write_atomic<D, F>(data: D, path: F) -> Result<()> where D: AsRef<[u8]>, F: AsRef<Path> {
    // Create the path and temp path
    let path = path.as_ref();
    let temp_path = {
        // Assemble the temp path
        let mut name = path.file_name().ok_or(ErrorKind::NotFound)?
            .to_os_string();
        name.push(".tmp");
        path.with_file_name(name)
    };
    
    // Write the file
    fs::write(&temp_path, data)?;
    fs::rename(&temp_path, &path)?;
    Ok(())
}
