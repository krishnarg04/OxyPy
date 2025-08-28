# Contributing to OxyPy

## Development Setup

### Prerequisites
- Rust (2021 edition or later)
- Cargo package manager
- Git

### Getting Started

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/OxyPy.git
   cd OxyPy
   ```

2. **Set up the development environment**
   ```bash
   cargo build
   cargo test  # Run tests (if available)
   ```

3. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Testing Your Changes

**Interactive Testing:**
```bash
cargo run
>> let x: i32 = 42
>> print(x)
```

**File Testing:**
Create a test file and run:
```bash
echo 'print("Hello, World!")' > test.lang
cargo run test.lang
```

### Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting
- Add comments for complex logic

### Architecture Guidelines

When adding features, consider:

1. **Tokenizer Changes** (`tokenizer.rs`)
   - Add new token types for new keywords/operators
   - Update token recognition logic

2. **Parser Changes** (`AstTree.rs`)
   - Add new AST node types
   - Implement parsing rules
   - Handle operator precedence

3. **Runtime Changes** (`runtime.rs`)
   - Implement execution logic for new features
   - Handle environment updates

4. **Built-in Functions** (`Functions.rs`)
   - Add new built-in functions
   - Ensure proper error handling

## Contribution Areas

### High Priority
- [ ] Error handling improvements
- [ ] More comprehensive test suite
- [ ] Better error messages
- [ ] Performance optimizations

### Medium Priority
- [ ] Module system and imports
- [ ] File I/O operations
- [ ] More built-in data structures
- [ ] Standard library expansion

### Low Priority
- [ ] IDE integration
- [ ] Debugging support
- [ ] Package manager
- [ ] Documentation improvements

## Submitting Changes

1. **Ensure your code works**
   ```bash
   cargo build
   cargo run  # Test REPL
   ```

2. **Create meaningful commits**
   ```bash
   git add .
   git commit -m "Add: New feature description"
   ```

3. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Create a Pull Request**
   - Describe what your changes do
   - Explain why the changes are needed
   - Include examples if applicable

## Code Review Process

- All changes require review before merging
- Be responsive to feedback
- Keep discussions constructive and focused
- Update documentation for user-facing changes

## Getting Help

- Open an issue for questions
- Check existing issues and documentation
- Reach out to maintainers for guidance

## Recognition

Contributors will be acknowledged in the project documentation and releases.