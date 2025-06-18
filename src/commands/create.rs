use anyhow::{Result, Context};
use console::style;
use dialoguer::{Input, theme::ColorfulTheme};
use indicatif::ProgressBar;
use std::process::Command;

pub async fn create_project(name: Option<String>) -> Result<()> {
    let project_name = match name {
        Some(name) => name,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter project name")
            .interact_text()?,
    };

    println!("{}", style("Creating Next.js project...").cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_message("Running create-next-app...");

    let status = Command::new("npx")
        .args(["create-next-app@latest", &project_name])
        .status()
        .context("Failed to run create-next-app")?;

    if !status.success() {
        anyhow::bail!("Failed to create Next.js project");
    }

    pb.finish_with_message("Project created successfully!");
    println!("\n{}", style("Next steps:").green());
    println!("  cd {}", project_name);
    println!("  nstack add <feature>");

    Ok(())
}
