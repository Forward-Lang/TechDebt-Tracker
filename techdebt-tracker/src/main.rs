use clap::{Command, Arg};
use walkdir::WalkDir;
use syn::{visit::Visit, Stmt};
use std::fs;
use std::path::Path;

#[derive(Default)]
struct CodeMetrics {
    loc: usize,
    kloc: f64,
    cyclomatic_complexity: usize,
    functions: usize,
    comments: usize,
    longest_function_loc: usize,
    max_nesting_depth: usize,
    file_with_max_complexity: String,
    max_file_complexity: usize,
    halstead_operators: usize,
    halstead_operands: usize,
    halstead_unique_operators: usize,
    halstead_unique_operands: usize,
    cyclomatic_distribution: [usize; 3], // [<=5, 6-10, >10]
}

struct CyclomaticComplexityVisitor {
    complexity: usize,
    max_nesting: usize,
    current_nesting: usize,
}

impl CyclomaticComplexityVisitor {
    fn new() -> Self {
        Self {
            complexity: 1, // Start with 1 for the function entry point
            max_nesting: 0,
            current_nesting: 0,
        }
    }
}

impl<'ast> Visit<'ast> for CyclomaticComplexityVisitor {
    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        // Increment complexity for conditional branches
        if matches!(stmt, Stmt::Expr(expr, _) if matches!(
            expr,
            syn::Expr::If(_) | syn::Expr::Match(_) | syn::Expr::While(_) | syn::Expr::ForLoop(_)
        )) {
            self.complexity += 1;
        }

        // Track nesting depth
        if let Stmt::Expr(syn::Expr::Block(_), _) = stmt {
            self.current_nesting += 1;
            self.max_nesting = self.max_nesting.max(self.current_nesting);
        }

        syn::visit::visit_stmt(self, stmt);

        if let Stmt::Expr(syn::Expr::Block(_), _) = stmt {
            self.current_nesting -= 1;
        }
    }
}

fn analyze_file(file_path: &Path) -> CodeMetrics {
    let mut metrics = CodeMetrics::default();

    if let Ok(content) = fs::read_to_string(file_path) {
        metrics.loc = content.lines().count();
        metrics.comments = content.lines().filter(|line| line.trim_start().starts_with("//")).count();

        if let Ok(syntax) = syn::parse_file(&content) {
            for item in syntax.items {
                if let syn::Item::Fn(func) = item {
                    metrics.functions += 1;

                    // Function lines of code
                    let function_loc = func.block.stmts.len();
                    metrics.longest_function_loc = metrics.longest_function_loc.max(function_loc);

                    // Cyclomatic complexity and nesting depth
                    let mut visitor = CyclomaticComplexityVisitor::new();
                    visitor.visit_item_fn(&func);

                    metrics.cyclomatic_complexity += visitor.complexity;
                    metrics.max_nesting_depth = metrics.max_nesting_depth.max(visitor.max_nesting);

                    // Cyclomatic complexity distribution
                    match visitor.complexity {
                        0..=5 => metrics.cyclomatic_distribution[0] += 1,
                        6..=10 => metrics.cyclomatic_distribution[1] += 1,
                        _ => metrics.cyclomatic_distribution[2] += 1,
                    }
                }
            }
        }
    }

    metrics
}

fn calculate_metrics(dir: &str) -> CodeMetrics {
    let mut total_metrics = CodeMetrics::default();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_metrics = analyze_file(path);

            // Aggregate metrics
            total_metrics.loc += file_metrics.loc;
            total_metrics.cyclomatic_complexity += file_metrics.cyclomatic_complexity;
            total_metrics.functions += file_metrics.functions;
            total_metrics.comments += file_metrics.comments;
            total_metrics.longest_function_loc = total_metrics.longest_function_loc.max(file_metrics.longest_function_loc);
            total_metrics.max_nesting_depth = total_metrics.max_nesting_depth.max(file_metrics.max_nesting_depth);

            for i in 0..3 {
                total_metrics.cyclomatic_distribution[i] += file_metrics.cyclomatic_distribution[i];
            }

            // Track file with max complexity
            if file_metrics.cyclomatic_complexity > total_metrics.max_file_complexity {
                total_metrics.max_file_complexity = file_metrics.cyclomatic_complexity;
                total_metrics.file_with_max_complexity = path.to_string_lossy().to_string();
            }
        }
    }

    total_metrics.kloc = total_metrics.loc as f64 / 1000.0;
    total_metrics
}

fn calculate_maintainability_index(metrics: &CodeMetrics) -> f64 {
    if metrics.functions == 0 {
        return 0.0;
    }

    let halstead_volume = (metrics.halstead_unique_operators + metrics.halstead_unique_operands) as f64
        * ((metrics.halstead_unique_operators + metrics.halstead_unique_operands) as f64).log2();
    let avg_cyclomatic = metrics.cyclomatic_complexity as f64 / metrics.functions as f64;

    let index = 171.0 - 5.2 * halstead_volume.log2() - 0.23 * avg_cyclomatic - 16.2 * (metrics.loc as f64).log2();
    index.max(0.0).min(100.0)
}

fn main() {
    let matches = Command::new("Code Metrics Tool")
        .version("2.0")
        .author("Your Name <your.email@example.com>")
        .about("Calculates advanced code metrics such as cyclomatic complexity, maintainability, and risk factors")
        .arg(
            Arg::new("path")
                .help("Path to the directory or file to analyze")
                .required(true)
                .index(1),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let metrics = calculate_metrics(path);
    let maintainability_index = calculate_maintainability_index(&metrics);

    println!("Code Metrics:");
    println!("Lines of Code (LOC): {}", metrics.loc);
    println!("KLOC: {:.2}", metrics.kloc);
    println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity);
    println!("Average Cyclomatic Complexity per Function: {:.2}",
        metrics.cyclomatic_complexity as f64 / metrics.functions as f64);
    println!("Cyclomatic Complexity Distribution: [Easy (<=5): {}, Moderate (6-10): {}, High (>10): {}]",
        metrics.cyclomatic_distribution[0], metrics.cyclomatic_distribution[1], metrics.cyclomatic_distribution[2]);
    println!("Number of Functions: {}", metrics.functions);
    println!("Longest Function (LOC): {}", metrics.longest_function_loc);
    println!("Maximum Nesting Depth: {}", metrics.max_nesting_depth);
    println!("Comment Density: {:.2}%",
        metrics.comments as f64 / metrics.loc as f64 * 100.0);
    println!("Maintainability Index: {:.2} (0-100)", maintainability_index);
    println!("File with Maximum Complexity: {}", metrics.file_with_max_complexity);
    println!("Maximum Cyclomatic Complexity in a File: {}", metrics.max_file_complexity);
}
