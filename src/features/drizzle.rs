use anyhow::{Result, Context};
use console::style;
use indicatif::ProgressBar;
use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;
use crate::project_structure::ProjectStructure;
use crate::package_manager::PackageManager;

#[derive(Debug, Clone)]
pub enum DatabaseProvider {
    PostgreSQL,
    Neon,
}

impl DatabaseProvider {
    fn as_str(&self) -> &'static str {
        match self {
            DatabaseProvider::PostgreSQL => "PostgreSQL",
            DatabaseProvider::Neon => "Neon",
        }
    }

    fn get_dependencies(&self) -> Vec<&'static str> {
        match self {
            DatabaseProvider::PostgreSQL => vec!["drizzle-orm", "pg", "dotenv"],
            DatabaseProvider::Neon => vec!["drizzle-orm", "@neondatabase/serverless", "dotenv"],
        }
    }

    fn get_dev_dependencies(&self) -> Vec<&'static str> {
        match self {
            DatabaseProvider::PostgreSQL => vec!["drizzle-kit", "tsx", "@types/pg"],
            DatabaseProvider::Neon => vec!["drizzle-kit", "tsx"],
        }
    }

    fn get_connection_code(&self) -> &'static str {
        match self {
            DatabaseProvider::PostgreSQL => r#"import 'dotenv/config';
import { drizzle } from 'drizzle-orm/node-postgres';
import { Pool } from 'pg';
import * as schema from './schema';

const pool = new Pool({
  connectionString: process.env.DATABASE_URL!,
});

export const db = drizzle(pool, { schema });"#,
            DatabaseProvider::Neon => r#"import 'dotenv/config';
import { drizzle } from 'drizzle-orm/neon-http';
import { neon } from '@neondatabase/serverless';
import * as schema from './schema';

const sql = neon(process.env.DATABASE_URL!);
export const db = drizzle(sql, { schema });"#,
        }
    }

    fn get_env_template(&self) -> &'static str {
        match self {
            DatabaseProvider::PostgreSQL => r#"# Database
DATABASE_URL="postgresql://username:password@localhost:5432/your_database"

# Add your other environment variables here"#,
            DatabaseProvider::Neon => r#"# Database
DATABASE_URL="postgresql://username:password@your-neon-db.neon.tech/your_database"

# Add your other environment variables here"#,
        }
    }

    fn get_description(&self) -> &'static str {
        match self {
            DatabaseProvider::PostgreSQL => "Traditional PostgreSQL database (local or hosted)",
            DatabaseProvider::Neon => "Neon serverless PostgreSQL database",
        }
    }
}

pub async fn add_drizzle() -> Result<()> {
    let package_manager = PackageManager::from_project_config()?;
    let project_structure = ProjectStructure::detect()?;

    println!(
        "{}",
        style(format!(
            "Using package manager: {}",
            package_manager.to_string()
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

    // Interactive database provider selection
    let providers = vec![DatabaseProvider::PostgreSQL, DatabaseProvider::Neon];
    let provider_names: Vec<String> = providers.iter()
        .map(|p| format!("{} - {}", p.as_str(), p.get_description()))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select your database provider")
        .default(0)
        .items(&provider_names)
        .interact()?;

    let selected_provider = &providers[selection];

    println!(
        "{}",
        style(format!("Selected: {}", selected_provider.as_str())).green().bold()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_message(format!("Installing Drizzle ORM dependencies for {}...", selected_provider.as_str()));

    // Install required dependencies
    let (cmd, install) = package_manager.install_command();
    let dependencies = selected_provider.get_dependencies();
    let mut install_args = vec![cmd, install];
    install_args.extend(dependencies.iter().map(|&s| s));
    
    Command::new(&install_args[0])
        .args(&install_args[1..])
        .status()
        .context(format!("Failed to install Drizzle ORM dependencies for {}", selected_provider.as_str()))?;

    // Install dev dependencies
    let (cmd, install_dev) = package_manager.install_dev_command();
    let dev_dependencies = selected_provider.get_dev_dependencies();
    let mut install_dev_args = vec![cmd, install_dev];
    install_dev_args.extend(dev_dependencies.iter().map(|&s| s));
    
    Command::new(&install_dev_args[0])
        .args(&install_dev_args[1..])
        .status()
        .context(format!("Failed to install Drizzle dev dependencies for {}", selected_provider.as_str()))?;

    pb.set_message("Setting up Drizzle configuration...");

    // Create drizzle.config.ts (same for all providers)
    let drizzle_config = r#"import 'dotenv/config';
import { defineConfig } from 'drizzle-kit';

export default defineConfig({
  out: './drizzle',
  schema: './src/db/schema.ts',
  dialect: 'postgresql',
  dbCredentials: {
    url: process.env.DATABASE_URL!,
  },
});"#;

    std::fs::write("drizzle.config.ts", drizzle_config)
        .context("Failed to create drizzle.config.ts")?;

    pb.set_message("Creating database schema and configuration...");

    // Create db directory and files
    let db_path = project_structure.get_db_path();
    std::fs::create_dir_all(&db_path).context("Failed to create db directory")?;

    // Create schema.ts (same for all providers)
    let schema_ts = r#"import { integer, pgTable, varchar, text, timestamp } from "drizzle-orm/pg-core";

// Users table
export const usersTable = pgTable("users", {
  id: integer("id").primaryKey().generatedAlwaysAsIdentity(),
  name: varchar("name", { length: 255 }).notNull(),
  email: varchar("email", { length: 255 }).notNull().unique(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  updatedAt: timestamp("updated_at").defaultNow().notNull(),
});

// Posts table
export const postsTable = pgTable("posts", {
  id: integer("id").primaryKey().generatedAlwaysAsIdentity(),
  title: text("title").notNull(),
  content: text("content").notNull(),
  authorId: integer("author_id").references(() => usersTable.id),
  createdAt: timestamp("created_at").defaultNow().notNull(),
  updatedAt: timestamp("updated_at").defaultNow().notNull(),
});

// Export types
export type User = typeof usersTable.$inferSelect;
export type NewUser = typeof usersTable.$inferInsert;
export type Post = typeof postsTable.$inferSelect;
export type NewPost = typeof postsTable.$inferInsert;"#;

    std::fs::write(format!("{}/schema.ts", db_path), schema_ts)
        .context("Failed to create schema.ts")?;

    // Create index.ts with provider-specific connection
    let index_ts = selected_provider.get_connection_code();
    std::fs::write(format!("{}/index.ts", db_path), index_ts)
        .context("Failed to create index.ts")?;

    // Create migrations directory
    std::fs::create_dir_all("drizzle").context("Failed to create drizzle directory")?;

    pb.set_message("Updating package.json scripts...");

    // Update package.json scripts
    let package_json_path = "package.json";

    if std::path::Path::new(package_json_path).exists() {
        let package_json_content = std::fs::read_to_string(package_json_path)
            .context("Failed to read package.json")?;

        // Add Drizzle scripts if they don't exist
        if !package_json_content.contains("\"db:generate\"") {
            let updated_content = package_json_content.replace(
                "\"scripts\": {",
                r#""scripts": {
    "db:generate": "drizzle-kit generate",
    "db:migrate": "drizzle-kit migrate",
    "db:studio": "drizzle-kit studio",
    "db:push": "drizzle-kit push","#,
            );
            std::fs::write(package_json_path, updated_content)
                .context("Failed to update package.json")?;
        }
    }

    pb.set_message("Creating environment variables template...");

    // Create or update .env file with provider-specific template
    let env_content = selected_provider.get_env_template();
    let env_path = ".env";
    if !std::path::Path::new(env_path).exists() {
        std::fs::write(env_path, env_content)
            .context("Failed to create .env")?;
    } else {
        // Append to existing .env if DATABASE_URL doesn't exist
        let existing_content = std::fs::read_to_string(env_path)
            .context("Failed to read .env")?;
        
        if !existing_content.contains("DATABASE_URL") {
            let updated_content = format!("{}\n\n{}", existing_content, env_content);
            std::fs::write(env_path, updated_content)
                .context("Failed to update .env")?;
        }
    }

    // Create example API route for database operations
    let api_path = if project_structure.is_app_router() {
        "src/app/api/users/route.ts"
    } else {
        "src/pages/api/users.ts"
    };

    std::fs::create_dir_all(std::path::Path::new(api_path).parent().unwrap())
        .context("Failed to create API directory")?;

    let api_route_content = if project_structure.is_app_router() {
        r#"import { NextRequest, NextResponse } from "next/server";
import { db } from "@/db";
import { usersTable } from "@/db/schema";
import { eq } from "drizzle-orm";

export async function GET() {
  try {
    const allUsers = await db.select().from(usersTable);
    return NextResponse.json(allUsers);
  } catch (error) {
    return NextResponse.json({ error: "Failed to fetch users" }, { status: 500 });
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const newUser = await db.insert(usersTable).values(body).returning();
    return NextResponse.json(newUser[0]);
  } catch (error) {
    return NextResponse.json({ error: "Failed to create user" }, { status: 500 });
  }
}"#
    } else {
        r#"import type { NextApiRequest, NextApiResponse } from "next";
import { db } from "@/db";
import { usersTable } from "@/db/schema";
import { eq } from "drizzle-orm";

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method === "GET") {
    try {
      const allUsers = await db.select().from(usersTable);
      res.status(200).json(allUsers);
    } catch (error) {
      res.status(500).json({ error: "Failed to fetch users" });
    }
  } else if (req.method === "POST") {
    try {
      const newUser = await db.insert(usersTable).values(req.body).returning();
      res.status(201).json(newUser[0]);
    } catch (error) {
      res.status(500).json({ error: "Failed to create user" });
    }
  } else {
    res.setHeader("Allow", ["GET", "POST"]);
    res.status(405).end(`Method ${req.method} Not Allowed`);
  }
}"#
    };

    std::fs::write(api_path, api_route_content)
        .context("Failed to create API route")?;

    // Create example usage file
    let example_path = "src/example-usage.ts";
    let example_content = match selected_provider {
        DatabaseProvider::PostgreSQL => r#"import 'dotenv/config';
import { drizzle } from 'drizzle-orm/node-postgres';
import { eq } from 'drizzle-orm';
import { usersTable } from './db/schema';
  
const db = drizzle(process.env.DATABASE_URL!);

async function main() {
  const user: typeof usersTable.$inferInsert = {
    name: 'John Doe',
    email: 'john@example.com',
  };

  await db.insert(usersTable).values(user);
  console.log('New user created!')

  const users = await db.select().from(usersTable);
  console.log('Getting all users from the database: ', users)

  await db
    .update(usersTable)
    .set({
      name: 'John Updated',
    })
    .where(eq(usersTable.email, user.email));
  console.log('User info updated!')

  await db.delete(usersTable).where(eq(usersTable.email, user.email));
  console.log('User deleted!')
}

main();"#,
        DatabaseProvider::Neon => r#"import 'dotenv/config';
import { drizzle } from 'drizzle-orm/neon-http';
import { eq } from 'drizzle-orm';
import { usersTable } from './db/schema';
  
const db = drizzle(process.env.DATABASE_URL!);

async function main() {
  const user: typeof usersTable.$inferInsert = {
    name: 'John Doe',
    email: 'john@example.com',
  };

  await db.insert(usersTable).values(user);
  console.log('New user created!')

  const users = await db.select().from(usersTable);
  console.log('Getting all users from the database: ', users)

  await db
    .update(usersTable)
    .set({
      name: 'John Updated',
    })
    .where(eq(usersTable.email, user.email));
  console.log('User info updated!')

  await db.delete(usersTable).where(eq(usersTable.email, user.email));
  console.log('User deleted!')
}

main();"#,
    };

    std::fs::write(example_path, example_content)
        .context("Failed to create example usage file")?;

    pb.finish_with_message(format!("Drizzle ORM setup completed for {}!", selected_provider.as_str()));

    println!("\n{}", style(format!("✅ Drizzle ORM has been successfully set up for {}!", selected_provider.as_str())).green().bold());
    println!("\n{}", style("Next steps:").cyan().bold());
    println!("1. Update your DATABASE_URL in .env");
    println!("2. Run 'npm run db:push' to push the schema to your database");
    println!("3. Run 'npm run db:generate' to generate migrations");
    println!("4. Run 'npm run db:studio' to open Drizzle Studio");
    println!("5. Test with: npx tsx src/example-usage.ts");
    println!("\n{}", style("Files created:").cyan().bold());
    println!("• drizzle.config.ts - Drizzle configuration");
    println!("• src/db/schema.ts - Database schema");
    println!("• src/db/index.ts - Database connection");
    println!("• src/app/api/users/route.ts - Example API route");
    println!("• src/example-usage.ts - Example usage file");
    println!("• .env - Environment variables template");
    println!("\n{}", style("Provider-specific details:").cyan().bold());
    println!("• Database: {}", selected_provider.as_str());
    println!("• Connection: {}", match selected_provider {
        DatabaseProvider::PostgreSQL => "node-postgres (pg)",
        DatabaseProvider::Neon => "neon-http serverless",
    });

    Ok(())
} 