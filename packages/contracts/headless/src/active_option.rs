//! Portable active-option navigation. Mirror of core `active-option.ts`.

use crate::nav::{find_next_enabled_index, first_enabled_index};

#[derive(Clone, Copy, Debug)]
pub struct ActiveOption<'a> {
    pub value: &'a str,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum ActiveOptionMove {
    Next,
    Previous,
    First,
    Last,
}

/// Move among enabled options. A missing or disabled active value starts at the requested end.
pub fn move_active_option<'a>(
    items: impl IntoIterator<Item = ActiveOption<'a>>,
    active_value: Option<&str>,
    movement: ActiveOptionMove,
    wrap: bool,
) -> Option<&'a str> {
    let items: Vec<_> = items.into_iter().collect();
    let disabled: Vec<_> = items.iter().map(|item| item.disabled).collect();
    let first = first_enabled_index(&disabled)?;
    let index = match movement {
        ActiveOptionMove::First => first,
        ActiveOptionMove::Last => find_next_enabled_index(&disabled, 0, -1)?,
        ActiveOptionMove::Next | ActiveOptionMove::Previous => {
            let current = items
                .iter()
                .position(|item| Some(item.value) == active_value && !item.disabled);
            let forward = matches!(movement, ActiveOptionMove::Next);
            let start = current.unwrap_or(if forward { items.len() - 1 } else { 0 });
            let next = find_next_enabled_index(&disabled, start, if forward { 1 } else { -1 })?;
            if !wrap
                && current.is_some()
                && ((forward && next < start) || (!forward && next > start))
            {
                start
            } else {
                next
            }
        }
    };
    Some(items[index].value)
}

/// Keep an enabled identity or use the enabled item nearest its former index, preferring the later item on a tie.
pub fn reconcile_active_option<'a>(
    previous_items: &[ActiveOption<'_>],
    items: &[ActiveOption<'a>],
    active_value: Option<&str>,
) -> Option<&'a str> {
    let active = active_value?;
    if let Some(item) = items
        .iter()
        .find(|item| item.value == active && !item.disabled)
    {
        return Some(item.value);
    }

    let previous_index = previous_items.iter().position(|item| item.value == active);
    items
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, item)| !item.disabled)
        .min_by_key(|(index, _)| previous_index.map_or(*index, |previous| index.abs_diff(previous)))
        .map(|(_, item)| item.value)
}
