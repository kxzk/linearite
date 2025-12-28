use chrono::{DateTime, Duration, Utc};
use tabled::Table;
use tabled::Tabled;
use tabled::settings::{
    Alignment, Color, Panel, Style, object::Columns, object::Rows, object::Segment,
    style::BorderColor, themes::BorderCorrection,
};

pub fn format_table<T: Tabled>(data: Vec<T>) -> String {
    Table::new(data)
        .with(Panel::header("◉ Linearite"))
        .with(Style::extended())
        .with(BorderCorrection::span())
        .modify(Segment::all(), BorderColor::filled(Color::FG_BRIGHT_BLACK))
        .modify(Rows::first(), (Alignment::center(), Color::FG_BRIGHT_BLACK))
        .modify(Rows::one(1), (Alignment::center(), Color::FG_BRIGHT_BLACK))
        .modify(Columns::first(), Color::FG_BRIGHT_BLACK)
        .to_string()
}

/// Used to ensure [`CompletedIssue`] Tabled trait works as expected
/// (i.e. all fields implement the Display trait)
pub fn format_option_f64(opt: &Option<f64>) -> String {
    opt.map(|v| v.to_string()).unwrap_or_else(|| "N/A".to_string())
}

pub fn format_option_assignee(opt: &Option<crate::types::Assignee>) -> String {
    opt.as_ref()
        .map(|a| format!("{} ({})", a.name, a.id))
        .unwrap_or_else(|| "Unassigned".to_string())
}

pub fn format_option_team(opt: &Option<crate::types::Team>) -> String {
    opt.as_ref().map(|t| format!("{} ({})", t.name, t.id)).unwrap_or_else(|| "No Team".to_string())
}

/// Converts a duration string (e.g., "7d", "14d", "30d") or date string to ISO 8601 format
/// that Linear's DateTimeOrDuration type expects.
pub fn parse_since(since: &str) -> Result<String, Box<dyn std::error::Error>> {
    let now = Utc::now();
    let parsed_date: DateTime<Utc>;

    if let Some(days_str) = since.strip_suffix('d')
        && let Ok(days) = days_str.parse::<i64>()
    {
        // Reject negative durations (e.g., "-7d") as they're confusing
        if days < 0 {
            return Err(format!("Negative durations like '{}' are not allowed. Use positive durations (e.g., '7d') or explicit dates.", since).into());
        }
        parsed_date = now - Duration::days(days);
    } else if let Ok(dt) = DateTime::parse_from_rfc3339(since) {
        parsed_date = dt.with_timezone(&Utc);
    } else if let Ok(date) = chrono::NaiveDate::parse_from_str(since, "%Y-%m-%d") {
        parsed_date = date.and_hms_opt(0, 0, 0).ok_or("Invalid date")?.and_utc();
    } else {
        return Err(format!("Invalid date/duration format: {}. Expected format: '7d', '14d', '30d', 'YYYY-MM-DD', or ISO 8601 date-time", since).into());
    }

    Ok(parsed_date.to_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Assignee, Team};

    #[test]
    fn test_parse_since_duration_strings() {
        // Test valid duration strings
        let result_7d = parse_since("7d").unwrap();
        assert!(result_7d.contains('T')); // Should be RFC3339 format
        assert!(DateTime::parse_from_rfc3339(&result_7d).is_ok());

        let result_14d = parse_since("14d").unwrap();
        assert!(DateTime::parse_from_rfc3339(&result_14d).is_ok());

        let result_30d = parse_since("30d").unwrap();
        assert!(DateTime::parse_from_rfc3339(&result_30d).is_ok());

        let result_0d = parse_since("0d").unwrap();
        assert!(DateTime::parse_from_rfc3339(&result_0d).is_ok());

        let result_365d = parse_since("365d").unwrap();
        assert!(DateTime::parse_from_rfc3339(&result_365d).is_ok());
    }

    #[test]
    fn test_parse_since_iso8601_datetime() {
        let iso8601 = "2025-01-15T10:30:00Z";
        let result = parse_since(iso8601).unwrap();
        assert!(DateTime::parse_from_rfc3339(&result).is_ok());
        // Verify the date/time is preserved (timezone format may vary)
        assert!(result.contains("2025-01-15"));
        assert!(result.contains("10:30:00"));

        let iso8601_with_offset = "2025-01-15T10:30:00+00:00";
        let result2 = parse_since(iso8601_with_offset).unwrap();
        assert!(DateTime::parse_from_rfc3339(&result2).is_ok());
        assert!(result2.contains("2025-01-15"));
        assert!(result2.contains("10:30:00"));
    }

    #[test]
    fn test_parse_since_date_string() {
        let date_str = "2025-01-15";
        let result = parse_since(date_str).unwrap();
        assert!(result.contains("2025-01-15"));
        assert!(result.contains("T00:00:00"));
        assert!(DateTime::parse_from_rfc3339(&result).is_ok());
    }

    #[test]
    fn test_parse_since_invalid_formats() {
        assert!(parse_since("").is_err());
        assert!(parse_since("invalid").is_err());
        assert!(parse_since("7days").is_err());
        assert!(parse_since("d7").is_err());
        assert!(parse_since("2025/01/15").is_err());
        assert!(parse_since("01-15-2025").is_err());
        assert!(parse_since("abc123").is_err());
    }

    #[test]
    fn test_parse_since_negative_days() {
        // Negative days should be rejected as they're confusing
        assert!(parse_since("-7d").is_err());
        assert!(parse_since("-1d").is_err());
        assert!(parse_since("-100d").is_err());
    }

    #[test]
    fn test_format_option_f64_some() {
        assert_eq!(format_option_f64(&Some(5.0)), "5");
        assert_eq!(format_option_f64(&Some(0.0)), "0");
        assert_eq!(format_option_f64(&Some(3.15)), "3.15");
        assert_eq!(format_option_f64(&Some(100.0)), "100");
    }

    #[test]
    fn test_format_option_f64_none() {
        assert_eq!(format_option_f64(&None), "N/A");
    }

    #[test]
    fn test_format_option_assignee_some() {
        let assignee = Assignee { id: "user-123".to_string(), name: "John Doe".to_string() };
        let result = format_option_assignee(&Some(assignee));
        assert_eq!(result, "John Doe (user-123)");
    }

    #[test]
    fn test_format_option_assignee_none() {
        assert_eq!(format_option_assignee(&None), "Unassigned");
    }

    #[test]
    fn test_format_option_team_some() {
        let team = Team { id: "team-456".to_string(), name: "Engineering".to_string() };
        let result = format_option_team(&Some(team));
        assert_eq!(result, "Engineering (team-456)");
    }

    #[test]
    fn test_format_option_team_none() {
        assert_eq!(format_option_team(&None), "No Team");
    }

    #[test]
    fn test_format_table_empty() {
        let empty: Vec<Team> = vec![];
        let result = format_table(empty);
        assert!(result.contains("◉ Linearite"));
        assert!(result.contains("Team Name"));
        assert!(result.contains("Team ID"));
    }

    #[test]
    fn test_format_table_single_row() {
        let teams = vec![Team { id: "team-1".to_string(), name: "Engineering".to_string() }];
        let result = format_table(teams);
        assert!(result.contains("◉ Linearite"));
        assert!(result.contains("Engineering"));
        assert!(result.contains("team-1"));
    }

    #[test]
    fn test_format_table_multiple_rows() {
        let teams = vec![
            Team { id: "team-1".to_string(), name: "Engineering".to_string() },
            Team { id: "team-2".to_string(), name: "Product".to_string() },
        ];
        let result = format_table(teams);
        assert!(result.contains("◉ Linearite"));
        assert!(result.contains("Engineering"));
        assert!(result.contains("team-1"));
        assert!(result.contains("Product"));
        assert!(result.contains("team-2"));
    }
}
