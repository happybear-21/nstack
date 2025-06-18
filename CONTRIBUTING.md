# Contributing to nstack

Thank you for your interest in contributing to nstack! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contributing Guidelines](#contributing-guidelines)
- [Feature Development](#feature-development)
- [Testing](#testing)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)
- [Reporting Bugs](#reporting-bugs)
- [Requesting Features](#requesting-features)
- [Community](#community)
- [Roadmap](#roadmap)

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

### Our Standards

- **Be respectful and inclusive** - Use welcoming and inclusive language
- **Be collaborative** - Work together to achieve common goals
- **Be constructive** - Provide constructive feedback and suggestions
- **Be patient** - Remember that contributors have different levels of experience

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- A code editor (VS Code, IntelliJ IDEA, etc.)

### Fork and Clone

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/nstack.git
   cd nstack
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/happybear-21/nstack.git
   ```

## Development Setup

### Initial Setup

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
cargo fmt

# Install pre-commit hooks (optional)
cargo install cargo-husky
cargo husky install
```

### Running the CLI

```bash
# Run in development mode
cargo run -- create
cargo run -- add
cargo run -- list

# Run with specific features
cargo run -- add --feature shadcn
cargo run -- add --feature drizzle
```

### Project Structure

```
nstack/
├── src/
│   ├── main.rs              # Application entry point
│   ├── cli.rs               # CLI argument parsing
│   ├── commands/            # Command implementations
│   │   ├── create.rs        # Project creation
│   │   ├── add.rs           # Feature addition
│   │   └── mod.rs
│   ├── features/            # Feature implementations
│   │   ├── shadcn.rs        # shadcn/ui integration
│   │   ├── drizzle.rs       # Drizzle ORM integration
│   │   ├── magicui.rs       # Magic UI integration
│   │   └── mod.rs
│   ├── package_manager.rs   # Package manager detection
│   └── project_structure.rs # Project structure detection
├── tests/                   # Integration tests
├── examples/                # Example projects
└── docs/                    # Documentation
```

## Contributing Guidelines

### Before You Start

1. **Check existing issues** - Search for similar issues or feature requests
2. **Discuss your idea** - Open a discussion or issue to get feedback
3. **Read the documentation** - Understand the current architecture and patterns

### Types of Contributions

We welcome various types of contributions:

- **Bug fixes** - Fix issues and improve stability
- **New features** - Add new functionality
- **Documentation** - Improve docs, examples, and guides
- **Tests** - Add or improve test coverage
- **Tooling** - Improve development tools and CI/CD
- **Localization** - Add translations and internationalization
- **UI/UX** - Improve user experience and interface

## Feature Development

### Adding New Features

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Follow the feature structure**:
   - Add feature logic in `src/features/`
   - Update command handling in `src/commands/`
   - Add tests in `tests/`
   - Update documentation

3. **Example feature structure**:
   ```rust
   // src/features/your_feature.rs
   use anyhow::Result;
   
   pub async fn add_your_feature() -> Result<()> {
       // Your feature implementation
       Ok(())
   }
   ```

### Adding New Database Providers

To add a new database provider to Drizzle:

1. **Update the enum** in `src/features/drizzle.rs`:
   ```rust
   pub enum DatabaseProvider {
       // ... existing providers
       YourProvider,
   }
   ```

2. **Implement required methods**:
   - `as_str()` - Display name
   - `get_dependencies()` - Required packages
   - `get_dev_dependencies()` - Dev packages
   - `get_connection_code()` - Connection setup
   - `get_schema_code()` - Schema definition
   - `get_env_template()` - Environment variables
   - `get_description()` - Provider description

3. **Add to provider list**:
   ```rust
   let providers = vec![
       // ... existing providers
       DatabaseProvider::YourProvider,
   ];
   ```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_functionality() {
        // Your test implementation
        assert!(true);
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test implementation
        assert!(true);
    }
}
```

### Test Guidelines

- **Unit tests** for individual functions
- **Integration tests** for feature workflows
- **Error handling tests** for edge cases
- **Mock external dependencies** when possible
- **Test both success and failure paths**

## Code Style

### Rust Style Guidelines

1. **Follow Rust conventions**:
   ```bash
   cargo fmt
   cargo clippy
   ```

2. **Use meaningful names**:
   ```rust
   // Good
   let user_count = users.len();
   
   // Bad
   let c = users.len();
   ```

3. **Add documentation**:
   ```rust
   /// Adds a new feature to the project
   /// 
   /// # Arguments
   /// * `feature_name` - The name of the feature to add
   /// 
   /// # Returns
   /// * `Result<()>` - Success or error
   pub async fn add_feature(feature_name: &str) -> Result<()> {
       // Implementation
   }
   ```

4. **Handle errors properly**:
   ```rust
   use anyhow::{Context, Result};
   
   pub fn process_data() -> Result<()> {
       let data = read_file("config.json")
           .context("Failed to read config file")?;
       // Process data
       Ok(())
   }
   ```

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes
- `refactor` - Code refactoring
- `test` - Test changes
- `chore` - Build/tooling changes

**Examples:**
```
feat(drizzle): add Bun SQL database provider support

fix(cli): resolve import statement syntax errors

docs(readme): add comprehensive installation guide
```

## Pull Request Process

### Before Submitting

1. **Ensure tests pass**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

2. **Update documentation** if needed

3. **Test your changes** manually

4. **Rebase on main**:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

### PR Guidelines

1. **Clear title** describing the change
2. **Detailed description** of what was changed and why
3. **Link related issues** using keywords
4. **Include screenshots** for UI changes
5. **Add tests** for new functionality
6. **Update documentation** if needed

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Other (please describe)

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated

## Related Issues
Closes #123
```

## Reporting Bugs

### Before Reporting

1. **Search existing issues** for duplicates
2. **Try to reproduce** the issue
3. **Check the documentation** for solutions

### Bug Report Template

```markdown
## Bug Description
Clear description of the bug

## Steps to Reproduce
1. Step 1
2. Step 2
3. Step 3

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: [e.g., macOS 14.0]
- Rust version: [e.g., 1.75.0]
- nstack version: [e.g., 0.1.0]

## Additional Information
- Error messages
- Screenshots
- Logs
```

## Requesting Features

### Feature Request Template

```markdown
## Feature Description
Clear description of the feature

## Use Case
Why this feature is needed

## Proposed Solution
How you think it should work

## Alternatives Considered
Other approaches you've considered

## Additional Context
Any other relevant information
```

## Community

### Getting Help

- **GitHub Issues** - For bugs and feature requests
- **GitHub Discussions** - For questions and general discussion
- **Documentation** - Check the README and Wiki

### Recognition

Contributors will be recognized in:
- **README.md** - For significant contributions
- **Release notes** - For each release
- **Contributors list** - On GitHub

### Mentorship

New contributors can:
- Ask for help in discussions
- Request code reviews
- Join community calls (if available)

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
- [ ] PWA support
- [ ] Internationalization (i18n)

### Database Providers
- [ ] MySQL support for Drizzle
- [ ] SQLite support for Drizzle
- [ ] MongoDB support
- [ ] Redis integration

## License

By contributing to nstack, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to nstack!** 