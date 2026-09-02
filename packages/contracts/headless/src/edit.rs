//! EditableLabel behavior machine. Mirror of core `edit.ts`;
//! contract: docs/contracts/components/editable-label.md.

fn is_trim_scalar(ch: char) -> bool {
    matches!(
        ch,
        '\u{0009}'..='\u{000D}'
            | '\u{0020}'
            | '\u{0085}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

/// Drop the longest prefix and suffix in portable set T. Interior scalars stay.
pub fn trim_editable_label(value: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let mut start = 0;
    let mut end = chars.len();
    while start < end && is_trim_scalar(chars[start]) {
        start += 1;
    }
    while end > start && is_trim_scalar(chars[end - 1]) {
        end -= 1;
    }
    chars[start..end].iter().collect()
}

/// Keep at most `max_length` Unicode scalar values.
pub fn clamp_editable_label_draft(draft: &str, max_length: Option<usize>) -> String {
    let Some(limit) = max_length else {
        return draft.to_string();
    };
    draft.chars().take(limit).collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditLabelContext {
    pub value: String,
    pub draft: String,
    pub disabled: bool,
    pub max_length: Option<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditLabelState {
    View,
    Editing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditLabelEvent {
    StartEdit,
    SetDraft { draft: String },
    Commit,
    CommitBlur,
    Cancel,
    ReplaceValue { value: String },
    SetDisabled { disabled: bool },
    Teardown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditLabelEffect {
    EmitEditStart,
    FocusInput,
    EmitCommit {
        value: String,
        previous_value: String,
        restore_focus: bool,
    },
    EmitCancel {
        restore_focus: bool,
    },
}

pub fn edit_label_transition(
    state: EditLabelState,
    context: EditLabelContext,
    event: EditLabelEvent,
) -> (EditLabelState, EditLabelContext, Vec<EditLabelEffect>) {
    match event {
        EditLabelEvent::StartEdit => {
            if state != EditLabelState::View || context.disabled {
                return (state, context, vec![]);
            }
            let next = EditLabelContext {
                draft: context.value.clone(),
                ..context
            };
            (
                EditLabelState::Editing,
                next,
                vec![EditLabelEffect::EmitEditStart, EditLabelEffect::FocusInput],
            )
        }
        EditLabelEvent::SetDraft { draft } => {
            if state != EditLabelState::Editing {
                return (state, context, vec![]);
            }
            let draft = clamp_editable_label_draft(&draft, context.max_length);
            (state, EditLabelContext { draft, ..context }, vec![])
        }
        EditLabelEvent::Commit => commit_from_editing(state, context, true),
        EditLabelEvent::CommitBlur => commit_from_editing(state, context, false),
        EditLabelEvent::Cancel => cancel_from_editing(state, context, true),
        EditLabelEvent::ReplaceValue { value } => {
            if state != EditLabelState::Editing {
                return (
                    state,
                    EditLabelContext {
                        draft: value.clone(),
                        value,
                        ..context
                    },
                    vec![],
                );
            }
            if value == context.value {
                return (state, context, vec![]);
            }
            (
                EditLabelState::View,
                EditLabelContext {
                    draft: value.clone(),
                    value,
                    ..context
                },
                vec![],
            )
        }
        EditLabelEvent::SetDisabled { disabled } => {
            if state == EditLabelState::Editing && disabled {
                let next = EditLabelContext {
                    disabled: true,
                    draft: context.value.clone(),
                    ..context
                };
                return (
                    EditLabelState::View,
                    next,
                    vec![EditLabelEffect::EmitCancel {
                        restore_focus: false,
                    }],
                );
            }
            (
                state,
                EditLabelContext {
                    disabled,
                    ..context
                },
                vec![],
            )
        }
        EditLabelEvent::Teardown => {
            if state != EditLabelState::Editing {
                return (state, context, vec![]);
            }
            let next = EditLabelContext {
                draft: context.value.clone(),
                ..context
            };
            (EditLabelState::View, next, vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::trim_editable_label;

    #[test]
    fn portable_trim_drops_nel_and_bom_that_str_trim_does_not_both_drop() {
        let raw = "\u{0085}Take\u{FEFF}";
        assert_eq!(trim_editable_label(raw), "Take");
        assert_ne!(raw.trim(), "Take");
        assert_eq!(
            trim_editable_label("\u{200B}Keep\u{200B}"),
            "\u{200B}Keep\u{200B}"
        );
    }
}

fn commit_from_editing(
    state: EditLabelState,
    context: EditLabelContext,
    restore_focus: bool,
) -> (EditLabelState, EditLabelContext, Vec<EditLabelEffect>) {
    if state != EditLabelState::Editing {
        return (state, context, vec![]);
    }
    let value = trim_editable_label(&context.draft);
    let previous_value = context.value.clone();
    let next = EditLabelContext {
        draft: context.value.clone(),
        ..context
    };
    (
        EditLabelState::View,
        next,
        vec![EditLabelEffect::EmitCommit {
            value,
            previous_value,
            restore_focus,
        }],
    )
}

fn cancel_from_editing(
    state: EditLabelState,
    context: EditLabelContext,
    restore_focus: bool,
) -> (EditLabelState, EditLabelContext, Vec<EditLabelEffect>) {
    if state != EditLabelState::Editing {
        return (state, context, vec![]);
    }
    let next = EditLabelContext {
        draft: context.value.clone(),
        ..context
    };
    (
        EditLabelState::View,
        next,
        vec![EditLabelEffect::EmitCancel { restore_focus }],
    )
}
