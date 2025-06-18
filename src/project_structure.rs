use anyhow::Result;
use std::path::Path;

#[derive(Debug)]
pub enum ProjectStructure {
    AppDir,
    SrcDir,
}

impl ProjectStructure {
    pub fn detect() -> Result<Self> {
        if Path::new("app").exists() {
            Ok(ProjectStructure::AppDir)
        } else if Path::new("src").exists() {
            Ok(ProjectStructure::SrcDir)
        } else {
            anyhow::bail!("Could not detect project structure. Neither 'app' nor 'src' directory found.")
        }
    }

    pub fn get_globals_css_path(&self) -> &'static str {
        match self {
            ProjectStructure::AppDir => "app/globals.css",
            ProjectStructure::SrcDir => "src/app/globals.css",
        }
    }

    pub fn get_lib_path(&self) -> &'static str {
        match self {
            ProjectStructure::AppDir => "lib",
            ProjectStructure::SrcDir => "src/lib",
        }
    }

    pub fn get_db_path(&self) -> &'static str {
        match self {
            ProjectStructure::AppDir => "db",
            ProjectStructure::SrcDir => "src/db",
        }
    }

    pub fn is_app_router(&self) -> bool {
        matches!(self, ProjectStructure::AppDir)
    }
}
