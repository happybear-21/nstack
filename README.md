# nstack

A powerful CLI tool for scaffolding and enhancing Next.js projects with modern features, built in Rust for blazing-fast performance.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Next.js](https://img.shields.io/badge/Next.js-000000?style=for-the-badge&logo=next.js&logoColor=white)](https://nextjs.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

## Beta Version Notice

**This project is currently in BETA version (v0.1.0).** While we strive for stability, there may be bugs, breaking changes, or incomplete features. Use at your own risk in production environments. Please report any issues you encounter on our [GitHub Issues](https://github.com/happybear-21/nstack/issues) page.

## Features

### Database & ORM
- **Drizzle ORM**: Full-stack type-safe database toolkit with support for:
  - **PostgreSQL** - Traditional PostgreSQL database (local or hosted)
  - **Neon** - Serverless PostgreSQL database
  - **Vercel Postgres** - Vercel's serverless PostgreSQL
  - **Supabase** - Open source Firebase alternative
  - **Xata** - Serverless data platform
  - **PGLite** - ElectricSQL's PostgreSQL-compatible database
  - **Nile** - PostgreSQL re-engineered for multi-tenant apps
  - **Bun SQL** - Bun's native PostgreSQL bindings

### UI Components & Styling (Coming Soon)
- **shadcn/ui**: Add beautiful, accessible UI components with Tailwind CSS *(planned for v0.2.0)*
- **Magic UI**: Integrate AI-powered UI components and design system *(planned for v0.3.0)*

## Quick Start

### Installation

#### From Source (Recommended)
```bash
# Clone the repository
git clone https://github.com/happybear-21/nstack.git
cd nstack

# Build and install
cargo build --release
cargo install --path .
```

#### From Cargo (Coming Soon)
```bash
cargo install nstack
```

### Basic Usage

#### Create a new Next.js project
```bash
# Interactive mode
nstack create

# Or specify a project name
nstack create --name my-awesome-app
```

#### Add features to your project
```bash
# Interactive mode - choose from available features
nstack add

# Or specify a feature directly
nstack add --feature drizzle
```

#### List available features
```bash
nstack list
```

## Detailed Usage

### Creating Projects

The `create` command scaffolds a new Next.js project with modern defaults:

```bash
nstack create [OPTIONS]
  --name <NAME>    Project name (optional)
```

**Features included by default:**
- Next.js 14+ with App Router
- TypeScript configuration
- ESLint and Prettier setup
- Tailwind CSS
- Modern project structure

### Adding Features

The `add` command enhances your existing Next.js project with additional features:

```bash
nstack add [OPTIONS]
  --feature <FEATURE>    Feature to add (optional)
```

#### Drizzle ORM Integration
```bash
nstack add --feature drizzle
```
**Interactive database provider selection:**
- Choose from 8 supported database providers
- Automatic dependency installation
- Schema generation
- Migration setup
- Example API routes
- Environment variable templates

**Supported Database Providers:**
- **PostgreSQL** - Traditional PostgreSQL database
- **Neon** - Serverless PostgreSQL database
- **Vercel Postgres** - Vercel's serverless PostgreSQL
- **Supabase** - Open source Firebase alternative
- **Xata** - Serverless data platform
- **PGLite** - ElectricSQL's PostgreSQL-compatible database
- **Nile** - PostgreSQL re-engineered for multi-tenant apps
- **Bun SQL** - Bun's native PostgreSQL bindings

## Development

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/happybear-21/nstack.git
cd nstack

# Build the project
cargo build

# Run in development mode
cargo run -- create
cargo run -- add
cargo run -- list

# Run tests
cargo test

# Check for issues
cargo check
cargo clippy
```

## Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) file for detailed guidelines.

### Quick Contribution Guide
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Setup
```bash
# Fork and clone
git clone https://github.com/YOUR_USERNAME/nstack.git
cd nstack

# Install dependencies
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt
```

## Bug Reports & Issues

Found a bug? Please help us improve by reporting it:

1. **Search existing issues** to avoid duplicates
2. **Create a new issue** with:
   - Clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)
   - Error messages or logs

## Roadmap

### Planned Features
- [ ] **v0.2.0**: shadcn/ui component integration
- [ ] **v0.3.0**: Magic UI component integration
- [ ] Authentication providers (NextAuth.js, Clerk, etc.)
- [ ] State management (Zustand, Redux Toolkit, etc.)
- [ ] Testing frameworks (Jest, Vitest, Playwright)
- [ ] Deployment configurations (Vercel, Netlify, etc.)
- [ ] Performance monitoring tools
- [ ] SEO optimization features
- [ ] Internationalization (i18n)

### Database Providers
- [ ] MySQL support for Drizzle
- [ ] SQLite support for Drizzle
- [ ] MongoDB support
- [ ] Redis integration

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/happybear-21/nstack/issues)
- **Discussions**: [GitHub Discussions](https://github.com/happybear-21/nstack/discussions)
- **Documentation**: [GitHub Wiki](https://github.com/happybear-21/nstack/wiki)
