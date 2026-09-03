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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_when_interactive_emits_effect_and_updates_context() {
        let ctx = SwitchContext {
            checked: false,
            disabled: false,
            read_only: false,
        };
        let (next_ctx, effects) = switch_transition(ctx, SwitchEvent::Toggle { next_checked: true });
        assert_eq!(
            next_ctx,
            SwitchContext {
                checked: true,
                disabled: false,
                read_only: false,
            }
        );
        assert_eq!(effects, vec![SwitchEffect::EmitCheckedChange { checked: true }]);
    }

    #[test]
    fn toggle_when_disabled_is_inert() {
        let ctx = SwitchContext {
            checked: false,
            disabled: true,
            read_only: false,
        };
        let (next_ctx, effects) = switch_transition(ctx, SwitchEvent::Toggle { next_checked: true });
        assert_eq!(next_ctx, ctx);
        assert!(effects.is_empty());
    }

    #[test]
    fn toggle_when_read_only_reverts_native_checked_without_emitting_change() {
        let ctx = SwitchContext {
            checked: true,
            disabled: false,
            read_only: true,
        };
        let (next_ctx, effects) = switch_transition(ctx, SwitchEvent::Toggle { next_checked: false });
        assert_eq!(next_ctx, ctx);
        assert_eq!(effects, vec![SwitchEffect::RevertNativeChecked]);
    }

    #[test]
    fn set_checked_updates_context_without_emitting_effects() {
        let ctx = SwitchContext {
            checked: false,
            disabled: false,
            read_only: false,
        };
        let (next_ctx, effects) = switch_transition(ctx, SwitchEvent::SetChecked { checked: true });
        assert_eq!(
            next_ctx,
            SwitchContext {
                checked: true,
                disabled: false,
                read_only: false,
            }
        );
        assert!(effects.is_empty());
    }
}
