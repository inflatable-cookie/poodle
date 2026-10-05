//! Token/chip entry machinery (TokenInput).
//!
//! Hand-ported from `packages/core/src/token.ts`. Pure token-list math:
//! merge-with-dedupe, separator splitting, and the empty-draft Backspace guard.
//! Token resolution/rejection callbacks stay adapter-side (app-defined hooks).

use std::collections::BTreeSet;

/// Merge committed tokens, keeping first-occurrence order. With `dedupe` set,
/// later duplicates are ignored; otherwise repeats are allowed.
pub fn merge_tokens(current: &[String], next: &[String], dedupe: bool) -> Vec<String> {
    let mut merged: Vec<String> = current.to_vec();
    if dedupe {
        for token in next {
            if !merged.contains(token) {
                merged.push(token.clone());
            }
        }
    } else {
        merged.extend(next.iter().cloned());
    }
    merged
}

/// The characters a caller's separator strings contribute. Each non-empty
/// character becomes part of the (deduplicated) split set, exactly as the
/// TypeScript `separatorChars` join does.
pub fn separator_chars(separators: &[String]) -> BTreeSet<char> {
    separators
        .iter()
        .flat_map(|separator| separator.chars())
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TokenSplit {
    /// Parts committed as tokens by this input.
    pub committed: Vec<String>,
    /// Text left in the input field.
    pub remainder: String,
}

/// Separator-driven splitting of raw input. Returns `None` when the input
/// contains no completed tokens yet (no separator hit), so callers can keep
/// treating the whole value as a live draft.
pub fn split_token_input(raw_value: &str, separators: &BTreeSet<char>) -> Option<TokenSplit> {
    if separators.is_empty() {
        return None;
    }

    let mut raw_parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_separator = false;
    for ch in raw_value.chars() {
        if separators.contains(&ch) {
            if !in_separator {
                raw_parts.push(std::mem::take(&mut current));
                in_separator = true;
            }
        } else {
            in_separator = false;
            current.push(ch);
        }
    }
    raw_parts.push(current);

    let ends_with_separator = raw_value
        .chars()
        .last()
        .is_some_and(|ch| separators.contains(&ch));

    if raw_parts.len() <= 1 && !ends_with_separator {
        return None;
    }

    Some(if ends_with_separator {
        TokenSplit {
            committed: raw_parts,
            remainder: String::new(),
        }
    } else {
        let remainder = raw_parts.pop().unwrap_or_default();
        TokenSplit {
            committed: raw_parts,
            remainder,
        }
    })
}

/// Backspace on an empty live draft removes the last committed chip.
pub fn token_backspace_removes(input_value: &str, token_count: usize) -> bool {
    input_value.is_empty() && token_count > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seps(value: &str) -> BTreeSet<char> {
        separator_chars(&[value.to_string()])
    }

    #[test]
    fn merge_preserves_first_occurrence_order() {
        let current = vec!["a".to_string(), "b".to_string()];
        let next = vec!["b".to_string(), "c".to_string()];
        assert_eq!(
            merge_tokens(&current, &next, true),
            vec!["a", "b", "c"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            merge_tokens(&current, &next, false),
            vec!["a", "b", "b", "c"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn split_matches_the_js_separator_run_semantics() {
        assert_eq!(split_token_input("abc", &seps(",")), None);
        assert_eq!(
            split_token_input("a,b", &seps(",")).expect("split"),
            TokenSplit {
                committed: vec!["a".to_string()],
                remainder: "b".to_string(),
            }
        );
        assert_eq!(
            split_token_input("a,,b", &seps(",")).expect("split"),
            TokenSplit {
                committed: vec!["a".to_string()],
                remainder: "b".to_string(),
            }
        );
        assert_eq!(
            split_token_input("a,", &seps(",")).expect("split"),
            TokenSplit {
                committed: vec!["a".to_string(), String::new()],
                remainder: String::new(),
            }
        );
        assert_eq!(
            split_token_input(",", &seps(",")).expect("split"),
            TokenSplit {
                committed: vec![String::new(), String::new()],
                remainder: String::new(),
            }
        );
        assert_eq!(split_token_input("a,b", &BTreeSet::new()), None);
    }

    #[test]
    fn backspace_guard_needs_an_empty_draft_and_a_token() {
        assert!(token_backspace_removes("", 2));
        assert!(!token_backspace_removes("x", 2));
        assert!(!token_backspace_removes("", 0));
    }
}
