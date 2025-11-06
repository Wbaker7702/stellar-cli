use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
pub struct Cmd {
    /// Show animated progress bar
    #[arg(long)]
    pub animated: bool,
    
    /// Path to analyze (defaults to current directory)
    #[arg(long, short = 'p')]
    pub path: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to analyze project: {0}")]
    AnalysisFailed(String),
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        let path = self.path.as_deref().unwrap_or(".");
        
        println!("\n📊 === STELLAR CLI PROJECT STATS === 📊\n");
        
        if self.animated {
            print!("Analyzing");
            for _ in 0..5 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).ok();
                std::thread::sleep(std::time::Duration::from_millis(300));
            }
            println!("\n");
        }
        
        let mut rust_files = 0;
        let mut toml_files = 0;
        let mut total_lines = 0;
        let mut contract_mentions = 0;
        
        if let Ok(entries) = Self::walk_dir(Path::new(path)) {
            for entry in entries {
                if let Some(ext) = entry.extension() {
                    if ext == "rs" {
                        rust_files += 1;
                        if let Ok(content) = fs::read_to_string(&entry) {
                            let lines = content.lines().count();
                            total_lines += lines;
                            contract_mentions += content.matches("contract").count();
                        }
                    } else if ext == "toml" {
                        toml_files += 1;
                    }
                }
            }
        }
        
        println!("🦀 Rust Files:          {}", rust_files);
        println!("📝 TOML Files:          {}", toml_files);
        println!("📏 Total Lines:         {}", total_lines);
        println!("🤝 Contract Mentions:   {}", contract_mentions);
        println!("🎯 Blockchain Quotient: {}%", (contract_mentions * 100) / total_lines.max(1));
        println!("🚀 Awesomeness Level:   {}⭐", "⭐".repeat((rust_files / 10).min(5)));
        
        println!("\n💡 Fun Fact: You have enough code to");
        if total_lines > 10000 {
            println!("   confuse a senior developer for at least 3 hours! 🎉");
        } else if total_lines > 1000 {
            println!("   make a junior developer cry! 😭");
        } else {
            println!("   get started on your blockchain journey! 🌱");
        }
        
        println!();
        Ok(())
    }
    
    fn walk_dir(path: &Path) -> std::io::Result<Vec<std::path::PathBuf>> {
        let mut files = Vec::new();
        
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                
                // Skip target and node_modules directories
                if let Some(name) = path.file_name() {
                    if name == "target" || name == "node_modules" || name == ".git" {
                        continue;
                    }
                }
                
                if path.is_dir() {
                    files.extend(Self::walk_dir(&path)?);
                } else {
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }
}
