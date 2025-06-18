use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

// Import each feature module here
use crate::features::shadcn;
use crate::features::magicui;
use crate::features::drizzle;

pub async fn add_feature(feature: Option<String>) -> Result<()> {
    let features = vec!["shadcn", "magicui", "drizzle"];
    let selected_feature = match feature {
        Some(f) => f,
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a feature to add")
                .default(0)
                .items(&features)
                .interact()?;
            features[selection].to_string()
        }
    };

    match selected_feature.as_str() {
        "shadcn" => shadcn::add_shadcn().await?,
        "magicui" => magicui::add_magicui().await?,
        "drizzle" => drizzle::add_drizzle().await?,
        _ => {
            println!("Unknown feature: {}", selected_feature);
        }
    }
    Ok(())
}

