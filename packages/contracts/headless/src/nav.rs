//! Roving-index navigation: wrapping, disabled-skipping.
//! Mirror of core `nav.ts`.

/// Next enabled index in `direction` (+1/-1), wrapping modulo item count and
/// skipping disabled items. Returns `start_index` when no other enabled item
/// exists, and `None` for an empty list.
pub fn find_next_enabled_index(
    disabled: &[bool],
    start_index: usize,
    direction: i32,
) -> Option<usize> {
    let count = disabled.len();

    if count == 0 {
        return None;
    }

    let mut index = start_index as i64;
    let step = direction.signum() as i64;

    for _ in 0..count {
        index = (index + step).rem_euclid(count as i64);

        if !disabled[index as usize] {
            return Some(index as usize);
        }
    }

    Some(start_index)
}

pub fn first_enabled_index(disabled: &[bool]) -> Option<usize> {
    disabled.iter().position(|is_disabled| !is_disabled)
}
