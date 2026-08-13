//! Menu machine (Menu, ContextMenu) and menu-list navigation machinery.
//! Mirror of core `menu.ts`.

#[path = "generated/machines/menu.rs"]
mod menu_interface;
pub use menu_interface::*;

fn open() -> (MenuState, Vec<MenuEffect>) {
    (
        MenuState::Open,
        vec![
            MenuEffect::EmitOpenChange { open: true },
            MenuEffect::FocusFirstItem,
        ],
    )
}

fn close(extra: Vec<MenuEffect>) -> (MenuState, Vec<MenuEffect>) {
    let mut effects = extra;
    effects.push(MenuEffect::EmitOpenChange { open: false });

    (MenuState::Closed, effects)
}

pub fn menu_transition(
    state: MenuState,
    context: MenuContext,
    event: MenuEvent,
) -> (MenuState, Vec<MenuEffect>) {
    if context.disabled {
        return (state, vec![]);
    }

    match (state, event) {
        (MenuState::Closed, MenuEvent::Toggle | MenuEvent::Open) => open(),
        (
            MenuState::Open,
            MenuEvent::Toggle | MenuEvent::Close | MenuEvent::Escape | MenuEvent::OutsideInteract,
        ) => close(vec![]),
        (MenuState::Open, MenuEvent::Action { value }) => {
            close(vec![MenuEffect::EmitAction { value }])
        }
        _ => (state, vec![]),
    }
}

// ── Menu list navigation machinery ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuListMove {
    Next,
    Prev,
    First,
    Last,
}

/// Next highlight index for a keyboard move over a list described by its
/// per-item disabled flags: wraps and skips disabled; `First`/`Last` land on
/// enabled boundaries. Returns the current index when nothing else is
/// enabled, and 0 for an empty list.
pub fn menu_list_navigate(disabled: &[bool], highlight_index: usize, mv: MenuListMove) -> usize {
    if disabled.is_empty() {
        return 0;
    }

    match mv {
        MenuListMove::Next => wrap_navigate(disabled, highlight_index, 1),
        MenuListMove::Prev => wrap_navigate(disabled, highlight_index, -1),
        MenuListMove::First => disabled
            .iter()
            .position(|is_disabled| !is_disabled)
            .unwrap_or(0),
        MenuListMove::Last => disabled
            .iter()
            .rposition(|is_disabled| !is_disabled)
            .unwrap_or(0),
    }
}

fn wrap_navigate(disabled: &[bool], start_index: usize, direction: i64) -> usize {
    let count = disabled.len() as i64;
    let mut index = start_index as i64;

    for _ in 0..count {
        index = (index + direction).rem_euclid(count);

        if !disabled[index as usize] {
            return index as usize;
        }
    }

    start_index
}
