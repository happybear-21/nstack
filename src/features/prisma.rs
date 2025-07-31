use anyhow::{Result, Context};
use console::style;
use indicatif::ProgressBar;
use std::process::Command;
use crate::package_manager::PackageManager;

pub async fn add_prisma() -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_message("Installing Prisma dependencies...");

    // Install prisma and @prisma/client
    let package_manager = PackageManager::from_project_config()?;
    let (pm, install_cmd) = package_manager.install_dev_command();
    let status = Command::new(pm)
        .arg(install_cmd)
        .arg("prisma tsx --save-dev")
        .status()
        .context("Failed to install prisma")?;
    if !status.success() {
        anyhow::bail!("Failed to install prisma");
    }
    let (pm, install_cmd) = package_manager.install_command();
    let status = Command::new(pm)
        .arg(install_cmd)
        .arg("@prisma/extension-accelerate @prisma/client")
        .status()
        .context("Failed to install @prisma/client")?;
    if !status.success() {
        anyhow::bail!("Failed to install @prisma/client");
    }

    pb.set_message("Initializing Prisma...");
    let status = Command::new("npx")
        .arg("prisma")
        .arg("init")
        .arg("--db")
        .arg("--output")
        .arg("../app/generated/prisma")
        .status()
        .context("Failed to run 'npx prisma init'")?;
    if !status.success() {
        anyhow::bail!("Failed to initialize Prisma");
    }

    pb.finish_with_message("Prisma setup completed!");

    println!("\n{}", style("✅ Prisma has been successfully set up!").green().bold());
    println!("\n{}", style("Next steps:").cyan().bold());
    println!("1. Define your schema in prisma/schema.prisma");
    println!("2. Run 'npx prisma migrate dev --name init' to create your first migration");
    println!("3. Use '@prisma/client' in your Next.js app to access your database");
    println!("\n{}", style("Files created:").cyan().bold());
    println!("• prisma/schema.prisma - Prisma schema");
    println!("• .env - Environment variables template");
    println!("• prisma/.gitignore - Ignore Prisma generated files");
    Ok(())
}
