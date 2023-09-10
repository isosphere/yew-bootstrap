# Example of automatically copying the bootstrap-icons files

This is copies the files to the dist directory, which is the recommended way.

Please see https://docs.rs/yew-bootstrap/latest/yew_bootstrap/icons/index.html for documentation.

The possible downside is that trunk does not add a hash and thus a change can't be detected easily.
But since the version of bootstrap-icons is included and it should not change, it's not a problem.

## Instructions

1. `Cargo.toml`

   Add the build-executed binary.

    ```toml
    [[bin]]
    name = "copy-bootstrap-icons"
    ```

2. `src/bin/copy-bootstrap-icons.rs`

   Create the program to copy the files.

    ```rust
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
    ```

3. `index.html`

   Set base reference, link the css and specify your wasm-program.

   (Since we'll be writing a build-executed program, there are now two binaries and trunk needs to know which is your WASM binary.)

    ```html
    <base data-trunk-public-url />
    <link rel="stylesheet" href="bootstrap-icons-v1.10.5/bootstrap-icons.css" />
    <link data-trunk rel="rust" data-bin="name-of-app" />
    ```

4. `Trunk.toml`

   Add a hook to run the program.

    ```toml
    [[hooks]]
    stage = "build"
    command = "cargo"
    command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
    ```
