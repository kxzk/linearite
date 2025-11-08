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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_create_command() {
        let cli = Cli::try_parse_from(&[
            "linearite",
            "create",
            "Test Issue",
            "--team-id",
            "team-123",
        ])
        .unwrap();

        match cli.command {
            Commands::Create {
                title,
                description,
                team_id,
                project_id,
            } => {
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
            Commands::Create {
                title,
                description,
                team_id,
                project_id,
            } => {
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
            Commands::Create {
                title,
                description,
                team_id,
                project_id,
            } => {
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
            Commands::Create {
                title,
                description,
                team_id,
                project_id,
            } => {
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
}

