use std::path::PathBuf;

use crate::error::AppResult;

#[cfg(unix)]
pub fn unpack(version: String, file_path: PathBuf, target_path: PathBuf) -> AppResult<()> {
    use crate::error::AppError;

    let file = std::fs::File::open(&file_path)?;
    let file_name = file_path
        .file_name()
        .ok_or(AppError(format!(
            "can't get filename from path:{}",
            file_path.to_string_lossy()
        )))?
        .to_string_lossy()
        .to_string();

    let xz = xz2::read::XzDecoder::new(file);
    let mut tar = tar::Archive::new(xz);
    tar.unpack(&target_path)?;

    std::fs::rename(
        target_path.join(file_name.trim_end_matches(".tar.xz")),
        target_path.join(version),
    )?;
    Ok(())
}
