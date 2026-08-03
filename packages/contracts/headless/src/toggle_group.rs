//! ToggleGroup machine (also Accordion via `allow_deactivation`).
//! Mirror of core `toggle-group.ts`.

use crate::single_select::SelectOption;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleGroupValue {
    Single(Option<String>),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    Single,
    Multiple,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleGroupContext {
    pub value: ToggleGroupValue,
    pub options: Vec<SelectOption>,
    pub selection_mode: SelectionMode,
    pub allow_deactivation: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleGroupEvent {
    Toggle { value: String },
    SetValue { value: ToggleGroupValue },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToggleGroupEffect {
    EmitValueChange { value: ToggleGroupValue },
}

pub fn toggle_group_transition(
    context: ToggleGroupContext,
    event: ToggleGroupEvent,
) -> (ToggleGroupContext, Vec<ToggleGroupEffect>) {
    match event {
        ToggleGroupEvent::Toggle { value } => {
            let option = context
                .options
                .iter()
                .find(|candidate| candidate.value == value);

            let Some(option) = option else {
                return (context, vec![]);
            };

            if context.disabled || option.disabled {
                return (context, vec![]);
            }

            let next_value = match (&context.value, context.selection_mode) {
                (ToggleGroupValue::Multiple(current), SelectionMode::Multiple) => {
                    let mut next = current.clone();

                    if let Some(position) = next.iter().position(|item| *item == value) {
                        next.remove(position);
                    } else {
                        next.push(value);
                    }

                    ToggleGroupValue::Multiple(next)
                }
                (_, SelectionMode::Multiple) => ToggleGroupValue::Multiple(vec![value]),
                (ToggleGroupValue::Single(current), SelectionMode::Single)
                    if context.allow_deactivation && current.as_deref() == Some(value.as_str()) =>
                {
                    ToggleGroupValue::Single(None)
                }
                (_, SelectionMode::Single) => ToggleGroupValue::Single(Some(value)),
            };

            let effects = vec![ToggleGroupEffect::EmitValueChange {
                value: next_value.clone(),
            }];

            (
                ToggleGroupContext {
                    value: next_value,
                    ..context
                },
                effects,
            )
        }
        ToggleGroupEvent::SetValue { value } => (ToggleGroupContext { value, ..context }, vec![]),
    }
}
