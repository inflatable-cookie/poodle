//! Hover-intent machine (Tooltip, HoverCard). Mirror of core `hover.ts`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverState {
    Closed,
    Opening,
    Open,
    Closing,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoverContext {
    pub open_delay_ms: f64,
    pub close_delay_ms: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverEvent {
    Enter,
    Leave,
    TimerFire,
    Dismiss,
    SetOpen { open: bool },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HoverEffect {
    StartTimer { ms: f64 },
    ClearTimer,
    EmitOpenChange { open: bool },
}

fn close_now() -> (HoverState, Vec<HoverEffect>) {
    (
        HoverState::Closed,
        vec![HoverEffect::ClearTimer, HoverEffect::EmitOpenChange { open: false }],
    )
}

pub fn hover_transition(state: HoverState, context: HoverContext, event: HoverEvent) -> (HoverState, Vec<HoverEffect>) {
    match event {
        HoverEvent::Enter => match state {
            HoverState::Open | HoverState::Closing => (HoverState::Open, vec![HoverEffect::ClearTimer]),
            _ => (
                HoverState::Opening,
                vec![HoverEffect::ClearTimer, HoverEffect::StartTimer { ms: context.open_delay_ms }],
            ),
        },
        HoverEvent::Leave => match state {
            HoverState::Closed => (state, vec![]),
            _ if context.close_delay_ms <= 0.0 => close_now(),
            _ => (
                HoverState::Closing,
                vec![HoverEffect::ClearTimer, HoverEffect::StartTimer { ms: context.close_delay_ms }],
            ),
        },
        HoverEvent::TimerFire => match state {
            HoverState::Opening => (HoverState::Open, vec![HoverEffect::EmitOpenChange { open: true }]),
            HoverState::Closing => (HoverState::Closed, vec![HoverEffect::EmitOpenChange { open: false }]),
            _ => (state, vec![]),
        },
        HoverEvent::Dismiss => match state {
            HoverState::Closed => (state, vec![]),
            _ => close_now(),
        },
        HoverEvent::SetOpen { open } => (
            if open { HoverState::Open } else { HoverState::Closed },
            vec![HoverEffect::ClearTimer],
        ),
    }
}
