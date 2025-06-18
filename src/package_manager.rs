use anyhow::{Result, Context};
use std::process::Command;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
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

    pub fn from_project_config() -> Result<Self> {
        let config_path = Path::new(".nstack").join("config");
        if !config_path.exists() {
            return Self::detect();
        }

        let config_content = fs::read_to_string(&config_path)
            .context("Failed to read .nstack/config file")?;

        for line in config_content.lines() {
            if line.starts_with("package_manager=") {
                let pm = line.split('=').nth(1).unwrap_or("").trim();
                return match pm {
                    "npm" => Ok(PackageManager::Npm),
                    "yarn" => Ok(PackageManager::Yarn),
                    "pnpm" => Ok(PackageManager::Pnpm),
                    "bun" => Ok(PackageManager::Bun),
                    _ => Self::detect(),
                };
            }
        }

        Self::detect()
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

    pub fn create_next_app_command(&self) -> (&'static str, Vec<&'static str>) {
        match self {
            PackageManager::Npm => ("npx", vec!["create-next-app@latest"]),
            PackageManager::Yarn => ("yarn", vec!["create", "next-app"]),
            PackageManager::Pnpm => ("pnpm", vec!["create", "next-app"]),
            PackageManager::Bun => ("bunx", vec!["create-next-app"]),
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }
}
