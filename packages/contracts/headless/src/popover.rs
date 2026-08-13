//! Popover machine. Mirror of core `popover.ts`.

#[path = "generated/machines/popover.rs"]
mod popover_interface;
pub use popover_interface::*;

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
