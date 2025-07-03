use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "unitt")]
#[command(version = "v3.0.0")]
#[command(about = "Lean unit testing tool for Arturo")]
pub struct Arguments {
    #[arg(long)]
    pub tests: Option<String>,
    #[arg(long)]
    pub cache: Option<String>,
    #[arg(long)]
    pub target: Option<String>,
    
    #[arg(long)]
    pub fail_fast: bool,
    #[arg(long)]
    pub supress: bool,
}