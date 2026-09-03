//! Select machine. Mirror of core `select.ts`.
//!
//! Pure open/query/highlight/value transitions. Adapters own async loading,
//! placement, portals, and DOM/native focus.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectOptionState {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectContext {
    pub value: String,
    pub open: bool,
    pub query: String,
    pub highlighted_value: Option<String>,
    pub options: Vec<SelectOptionState>,
    pub clear_value: String,
    pub searchable: bool,
    pub freeform: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectEvent {
    Open,
    Close,
    Toggle,
    Query { query: String },
    Highlight { value: String },
    HighlightPrev,
    HighlightNext,
    HighlightFirst,
    HighlightLast,
    CommitHighlighted,
    CommitOption { value: String },
    CommitFreeform,
    Clear,
    OptionsChanged { options: Vec<SelectOptionState> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectEffect {
    OpenChanged { open: bool },
    QueryChanged { query: String },
    ValueChanged { value: String },
}

pub fn select_freeform_enabled(context: &SelectContext) -> bool {
    context.searchable && context.freeform
}

pub fn select_matches_query(label: &str, query: &str) -> bool {
    query.is_empty() || label.to_lowercase().contains(&query.to_lowercase())
}

pub fn select_visible_options(context: &SelectContext) -> Vec<&SelectOptionState> {
    context
        .options
        .iter()
        .filter(|option| select_matches_query(&option.label, &context.query))
        .collect()
}

pub fn select_enabled_visible_values(context: &SelectContext) -> Vec<String> {
    select_visible_options(context)
        .into_iter()
        .filter(|option| !option.disabled)
        .map(|option| option.value.clone())
        .collect()
}

pub fn select_committed_query(context: &SelectContext) -> String {
    if context.value == context.clear_value {
        return String::new();
    }

    context
        .options
        .iter()
        .find(|option| option.value == context.value)
        .map(|option| option.label.clone())
        .unwrap_or_default()
}

pub fn select_query_highlight_value(context: &SelectContext) -> Option<String> {
    select_enabled_visible_values(context).into_iter().next()
}

pub fn select_open_highlight_value(context: &SelectContext) -> Option<String> {
    let enabled = select_enabled_visible_values(context);

    if enabled.is_empty() {
        return None;
    }

    if context.value != context.clear_value && enabled.iter().any(|value| value == &context.value) {
        return Some(context.value.clone());
    }

    enabled.into_iter().next()
}

fn inert(context: SelectContext) -> (SelectContext, Vec<SelectEffect>) {
    (context, Vec::new())
}

fn ordered_effects(
    previous: &SelectContext,
    next: &SelectContext,
    include_query: bool,
) -> Vec<SelectEffect> {
    let mut effects = Vec::new();

    if previous.open != next.open {
        effects.push(SelectEffect::OpenChanged { open: next.open });
    }

    if include_query && previous.query != next.query {
        effects.push(SelectEffect::QueryChanged {
            query: next.query.clone(),
        });
    }

    if previous.value != next.value {
        effects.push(SelectEffect::ValueChanged {
            value: next.value.clone(),
        });
    }

    effects
}

fn open_list(context: SelectContext) -> (SelectContext, Vec<SelectEffect>) {
    if context.open {
        return inert(context);
    }

    let mut next = context.clone();
    next.open = true;
    next.highlighted_value = select_open_highlight_value(&next);
    let effects = ordered_effects(&context, &next, false);
    (next, effects)
}

fn close_list(context: SelectContext) -> (SelectContext, Vec<SelectEffect>) {
    if !context.open {
        return inert(context);
    }

    let mut next = context.clone();
    next.open = false;
    if !select_freeform_enabled(&context) {
        next.query = select_committed_query(&context);
    }
    let effects = ordered_effects(&context, &next, false);
    (next, effects)
}

fn find_option<'a>(context: &'a SelectContext, value: &str) -> Option<&'a SelectOptionState> {
    context.options.iter().find(|option| option.value == value)
}

fn move_highlight(context: SelectContext, direction: i8) -> (SelectContext, Vec<SelectEffect>) {
    if !context.open {
        return open_list(context);
    }

    let enabled = select_enabled_visible_values(&context);

    if enabled.is_empty() {
        if context.highlighted_value.is_none() {
            return inert(context);
        }

        let mut next = context;
        next.highlighted_value = None;
        return (next, Vec::new());
    }

    let current_index = context
        .highlighted_value
        .as_ref()
        .and_then(|value| enabled.iter().position(|item| item == value));
    let last = enabled.len() - 1;
    let next_index = match current_index {
        None if direction > 0 => 0,
        None => last,
        Some(index) if direction > 0 => index.saturating_add(1).min(last),
        Some(index) => index.saturating_sub(1),
    };
    let highlighted_value = enabled.get(next_index).cloned();

    if highlighted_value == context.highlighted_value {
        return inert(context);
    }

    let mut next = context;
    next.highlighted_value = highlighted_value;
    (next, Vec::new())
}

fn jump_highlight(context: SelectContext, first: bool) -> (SelectContext, Vec<SelectEffect>) {
    if !context.open {
        return inert(context);
    }

    let enabled = select_enabled_visible_values(&context);
    let highlighted_value = if first {
        enabled.first().cloned()
    } else {
        enabled.last().cloned()
    };

    if highlighted_value == context.highlighted_value {
        return inert(context);
    }

    let mut next = context;
    next.highlighted_value = highlighted_value;
    (next, Vec::new())
}

fn commit_option(context: SelectContext, value: &str) -> (SelectContext, Vec<SelectEffect>) {
    let Some(option) = find_option(&context, value) else {
        return inert(context);
    };

    if option.disabled {
        return inert(context);
    }

    let mut next = context.clone();
    next.value = option.value.clone();
    next.query = option.label.clone();
    next.open = false;
    next.highlighted_value = Some(option.value.clone());
    let effects = ordered_effects(&context, &next, true);
    (next, effects)
}

fn commit_freeform(context: SelectContext) -> (SelectContext, Vec<SelectEffect>) {
    if !select_freeform_enabled(&context)
        || context.highlighted_value.is_some()
        || context.query == context.value
    {
        return inert(context);
    }

    let mut next = context.clone();
    next.value = context.query.clone();
    next.open = false;
    next.highlighted_value = None;
    let effects = ordered_effects(&context, &next, false);
    (next, effects)
}

pub fn select_transition(
    context: SelectContext,
    event: SelectEvent,
) -> (SelectContext, Vec<SelectEffect>) {
    if context.disabled {
        return inert(context);
    }

    match event {
        SelectEvent::Open => open_list(context),
        SelectEvent::Close => close_list(context),
        SelectEvent::Toggle => {
            if context.open {
                close_list(context)
            } else {
                open_list(context)
            }
        }
        SelectEvent::Query { query } => {
            let mut next = context.clone();
            next.query = query;
            next.open = true;
            next.highlighted_value = select_query_highlight_value(&next);
            let effects = ordered_effects(&context, &next, true);
            (next, effects)
        }
        SelectEvent::Highlight { value } => {
            let highlighted = {
                let Some(option) = find_option(&context, &value) else {
                    return inert(context);
                };

                if option.disabled || !select_matches_query(&option.label, &context.query) {
                    return inert(context);
                }

                if context.highlighted_value.as_deref() == Some(option.value.as_str()) {
                    return inert(context);
                }

                option.value.clone()
            };
            let mut next = context;
            next.highlighted_value = Some(highlighted);
            (next, Vec::new())
        }
        SelectEvent::HighlightPrev => move_highlight(context, -1),
        SelectEvent::HighlightNext => move_highlight(context, 1),
        SelectEvent::HighlightFirst => jump_highlight(context, true),
        SelectEvent::HighlightLast => jump_highlight(context, false),
        SelectEvent::CommitHighlighted => match context.highlighted_value.clone() {
            Some(value) => commit_option(context, &value),
            None => commit_freeform(context),
        },
        SelectEvent::CommitOption { value } => commit_option(context, &value),
        SelectEvent::CommitFreeform => commit_freeform(context),
        SelectEvent::Clear => {
            let mut next = context.clone();
            next.value = context.clear_value.clone();
            next.query = String::new();
            next.highlighted_value = if context.open {
                select_open_highlight_value(&next)
            } else {
                None
            };
            let effects = ordered_effects(&context, &next, true);
            (next, effects)
        }
        SelectEvent::OptionsChanged { options } => {
            let mut next = context;
            next.options = options;
            if next.open {
                next.highlighted_value = select_query_highlight_value(&next);
            }
            (next, Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fruit() -> Vec<SelectOptionState> {
        vec![
            SelectOptionState {
                value: "apple".into(),
                label: "Apple".into(),
                disabled: false,
            },
            SelectOptionState {
                value: "banana".into(),
                label: "Banana".into(),
                disabled: false,
            },
            SelectOptionState {
                value: "cherry".into(),
                label: "Cherry".into(),
                disabled: false,
            },
            SelectOptionState {
                value: "spinach".into(),
                label: "Spinach".into(),
                disabled: true,
            },
        ]
    }

    fn context() -> SelectContext {
        SelectContext {
            value: String::new(),
            open: false,
            query: String::new(),
            highlighted_value: None,
            options: fruit(),
            clear_value: "apple".into(),
            searchable: false,
            freeform: false,
            disabled: false,
        }
    }

    #[test]
    fn disabled_context_is_inert_in_every_direction() {
        let mut disabled = context();
        disabled.disabled = true;
        for event in [
            SelectEvent::Open,
            SelectEvent::Toggle,
            SelectEvent::HighlightNext,
            SelectEvent::CommitOption {
                value: "banana".into(),
            },
            SelectEvent::Clear,
        ] {
            let (next, effects) = select_transition(disabled.clone(), event);
            assert_eq!(next, disabled);
            assert!(effects.is_empty());
        }
    }

    #[test]
    fn disabled_option_commit_is_inert() {
        let mut open = context();
        open.open = true;
        let (next, effects) = select_transition(
            open.clone(),
            SelectEvent::CommitOption {
                value: "spinach".into(),
            },
        );
        assert_eq!(next, open);
        assert!(effects.is_empty());
    }

    #[test]
    fn highlight_navigation_skips_disabled_and_lands_on_enabled_bounds() {
        let mut open = context();
        open.open = true;
        open.highlighted_value = Some("cherry".into());
        let (next, effects) = select_transition(open, SelectEvent::HighlightNext);
        assert_eq!(next.highlighted_value.as_deref(), Some("cherry"));
        assert!(effects.is_empty());

        let mut open = context();
        open.open = true;
        let (next, _) = select_transition(open, SelectEvent::HighlightLast);
        assert_eq!(next.highlighted_value.as_deref(), Some("cherry"));
    }

    #[test]
    fn options_changed_revalidates_a_stale_highlight() {
        let mut open = context();
        open.open = true;
        open.highlighted_value = Some("banana".into());
        let mut options = fruit();
        options.retain(|option| option.value != "banana");
        let (next, effects) = select_transition(open, SelectEvent::OptionsChanged { options });
        assert_eq!(next.highlighted_value.as_deref(), Some("apple"));
        assert!(effects.is_empty());
        let (committed, commit_effects) = select_transition(next, SelectEvent::CommitHighlighted);
        assert_eq!(committed.value, "apple");
        assert!(commit_effects
            .iter()
            .any(|effect| matches!(effect, SelectEffect::ValueChanged { value } if value == "apple")));
        assert!(commit_effects.iter().all(
            |effect| !matches!(effect, SelectEffect::ValueChanged { value } if value == "banana")
        ));
    }

    #[test]
    fn clear_restores_the_authored_default() {
        let mut selected = context();
        selected.value = "banana".into();
        let (next, effects) = select_transition(selected, SelectEvent::Clear);
        assert_eq!(next.value, "apple");
        assert_eq!(
            effects,
            vec![SelectEffect::ValueChanged {
                value: "apple".into()
            }]
        );
    }

    #[test]
    fn effects_are_ordered_open_then_query_then_value() {
        let mut searchable = context();
        searchable.searchable = true;
        searchable.freeform = true;
        searchable.open = true;
        searchable.query = "mango".into();
        searchable.highlighted_value = None;
        let (next, effects) = select_transition(searchable, SelectEvent::CommitFreeform);
        assert_eq!(next.value, "mango");
        assert!(!next.open);
        assert_eq!(
            effects,
            vec![
                SelectEffect::OpenChanged { open: false },
                SelectEffect::ValueChanged {
                    value: "mango".into()
                },
            ]
        );
    }

    #[test]
    fn query_opens_and_highlights_the_first_enabled_match() {
        let mut searchable = context();
        searchable.searchable = true;
        let (next, effects) = select_transition(
            searchable,
            SelectEvent::Query {
                query: "ban".into(),
            },
        );
        assert!(next.open);
        assert_eq!(next.query, "ban");
        assert_eq!(next.highlighted_value.as_deref(), Some("banana"));
        assert_eq!(
            effects,
            vec![
                SelectEffect::OpenChanged { open: true },
                SelectEffect::QueryChanged {
                    query: "ban".into()
                },
            ]
        );
    }
}
