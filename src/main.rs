use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;
use std::process::Command;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short, long)]
        name: Option<String>,
    },
    Add {
        #[arg(short, long)]
        feature: Option<String>,
    },
    List,
}

#[derive(Debug)]
enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

#[derive(Debug)]
enum ProjectStructure {
    AppDir,
    SrcDir,
}

impl ProjectStructure {
    fn detect() -> Result<Self> {
        if Path::new("app").exists() {
            Ok(ProjectStructure::AppDir)
        } else if Path::new("src").exists() {
            Ok(ProjectStructure::SrcDir)
        } else {
            anyhow::bail!("Could not detect project structure. Neither 'app' nor 'src' directory found.")
        }
    }

    fn get_globals_css_path(&self) -> &'static str {
        match self {
            ProjectStructure::AppDir => "app/globals.css",
            ProjectStructure::SrcDir => "src/app/globals.css",
        }
    }

    fn get_lib_path(&self) -> &'static str {
        match self {
            ProjectStructure::AppDir => "lib",
            ProjectStructure::SrcDir => "src/lib",
        }
    }
}

impl PackageManager {
    fn detect() -> Result<Self> {
        // Check for Bun
        if Command::new("bun").arg("--version").output().is_ok() {
            return Ok(PackageManager::Bun);
        }
        // Check for pnpm
        if Command::new("pnpm").arg("--version").output().is_ok() {
            return Ok(PackageManager::Pnpm);
        }
        // Check for Yarn
        if Command::new("yarn").arg("--version").output().is_ok() {
            return Ok(PackageManager::Yarn);
        }
        // Check for npm
        if Command::new("npm").arg("--version").output().is_ok() {
            return Ok(PackageManager::Npm);
        }

        anyhow::bail!("No package manager found. Please install npm, yarn, pnpm, or bun.")
    }

    fn install_command(&self) -> (&'static str, &'static str) {
        match self {
            PackageManager::Npm => ("npm", "install"),
            PackageManager::Yarn => ("yarn", "add"),
            PackageManager::Pnpm => ("pnpm", "add"),
            PackageManager::Bun => ("bun", "add"),
        }
    }

    fn install_dev_command(&self) -> (&'static str, &'static str) {
        match self {
            PackageManager::Npm => ("npm", "install -D"),
            PackageManager::Yarn => ("yarn", "add -D"),
            PackageManager::Pnpm => ("pnpm", "add -D"),
            PackageManager::Bun => ("bun", "add -D"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name } => {
            create_project(name).await?;
        }
        Commands::Add { feature } => {
            add_feature(feature).await?;
        }
        Commands::List => {
            list_features()?;
        }
    }

    Ok(())
}

fn list_features() -> Result<()> {
    let features = vec![("shadcn", "Add shadcn/ui components and configuration")];

    println!("\n{}", style("Available Features:").cyan().bold());
    println!("{}", style("----------------").cyan());

    for (name, description) in features {
        println!("{} - {}", style(name).green().bold(), description);
    }

    println!("\n{}", style("Usage:").cyan().bold());
    println!("  nstack add --feature <feature-name>");
    println!("  nstack add (for interactive selection)");

    Ok(())
}

async fn create_project(name: Option<String>) -> Result<()> {
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

async fn add_feature(feature: Option<String>) -> Result<()> {
    let features = vec!["shadcn"];

    let selected_feature = match feature {
        Some(f) => {
            if !features.contains(&f.as_str()) {
                anyhow::bail!(
                    "Invalid feature. Available features: {}",
                    features.join(", ")
                );
            }
            f
        }
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a feature to add")
                .items(&features)
                .interact()?;
            features[selection].to_string()
        }
    };

    println!(
        "{}",
        style(format!("Adding {}...", selected_feature)).cyan()
    );

    match selected_feature.as_str() {
        "shadcn" => add_shadcn().await?,
        _ => unreachable!(),
    }

    Ok(())
}

async fn add_shadcn() -> Result<()> {
    let package_manager = PackageManager::detect()?;
    let project_structure = ProjectStructure::detect()?;

    println!(
        "{}",
        style(format!(
            "Using package manager: {}",
            format!("{:?}", package_manager).to_lowercase()
        ))
        .yellow()
    );
    println!(
        "{}",
        style(format!(
            "Project structure: {}",
            format!("{:?}", project_structure).to_lowercase()
        ))
        .yellow()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_message("Installing shadcn/ui dependencies...");

    // Install required dependencies
    let (cmd, install) = package_manager.install_command();
    Command::new(cmd)
        .args([
            install,
            "class-variance-authority",
            "clsx",
            "tailwind-merge",
            "lucide-react",
            "tw-animate-css",
        ])
        .status()
        .context("Failed to install shadcn/ui dependencies")?;

    pb.set_message("Setting up configuration files...");

    // Create components.json
    let components_json = r#"{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "new-york",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "app/globals.css",
    "baseColor": "neutral",
    "cssVariables": true,
    "prefix": ""
  },
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  },
  "iconLibrary": "lucide"
}"#;

    std::fs::write("components.json", components_json)
        .context("Failed to create components.json")?;

    // Create utils.ts
    let utils_ts = r#"import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
 
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}"#;

    let lib_path = project_structure.get_lib_path();
    std::fs::create_dir_all(lib_path).context("Failed to create lib directory")?;
    std::fs::write(format!("{}/utils.ts", lib_path), utils_ts)
        .context("Failed to create utils.ts")?;

    // Update globals.css
    let globals_css = r#"@import "tailwindcss";
@import "tw-animate-css";

@custom-variant dark (&:is(.dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.577 0.245 27.325);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --radius: 0.625rem;
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.145 0 0);
  --sidebar-primary: oklch(0.205 0 0);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.97 0 0);
  --sidebar-accent-foreground: oklch(0.205 0 0);
  --sidebar-border: oklch(0.922 0 0);
  --sidebar-ring: oklch(0.708 0 0);
}

.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.145 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.145 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.985 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.396 0.141 25.723);
  --destructive-foreground: oklch(0.637 0.237 25.331);
  --border: oklch(0.269 0 0);
  --input: oklch(0.269 0 0);
  --ring: oklch(0.439 0 0);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.205 0 0);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.269 0 0);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(0.269 0 0);
  --sidebar-ring: oklch(0.439 0 0);
}

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --color-chart-1: var(--chart-1);
  --color-chart-2: var(--chart-2);
  --color-chart-3: var(--chart-3);
  --color-chart-4: var(--chart-4);
  --color-chart-5: var(--chart-5);
  --radius-sm: calc(var(--radius) - 4px);
  --radius-md: calc(var(--radius) - 2px);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) + 4px);
  --color-sidebar: var(--sidebar);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-ring: var(--sidebar-ring);
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }
}"#;

    let globals_css_path = project_structure.get_globals_css_path();
    std::fs::write(globals_css_path, globals_css)
        .context("Failed to update globals.css")?;

    pb.finish_with_message("shadcn/ui installed successfully!");
    println!("\n{}", style("Next steps:").green());
    println!("1. Create a components directory: mkdir components");
    println!("2. Add components using: npx shadcn@latest add <component-name>");
    println!("3. Import and use components in your app");

    Ok(())
}
