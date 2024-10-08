use fs_extra::dir::CopyOptions;
use std::fs;
use std::path::Path;

pub fn copy_dir(from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut from = from.to_path_buf();
    if to.exists() {
        fs::remove_dir_all(to).unwrap();
    }

    if !from.exists() {
        from = from.join("..");
    }

    fs::create_dir_all(to).unwrap();

    let options = CopyOptions::new().overwrite(true);

    copy_recursive(from.as_path(), to, &options)
}

fn copy_recursive(
    from: &Path,
    to: &Path,
    options: &CopyOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let from_meta = fs::metadata(from)?;

    if options.overwrite && to.exists() {
        fs::remove_dir_all(to)?;
    }

    if from_meta.is_dir() {
        symlink_dir(from, to)?;
    }

    Ok(())
}

#[cfg(unix)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    std::os::unix::fs::symlink(from, to)?;
    Ok(())
}

#[cfg(windows)]
pub fn symlink_dir<P: AsRef<Path>, U: AsRef<Path>>(from: P, to: U) -> std::io::Result<()> {
    junction::create(from, to)?;
    Ok(())
}
