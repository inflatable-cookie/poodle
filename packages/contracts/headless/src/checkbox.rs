//! Checkbox behavior machine. Mirror of core `checkbox.ts`;
//! contract: docs/contracts/components/checkbox.md.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckboxContext {
    pub checked: bool,
    pub mixed: bool,
    pub disabled: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxEvent {
    Toggle { next_checked: bool },
    SetChecked { checked: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxEffect {
    RevertNativeChecked,
    EmitCheckedChange { checked: bool },
}

pub fn checkbox_transition(context: CheckboxContext, event: CheckboxEvent) -> (CheckboxContext, Vec<CheckboxEffect>) {
    match event {
        CheckboxEvent::Toggle { next_checked } => {
            if context.disabled {
                return (context, vec![]);
            }

            if context.read_only {
                return (context, vec![CheckboxEffect::RevertNativeChecked]);
            }

            let checked = if context.mixed { true } else { next_checked };

            (
                CheckboxContext { checked, ..context },
                vec![CheckboxEffect::EmitCheckedChange { checked }],
            )
        }
        CheckboxEvent::SetChecked { checked } => (CheckboxContext { checked, ..context }, vec![]),
    }
}

pub fn checkbox_state(context: CheckboxContext) -> &'static str {
    if context.mixed {
        "mixed"
    } else if context.checked {
        "checked"
    } else {
        "unchecked"
    }
}
