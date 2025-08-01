use clap::Parser;
use anyhow::{Context, Result};
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()>{
    let args = Cli::parse(); 
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("Error reading: `{}`", args.path.display()))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {       
            println!("{}", line);
        }
    }
    Ok(())
}


