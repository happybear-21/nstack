pub mod shadcn;
pub mod magicui;

use console::style;
use anyhow::Result;

pub fn list_features() -> Result<()> {
    let features = vec![("shadcn", "Add shadcn/ui components and configuration"),
    ("magicui", "Add magicui components and configuration")];

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