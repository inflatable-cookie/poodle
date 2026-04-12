use std::path::Path;

use crate::component_registry::contract_doc_path;

#[derive(Clone, Debug, Default)]
pub struct ContractUsageDocs {
    pub exists: bool,
    pub status: Option<String>,
    pub updated: Option<String>,
    pub summary: Option<String>,
    pub anatomy: Option<String>,
    pub usage: Option<String>,
    pub props: Vec<UsageProp>,
    pub slots: Vec<UsageSlot>,
    pub events: Vec<UsageEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageProp {
    pub name: String,
    pub type_name: String,
    pub default_value: Option<String>,
    pub required: bool,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageSlot {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsageEvent {
    pub name: String,
    pub payload: String,
    pub description: String,
}

pub fn load_contract_usage_docs(slug: &str) -> ContractUsageDocs {
    let path = contract_doc_path(slug);
    let full_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(&path);

    let mut docs = ContractUsageDocs {
        exists: full_path.exists(),
        ..Default::default()
    };

    let Ok(contents) = std::fs::read_to_string(&full_path) else {
        return docs;
    };

    docs.status = metadata_value(&contents, "Status: ");
    docs.updated = metadata_value(&contents, "Updated: ");
    docs.summary = purpose_summary(&contents);
    docs.anatomy = fenced_block(&named_section(&contents, &["Anatomy"]));
    docs.usage = usage_block(&contents);
    docs.props = parse_props(&contents);
    docs.slots = parse_slots(&contents);
    docs.events = parse_events(&contents);
    docs
}

fn metadata_value(contents: &str, prefix: &str) -> Option<String> {
    contents.lines().find_map(|line| {
        line.strip_prefix(prefix)
            .map(|value| value.trim().to_string())
    })
}

fn purpose_summary(contents: &str) -> Option<String> {
    let body = named_section(contents, &["Purpose"]);
    body.lines().find_map(|line| {
        line.trim()
            .strip_prefix("- Summary: ")
            .map(|value| value.trim().to_string())
    })
}

fn usage_block(contents: &str) -> Option<String> {
    for section_name in ["Usage", "Examples", "Props And Inputs", "Props"] {
        let body = named_section(contents, &[section_name]);
        if let Some(block) = fenced_block(&body) {
            return Some(block);
        }
    }
    None
}

fn parse_props(contents: &str) -> Vec<UsageProp> {
    parse_table(&first_present_section(
        contents,
        &["Public Props", "Props And Inputs", "Props"],
    ))
    .into_iter()
    .filter(|row| row.len() >= 5)
    .map(|row| UsageProp {
        name: strip_code(&row[0]),
        type_name: strip_code(&row[1]),
        default_value: normalize_default(&row[2]),
        required: matches!(row[3].trim(), "yes" | "true"),
        description: row[4].trim().to_string(),
    })
    .collect()
}

fn parse_slots(contents: &str) -> Vec<UsageSlot> {
    parse_table(&named_section(contents, &["Slots"]))
        .into_iter()
        .filter(|row| row.len() >= 2)
        .map(|row| UsageSlot {
            name: strip_code(&row[0]),
            description: row[1].trim().to_string(),
        })
        .collect()
}

fn parse_events(contents: &str) -> Vec<UsageEvent> {
    parse_table(&named_section(contents, &["Events"]))
        .into_iter()
        .filter(|row| row.len() >= 4)
        .map(|row| UsageEvent {
            name: strip_code(&row[0]),
            payload: strip_code(&row[2]),
            description: row[3].trim().to_string(),
        })
        .collect()
}

fn named_section(contents: &str, names: &[&str]) -> String {
    let lines: Vec<&str> = contents.lines().collect();
    let Some(start) = lines.iter().position(|line| matches_heading(line, names)) else {
        return String::new();
    };

    let mut body = Vec::new();
    for line in lines.iter().skip(start + 1) {
        if is_heading(line) {
            break;
        }
        body.push(*line);
    }

    body.join("\n")
}

fn first_present_section(contents: &str, names: &[&str]) -> String {
    for name in names {
        let body = named_section(contents, &[*name]);
        if !body.trim().is_empty() {
            return body;
        }
    }
    String::new()
}

fn is_heading(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with('#')
}

fn matches_heading(line: &str, names: &[&str]) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return false;
    }

    let heading = trimmed.trim_start_matches('#').trim();
    let normalized = normalize_heading(heading);

    names
        .iter()
        .any(|name| normalized == normalize_heading(name))
}

fn normalize_heading(value: &str) -> String {
    let mut heading = value.trim();
    if let Some((_, rest)) = heading.split_once(". ") {
        if heading.chars().take_while(|ch| ch.is_ascii_digit()).count() > 0 {
            heading = rest.trim();
        }
    }

    heading
        .trim_matches(':')
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn fenced_block(contents: &str) -> Option<String> {
    let mut in_block = false;
    let mut block = Vec::new();

    for line in contents.lines() {
        if line.trim_start().starts_with("```") {
            if in_block {
                break;
            }
            in_block = true;
            continue;
        }

        if in_block {
            block.push(line);
        }
    }

    let joined = block.join("\n").trim().to_string();
    if joined.is_empty() {
        None
    } else {
        Some(joined)
    }
}

fn parse_table(contents: &str) -> Vec<Vec<String>> {
    contents
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .map(parse_table_row)
        .filter(|row| !row.is_empty())
        .filter(|row| {
            !row.iter().all(|cell| {
                let trimmed = cell.trim();
                !trimmed.is_empty() && trimmed.chars().all(|ch| ch == '-' || ch == ':')
            })
        })
        .skip(1)
        .collect()
}

fn parse_table_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect()
}

fn strip_code(value: &str) -> String {
    value.trim().trim_matches('`').to_string()
}

fn normalize_default(value: &str) -> Option<String> {
    let trimmed = strip_code(value);
    if trimmed.is_empty() || trimmed == "—" {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_button_contract_usage_data() {
        let docs = load_contract_usage_docs("button");
        assert!(docs.exists);
        assert_eq!(docs.status.as_deref(), Some("detailed contract"));
        assert!(docs
            .summary
            .as_deref()
            .unwrap_or_default()
            .contains("general action"));
        assert!(docs.props.iter().any(|prop| prop.name == "variant"));
        assert!(docs.props.iter().any(|prop| prop.name == "pressed"));
        assert!(docs.slots.iter().any(|slot| slot.name == "default"));
        assert!(docs.events.iter().any(|event| event.name == "click"));
        assert!(docs
            .anatomy
            .as_deref()
            .unwrap_or_default()
            .contains("[Root .button]"));
    }

    #[test]
    fn parses_sidebar_nav_contract_usage_data() {
        let docs = load_contract_usage_docs("sidebar-nav");
        assert!(docs.exists);
        assert!(docs.props.iter().any(|prop| prop.name == "groups"));
        assert!(docs.events.iter().any(|event| event.name == "valueChange"));
        assert!(docs.slots.is_empty());
    }

    #[test]
    fn parses_contracts_with_shifted_heading_numbers() {
        let docs = load_contract_usage_docs("media-preview");
        assert!(docs.exists);
        assert!(docs.props.iter().any(|prop| prop.name == "title"));
        assert!(docs.slots.iter().any(|slot| slot.name == "media"));
        assert!(docs
            .anatomy
            .as_deref()
            .unwrap_or_default()
            .contains("[Card]"));
    }

    #[test]
    fn parses_slots_and_events_without_public_props_heading() {
        let docs = load_contract_usage_docs("empty-state");
        assert!(docs.exists);
        assert!(docs.props.iter().any(|prop| prop.name == "title"));
        assert!(docs.slots.iter().any(|slot| slot.name == "visual"));
        assert!(docs.events.is_empty());
    }
}
