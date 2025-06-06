# nstack

A CLI tool for scaffolding and enhancing Next.js projects with modern features.

## Installation

```bash
cargo install --path .
```

## Usage

### Create a new Next.js project

```bash
nstack create
# or specify a project name
nstack create --name my-app
```

### Add features to your project

```bash
# Interactive mode
nstack add

# Or specify a feature directly
nstack add --feature shadcn
nstack add --feature supabase
nstack add --feature auth
nstack add --feature prisma
```

### List available features

```bash
nstack list
```

## Available Features

- **shadcn**: Adds shadcn/ui components and configuration
- **supabase**: Sets up Supabase client and configuration
- **auth**: Configures authentication with Supabase
- **prisma**: Sets up Prisma ORM with initial configuration

## Development

```bash
# Build the project
cargo build

# Run in development mode
cargo run -- create
cargo run -- add
cargo run -- list
``` 