use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default)]
struct PackageStats {
    package_name: String,
    windows_blocks: usize,
    unix_blocks: usize,
    target_os_windows: usize,
    target_os_unix: usize,
    files_with_windows: Vec<PathBuf>,
    files_with_unix: Vec<PathBuf>,
}

impl PackageStats {
    fn parity_percentage(&self) -> f64 {
        if self.unix_blocks == 0 {
            return 100.0;
        }
        (self.windows_blocks as f64 / self.unix_blocks as f64) * 100.0
    }
}

fn main() -> anyhow::Result<()> {
    let packages_dir = Path::new("packages");
    let mut stats_map: HashMap<String, PackageStats> = HashMap::new();
    
    // Regex patterns
    let cfg_windows = Regex::new(r#"#\[cfg\(windows\)\]"#)?;
    let cfg_unix = Regex::new(r#"#\[cfg\(unix\)\]"#)?;
    let target_os_windows = Regex::new(r#"cfg\(target_os = "windows"\)"#)?;
    let target_os_unix = Regex::new(r#"cfg\(target_os = "(linux|macos)"\)"#)?;
    
    for package_entry in fs::read_dir(packages_dir)? {
        let package_path = package_entry?.path();
        if !package_path.is_dir() {
            continue;
        }
        
        let package_name = match package_path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => continue,
        };
        
        let mut stats = PackageStats {
            package_name: package_name.clone(),
            ..Default::default()
        };
        
        // Walk all Rust files
        for entry in WalkDir::new(&package_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let path = entry.path().to_path_buf();
            
            let win_count = cfg_windows.find_iter(&content).count();
            let unix_count = cfg_unix.find_iter(&content).count();
            let target_win = target_os_windows.find_iter(&content).count();
            let target_unix = target_os_unix.find_iter(&content).count();
            
            if win_count > 0 || target_win > 0 {
                stats.files_with_windows.push(path.clone());
            }
            if unix_count > 0 || target_unix > 0 {
                stats.files_with_unix.push(path);
            }
            
            stats.windows_blocks += win_count;
            stats.unix_blocks += unix_count;
            stats.target_os_windows += target_win;
            stats.target_os_unix += target_unix;
        }
        
        stats_map.insert(package_name, stats);
    }
    
    // Generate Markdown report
    generate_report(&stats_map)?;
    
    Ok(())
}

fn generate_report(stats: &HashMap<String, PackageStats>) -> anyhow::Result<()> {
    let mut output = String::from("# Platform Code Parity Report\n\n");
    output.push_str(&format!("Generated: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    // Sort by package name
    let mut packages: Vec<_> = stats.values().collect();
    packages.sort_by(|a, b| a.package_name.cmp(&b.package_name));
    
    // Summary table
    output.push_str("## Summary Table\n\n");
    output.push_str("| Package | Windows Blocks | Unix Blocks | Parity % | Status |\n");
    output.push_str("|---------|----------------|-------------|----------|--------|\n");
    
    for pkg in &packages {
        let parity = pkg.parity_percentage();
        let status = if parity >= 80.0 {
            "✅ Good"
        } else if parity >= 50.0 {
            "⚠️ Partial"
        } else {
            "❌ Missing"
        };
        
        output.push_str(&format!(
            "| {} | {} | {} | {:.1}% | {} |\n",
            pkg.package_name,
            pkg.windows_blocks + pkg.target_os_windows,
            pkg.unix_blocks + pkg.target_os_unix,
            parity,
            status
        ));
    }
    
    // Detailed breakdown
    output.push_str("\n## Detailed Breakdown\n\n");
    
    for pkg in &packages {
        output.push_str(&format!("### {}\n\n", pkg.package_name));
        output.push_str(&format!("- **Windows blocks**: {} (#[cfg(windows)]) + {} (target_os)\n", 
            pkg.windows_blocks, pkg.target_os_windows));
        output.push_str(&format!("- **Unix blocks**: {} (#[cfg(unix)]) + {} (target_os)\n",
            pkg.unix_blocks, pkg.target_os_unix));
        output.push_str(&format!("- **Parity**: {:.1}%\n", pkg.parity_percentage()));
        
        if !pkg.files_with_windows.is_empty() {
            output.push_str("\n**Files with Windows code**:\n");
            for file in &pkg.files_with_windows {
                output.push_str(&format!("- `{}`\n", file.display()));
            }
        }
        
        output.push_str("\n");
    }
    
    fs::write("task/platform_parity_report.md", output)?;
    println!("Report written to task/platform_parity_report.md");
    
    Ok(())
}
