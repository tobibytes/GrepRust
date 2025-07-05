use anyhow::{Context, Result};
use log::{info, warn};
use clap::Parser;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()>{

    let args = Cli::parse(); 
    env_logger::init();
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("Error reading: `{}`", args.path.display()))?;

    for line in content.lines() {
        warn!("not here");
        if line.contains(&args.pattern) {
            info!("working");
            
            println!("{}", line);
        }

    }
    Ok(())
}
