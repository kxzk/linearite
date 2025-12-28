use clap::Parser;
use linearite::api::get_api_key;
use linearite::cli::{Cli, Commands};
use linearite::commands;

#[tokio::main]
async fn main() {
    let api_key = get_api_key().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Create { title, description, team_id, project_id } => {
            commands::handle_create(&api_key, title, description, team_id, project_id).await
        }
        Commands::ListTeams => commands::handle_list_teams(&api_key).await,
        Commands::ListProjects => commands::handle_list_projects(&api_key).await,
        Commands::RankTeams { since, top } => {
            commands::handle_rank_teams(&api_key, since, *top).await
        }
        Commands::RankUsers { since, top } => {
            commands::handle_rank_users(&api_key, since, *top).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
