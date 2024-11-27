use clap::{Command, Arg};
use walkdir::WalkDir;
use syn::{Stmt};
use syn::{visit::Visit};

use std::fs;
use std::path::Path;

#[derive(Default)]
struct CodeMetrics {
    loc: usize,
    kloc: f64,
    cyclomatic_complexity: usize,
    functions: usize,
}

struct CyclomaticComplexityVisitor {
    complexity: usize,
}

impl CyclomaticComplexityVisitor {
    fn new() -> Self {
        Self { complexity: 1 } // Start with 1 (entry point)
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
        syn::visit::visit_stmt(self, stmt);
    }
}

fn analyze_file(file_path: &Path) -> CodeMetrics {
    let mut metrics = CodeMetrics::default();
    
    if let Ok(content) = fs::read_to_string(file_path) {
        metrics.loc = content.lines().count();
        
        if let Ok(syntax) = syn::parse_file(&content) {
            for item in syntax.items {
                if let syn::Item::Fn(func) = item {
                    metrics.functions += 1;
                    let mut visitor = CyclomaticComplexityVisitor::new();
                    visitor.visit_item_fn(&func);
                    metrics.cyclomatic_complexity += visitor.complexity;
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
            total_metrics.loc += file_metrics.loc;
            total_metrics.cyclomatic_complexity += file_metrics.cyclomatic_complexity;
            total_metrics.functions += file_metrics.functions;
        }
    }

    total_metrics.kloc = total_metrics.loc as f64 / 1000.0;
    total_metrics
}

fn main() {
    let matches = Command::new("Code Metrics Tool")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Calculates code metrics such as cyclomatic complexity and KLOC")
        .arg(
            Arg::new("path")
                .help("Path to the directory or file to analyze")
                .required(true)
                .index(1),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();

    let metrics = calculate_metrics(path);

    println!("Code Metrics:");
    println!("Lines of Code (LOC): {}", metrics.loc);
    println!("KLOC: {:.2}", metrics.kloc);
    println!("Cyclomatic Complexity: {}", metrics.cyclomatic_complexity);
    println!("Number of Functions: {}", metrics.functions);

    if metrics.cyclomatic_complexity > 10 {
        println!("Warning: High cyclomatic complexity. Consider refactoring!");
    }
}
