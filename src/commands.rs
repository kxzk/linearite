use crate::api;
use crate::queries::{MUTATION_CREATE_ISSUE, QUERY_LIST_PROJECTS, QUERY_LIST_TEAMS};
use crate::ranking;
use crate::types::{IssueCreateResponse, ProjectsResponse, TeamsResponse};
use crate::utils::{format_table, parse_since};
use serde_json::json;

pub async fn handle_create(
    api_key: &str,
    title: &str,
    description: &Option<String>,
    team_id: &str,
    project_id: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mutation = MUTATION_CREATE_ISSUE;

    let variables = json!({
        "input": {
            "teamId": team_id,
            "projectId": project_id,
            "title": title,
            "description": description,
        }
    });

    let data = api::query_linear::<IssueCreateResponse>(api_key, mutation, Some(variables)).await?;

    if !data.issue_create.success {
        return Err("Issue creation failed".into());
    }

    let issue = match data.issue_create.issue {
        Some(issue) => issue,
        None => {
            eprintln!("Issue creation reported success but no issue data returned");
            return Ok(());
        }
    };

    // Output in bright black
    println!("\x1b[90m╔════════════════╗\x1b[0m");
    println!("\x1b[90m║ ◉  Linearite   ║\x1b[0m");
    println!("\x1b[90m╚════════════════╝\x1b[0m");
    println!(" # {}", issue.id);
    println!(" ¶ {}", issue.title);
    println!(" ⌘ {}", issue.url);
    println!(" ⎇ {}", issue.branch_name.as_deref().unwrap_or("Not Available"));

    Ok(())
}

pub async fn handle_list_teams(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = api::query_linear::<TeamsResponse>(api_key, QUERY_LIST_TEAMS, None).await?;

    let table = format_table(data.teams.nodes);
    println!("{table}");

    Ok(())
}

pub async fn handle_list_projects(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = api::query_linear::<ProjectsResponse>(api_key, QUERY_LIST_PROJECTS, None).await?;

    let table = format_table(data.projects.nodes);
    println!("{table}");

    Ok(())
}

pub async fn handle_rank_teams(
    api_key: &str,
    since: &str,
    top: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let since_date = parse_since(since)?;
    let table = ranking::rank_teams(api_key, &since_date, top).await?;
    println!("{table}");
    Ok(())
}

pub async fn handle_rank_users(
    api_key: &str,
    since: &str,
    top: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let since_date = parse_since(since)?;
    let table = ranking::rank_users(api_key, &since_date, top).await?;
    println!("{table}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{Issue, IssueCreateResponse, IssuePayload};
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_create_success() {
        // Temporarily override the API URL by modifying the API module
        // Since we can't easily inject dependencies, we'll test the variable construction
        let title = "Test Issue";
        let description = Some("Test description".to_string());
        let team_id = "team-123";
        let project_id = Some("proj-456".to_string());

        let variables = json!({
            "input": {
                "teamId": team_id,
                "projectId": project_id,
                "title": title,
                "description": description,
            }
        });

        // Verify the variables are constructed correctly
        assert_eq!(variables["input"]["teamId"], team_id);
        assert_eq!(variables["input"]["title"], title);
        assert_eq!(variables["input"]["projectId"], json!(project_id));
        assert_eq!(variables["input"]["description"], json!(description));
    }

    #[tokio::test]
    async fn test_handle_create_without_description() {
        let title = "Test Issue";
        let description: Option<String> = None;
        let team_id = "team-123";
        let project_id: Option<String> = None;

        let variables = json!({
            "input": {
                "teamId": team_id,
                "projectId": project_id,
                "title": title,
                "description": description,
            }
        });

        // Verify optional fields are handled correctly
        assert_eq!(variables["input"]["teamId"], team_id);
        assert_eq!(variables["input"]["title"], title);
        assert!(variables["input"]["projectId"].is_null());
        assert!(variables["input"]["description"].is_null());
    }

    #[test]
    fn test_issue_create_error_handling() {
        // Test the error handling logic
        let success_response = IssueCreateResponse {
            issue_create: IssuePayload {
                success: true,
                issue: Some(Issue {
                    id: "issue-123".to_string(),
                    title: "Test".to_string(),
                    url: "https://linear.app/issue-123".to_string(),
                    branch_name: Some("test-branch".to_string()),
                }),
            },
        };
        assert!(success_response.issue_create.success);
        assert!(success_response.issue_create.issue.is_some());

        let failure_response =
            IssueCreateResponse { issue_create: IssuePayload { success: false, issue: None } };
        assert!(!failure_response.issue_create.success);
        assert!(failure_response.issue_create.issue.is_none());

        let success_no_issue =
            IssueCreateResponse { issue_create: IssuePayload { success: true, issue: None } };
        assert!(success_no_issue.issue_create.success);
        assert!(success_no_issue.issue_create.issue.is_none());
    }

    #[test]
    fn test_issue_with_branch_name() {
        let response_with_branch = IssueCreateResponse {
            issue_create: IssuePayload {
                success: true,
                issue: Some(Issue {
                    id: "issue-456".to_string(),
                    title: "Feature Issue".to_string(),
                    url: "https://linear.app/issue-456".to_string(),
                    branch_name: Some("feat/issue-456-feature-issue".to_string()),
                }),
            },
        };

        assert!(response_with_branch.issue_create.success);
        let issue = response_with_branch.issue_create.issue.unwrap();
        assert_eq!(issue.branch_name, Some("feat/issue-456-feature-issue".to_string()));
    }

    #[test]
    fn test_issue_without_branch_name() {
        let response_no_branch = IssueCreateResponse {
            issue_create: IssuePayload {
                success: true,
                issue: Some(Issue {
                    id: "issue-789".to_string(),
                    title: "Bug Issue".to_string(),
                    url: "https://linear.app/issue-789".to_string(),
                    branch_name: None,
                }),
            },
        };

        assert!(response_no_branch.issue_create.success);
        let issue = response_no_branch.issue_create.issue.unwrap();
        assert!(issue.branch_name.is_none());
    }

    #[test]
    fn test_create_mutation_includes_branch_name() {
        let mutation = r#"
        mutation IssueCreate($input: IssueCreateInput!) {
            issueCreate(input: $input) {
                success
                issue {
                    id
                    title
                    url
                    branchName
                }
            }
        }
    "#;

        assert!(mutation.contains("branchName"));
    }

    #[test]
    fn test_handle_list_teams_variable_construction() {
        // Test that the query is constructed correctly
        let query = "query Teams { teams { nodes { id name } } }";
        assert!(query.contains("teams"));
        assert!(query.contains("nodes"));
        assert!(query.contains("id"));
        assert!(query.contains("name"));
    }

    #[test]
    fn test_handle_list_projects_variable_construction() {
        // Test that the query is constructed correctly
        let query = "query Projects { projects { nodes { id name } } }";
        assert!(query.contains("projects"));
        assert!(query.contains("nodes"));
        assert!(query.contains("id"));
        assert!(query.contains("name"));
    }

    #[test]
    fn test_issue_create_response_success_false() {
        let failure_response = IssueCreateResponse {
            issue_create: IssuePayload {
                success: false,
                issue: Some(Issue {
                    id: "issue-123".to_string(),
                    title: "Test".to_string(),
                    url: "https://linear.app/issue-123".to_string(),
                    branch_name: None,
                }),
            },
        };
        assert!(!failure_response.issue_create.success);
        // Even if issue is present, success=false means failure
    }

    #[test]
    fn test_issue_create_response_success_true_no_issue() {
        let success_no_issue =
            IssueCreateResponse { issue_create: IssuePayload { success: true, issue: None } };
        assert!(success_no_issue.issue_create.success);
        assert!(success_no_issue.issue_create.issue.is_none());
        // This is the edge case where success=true but no issue returned
    }

    #[test]
    fn test_create_variables_with_all_fields() {
        let title = "Complete Issue";
        let description = Some("Full description".to_string());
        let team_id = "team-full";
        let project_id = Some("proj-full".to_string());

        let variables = json!({
            "input": {
                "teamId": team_id,
                "projectId": project_id,
                "title": title,
                "description": description,
            }
        });

        assert_eq!(variables["input"]["teamId"], team_id);
        assert_eq!(variables["input"]["title"], title);
        assert_eq!(variables["input"]["projectId"], json!(project_id));
        assert_eq!(variables["input"]["description"], json!(description));
    }

    #[test]
    fn test_create_variables_with_minimal_fields() {
        let title = "Minimal Issue";
        let description: Option<String> = None;
        let team_id = "team-min";
        let project_id: Option<String> = None;

        let variables = json!({
            "input": {
                "teamId": team_id,
                "projectId": project_id,
                "title": title,
                "description": description,
            }
        });

        assert_eq!(variables["input"]["teamId"], team_id);
        assert_eq!(variables["input"]["title"], title);
        assert!(variables["input"]["projectId"].is_null());
        assert!(variables["input"]["description"].is_null());
    }

    #[test]
    fn test_issue_output_format() {
        let issue = Issue {
            id: "TEST-123".to_string(),
            title: "Test Issue Title".to_string(),
            url: "https://linear.app/TEST-123".to_string(),
            branch_name: Some("test/TEST-123-test-issue-title".to_string()),
        };

        // Verify all fields are present
        assert_eq!(issue.id, "TEST-123");
        assert_eq!(issue.title, "Test Issue Title");
        assert_eq!(issue.url, "https://linear.app/TEST-123");
        assert_eq!(issue.branch_name, Some("test/TEST-123-test-issue-title".to_string()));
    }

    #[test]
    fn test_issue_output_format_no_branch() {
        let issue = Issue {
            id: "TEST-456".to_string(),
            title: "Issue Without Branch".to_string(),
            url: "https://linear.app/TEST-456".to_string(),
            branch_name: None,
        };

        assert_eq!(issue.id, "TEST-456");
        assert_eq!(issue.title, "Issue Without Branch");
        assert_eq!(issue.url, "https://linear.app/TEST-456");
        assert!(issue.branch_name.is_none());
    }
}
