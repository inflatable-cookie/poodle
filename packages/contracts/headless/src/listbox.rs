//! Listbox machine. Mirror of `packages/core/src/listbox.ts`.
//!
//! Focus, range selection, typeahead, and selection transitions are pure.
//! Renderers own rows, native focus, and execution of the returned effects.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListboxItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl ListboxItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ListboxSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ListboxOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ListboxOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListboxContext {
    pub items: Vec<ListboxItem>,
    pub selection_mode: ListboxSelectionMode,
    pub orientation: ListboxOrientation,
    pub disabled: bool,
    pub selected_values: Vec<String>,
    pub focused_value: Option<String>,
    pub anchor_value: Option<String>,
    pub typeahead: String,
    pub typeahead_at: u64,
}

impl ListboxContext {
    pub fn new(items: Vec<ListboxItem>) -> Self {
        let focused_value = listbox_initial_focus(&items, &[], false);
        Self {
            items,
            selection_mode: ListboxSelectionMode::Single,
            orientation: ListboxOrientation::Vertical,
            disabled: false,
            selected_values: Vec::new(),
            focused_value,
            anchor_value: None,
            typeahead: String::new(),
            typeahead_at: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListboxEvent {
    Focus {
        value: String,
    },
    Move {
        direction: i8,
        extend_selection: bool,
    },
    Boundary {
        boundary: ListboxBoundary,
    },
    Typeahead {
        character: String,
        now: u64,
    },
    Space,
    Select {
        value: String,
        additive: bool,
        range: bool,
    },
    SelectAll,
    Activate {
        value: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListboxBoundary {
    First,
    Last,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ListboxEffect {
    Focus { value: String },
    SelectionChanged { values: Vec<String> },
    Activate { value: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListboxResult {
    pub context: ListboxContext,
    pub effects: Vec<ListboxEffect>,
}

const TYPEAHEAD_TIMEOUT_MS: u64 = 500;

pub fn listbox_enabled_items(context: &ListboxContext) -> Vec<&ListboxItem> {
    if context.disabled {
        Vec::new()
    } else {
        context.items.iter().filter(|item| !item.disabled).collect()
    }
}

pub fn listbox_initial_focus(
    items: &[ListboxItem],
    selected_values: &[String],
    disabled: bool,
) -> Option<String> {
    if disabled {
        return None;
    }
    items
        .iter()
        .find(|item| !item.disabled && selected_values.contains(&item.value))
        .or_else(|| items.iter().find(|item| !item.disabled))
        .map(|item| item.value.clone())
}

fn normalize_values(values: &[String], context: &ListboxContext) -> Vec<String> {
    let mut unique = Vec::new();
    for value in values {
        if context
            .items
            .iter()
            .any(|item| item.value == *value && !item.disabled)
            && !unique.contains(value)
        {
            unique.push(value.clone());
        }
    }
    if context.selection_mode == ListboxSelectionMode::Single {
        unique.truncate(1);
    }
    unique
}

fn focus_result(
    context: &ListboxContext,
    value: &str,
    anchor_value: Option<String>,
) -> ListboxResult {
    ListboxResult {
        context: ListboxContext {
            focused_value: Some(value.to_string()),
            anchor_value,
            ..context.clone()
        },
        effects: vec![ListboxEffect::Focus {
            value: value.to_string(),
        }],
    }
}

fn with_selection(context: &ListboxContext, values: &[String]) -> ListboxResult {
    let normalized = normalize_values(values, context);
    let changed = normalized != context.selected_values;
    let mut effects = Vec::new();
    if changed {
        effects.push(ListboxEffect::SelectionChanged {
            values: normalized.clone(),
        });
    }
    ListboxResult {
        context: ListboxContext {
            selected_values: normalized,
            ..context.clone()
        },
        effects,
    }
}

fn enabled_index(context: &ListboxContext, value: Option<&str>) -> Option<usize> {
    listbox_enabled_items(context)
        .iter()
        .position(|item| Some(item.value.as_str()) == value)
}

fn enabled_range(context: &ListboxContext, start_value: &str, end_value: &str) -> Vec<String> {
    let Some(start) = context
        .items
        .iter()
        .position(|item| item.value == start_value)
    else {
        return Vec::new();
    };
    let Some(end) = context
        .items
        .iter()
        .position(|item| item.value == end_value)
    else {
        return Vec::new();
    };
    let (first, last) = if start <= end {
        (start, end)
    } else {
        (end, start)
    };
    context.items[first..=last]
        .iter()
        .filter(|item| !item.disabled)
        .map(|item| item.value.clone())
        .collect()
}

fn move_focus(context: &ListboxContext, direction: i8, extend_selection: bool) -> ListboxResult {
    let enabled = listbox_enabled_items(context);
    if enabled.is_empty() {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    }

    let current = enabled_index(context, context.focused_value.as_deref());
    let next_index = match current {
        None if direction > 0 => 0,
        None => enabled.len() - 1,
        Some(index) if direction > 0 => (index + 1).min(enabled.len() - 1),
        Some(index) => index.saturating_sub(1),
    };
    let next = enabled[next_index];
    if context.focused_value.as_deref() == Some(next.value.as_str()) {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    }

    let focused = ListboxContext {
        focused_value: Some(next.value.clone()),
        ..context.clone()
    };
    let focus_effect = ListboxEffect::Focus {
        value: next.value.clone(),
    };
    match context.selection_mode {
        ListboxSelectionMode::Single => {
            let mut selected = with_selection(&focused, &[next.value.clone()]);
            selected.context.anchor_value = Some(next.value.clone());
            selected.effects.insert(0, focus_effect);
            selected
        }
        ListboxSelectionMode::Multiple if extend_selection => {
            let anchor = context
                .anchor_value
                .as_deref()
                .or(context.focused_value.as_deref())
                .unwrap_or(next.value.as_str());
            let range = enabled_range(context, anchor, &next.value);
            let mut selected = with_selection(context, &range);
            selected.context.focused_value = Some(next.value.clone());
            selected.context.anchor_value = Some(anchor.to_string());
            selected.effects.insert(0, focus_effect);
            selected
        }
        ListboxSelectionMode::Multiple => ListboxResult {
            context: ListboxContext {
                focused_value: Some(next.value.clone()),
                ..context.clone()
            },
            effects: vec![focus_effect],
        },
    }
}

fn focus_boundary(context: &ListboxContext, boundary: ListboxBoundary) -> ListboxResult {
    let enabled = listbox_enabled_items(context);
    let item = match boundary {
        ListboxBoundary::First => enabled.first().copied(),
        ListboxBoundary::Last => enabled.last().copied(),
    };
    let Some(item) = item else {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    };
    let mut result = focus_result(context, &item.value, Some(item.value.clone()));
    if context.selection_mode == ListboxSelectionMode::Single {
        let mut selected = with_selection(&result.context, &[item.value.clone()]);
        selected.effects.splice(0..0, result.effects);
        result = selected;
    }
    result
}

fn focus_typeahead(context: &ListboxContext, character: &str, now: u64) -> ListboxResult {
    if character.chars().count() != 1 || character.trim().is_empty() {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    }
    let lower = character.to_lowercase();
    let continuing = now.saturating_sub(context.typeahead_at) <= TYPEAHEAD_TIMEOUT_MS;
    let repeated = continuing
        && !context.typeahead.is_empty()
        && context
            .typeahead
            .chars()
            .all(|item| item.to_string() == lower);
    let query = if continuing {
        if repeated {
            lower.clone()
        } else {
            format!("{}{}", context.typeahead, lower)
        }
    } else {
        lower.clone()
    };

    let enabled = listbox_enabled_items(context);
    let mut typed = context.clone();
    typed.typeahead = query.clone();
    typed.typeahead_at = now;
    if enabled.is_empty() {
        return ListboxResult {
            context: typed,
            effects: Vec::new(),
        };
    }

    let start = enabled_index(context, context.focused_value.as_deref());
    let focused_item = start.and_then(|index| enabled.get(index).copied());
    if continuing
        && !repeated
        && !context.typeahead.is_empty()
        && focused_item
            .is_some_and(|item| item.label.trim().to_lowercase().starts_with(query.as_str()))
    {
        let item = focused_item.unwrap();
        let mut result = focus_result(&typed, &item.value, Some(item.value.clone()));
        if context.selection_mode == ListboxSelectionMode::Single {
            let mut selected = with_selection(&result.context, &[item.value.clone()]);
            selected.effects.splice(0..0, result.effects);
            result = selected;
        }
        return result;
    }

    let start = start.unwrap_or(enabled.len() - 1);
    for offset in 1..=enabled.len() {
        let item = enabled[(start + offset) % enabled.len()];
        if item.label.trim().to_lowercase().starts_with(query.as_str()) {
            let mut result = focus_result(&typed, &item.value, Some(item.value.clone()));
            if context.selection_mode == ListboxSelectionMode::Single {
                let mut selected = with_selection(&result.context, &[item.value.clone()]);
                selected.effects.splice(0..0, result.effects);
                result = selected;
            }
            return result;
        }
    }
    ListboxResult {
        context: typed,
        effects: Vec::new(),
    }
}

fn select_range(context: &ListboxContext, value: &str, additive: bool) -> ListboxResult {
    let anchor = context
        .anchor_value
        .as_deref()
        .or(context.focused_value.as_deref())
        .unwrap_or(value);
    let range = enabled_range(context, anchor, value);
    if range.is_empty() {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    }
    let values = if additive {
        let mut values = context.selected_values.clone();
        values.extend(range);
        values
    } else {
        range
    };
    let mut result = with_selection(context, &values);
    result.context.focused_value = Some(value.to_string());
    result.context.anchor_value = Some(anchor.to_string());
    result
}

pub fn listbox_transition(context: &ListboxContext, event: ListboxEvent) -> ListboxResult {
    if context.disabled {
        return ListboxResult {
            context: context.clone(),
            effects: Vec::new(),
        };
    }

    match event {
        ListboxEvent::Focus { value } => {
            let enabled = context
                .items
                .iter()
                .any(|item| item.value == value && !item.disabled);
            if enabled {
                focus_result(context, &value, context.anchor_value.clone())
            } else {
                ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                }
            }
        }
        ListboxEvent::Move {
            direction,
            extend_selection,
        } => move_focus(context, direction, extend_selection),
        ListboxEvent::Boundary { boundary } => focus_boundary(context, boundary),
        ListboxEvent::Typeahead { character, now } => focus_typeahead(context, &character, now),
        ListboxEvent::Space => {
            let Some(value) = context.focused_value.as_deref() else {
                return ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                };
            };
            if !context
                .items
                .iter()
                .any(|item| item.value == value && !item.disabled)
            {
                return ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                };
            }
            if context.selection_mode == ListboxSelectionMode::Single {
                with_selection(context, &[value.to_string()])
            } else {
                let values = if context.selected_values.iter().any(|item| item == value) {
                    context
                        .selected_values
                        .iter()
                        .filter(|item| item.as_str() != value)
                        .cloned()
                        .collect()
                } else {
                    let mut values = context.selected_values.clone();
                    values.push(value.to_string());
                    values
                };
                let mut result = with_selection(context, &values);
                result.context.anchor_value = Some(value.to_string());
                result
            }
        }
        ListboxEvent::Select {
            value,
            additive,
            range,
        } => {
            if !context
                .items
                .iter()
                .any(|item| item.value == value && !item.disabled)
            {
                return ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                };
            }
            if context.selection_mode == ListboxSelectionMode::Single {
                let mut result = with_selection(
                    &ListboxContext {
                        focused_value: Some(value.clone()),
                        anchor_value: Some(value.clone()),
                        ..context.clone()
                    },
                    &[value.clone()],
                );
                result.effects.insert(
                    0,
                    ListboxEffect::Focus {
                        value: value.clone(),
                    },
                );
                return result;
            }
            if range {
                let mut result = select_range(context, &value, additive);
                result.effects.insert(
                    0,
                    ListboxEffect::Focus {
                        value: value.clone(),
                    },
                );
                return result;
            }
            let values = if additive {
                if context.selected_values.iter().any(|item| item == &value) {
                    context
                        .selected_values
                        .iter()
                        .filter(|item| item.as_str() != value)
                        .cloned()
                        .collect()
                } else {
                    let mut values = context.selected_values.clone();
                    values.push(value.clone());
                    values
                }
            } else {
                vec![value.clone()]
            };
            let mut result = with_selection(
                &ListboxContext {
                    focused_value: Some(value.clone()),
                    anchor_value: Some(value.clone()),
                    ..context.clone()
                },
                &values,
            );
            result.effects.insert(
                0,
                ListboxEffect::Focus {
                    value: value.clone(),
                },
            );
            result
        }
        ListboxEvent::SelectAll => {
            if context.selection_mode == ListboxSelectionMode::Multiple {
                let values: Vec<String> = listbox_enabled_items(context)
                    .iter()
                    .map(|item| item.value.clone())
                    .collect();
                with_selection(context, &values)
            } else {
                ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                }
            }
        }
        ListboxEvent::Activate { value } => {
            let target = value.as_deref().or(context.focused_value.as_deref());
            let Some(target) = target else {
                return ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                };
            };
            if context
                .items
                .iter()
                .any(|item| item.value == target && !item.disabled)
            {
                ListboxResult {
                    context: context.clone(),
                    effects: vec![ListboxEffect::Activate {
                        value: target.to_string(),
                    }],
                }
            } else {
                ListboxResult {
                    context: context.clone(),
                    effects: Vec::new(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<ListboxItem> {
        vec![
            ListboxItem::new("amber", "Amber"),
            ListboxItem::new("blue", "Blue").with_disabled(true),
            ListboxItem::new("cobalt", "Cobalt"),
            ListboxItem::new("dune", "Dune"),
        ]
    }

    fn multiple_context() -> ListboxContext {
        let mut context = ListboxContext::new(items());
        context.selection_mode = ListboxSelectionMode::Multiple;
        context.selected_values = vec!["amber".into()];
        context.focused_value = Some("amber".into());
        context
    }

    #[test]
    fn initial_focus_prefers_selected_enabled_then_first_enabled() {
        assert_eq!(
            listbox_initial_focus(&items(), &["blue".into(), "cobalt".into()], false),
            Some("cobalt".into())
        );
        assert_eq!(listbox_initial_focus(&items(), &[], true), None);
    }

    #[test]
    fn single_move_skips_disabled_selects_and_stops_at_ends() {
        let mut context = ListboxContext::new(items());
        assert_eq!(context.focused_value.as_deref(), Some("amber"));
        let next = listbox_transition(
            &context,
            ListboxEvent::Move {
                direction: 1,
                extend_selection: false,
            },
        );
        assert_eq!(next.context.focused_value.as_deref(), Some("cobalt"));
        assert_eq!(next.context.selected_values, vec!["cobalt"]);
        assert_eq!(
            next.effects,
            vec![
                ListboxEffect::Focus {
                    value: "cobalt".into()
                },
                ListboxEffect::SelectionChanged {
                    values: vec!["cobalt".into()]
                }
            ]
        );
        context.focused_value = Some("dune".into());
        assert!(listbox_transition(
            &context,
            ListboxEvent::Move {
                direction: 1,
                extend_selection: false,
            }
        )
        .effects
        .is_empty());
    }

    #[test]
    fn multiple_move_preserves_selection_and_shift_replaces_with_enabled_range() {
        let context = multiple_context();
        let moved = listbox_transition(
            &context,
            ListboxEvent::Move {
                direction: 1,
                extend_selection: false,
            },
        );
        assert_eq!(moved.context.focused_value.as_deref(), Some("cobalt"));
        assert_eq!(moved.context.selected_values, vec!["amber"]);
        assert_eq!(
            moved.effects,
            vec![ListboxEffect::Focus {
                value: "cobalt".into()
            }]
        );

        let extended = listbox_transition(
            &moved.context,
            ListboxEvent::Move {
                direction: 1,
                extend_selection: true,
            },
        );
        assert_eq!(extended.context.focused_value.as_deref(), Some("dune"));
        assert_eq!(extended.context.selected_values, vec!["cobalt", "dune"]);
        assert!(!extended
            .context
            .selected_values
            .iter()
            .any(|value| value == "blue"));
    }

    #[test]
    fn typeahead_cycles_repeats_resets_and_skips_disabled_items() {
        let mut context = ListboxContext::new(vec![
            ListboxItem::new("alfa", "Alfa"),
            ListboxItem::new("alpine", "Alpine"),
            ListboxItem::new("beta", "Beta").with_disabled(true),
            ListboxItem::new("bravo", "Bravo"),
        ]);
        let first = listbox_transition(
            &context,
            ListboxEvent::Typeahead {
                character: "a".into(),
                now: 1_000,
            },
        );
        assert_eq!(first.context.focused_value.as_deref(), Some("alpine"));
        context = first.context;
        let cycle = listbox_transition(
            &context,
            ListboxEvent::Typeahead {
                character: "a".into(),
                now: 1_100,
            },
        );
        assert_eq!(cycle.context.focused_value.as_deref(), Some("alfa"));
        let reset = listbox_transition(
            &cycle.context,
            ListboxEvent::Typeahead {
                character: "b".into(),
                now: 1_601,
            },
        );
        assert_eq!(reset.context.focused_value.as_deref(), Some("bravo"));
    }

    #[test]
    fn select_space_select_all_and_activate_follow_mode_and_disabled_guards() {
        let context = multiple_context();
        let toggled = listbox_transition(&context, ListboxEvent::Space);
        assert_eq!(toggled.context.selected_values, Vec::<String>::new());
        let all = listbox_transition(&context, ListboxEvent::SelectAll);
        assert_eq!(all.context.selected_values, vec!["amber", "cobalt", "dune"]);
        let activated = listbox_transition(&context, ListboxEvent::Activate { value: None });
        assert_eq!(
            activated.effects,
            vec![ListboxEffect::Activate {
                value: "amber".into()
            }]
        );
        assert!(listbox_transition(
            &context,
            ListboxEvent::Select {
                value: "blue".into(),
                additive: false,
                range: false,
            }
        )
        .effects
        .is_empty());
    }

    #[test]
    fn click_modifiers_toggle_or_replace_selection_with_enabled_ranges() {
        let context = multiple_context();
        let additive = listbox_transition(
            &context,
            ListboxEvent::Select {
                value: "cobalt".into(),
                additive: true,
                range: false,
            },
        );
        assert_eq!(additive.context.selected_values, vec!["amber", "cobalt"]);
        let ranged = listbox_transition(
            &ListboxContext {
                anchor_value: Some("amber".into()),
                ..context
            },
            ListboxEvent::Select {
                value: "dune".into(),
                additive: false,
                range: true,
            },
        );
        assert_eq!(
            ranged.context.selected_values,
            vec!["amber", "cobalt", "dune"]
        );
    }

    #[test]
    fn globally_disabled_machine_is_inert() {
        let context = ListboxContext {
            disabled: true,
            ..multiple_context()
        };
        for event in [
            ListboxEvent::Move {
                direction: 1,
                extend_selection: true,
            },
            ListboxEvent::Boundary {
                boundary: ListboxBoundary::Last,
            },
            ListboxEvent::Typeahead {
                character: "d".into(),
                now: 1_000,
            },
            ListboxEvent::Space,
            ListboxEvent::SelectAll,
            ListboxEvent::Activate { value: None },
        ] {
            let result = listbox_transition(&context, event);
            assert_eq!(result.context, context);
            assert!(result.effects.is_empty());
        }
    }
}
