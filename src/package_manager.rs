use anyhow::Result;
use std::process::Command;

#[derive(Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn detect() -> Result<Self> {
        if Command::new("bun").arg("--version").output().is_ok() {
            return Ok(PackageManager::Bun);
        }
        if Command::new("pnpm").arg("--version").output().is_ok() {
            return Ok(PackageManager::Pnpm);
        }
        if Command::new("yarn").arg("--version").output().is_ok() {
            return Ok(PackageManager::Yarn);
        }
        if Command::new("npm").arg("--version").output().is_ok() {
            return Ok(PackageManager::Npm);
        }
        anyhow::bail!("No package manager found. Please install npm, yarn, pnpm, or bun.")
    }

    pub fn install_command(&self) -> (&'static str, &'static str) {
        match self {
            PackageManager::Npm => ("npm", "install"),
            PackageManager::Yarn => ("yarn", "add"),
            PackageManager::Pnpm => ("pnpm", "add"),
            PackageManager::Bun => ("bun", "add"),
        }
    }

    pub fn install_dev_command(&self) -> (&'static str, &'static str) {
        match self {
            PackageManager::Npm => ("npm", "install -D"),
            PackageManager::Yarn => ("yarn", "add -D"),
            PackageManager::Pnpm => ("pnpm", "add -D"),
            PackageManager::Bun => ("bun", "add -D"),
        }
    }
}
