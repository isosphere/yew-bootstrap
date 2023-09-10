use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use std::collections::BTreeMap;
use std::env;
use std::io::Write;
use std::path::Path;

fn bootstrap_icons<W: Write>(output: &mut W) -> Result<()> {
    let mut sorted_icons = BTreeMap::new();
    for ln in include_str!("bootstrap-icons-v1.10.5/bootstrap-icons.css").lines() {
        if let Some(ln) = ln.strip_prefix(".bi-") {
            if let Some((lower_kebab_case, _)) = ln.split_once("::before") {
                if !lower_kebab_case.is_empty() {
                    let mut upper_snake_case = lower_kebab_case
                        .from_case(Case::Kebab)
                        .to_case(Case::UpperSnake);
                    if upper_snake_case
                        .chars()
                        .all(|c| c == '_' || c.is_ascii_alphanumeric())
                    {
                        let first = upper_snake_case.chars().next().unwrap();
                        if !first.is_ascii_alphabetic() && first != '_' {
                            upper_snake_case.insert(0, '_');
                        }
                        sorted_icons.insert(lower_kebab_case, format!(
                            "    pub const {upper_snake_case}: BI = BI(\"<i class=\\\"bi bi-{lower_kebab_case}\\\"></i>\");\n"
                        ));
                    }
                }
            }
        }
    }
    output.write_all("#[allow(missing_docs)]\n".as_bytes())?;
    output.write_all("impl BI{\n".as_bytes())?;
    sorted_icons
        .values()
        .try_for_each(|i| output.write_all(i.as_bytes()))?;
    output.write_all("}\n".as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").context("OUT_DIR not found")?;
    let dest_path = Path::new(&out_dir).join("bootstrap_icons_generated.rs");
    bootstrap_icons(&mut std::fs::File::create(dest_path)?)
}
