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

#[cfg(test)]
mod tests {
    use super::*;

    fn context(initial_focus: PopoverInitialFocus) -> PopoverContext {
        PopoverContext {
            disabled: false,
            dismiss_on_outside_interact: true,
            initial_focus,
        }
    }

    #[test]
    fn toggle_and_open_from_closed_emit_open_and_focus_effects() {
        for event in [PopoverEvent::Toggle, PopoverEvent::Open] {
            let (state, effects) = popover_transition(
                PopoverState::Closed,
                context(PopoverInitialFocus::FirstFocusable),
                event,
            );
            assert_eq!(state, PopoverState::Open);
            assert_eq!(
                effects,
                vec![
                    PopoverEffect::EmitOpenChange { open: true },
                    PopoverEffect::FocusOnOpen {
                        strategy: PopoverInitialFocus::FirstFocusable,
                    },
                ]
            );
        }
    }

    #[test]
    fn every_close_axis_restores_trigger_focus() {
        // Contract: focus returns to the trigger on every close path — toggle,
        // explicit close, Escape, and outside dismissal.
        for event in [
            PopoverEvent::Toggle,
            PopoverEvent::Close,
            PopoverEvent::Escape,
            PopoverEvent::OutsideInteract,
        ] {
            let (state, effects) = popover_transition(
                PopoverState::Open,
                context(PopoverInitialFocus::Content),
                event,
            );
            assert_eq!(state, PopoverState::Closed);
            assert_eq!(
                effects,
                vec![
                    PopoverEffect::EmitOpenChange { open: false },
                    PopoverEffect::RestoreTriggerFocus,
                ]
            );
        }
    }

    #[test]
    fn open_events_are_inert_while_closed_but_toggle_closes_an_open_surface() {
        let (state, effects) = popover_transition(
            PopoverState::Closed,
            context(PopoverInitialFocus::None),
            PopoverEvent::Close,
        );
        assert_eq!(state, PopoverState::Closed);
        assert!(effects.is_empty());

        let (state, effects) = popover_transition(
            PopoverState::Open,
            context(PopoverInitialFocus::None),
            PopoverEvent::Open,
        );
        assert_eq!(state, PopoverState::Open);
        assert!(effects.is_empty());
    }

    #[test]
    fn outside_interact_obeys_dismiss_on_outside_interact() {
        let guarded = PopoverContext {
            dismiss_on_outside_interact: false,
            ..context(PopoverInitialFocus::FirstFocusable)
        };
        let (state, effects) =
            popover_transition(PopoverState::Open, guarded, PopoverEvent::OutsideInteract);
        assert_eq!(state, PopoverState::Open);
        assert!(effects.is_empty());

        let (state, effects) = popover_transition(
            PopoverState::Open,
            context(PopoverInitialFocus::FirstFocusable),
            PopoverEvent::OutsideInteract,
        );
        assert_eq!(state, PopoverState::Closed);
        assert_eq!(effects.len(), 2);
    }

    #[test]
    fn disabled_blocks_every_open_direction_and_keeps_state() {
        // Contract: disabled blocks `setOpen` in all directions.
        let disabled = PopoverContext {
            disabled: true,
            dismiss_on_outside_interact: true,
            initial_focus: PopoverInitialFocus::FirstFocusable,
        };
        for event in [
            PopoverEvent::Toggle,
            PopoverEvent::Open,
            PopoverEvent::Close,
            PopoverEvent::Escape,
            PopoverEvent::OutsideInteract,
        ] {
            for state in [PopoverState::Closed, PopoverState::Open] {
                let (next, effects) = popover_transition(state, disabled, event);
                assert_eq!(next, state, "disabled must never change state for {event:?}");
                assert!(effects.is_empty());
            }
        }
    }

    #[test]
    fn focus_effect_carries_the_declared_strategy() {
        for strategy in [
            PopoverInitialFocus::FirstFocusable,
            PopoverInitialFocus::Content,
            PopoverInitialFocus::None,
        ] {
            let (_, effects) = popover_transition(
                PopoverState::Closed,
                context(strategy),
                PopoverEvent::Open,
            );
            assert_eq!(
                effects,
                vec![
                    PopoverEffect::EmitOpenChange { open: true },
                    PopoverEffect::FocusOnOpen { strategy },
                ]
            );
        }
    }
}
