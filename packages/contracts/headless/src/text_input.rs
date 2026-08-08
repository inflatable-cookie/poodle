//! TextInput editing: the caret, the selection, and what a keystroke does to
//! them.
//!
//! Runtime-agnostic on purpose. This started life inside the GPUI backend,
//! which meant only one target could edit text and the other would have had to
//! reimplement it. The rules here are the contract's §Keyboard table, and every
//! target drives the same ones.

/// A caret plus selection, as char indices into the value. `anchor == head`
/// is a plain caret; otherwise the selection spans between them and `head` is
/// the end that moves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EditState {
    pub anchor: usize,
    pub head: usize,
}

impl EditState {
    fn at_end(value: &str) -> Self {
        let n = value.chars().count();
        Self {
            anchor: n,
            head: n,
        }
    }

    fn range(self) -> (usize, usize) {
        (self.anchor.min(self.head), self.anchor.max(self.head))
    }

    fn collapsed(index: usize) -> Self {
        Self {
            anchor: index,
            head: index,
        }
    }

    /// Keep the cursor inside a value that changed under it (the host owns the
    /// value and may rewrite it between frames).
    fn clamped(self, value: &str) -> Self {
        let n = value.chars().count();
        Self {
            anchor: self.anchor.min(n),
            head: self.head.min(n),
        }
    }
}

/// What a keystroke did: the value may be unchanged while the cursor moved.
pub struct EditOutcome {
    pub value: Option<String>,
    pub state: EditState,
}

fn splice(value: &str, from: usize, to: usize, insert: &str) -> String {
    let chars: Vec<char> = value.chars().collect();
    let head: String = chars[..from.min(chars.len())].iter().collect();
    let tail: String = chars[to.min(chars.len())..].iter().collect();
    format!("{head}{insert}{tail}")
}

/// Apply one keystroke to `value` at `state`. `None` means the key is not ours
/// — the caller leaves it for other handlers (Enter, Tab, Escape, shortcuts we
/// do not implement).
///
/// Pure, so the whole editing model is testable without a window.
pub fn edit_transition(
    value: &str,
    state: EditState,
    key: &str,
    shift: bool,
    accel: bool,
) -> Option<EditOutcome> {
    let state = state.clamped(value);
    let len = value.chars().count();
    let (start, end) = state.range();
    let has_selection = start != end;

    // Move the head, taking the anchor with it unless shift is extending.
    let moved = |head: usize| EditOutcome {
        value: None,
        state: if shift {
            EditState {
                anchor: state.anchor,
                head,
            }
        } else {
            EditState::collapsed(head)
        },
    };

    match key {
        "left" => Some(if has_selection && !shift {
            moved(start)
        } else if accel {
            moved(0)
        } else {
            moved(state.head.saturating_sub(1))
        }),
        "right" => Some(if has_selection && !shift {
            moved(end)
        } else if accel {
            moved(len)
        } else {
            moved((state.head + 1).min(len))
        }),
        "home" => Some(moved(0)),
        "end" => Some(moved(len)),
        "backspace" => {
            if has_selection {
                Some(EditOutcome {
                    value: Some(splice(value, start, end, "")),
                    state: EditState::collapsed(start),
                })
            } else if state.head == 0 {
                // Nothing to delete, but the key was still ours: swallow it so
                // it cannot fall through to another handler.
                Some(EditOutcome {
                    value: None,
                    state,
                })
            } else {
                let from = state.head - 1;
                Some(EditOutcome {
                    value: Some(splice(value, from, state.head, "")),
                    state: EditState::collapsed(from),
                })
            }
        }
        "delete" => {
            if has_selection {
                Some(EditOutcome {
                    value: Some(splice(value, start, end, "")),
                    state: EditState::collapsed(start),
                })
            } else if state.head >= len {
                Some(EditOutcome {
                    value: None,
                    state,
                })
            } else {
                Some(EditOutcome {
                    value: Some(splice(value, state.head, state.head + 1, "")),
                    state: EditState::collapsed(state.head),
                })
            }
        }
        "a" if accel => Some(EditOutcome {
            value: None,
            state: EditState { anchor: 0, head: len },
        }),
        _ => {
            // A single printable character replaces the selection.
            if key.chars().count() == 1 && !accel {
                let next = splice(value, start, end, key);
                Some(EditOutcome {
                    value: Some(next),
                    state: EditState::collapsed(start + 1),
                })
            } else {
                None
            }
        }
    }
}

/// Insert `text` at the caret, replacing any selection.
///
/// The paste primitive, and the same shape IME commit will need: the caller
/// supplies text from somewhere the edit model cannot reach (a clipboard, a
/// composition buffer, a drop) and the model owns where it lands.
pub fn insert_transition(value: &str, state: EditState, text: &str) -> EditOutcome {
    let state = state.clamped(value);
    let (start, end) = state.range();
    EditOutcome {
        value: Some(splice(value, start, end, text)),
        state: EditState::collapsed(start + text.chars().count()),
    }
}

/// The text currently selected, for a copy or a cut. Empty when the caret is
/// collapsed — copying nothing must not clear the clipboard.
pub fn selected_text(value: &str, state: EditState) -> String {
    let state = state.clamped(value);
    let (start, end) = state.range();
    value
        .chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

/// The word around `index`, as a character range.
///
/// "Word" is a run of alphanumerics or `_`; anything else is its own run, so
/// double-clicking punctuation selects the punctuation rather than swallowing
/// the words either side. Matches what a browser input does closely enough
/// that the two do not feel different side by side.
pub fn word_range_at(value: &str, index: usize) -> (usize, usize) {
    let chars: Vec<char> = value.chars().collect();
    if chars.is_empty() {
        return (0, 0);
    }
    let index = index.min(chars.len());
    // A caret at the very end belongs to the word before it.
    let probe = if index == chars.len() { index - 1 } else { index };
    let wordish = |c: char| c.is_alphanumeric() || c == '_';
    let in_word = wordish(chars[probe]);

    let mut start = probe;
    while start > 0 && wordish(chars[start - 1]) == in_word {
        start -= 1;
    }
    let mut end = probe + 1;
    while end < chars.len() && wordish(chars[end]) == in_word {
        end += 1;
    }
    (start, end)
}

#[cfg(test)]
mod tests {

    #[test]
    fn digits_fill_slots_left_to_right_and_stop_at_the_length() {
        let mut value = String::new();
        for key in ["1", "2", "3", "4"] {
            value = code_transition(&value, key, 3, true).expect("digits are ours");
        }
        assert_eq!(value, "123", "the fourth digit has nowhere to go");
    }

    /// Consumed, not forwarded: a letter in a digits-only code must not fall
    /// through to whatever handles plain keys.
    #[test]
    fn letters_are_swallowed_by_a_numbers_only_code() {
        assert_eq!(code_transition("12", "a", 6, true).as_deref(), Some("12"));
        assert_eq!(code_transition("12", "a", 6, false).as_deref(), Some("12a"));
    }

    #[test]
    fn backspace_removes_the_last_slot_and_is_inert_when_empty() {
        assert_eq!(code_transition("123", "backspace", 6, true).as_deref(), Some("12"));
        assert_eq!(code_transition("", "backspace", 6, true).as_deref(), Some(""));
    }

    /// Escape only claims the key when there is something to clear, so an empty
    /// code still lets a dialog close on Escape.
    #[test]
    fn escape_clears_a_filled_code_and_passes_through_an_empty_one() {
        assert_eq!(code_transition("123", "escape", 6, true).as_deref(), Some(""));
        assert_eq!(code_transition("", "escape", 6, true), None);
    }

    #[test]
    fn multi_character_keys_are_not_ours() {
        assert_eq!(code_transition("1", "enter", 6, true), None);
        assert_eq!(code_transition("1", "tab", 6, true), None);
        assert_eq!(code_transition("1", "left", 6, true), None);
    }

    #[test]
    fn paste_sanitizes_and_clamps_like_typing() {
        assert_eq!(code_paste("12-34-56-78", 6, true), "123456");
        assert_eq!(code_paste("ab12", 6, true), "12");
        assert_eq!(code_paste("ab12", 6, false), "ab12");
    }

    #[test]
    fn insert_replaces_the_selection_and_leaves_the_caret_after_the_text() {
        let outcome = insert_transition("hello", EditState { anchor: 1, head: 4 }, "EY");
        assert_eq!(outcome.value.as_deref(), Some("hEYo"));
        assert_eq!(outcome.state, EditState::collapsed(3));
    }

    #[test]
    fn insert_at_a_collapsed_caret_adds_without_removing() {
        let outcome = insert_transition("ac", EditState::collapsed(1), "b");
        assert_eq!(outcome.value.as_deref(), Some("abc"));
        assert_eq!(outcome.state, EditState::collapsed(2));
    }

    /// A cut copies before it deletes, so the copied text has to come from the
    /// value *before* the edit — counted in characters, not bytes.
    #[test]
    fn selected_text_reads_the_range_by_character() {
        assert_eq!(
            selected_text("héllo wörld", EditState { anchor: 6, head: 11 }),
            "wörld"
        );
        assert_eq!(selected_text("hello", EditState::collapsed(2)), "");
    }

    #[test]
    fn word_ranges_cover_words_runs_of_punctuation_and_the_trailing_caret() {
        let text = "one two_three, four";
        assert_eq!(word_range_at(text, 1), (0, 3)); // inside "one"
        assert_eq!(word_range_at(text, 6), (4, 13)); // "two_three" keeps its underscore
        assert_eq!(word_range_at(text, 13), (13, 15)); // ", " is its own run
        assert_eq!(word_range_at(text, text.chars().count()), (15, 19)); // caret at end -> "four"
        assert_eq!(word_range_at("", 0), (0, 0));
    }
    // ── Text editing ────────────────────────────────────────────────────
    //
    // The editing model is pure, so the whole contract §Keyboard table is testable
    // with no window. It replaced an append-and-backspace stub that ignored the
    // caret entirely.

    use super::*;

    /// Apply a run of keys, returning the final value and cursor.
    fn type_keys(start: &str, keys: &[(&str, bool, bool)]) -> (String, EditState) {
        let mut value = start.to_string();
        let mut state = EditState {
            anchor: value.chars().count(),
            head: value.chars().count(),
        };
        for (key, shift, accel) in keys {
            if let Some(outcome) = edit_transition(&value, state, key, *shift, *accel) {
                state = outcome.state;
                if let Some(next) = outcome.value {
                    value = next;
                }
            }
        }
        (value, state)
    }

    #[test]
    fn characters_insert_at_the_caret_not_at_the_end() {
        // The old stub appended unconditionally; moving left then typing proves
        // the caret is respected.
        let (value, state) = type_keys(
            "ac",
            &[("left", false, false), ("b", false, false)],
        );
        assert_eq!(value, "abc");
        assert_eq!(state, EditState { anchor: 2, head: 2 });
    }

    #[test]
    fn backspace_deletes_before_the_caret() {
        let (value, _) = type_keys("abc", &[("left", false, false), ("backspace", false, false)]);
        assert_eq!(value, "ac");
    }

    #[test]
    fn delete_removes_the_character_at_the_caret() {
        let (value, _) = type_keys("abc", &[("home", false, false), ("delete", false, false)]);
        assert_eq!(value, "bc");
    }

    #[test]
    fn backspace_at_the_start_and_delete_at_the_end_are_inert_but_consumed() {
        // Consumed matters: an unhandled key would fall through to another
        // handler. `Some` with no value change is the contract.
        let outcome = edit_transition("abc", EditState { anchor: 0, head: 0 }, "backspace", false, false)
            .expect("backspace is ours even with nothing to delete");
        assert!(outcome.value.is_none());

        let outcome = edit_transition("abc", EditState { anchor: 3, head: 3 }, "delete", false, false)
            .expect("delete is ours even at the end");
        assert!(outcome.value.is_none());
    }

    #[test]
    fn arrows_home_and_end_move_without_changing_the_value() {
        for key in ["left", "right", "home", "end"] {
            let outcome = edit_transition("abc", EditState { anchor: 1, head: 1 }, key, false, false)
                .unwrap_or_else(|| panic!("{key} moves the caret"));
            assert!(outcome.value.is_none(), "{key} must not edit");
        }
        let (_, state) = type_keys("abc", &[("home", false, false)]);
        assert_eq!(state, EditState { anchor: 0, head: 0 });
    }

    #[test]
    fn shift_arrow_extends_a_selection_and_typing_replaces_it() {
        let (value, state) = type_keys(
            "abcd",
            &[("home", false, false), ("right", true, false), ("right", true, false)],
        );
        assert_eq!(value, "abcd", "extending must not edit");
        assert_eq!(state, EditState { anchor: 0, head: 2 });

        let (value, state) = type_keys(
            "abcd",
            &[
                ("home", false, false),
                ("right", true, false),
                ("right", true, false),
                ("X", false, false),
            ],
        );
        assert_eq!(value, "Xcd");
        assert_eq!(state, EditState { anchor: 1, head: 1 });
    }

    #[test]
    fn a_plain_arrow_collapses_a_selection_to_its_edge() {
        let state = EditState { anchor: 1, head: 3 };
        let left = edit_transition("abcd", state, "left", false, false).unwrap();
        assert_eq!(left.state, EditState { anchor: 1, head: 1 });
        let right = edit_transition("abcd", state, "right", false, false).unwrap();
        assert_eq!(right.state, EditState { anchor: 3, head: 3 });
    }

    #[test]
    fn select_all_then_type_replaces_everything() {
        let (value, _) = type_keys("abcd", &[("a", false, true), ("Z", false, false)]);
        assert_eq!(value, "Z");
    }

    #[test]
    fn backspace_over_a_selection_deletes_the_whole_range() {
        let (value, state) = type_keys("abcd", &[("a", false, true), ("backspace", false, false)]);
        assert_eq!(value, "");
        assert_eq!(state, EditState { anchor: 0, head: 0 });
    }

    #[test]
    fn keys_we_do_not_own_are_left_alone() {
        // Enter, Tab and Escape belong to submit/cancel/focus, and an accel
        // shortcut we do not implement must not be swallowed as text.
        for (key, accel) in [("enter", false), ("tab", false), ("escape", false), ("c", true)] {
            assert!(
                edit_transition("abc", EditState { anchor: 3, head: 3 }, key, false, accel).is_none(),
                "{key} must fall through"
            );
        }
    }

    #[test]
    fn a_cursor_past_a_shortened_value_is_clamped() {
        // The host owns the value and can rewrite it between frames.
        let outcome = edit_transition("ab", EditState { anchor: 9, head: 9 }, "backspace", false, false)
            .unwrap();
        assert_eq!(outcome.value.as_deref(), Some("a"));
    }

    #[test]
    fn multibyte_text_edits_by_character_not_byte() {
        let (value, _) = type_keys("héllo", &[("home", false, false), ("right", false, false), ("delete", false, false)]);
        assert_eq!(value, "hllo");
    }
}

// ── Slotted codes ───────────────────────────────────────────────────
//
// A code input is one value shown across N slots. The contract's web target
// hides a real `<input>` behind the slots and lets the browser own typing; the
// Rust targets have no such input, so the same rules live here — once, for
// every target — rather than in each backend's key handler.

/// What a keystroke does to a slotted code.
///
/// Returns the new value, or `None` when the key is not ours (so the host's
/// submit/cancel handling still sees it). A key that *is* ours but changes
/// nothing — backspace on an empty code, a digit when every slot is full —
/// returns the value unchanged, because it was still consumed.
pub fn code_transition(
    value: &str,
    key: &str,
    length: usize,
    numbers_only: bool,
) -> Option<String> {
    let mut chars: Vec<char> = value
        .chars()
        .filter(|c| !numbers_only || c.is_ascii_digit())
        .take(length)
        .collect();

    match key {
        "backspace" => {
            chars.pop();
            Some(chars.into_iter().collect())
        }
        // Clearing the whole code is worth a key of its own: with no caret to
        // hold down backspace against, six presses is the alternative.
        "escape" if !chars.is_empty() => Some(String::new()),
        _ => {
            let mut typed = key.chars();
            let (Some(c), None) = (typed.next(), typed.next()) else {
                return None;
            };
            if numbers_only && !c.is_ascii_digit() {
                // Consumed deliberately: a letter typed into a digits-only code
                // must not fall through and trigger a submit.
                return Some(chars.into_iter().collect());
            }
            if chars.len() < length {
                chars.push(c);
            }
            Some(chars.into_iter().collect())
        }
    }
}

/// Distribute pasted text into a code, replacing whatever was there.
///
/// Paste is the reason this component exists — one-time codes arrive on the
/// clipboard — so it sanitizes and clamps the same way typing does.
pub fn code_paste(text: &str, length: usize, numbers_only: bool) -> String {
    text.chars()
        .filter(|c| !numbers_only || c.is_ascii_digit())
        .take(length)
        .collect()
}
