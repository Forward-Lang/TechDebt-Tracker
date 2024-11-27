# TechDebt-Tracker

**TechDebt-Tracker** is a command-line tool designed to help developers track and manage technical debt in their codebases. It calculates various code metrics such as Cyclomatic Complexity, KLOC (thousands of lines of code), and other important maintainability metrics. This tool helps you identify areas of your code that are harder to maintain, prone to bugs, or in need of refactoring.

## Features

- **Cyclomatic Complexity**: Measures the complexity of your code by counting the number of linearly independent paths through the program.
- **KLOC**: Estimates the size of your codebase in thousands of lines of code.
- **Maintainability Index**: Provides a numeric value representing the maintainability of your code.
- **Halstead Complexity Measures**: Calculates software metrics like volume, difficulty, and effort based on the operations in the code.
- **Risk of Errors**: Tracks code patterns that indicate higher risk of errors, making it easier to target refactoring efforts.
- **Easy Integration**: Can be used in CI/CD pipelines to automatically flag code that may need refactoring or further attention.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) installed on your system.
  
### Build from Source

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/TechDebt-Tracker.git
    cd TechDebt-Tracker
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. Run the tool:
    ```bash
    ./target/release/techdebt-tracker --help
    ```

> [!NOTE]  
> Not yet implemented: TODO

Alternatively, you can install it using Cargo directly:
```bash
cargo install techdebt-tracker
```

---

## Usage

### Basic Command

Run the tool to analyze a specific directory or file:

```bash
techdebt-tracker <path>
```

Where `<path>` is the path to the file or directory you want to analyze. The tool will calculate various metrics and display them in the console.

### Example

```bash
techdebt-tracker ./src
```

This command will analyze the `./src` directory and calculate code metrics like cyclomatic complexity, KLOC, and others for all files within it.

### Available Arguments

- `path`: **Required**. The path to the source code files or directory you want to analyze.

### Example Output

```bash
Analyzing path: ./src

Cyclomatic Complexity: 15
KLOC (thousands of lines): 2.1
Maintainability Index: 68
Halstead Volume: 1100
Halstead Difficulty: 35
Halstead Effort: 38500
Risk of Errors: High (complex code detected)
```

---

## Integration with CI/CD

You can integrate **TechDebt-Tracker** into your CI/CD pipeline to monitor technical debt over time. Here’s an example using a simple GitHub Actions workflow:

### Example GitHub Actions Workflow

```yaml
name: Analyze Code Metrics

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  analysis:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v2

    - name: Install Rust
      uses: actions/setup-rust@v1
      with:
        rust-version: 'stable'

    # Not yet implemented: TODO
    - name: Build and run TechDebt-Tracker
      run: |
        cargo install --path .
        techdebt-tracker ./src
```

---

## Contributing

Contributions are welcome! If you want to contribute to the **TechDebt-Tracker** project, follow these steps:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Make your changes and commit them (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature-name`).
5. Create a new Pull Request.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- **Clap**: For building the command-line interface.
- **Syn**: For parsing Rust code and extracting useful metrics.
