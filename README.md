# nstack

A CLI tool for scaffolding and enhancing Next.js projects with modern features.

## Installation

### From Source
```bash
# Clone the repository
git clone https://github.com/happybear-21/nstack.git
cd nstack

# Build and install
cargo build --release
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
```

### List available features

```bash
nstack list
```

## Available Features

- **shadcn**: Adds shadcn/ui components and configuration

## Development

```bash
# Build the project
cargo build

# Run in development mode
cargo run -- create
cargo run -- add
cargo run -- list
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 