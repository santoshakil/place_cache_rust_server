use anyhow::Result;

use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static DATADIR: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("./data")));

pub async fn init_db() -> Result<()> {
    std::fs::create_dir_all(&*DATADIR.read().unwrap())?;
    Ok(())
}
