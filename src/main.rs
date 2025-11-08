use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "linearite")]
#[command(about = "Opinionated Linear CLI designed for AI agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new issue in Linear
    Create {
        /// Title of the issue
        title: String,
        /// Description of the issue
        #[arg(short, long)]
        description: Option<String>,
        /// Team ID or identifier
        #[arg(short, long)]
        team: Option<String>,
        /// Project ID to associate the issue with
        #[arg(short = 'p', long = "project-id")]
        project_id: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { title, description, team, project_id } => {
            println!("Creating issue: {}", title);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            }
            if let Some(t) = team {
                println!("Team: {}", t);
            }
            if let Some(pid) = project_id {
                println!("Project ID: {}", pid);
            }
            // TODO: Implement actual Linear API call
        }
    }
}
