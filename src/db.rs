use anyhow::Result;

use std::sync::RwLock;

use once_cell::sync::Lazy;

pub static DOCDIR: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::from("./")));

pub async fn init_db() -> Result<()> {
    Ok(())
}
