use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Ok, Result};
///Returns a temporary and ephemeral directory to be used
pub fn get_tempdir() -> String {
    "./temp_dir".into()
}

///Removes the french and the russian language from tmp-db
pub fn cleanup() -> Result<()> {
    const DATA_DIR: &str = "./tmp-db";
    Ok({
        Command::new("rm").arg("-rf").arg(DATA_DIR).status()?;
    })
}
