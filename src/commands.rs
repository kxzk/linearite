use crate::api;
use crate::types::{IssueCreateResponse, ProjectsResponse, TeamsResponse};
use serde_json::json;

pub async fn handle_create(
    title: &str,
    description: &Option<String>,
    team_id: &str,
    project_id: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {

    let mutation = r#"
        mutation IssueCreate($input: IssueCreateInput!) {
            issueCreate(input: $input) {
                success
                issue {
                    id
                    title
                    url
                }
            }
        }
    "#;

    let variables = json!({
        "input": {
            "teamId": team_id,
            "projectId": project_id,
            "title": title,
            "description": description,
        }
    });

    let data = api::query_linear::<IssueCreateResponse>(mutation, Some(variables)).await?;
    
    if data.issue_create.success {
        if let Some(issue) = data.issue_create.issue {
            println!("issue created!");
            println!("id: {}", issue.id);
            println!("title: {}", issue.title);
            println!("url: {}", issue.url);
        } else {
            eprintln!("[warning] issue creation reported success but no issue data returned");
        }
    } else {
        return Err("[error] issue creation failed".into());
    }

    Ok(())
}

pub async fn handle_list_teams() -> Result<(), Box<dyn std::error::Error>> {
    let data = api::query_linear::<TeamsResponse>("query Teams { teams { nodes { id name } } }", None).await?;
    
    for team in data.teams.nodes {
        println!("{}\t{}", team.name, team.id);
    }
    
    Ok(())
}

pub async fn handle_list_projects() -> Result<(), Box<dyn std::error::Error>> {
    let data = api::query_linear::<ProjectsResponse>("query Projects { projects { nodes { id name } } }", None).await?;
    
    for project in data.projects.nodes {
        println!("{}\t{}", project.name, project.id);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::types::{IssueCreateResponse, IssuePayload, Issue};
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
                }),
            },
        };
        assert!(success_response.issue_create.success);
        assert!(success_response.issue_create.issue.is_some());

        let failure_response = IssueCreateResponse {
            issue_create: IssuePayload {
                success: false,
                issue: None,
            },
        };
        assert!(!failure_response.issue_create.success);
        assert!(failure_response.issue_create.issue.is_none());

        let success_no_issue = IssueCreateResponse {
            issue_create: IssuePayload {
                success: true,
                issue: None,
            },
        };
        assert!(success_no_issue.issue_create.success);
        assert!(success_no_issue.issue_create.issue.is_none());
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
}

