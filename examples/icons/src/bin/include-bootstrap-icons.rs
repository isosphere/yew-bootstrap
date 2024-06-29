use std::path::PathBuf;
use yew_bootstrap::icons::BIFiles;

fn main() -> Result<(), std::io::Error> {
    let staging_dir = PathBuf::from(
        std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
    );

    let path = staging_dir.join(BIFiles::NAME);
    if !path.is_dir() {
        std::fs::create_dir(&path)?;
    }
    BIFiles::copy(&path)?;

    let path = staging_dir.join("index.html");
    let index = std::fs::read_to_string(&path)?;
    let index = index.replace(
        "<!include-bootstrap-icons>",
        &format!(
            r#"<link rel="stylesheet" href="bootstrap-icons-{}/bootstrap-icons.css"/>"#,
            BIFiles::VERSION
        ),
    );
    std::fs::write(&path, index)?;

    Ok(())
}
