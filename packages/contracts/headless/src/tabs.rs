//! Tabs machine. Mirror of core `tabs.ts` (main chart; the tooltip
//! sub-machine and DOM drag plumbing stay adapter-side).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabPin {
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsItem {
    pub value: String,
    pub disabled: bool,
    pub closable: bool,
    pub pinned: Option<TabPin>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationMode {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsContext {
    pub items: Vec<TabsItem>,
    pub value: Option<String>,
    pub focus_index: usize,
    pub activation_mode: ActivationMode,
    pub reorderable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Next,
    Prev,
    First,
    Last,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsEvent {
    Select {
        value: String,
    },
    FocusMove {
        direction: FocusDirection,
        from_index: Option<usize>,
    },
    Activate {
        index: Option<usize>,
    },
    Close {
        value: String,
    },
    ReorderStep {
        direction: i32,
        from_index: Option<usize>,
    },
    Reorder {
        from_index: usize,
        to_index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabsEffect {
    EmitValueChange { value: String },
    EmitReorder { order: Vec<String> },
    EmitClose { value: String },
    FocusTab { index: usize },
}

fn disabled_flags(items: &[TabsItem]) -> Vec<bool> {
    items.iter().map(|item| item.disabled).collect()
}

/// Selected value with the contract fallback: first enabled item.
pub fn resolve_tabs_value(items: &[TabsItem], value: Option<&str>) -> Option<String> {
    if let Some(value) = value {
        if items.iter().any(|item| item.value == value) {
            return Some(value.to_string());
        }
    }

    crate::nav::first_enabled_index(&disabled_flags(items)).map(|index| items[index].value.clone())
}

pub fn apply_reorder(
    items: &[TabsItem],
    from_index: usize,
    to_index: usize,
) -> (Vec<TabsItem>, usize) {
    if from_index == to_index || from_index >= items.len() || to_index >= items.len() {
        return (items.to_vec(), from_index);
    }

    let mut next = items.to_vec();
    let moved = next.remove(from_index);
    next.insert(to_index, moved);

    (next, to_index)
}

/// Pinned partition rule, mirroring core `tabs.ts`: `Start` items form the
/// leading contiguous run, `End` items the trailing contiguous run.
pub fn is_valid_tabs_pinned_order(items: &[TabsItem]) -> bool {
    #[derive(PartialEq)]
    enum Phase {
        Start,
        Middle,
        End,
    }
    let mut phase = Phase::Start;
    for item in items {
        match phase {
            Phase::Start => match item.pinned {
                Some(TabPin::Start) => {}
                Some(TabPin::End) => phase = Phase::End,
                None => phase = Phase::Middle,
            },
            Phase::Middle => match item.pinned {
                Some(TabPin::Start) => return false,
                Some(TabPin::End) => phase = Phase::End,
                None => {}
            },
            Phase::End => {
                if item.pinned != Some(TabPin::End) {
                    return false;
                }
            }
        }
    }
    true
}

/// Pinned items are never reorder sources; no move may land inside a pinned
/// partition or cross one. Simulates the move and revalidates the partition.
pub fn is_tabs_reorder_allowed(items: &[TabsItem], from_index: usize, to_index: usize) -> bool {
    if from_index == to_index || from_index >= items.len() || to_index >= items.len() {
        return false;
    }
    if items[from_index].pinned.is_some() {
        return false;
    }
    let mut next = items.to_vec();
    let moved = next.remove(from_index);
    next.insert(to_index, moved);
    is_valid_tabs_pinned_order(&next)
}

fn select(context: TabsContext, value: String) -> (TabsContext, Vec<TabsEffect>) {
    let index = context.items.iter().position(|item| item.value == value);

    match index {
        Some(index) if !context.items[index].disabled => {
            let effects = vec![TabsEffect::EmitValueChange {
                value: value.clone(),
            }];

            (
                TabsContext {
                    value: Some(value),
                    focus_index: index,
                    ..context
                },
                effects,
            )
        }
        _ => (context, vec![]),
    }
}

fn reorder(context: TabsContext, from_index: i64, to_index: i64) -> (TabsContext, Vec<TabsEffect>) {
    let count = context.items.len() as i64;

    if !context.reorderable
        || from_index < 0
        || to_index < 0
        || from_index >= count
        || to_index >= count
        || !is_tabs_reorder_allowed(&context.items, from_index as usize, to_index as usize)
    {
        return (context, vec![]);
    }

    let (items, focus_index) =
        apply_reorder(&context.items, from_index as usize, to_index as usize);
    let order = items.iter().map(|item| item.value.clone()).collect();

    (
        TabsContext {
            items,
            focus_index,
            ..context
        },
        vec![
            TabsEffect::FocusTab { index: focus_index },
            TabsEffect::EmitReorder { order },
        ],
    )
}

pub fn tabs_transition(context: TabsContext, event: TabsEvent) -> (TabsContext, Vec<TabsEffect>) {
    match event {
        TabsEvent::Select { value } => select(context, value),

        TabsEvent::FocusMove {
            direction,
            from_index,
        } => {
            let flags = disabled_flags(&context.items);
            let from = from_index.unwrap_or(context.focus_index);
            let next_index = match direction {
                FocusDirection::Next => crate::nav::find_next_enabled_index(&flags, from, 1),
                FocusDirection::Prev => crate::nav::find_next_enabled_index(&flags, from, -1),
                FocusDirection::First => crate::nav::first_enabled_index(&flags),
                FocusDirection::Last => crate::nav::find_next_enabled_index(&flags, 0, -1),
            };

            let Some(next_index) = next_index else {
                return (context, vec![]);
            };

            let mut effects = vec![TabsEffect::FocusTab { index: next_index }];
            let mut next_context = TabsContext {
                focus_index: next_index,
                ..context
            };

            if next_context.activation_mode == ActivationMode::Automatic {
                let next_value = next_context.items[next_index].value.clone();

                if next_context.value.as_deref() != Some(next_value.as_str()) {
                    effects.push(TabsEffect::EmitValueChange {
                        value: next_value.clone(),
                    });
                    next_context.value = Some(next_value);
                }
            }

            (next_context, effects)
        }

        TabsEvent::Activate { index } => {
            if context.activation_mode != ActivationMode::Manual {
                return (context, vec![]);
            }

            let index = index.unwrap_or(context.focus_index);

            match context.items.get(index) {
                Some(item) => {
                    let value = item.value.clone();
                    select(context, value)
                }
                None => (context, vec![]),
            }
        }

        TabsEvent::Close { value } => {
            let closable = context
                .items
                .iter()
                .any(|item| item.value == value && item.closable);

            if closable {
                (context, vec![TabsEffect::EmitClose { value }])
            } else {
                (context, vec![])
            }
        }

        TabsEvent::ReorderStep {
            direction,
            from_index,
        } => {
            let from = from_index.unwrap_or(context.focus_index) as i64;

            reorder(context, from, from + direction.signum() as i64)
        }

        TabsEvent::Reorder {
            from_index,
            to_index,
        } => reorder(context, from_index as i64, to_index as i64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(value: &str, pinned: Option<TabPin>) -> TabsItem {
        TabsItem {
            value: value.to_string(),
            disabled: false,
            closable: false,
            pinned,
        }
    }

    fn context(items: Vec<TabsItem>) -> TabsContext {
        TabsContext {
            items,
            value: None,
            focus_index: 0,
            activation_mode: ActivationMode::Automatic,
            reorderable: true,
        }
    }

    #[test]
    fn pinned_partitions_validate() {
        assert!(is_valid_tabs_pinned_order(&[
            item("a", Some(TabPin::Start)),
            item("b", None),
            item("c", Some(TabPin::End)),
        ]));
        assert!(!is_valid_tabs_pinned_order(&[
            item("a", None),
            item("b", Some(TabPin::Start)),
        ]));
        assert!(!is_valid_tabs_pinned_order(&[
            item("a", Some(TabPin::End)),
            item("b", None),
        ]));
    }

    #[test]
    fn pinned_reorders_are_refused_without_effects() {
        let items = vec![
            item("a", Some(TabPin::Start)),
            item("b", None),
            item("c", None),
            item("d", Some(TabPin::End)),
        ];
        assert!(!is_tabs_reorder_allowed(&items, 0, 1));
        assert!(!is_tabs_reorder_allowed(&items, 1, 0));
        assert!(!is_tabs_reorder_allowed(&items, 2, 3));
        assert!(is_tabs_reorder_allowed(&items, 1, 2));

        let (next, effects) = tabs_transition(
            context(items),
            TabsEvent::Reorder {
                from_index: 0,
                to_index: 1,
            },
        );
        assert!(effects.is_empty());
        assert_eq!(next.items[0].value, "a");
    }
}
