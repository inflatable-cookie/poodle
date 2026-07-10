//! Modal machine (Dialog, AlertDialog, Drawer). Mirror of core `modal.ts`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalState {
    Closed,
    Open,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModalContext {
    pub dismiss_on_escape: bool,
    pub dismiss_on_backdrop: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalEvent {
    Open,
    Close,
    RequestClose,
    Escape,
    BackdropClick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalEffect {
    EmitOpenChange { open: bool },
    EmitRequestClose,
    SaveFocusAndEnter,
    LockBodyScroll,
    UnlockBodyScroll,
    RestoreFocus,
}

fn open() -> (ModalState, Vec<ModalEffect>) {
    (
        ModalState::Open,
        vec![
            ModalEffect::EmitOpenChange { open: true },
            ModalEffect::SaveFocusAndEnter,
            ModalEffect::LockBodyScroll,
        ],
    )
}

fn close(requested: bool) -> (ModalState, Vec<ModalEffect>) {
    let mut effects = Vec::new();

    if requested {
        effects.push(ModalEffect::EmitRequestClose);
    }

    effects.extend([
        ModalEffect::EmitOpenChange { open: false },
        ModalEffect::UnlockBodyScroll,
        ModalEffect::RestoreFocus,
    ]);

    (ModalState::Closed, effects)
}

pub fn modal_transition(state: ModalState, context: ModalContext, event: ModalEvent) -> (ModalState, Vec<ModalEffect>) {
    match (state, event) {
        (ModalState::Closed, ModalEvent::Open) => open(),
        (ModalState::Open, ModalEvent::Close) => close(false),
        (ModalState::Open, ModalEvent::RequestClose) => close(true),
        (ModalState::Open, ModalEvent::Escape) if context.dismiss_on_escape => close(true),
        (ModalState::Open, ModalEvent::BackdropClick) if context.dismiss_on_backdrop => close(true),
        _ => (state, vec![]),
    }
}
