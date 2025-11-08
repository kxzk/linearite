use clap::Parser;
use linearite::cli::{Cli, Commands};
use linearite::commands;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Create { title, description, team_id, project_id } => {
            commands::handle_create(title, description, team_id, project_id).await
        }
        Commands::ListTeams => {
            commands::handle_list_teams().await
        }
        Commands::ListProjects => {
            commands::handle_list_projects().await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
