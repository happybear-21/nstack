use anyhow::{Result, Context};
use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;
use std::process::Command;
use crate::package_manager::PackageManager;

pub async fn create_project(name: Option<String>) -> Result<()> {
    let project_name = match name {
        Some(name) => name,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter project name")
            .interact_text()?,
    };

    // Ask user to choose package manager
    let package_managers = vec!["npm", "yarn", "pnpm", "bun"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your package manager")
        .items(&package_managers)
        .default(0)
        .interact()?;

    let chosen_pm = match selection {
        0 => PackageManager::Npm,
        1 => PackageManager::Yarn,
        2 => PackageManager::Pnpm,
        3 => PackageManager::Bun,
        _ => unreachable!(),
    };

    println!("{}", style(format!("Creating Next.js project with {}...", chosen_pm.to_string())).cyan());

    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("Running create-next-app with {}...", chosen_pm.to_string()));

    let (command, args) = chosen_pm.create_next_app_command();
    let mut cmd = Command::new(command);
    cmd.args(args);
    cmd.arg(&project_name);

    let status = cmd
        .status()
        .context(format!("Failed to run create-next-app with {}", chosen_pm.to_string()))?;

    if !status.success() {
        anyhow::bail!("Failed to create Next.js project");
    }

    // Save the chosen package manager to a config file for future use
    save_package_manager_config(&project_name, &chosen_pm)?;

    pb.finish_with_message("Project created successfully!");
    println!("\n{}", style("Next steps:").green());
    println!("  cd {}", project_name);
    println!("  nstack add <feature>");

    Ok(())
}

fn save_package_manager_config(project_name: &str, package_manager: &PackageManager) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let config_content = format!("package_manager={}\n", package_manager.to_string());
    let config_path = Path::new(project_name).join(".nstack");
    
    fs::create_dir_all(&config_path)?;
    fs::write(config_path.join("config"), config_content)?;
    
    Ok(())
}
