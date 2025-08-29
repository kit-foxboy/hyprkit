use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

/// Recursively copy a directory and all its contents
pub fn copy_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    fs::create_dir_all(dest)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)
                .context(format!("Failed to copy file {} to {}", src_path.display(), dest_path.display()))?;
        }
    }
    
    Ok(())
}

/// Move a directory by copying it and then removing the source
pub fn move_dir_recursive(src: &PathBuf, dest: &PathBuf) -> Result<()> {
    // First copy the directory structure
    copy_dir_recursive(src, dest)?;
    
    // Then remove the source directory
    fs::remove_dir_all(src)
        .context(format!("Failed to remove source directory {}", src.display()))?;
    
    Ok(())
}
