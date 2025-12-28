use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "linearite")]
#[command(about = "Tiny Linear CLI designed for AI agents", long_about = None)]
#[command(after_help = r#"
EXAMPLES:
  # List all teams to get a team ID
  linearite list-teams

  # List all projects to get a project ID
  linearite list-projects

  # Create an issue with team ID and description
  linearite create "Fix bug in API" --team-id abc123 --description "The API is broken"

  # Create an issue with team ID, description, and project ID
  linearite create "Add new feature" --team-id abc123 --description "Implement feature X" --project-id xyz789
"#)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new issue in Linear
    Create {
        /// Title of the issue
        title: String,
        /// Description of the issue
        #[arg(short, long)]
        description: Option<String>,
        /// Team ID to associate the issue with
        #[arg(short = 't', long = "team-id", required = true)]
        team_id: String,
        /// Project ID to associate the issue with
        #[arg(short = 'p', long = "project-id")]
        project_id: Option<String>,
    },
    /// List all teams (name + id)
    ListTeams,
    /// List all projects (name + id)
    ListProjects,
    /// Rank teams by completed issue points
    RankTeams {
        /// Rank teams by completed issue points since this date (DateTimeOrDuration format, e.g.,
        /// "7d", "30d", "2025-12-27")
        #[arg(short = 's', long = "since", default_value = "14d")]
        since: String,
        /// Number of top results to return
        #[arg(short = 't', long = "top", default_value = "10")]
        top: usize,
    },
    RankUsers {
        /// Rank users by completed issue points since this date (DateTimeOrDuration format, e.g.,
        /// "7d", "30d", "2025-12-27")
        #[arg(short = 's', long = "since", default_value = "14d")]
        since: String,
        /// Number of top results to return
        #[arg(short = 't', long = "top", default_value = "10")]
        top: usize,
    },
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrows_for_generic_args)]
    use super::*;

    #[test]
    fn test_cli_parse_create_command() {
        let cli =
            Cli::try_parse_from(&["linearite", "create", "Test Issue", "--team-id", "team-123"])
                .unwrap();

        match cli.command {
            Commands::Create { title, description, team_id, project_id } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-123");
                assert!(description.is_none());
                assert!(project_id.is_none());
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parse_create_with_description() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "--team-id",
            "team-123",
            "--description",
            "This is a test description",
        ])
        .unwrap();

        match cli.command {
            Commands::Create { title, description, team_id, project_id } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-123");
                assert_eq!(description, Some("This is a test description".to_string()));
                assert!(project_id.is_none());
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parse_create_with_all_options() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "--team-id",
            "team-123",
            "--description",
            "Test description",
            "--project-id",
            "proj-456",
        ])
        .unwrap();

        match cli.command {
            Commands::Create { title, description, team_id, project_id } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-123");
                assert_eq!(description, Some("Test description".to_string()));
                assert_eq!(project_id, Some("proj-456".to_string()));
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parse_create_with_short_flags() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "-t",
            "team-123",
            "-d",
            "Test description",
            "-p",
            "proj-456",
        ])
        .unwrap();

        match cli.command {
            Commands::Create { title, description, team_id, project_id } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-123");
                assert_eq!(description, Some("Test description".to_string()));
                assert_eq!(project_id, Some("proj-456".to_string()));
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parse_create_missing_team_id() {
        let result = Cli::try_parse_from(&["linearite", "create", "Test Issue"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_parse_list_teams() {
        let cli = Cli::try_parse_from(&["linearite", "list-teams"]).unwrap();
        match cli.command {
            Commands::ListTeams => {}
            _ => panic!("Expected ListTeams command"),
        }
    }

    #[test]
    fn test_cli_parse_list_projects() {
        let cli = Cli::try_parse_from(&["linearite", "list-projects"]).unwrap();
        match cli.command {
            Commands::ListProjects => {}
            _ => panic!("Expected ListProjects command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_teams_defaults() {
        let cli = Cli::try_parse_from(&["linearite", "rank-teams"]).unwrap();
        match cli.command {
            Commands::RankTeams { since, top } => {
                assert_eq!(since, "14d");
                assert_eq!(top, 10);
            }
            _ => panic!("Expected RankTeams command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_teams_custom_values() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "rank-teams",
            "--since",
            "30d",
            "--top",
            "5",
        ])
        .unwrap();
        match cli.command {
            Commands::RankTeams { since, top } => {
                assert_eq!(since, "30d");
                assert_eq!(top, 5);
            }
            _ => panic!("Expected RankTeams command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_teams_short_flags() {
        let cli = Cli::try_parse_from(&["linearite", "rank-teams", "-s", "7d", "-t", "3"]).unwrap();
        match cli.command {
            Commands::RankTeams { since, top } => {
                assert_eq!(since, "7d");
                assert_eq!(top, 3);
            }
            _ => panic!("Expected RankTeams command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_teams_with_date() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "rank-teams",
            "--since",
            "2025-01-15",
            "--top",
            "20",
        ])
        .unwrap();
        match cli.command {
            Commands::RankTeams { since, top } => {
                assert_eq!(since, "2025-01-15");
                assert_eq!(top, 20);
            }
            _ => panic!("Expected RankTeams command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_users_defaults() {
        let cli = Cli::try_parse_from(&["linearite", "rank-users"]).unwrap();
        match cli.command {
            Commands::RankUsers { since, top } => {
                assert_eq!(since, "14d");
                assert_eq!(top, 10);
            }
            _ => panic!("Expected RankUsers command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_users_custom_values() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "rank-users",
            "--since",
            "60d",
            "--top",
            "15",
        ])
        .unwrap();
        match cli.command {
            Commands::RankUsers { since, top } => {
                assert_eq!(since, "60d");
                assert_eq!(top, 15);
            }
            _ => panic!("Expected RankUsers command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_users_short_flags() {
        let cli = Cli::try_parse_from(&["linearite", "rank-users", "-s", "21d", "-t", "8"]).unwrap();
        match cli.command {
            Commands::RankUsers { since, top } => {
                assert_eq!(since, "21d");
                assert_eq!(top, 8);
            }
            _ => panic!("Expected RankUsers command"),
        }
    }

    #[test]
    fn test_cli_parse_rank_users_with_iso8601() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "rank-users",
            "--since",
            "2025-01-15T10:30:00Z",
            "--top",
            "25",
        ])
        .unwrap();
        match cli.command {
            Commands::RankUsers { since, top } => {
                assert_eq!(since, "2025-01-15T10:30:00Z");
                assert_eq!(top, 25);
            }
            _ => panic!("Expected RankUsers command"),
        }
    }

    #[test]
    fn test_cli_parse_create_with_short_team_flag() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "-t",
            "team-short",
        ])
        .unwrap();
        match cli.command {
            Commands::Create { title, team_id, .. } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-short");
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_parse_create_with_long_flags() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "--team-id",
            "team-long",
            "--description",
            "Long description",
            "--project-id",
            "proj-long",
        ])
        .unwrap();
        match cli.command {
            Commands::Create { title, description, team_id, project_id } => {
                assert_eq!(title, "Test Issue");
                assert_eq!(team_id, "team-long");
                assert_eq!(description, Some("Long description".to_string()));
                assert_eq!(project_id, Some("proj-long".to_string()));
            }
            _ => panic!("Expected Create command"),
        }
    }
}
