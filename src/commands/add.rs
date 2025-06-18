use anyhow::Result;
use dialoguer::{Select, theme::ColorfulTheme};

// Import each feature module here
use crate::features::shadcn;

pub async fn add_feature(feature: Option<String>) -> Result<()> {
    let features = vec!["shadcn" /*, "other_feature" */];
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
        // "other_feature" => other_feature::add_other().await?,
        _ => {
            println!("Unknown feature: {}", selected_feature);
        }
    }
    Ok(())
}

