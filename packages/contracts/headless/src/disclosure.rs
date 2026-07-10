//! Disclosure machine (Collapsible). Mirror of core `disclosure.ts`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisclosureContext {
    pub open: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisclosureEvent {
    Toggle,
    SetOpen { open: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisclosureEffect {
    EmitOpenChange { open: bool },
}

pub fn disclosure_transition(
    context: DisclosureContext,
    event: DisclosureEvent,
) -> (DisclosureContext, Vec<DisclosureEffect>) {
    match event {
        DisclosureEvent::Toggle => {
            if context.disabled {
                return (context, vec![]);
            }

            let open = !context.open;

            (
                DisclosureContext { open, ..context },
                vec![DisclosureEffect::EmitOpenChange { open }],
            )
        }
        DisclosureEvent::SetOpen { open } => (DisclosureContext { open, ..context }, vec![]),
    }
}
