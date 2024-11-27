# **🚀 TechDebt-Tracker** 🚀

**TechDebt-Tracker** is a powerful command-line tool that helps developers **track and manage technical debt** in their codebases. It calculates key **code metrics** like Cyclomatic Complexity, KLOC (thousands of lines of code), and many others to help you identify **areas of your code that need attention**. Whether you're working solo or as part of a team, **TechDebt-Tracker** gives you the insights you need to maintain a **cleaner, more maintainable codebase**.

⚡ **Don't let technical debt slow you down – track it, manage it, and refactor it!**

## 🌟 **Features** 🌟

- **🔢 Cyclomatic Complexity**: Measures the complexity of your code and identifies the parts that might be hard to maintain or prone to errors.
- **📏 KLOC**: Shows the size of your codebase in thousands of lines, helping you estimate the scale of your project.
- **📉 Maintainability Index**: Provides a numeric value indicating how maintainable your code is. The higher, the better!
- **🔍 Halstead Complexity Measures**: Calculates various software metrics such as volume, difficulty, and effort based on operations in your code.
- **⚠️ Risk of Errors**: Flags areas of code that are more prone to errors due to high complexity.
- **📈 Easy Integration**: Integrate easily with CI/CD pipelines to automatically flag **code debt** and prompt for necessary refactoring.

With **TechDebt-Tracker**, you'll be able to keep your codebase **clean, maintainable**, and **error-free** for the long term! 💪

## 🔧 **Installation** 🔧

### **Prerequisites** 📋

- [Rust](https://www.rust-lang.org/learn/get-started) installed on your system.

### **Build from Source** 🔨

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

Alternatively, you can install **TechDebt-Tracker** directly from Cargo:

> [!NOTE]  
> Not yet implemented: TODO

```bash
cargo install techdebt-tracker
```

## 🏃‍♂️ **Usage**

### **Basic Command** 🖥️

Run the tool to analyze a specific **directory** or **file**:

```bash
techdebt-tracker <path>
```

Where `<path>` is the path to the **file** or **directory** you want to analyze.

### **Example** 🔍

```bash
techdebt-tracker ./src
```

This command will analyze the `./src` directory and calculate important metrics like **Cyclomatic Complexity**, **KLOC**, and more for all files within it. 📊

### **Available Arguments** ⚙️

- `path`: **Required**. The path to the source code files or directory you want to analyze.

### **Example Output** 💡

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

## 🤖 **CI/CD Integration**

**TechDebt-Tracker** is perfect for integrating into your **CI/CD pipeline** to ensure that **technical debt** is automatically tracked over time. Here's an example using **GitHub Actions**:

### **Example GitHub Actions Workflow** ⚙️

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

This will automatically run **TechDebt-Tracker** every time changes are pushed or a pull request is made to the `main` branch, ensuring **code quality** is always monitored. 🚀

## 🧑‍🤝‍🧑 **Contributing**

We would **love** your help in making **TechDebt-Tracker** even better! Whether you have a bug fix, a feature idea, or just want to improve the documentation, your contributions are always welcome. 🌟

### **How to Contribute** 👩‍💻👨‍💻

1. Fork the repository 🍴
2. Create a new branch (`git checkout -b feature-name`) 🌱
3. Make your changes ✏️
4. Commit your changes (`git commit -am 'Add new feature'`) 📝
5. Push to your branch (`git push origin feature-name`) 🚀
6. Open a **Pull Request** to the `main` branch 🔄

By contributing, you're helping others build **cleaner, more maintainable code** and fostering a healthy developer ecosystem. 🤝

## 📝 **License**

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. ⚖️

## 💖 **Acknowledgments**

A huge thanks to the following libraries and projects that helped make **TechDebt-Tracker** possible:

- **Clap**: For building the amazing command-line interface. 🖥️
- **Syn**: For parsing Rust code and extracting valuable metrics. 📊

---

✨ **TechDebt-Tracker** – Track your tech debt before it tracks you! ✨
