//! Popover machine. Mirror of core `popover.ts`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverState {
    Closed,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverInitialFocus {
    FirstFocusable,
    Content,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopoverContext {
    pub disabled: bool,
    pub dismiss_on_outside_interact: bool,
    pub initial_focus: PopoverInitialFocus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverEvent {
    Toggle,
    Open,
    Close,
    Escape,
    OutsideInteract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverEffect {
    EmitOpenChange { open: bool },
    FocusOnOpen { strategy: PopoverInitialFocus },
    RestoreTriggerFocus,
}

fn open(context: PopoverContext) -> (PopoverState, Vec<PopoverEffect>) {
    (
        PopoverState::Open,
        vec![
            PopoverEffect::EmitOpenChange { open: true },
            PopoverEffect::FocusOnOpen {
                strategy: context.initial_focus,
            },
        ],
    )
}

fn close() -> (PopoverState, Vec<PopoverEffect>) {
    (
        PopoverState::Closed,
        vec![
            PopoverEffect::EmitOpenChange { open: false },
            PopoverEffect::RestoreTriggerFocus,
        ],
    )
}

pub fn popover_transition(
    state: PopoverState,
    context: PopoverContext,
    event: PopoverEvent,
) -> (PopoverState, Vec<PopoverEffect>) {
    if context.disabled {
        return (state, vec![]);
    }

    match (state, event) {
        (PopoverState::Closed, PopoverEvent::Toggle | PopoverEvent::Open) => open(context),
        (PopoverState::Open, PopoverEvent::Toggle | PopoverEvent::Close | PopoverEvent::Escape) => {
            close()
        }
        (PopoverState::Open, PopoverEvent::OutsideInteract)
            if context.dismiss_on_outside_interact =>
        {
            close()
        }
        _ => (state, vec![]),
    }
}
