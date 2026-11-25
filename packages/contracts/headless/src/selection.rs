//! Ordered selection independent of visible items. Mirror of core `selection.ts`.

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Default)]
pub enum SelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SelectionOptions {
    pub mode: SelectionMode,
    pub max: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
pub enum SelectionEvent<'a, T> {
    Select { item: &'a T, disabled: bool },
    Toggle { item: &'a T, disabled: bool },
    Remove { item: &'a T },
    Clear,
    Replace { items: &'a [T] },
}

/// Update the selection in insertion order. The caller supplies the stable key.
pub fn selection_transition<T: Clone>(
    selected: &[T],
    event: SelectionEvent<'_, T>,
    options: SelectionOptions,
    key: impl Fn(&T) -> &str,
) -> Vec<T> {
    let max = options.max.unwrap_or(usize::MAX);
    let limit = match options.mode {
        SelectionMode::Single => max.min(1),
        SelectionMode::Multiple => max,
    };
    let (item, toggle) = match event {
        SelectionEvent::Replace { items } => {
            let mut seen = HashSet::new();
            return items
                .iter()
                .filter(|item| seen.insert(key(item)))
                .take(limit)
                .cloned()
                .collect();
        }
        SelectionEvent::Clear => return Vec::new(),
        SelectionEvent::Remove { item } => {
            return selected
                .iter()
                .filter(|candidate| key(candidate) != key(item))
                .cloned()
                .collect();
        }
        SelectionEvent::Select { disabled: true, .. }
        | SelectionEvent::Toggle { disabled: true, .. } => return selected.to_vec(),
        SelectionEvent::Select { item, .. } => (item, false),
        SelectionEvent::Toggle { item, .. } => (item, true),
    };

    let item_key = key(item);
    if selected.iter().any(|candidate| key(candidate) == item_key) {
        return if toggle {
            selected
                .iter()
                .filter(|candidate| key(candidate) != item_key)
                .cloned()
                .collect()
        } else {
            selected.to_vec()
        };
    }
    if limit == 0 {
        return selected.to_vec();
    }
    if matches!(options.mode, SelectionMode::Single) {
        return vec![item.clone()];
    }
    if selected.len() >= limit {
        return selected.to_vec();
    }
    selected
        .iter()
        .chain(std::iter::once(item))
        .cloned()
        .collect()
}
