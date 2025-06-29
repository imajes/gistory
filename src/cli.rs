use clap::Parser;

/// Command-line arguments for gistory.
#[derive(Parser, Debug)]
#[command(name = "gistory", about = "Explore git history with natural language date filters")]
pub struct Cli {
    /// Natural language date range (e.g. "this year", "jan 1 to feb 15")
    pub range: String,

    /// Path to the git repository (default: current directory)
    #[arg(short, long)]
    pub repo: Option<String>,
}