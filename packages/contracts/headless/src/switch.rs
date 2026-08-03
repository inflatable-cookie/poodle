//! Switch behavior machine. Mirror of core `switch.ts`;
//! contract: docs/contracts/components/switch.md.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchContext {
    pub checked: bool,
    pub disabled: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchEvent {
    Toggle { next_checked: bool },
    SetChecked { checked: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchEffect {
    RevertNativeChecked,
    EmitCheckedChange { checked: bool },
}

pub fn switch_transition(
    context: SwitchContext,
    event: SwitchEvent,
) -> (SwitchContext, Vec<SwitchEffect>) {
    match event {
        SwitchEvent::Toggle { next_checked } => {
            if context.disabled {
                return (context, vec![]);
            }

            if context.read_only {
                return (context, vec![SwitchEffect::RevertNativeChecked]);
            }

            (
                SwitchContext {
                    checked: next_checked,
                    ..context
                },
                vec![SwitchEffect::EmitCheckedChange {
                    checked: next_checked,
                }],
            )
        }
        SwitchEvent::SetChecked { checked } => (SwitchContext { checked, ..context }, vec![]),
    }
}
