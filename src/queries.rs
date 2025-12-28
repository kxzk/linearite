pub const MUTATION_CREATE_ISSUE: &str = r#"
    mutation IssueCreate($input:  IssueCreateInput!) {
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

pub const QUERY_LIST_TEAMS: &str = "query Teams { teams { nodes { id name } } }";

pub const QUERY_LIST_PROJECTS: &str = "query Projects { projects { nodes { id name } } } ";

pub const QUERY_COMPLETED_ISSUES: &str = r#"
    query CompletedIssues($cursor: String, $since: DateTimeOrDuration!) {
        issues(
            filter: {
                state: { type: { eq: "completed" } }
                estimate: { gt: 0 }
                completedAt: { gte: $since }
            }
            first: 100
            after: $cursor
        ) {
            nodes {
                estimate
                assignee { id name }
                team { id name }
            }
            pageInfo { hasNextPage endCursor }
        }
    }
"#;
