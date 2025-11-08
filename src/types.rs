use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

#[derive(Serialize)]
pub struct GraphQLRequest<'a> {
    pub query: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,
}

#[derive(Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct TeamsResponse {
    pub teams: TeamsData,
}

#[derive(Deserialize)]
pub struct TeamsData {
    pub nodes: Vec<Team>,
}

#[derive(Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProjectsResponse {
    pub projects: ProjectsData,
}

#[derive(Deserialize)]
pub struct ProjectsData {
    pub nodes: Vec<Project>,
}

#[derive(Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct IssueCreateResponse {
    #[serde(rename = "issueCreate")]
    pub issue_create: IssuePayload,
}

#[derive(Deserialize)]
pub struct IssuePayload {
    pub success: bool,
    pub issue: Option<Issue>,
}

#[derive(Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_graphql_request_serialization() {
        let request = GraphQLRequest {
            query: Cow::Borrowed("query { teams { nodes { id name } } }"),
            variables: None,
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("query"));
        assert!(json.contains("teams"));
    }

    #[test]
    fn test_graphql_request_with_variables() {
        let request = GraphQLRequest {
            query: Cow::Borrowed("mutation IssueCreate($input: IssueCreateInput!) { issueCreate(input: $input) { success } }"),
            variables: Some(json!({
                "input": {
                    "teamId": "team-123",
                    "title": "Test Issue"
                }
            })),
        };
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("teamId"));
        assert!(json.contains("Test Issue"));
    }

    #[test]
    fn test_teams_response_deserialization() {
        let json = json!({
            "teams": {
                "nodes": [
                    {"id": "team-1", "name": "Engineering"},
                    {"id": "team-2", "name": "Product"}
                ]
            }
        });
        let response: TeamsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.teams.nodes.len(), 2);
        assert_eq!(response.teams.nodes[0].id, "team-1");
        assert_eq!(response.teams.nodes[0].name, "Engineering");
        assert_eq!(response.teams.nodes[1].id, "team-2");
        assert_eq!(response.teams.nodes[1].name, "Product");
    }

    #[test]
    fn test_projects_response_deserialization() {
        let json = json!({
            "projects": {
                "nodes": [
                    {"id": "proj-1", "name": "Project Alpha"},
                    {"id": "proj-2", "name": "Project Beta"}
                ]
            }
        });
        let response: ProjectsResponse = serde_json::from_value(json).unwrap();
        assert_eq!(response.projects.nodes.len(), 2);
        assert_eq!(response.projects.nodes[0].id, "proj-1");
        assert_eq!(response.projects.nodes[0].name, "Project Alpha");
    }

    #[test]
    fn test_issue_create_response_success() {
        let json = json!({
            "issueCreate": {
                "success": true,
                "issue": {
                    "id": "issue-123",
                    "title": "Test Issue",
                    "url": "https://linear.app/issue-123"
                }
            }
        });
        let response: IssueCreateResponse = serde_json::from_value(json).unwrap();
        assert!(response.issue_create.success);
        assert!(response.issue_create.issue.is_some());
        let issue = response.issue_create.issue.unwrap();
        assert_eq!(issue.id, "issue-123");
        assert_eq!(issue.title, "Test Issue");
        assert_eq!(issue.url, "https://linear.app/issue-123");
    }

    #[test]
    fn test_issue_create_response_failure() {
        let json = json!({
            "issueCreate": {
                "success": false,
                "issue": null
            }
        });
        let response: IssueCreateResponse = serde_json::from_value(json).unwrap();
        assert!(!response.issue_create.success);
        assert!(response.issue_create.issue.is_none());
    }

    #[test]
    fn test_graphql_response_deserialization() {
        let json = json!({
            "data": {
                "teams": {
                    "nodes": [
                        {"id": "team-1", "name": "Engineering"}
                    ]
                }
            }
        });
        let response: GraphQLResponse<TeamsResponse> = serde_json::from_value(json).unwrap();
        assert_eq!(response.data.teams.nodes.len(), 1);
        assert_eq!(response.data.teams.nodes[0].name, "Engineering");
    }
}

