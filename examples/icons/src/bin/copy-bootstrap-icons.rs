use std::path::PathBuf;
use yew_bootstrap::icons::BIFiles;

fn main() -> Result<(), std::io::Error> {
    let path = PathBuf::from(
        std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
    )
    .join(BIFiles::NAME);
    if !path.is_dir() {
        std::fs::create_dir(&path)?;
    }
    BIFiles::copy(&path)
}
