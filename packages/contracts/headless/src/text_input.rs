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
                Some(EditOutcome { value: None, state })
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
                Some(EditOutcome { value: None, state })
            } else {
                Some(EditOutcome {
                    value: Some(splice(value, state.head, state.head + 1, "")),
                    state: EditState::collapsed(state.head),
                })
            }
        }
        "a" if accel => Some(EditOutcome {
            value: None,
            state: EditState {
                anchor: 0,
                head: len,
            },
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
    let probe = if index == chars.len() {
        index - 1
    } else {
        index
    };
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

    /// Clicking a filled slot selects its character; clicking past the value
    /// collapses at the end, so typing appends rather than overwriting nothing.
    #[test]
    fn slot_clicks_select_a_filled_character_and_collapse_past_the_end() {
        assert_eq!(code_slot_selection(1, 4), (1, 2));
        assert_eq!(code_slot_selection(3, 4), (3, 4));
        // At or past the value length: collapsed.
        assert_eq!(code_slot_selection(4, 4), (4, 4));
        assert_eq!(code_slot_selection(5, 4), (4, 4));
        assert_eq!(code_slot_selection(0, 0), (0, 0));
    }

    /// Group ends follow the explicit partition; the last group never gets a
    /// trailing end.
    #[test]
    fn group_ends_follow_explicit_partitions() {
        assert_eq!(code_group_end_indices(20, &[5, 5, 5, 5]), vec![4, 9, 14]);
        assert_eq!(code_group_end_indices(6, &[3, 3]), vec![2]);
        assert_eq!(code_group_end_indices(6, &[6]), Vec::<usize>::new());
    }

    /// A single group, a partial partition, a zero group, or an empty length
    /// produces no breaks — grouping is never inferred from `length`.
    #[test]
    fn invalid_or_single_group_patterns_produce_no_breaks() {
        assert_eq!(code_group_end_indices(6, &[]), Vec::<usize>::new());
        assert_eq!(code_group_end_indices(6, &[2, 2]), Vec::<usize>::new());
        assert_eq!(code_group_end_indices(6, &[0, 6]), Vec::<usize>::new());
        assert_eq!(code_group_end_indices(6, &[7, 7]), Vec::<usize>::new());
        assert_eq!(code_group_end_indices(0, &[3, 3]), Vec::<usize>::new());
    }

    #[test]
    fn typing_over_a_selected_slot_replaces_it_in_place() {
        // "1234", slot 1 selected -> typing 9 gives "1934", caret after it.
        assert_eq!(
            code_insert_replacement("1234", "9", (1, 2), 4, true),
            Some(("1934".to_string(), 2))
        );
    }

    #[test]
    fn typing_at_the_end_appends_and_stops_at_the_length() {
        assert_eq!(
            code_insert_replacement("123", "4", (3, 3), 4, true),
            Some(("1234".to_string(), 3))
        );
        // Full: the value cannot grow, and the caret stays on the last slot.
        assert_eq!(
            code_insert_replacement("1234", "9", (4, 4), 4, true),
            Some(("1234".to_string(), 3))
        );
    }

    #[test]
    fn a_paste_overwrites_from_the_selection_and_clamps() {
        assert_eq!(
            code_insert_replacement("123456", "99", (1, 2), 6, true),
            Some(("199456".to_string(), 3))
        );
    }

    #[test]
    fn sanitized_away_input_reports_nothing() {
        assert_eq!(code_insert_replacement("12", "a", (2, 2), 6, true), None);
        assert_eq!(
            code_insert_replacement("12", "a", (2, 2), 6, false),
            Some(("12a".to_string(), 3))
        );
    }

    fn snap(value: &str, head: usize) -> EditSnapshot {
        EditSnapshot {
            value: value.to_string(),
            state: EditState { anchor: head, head },
        }
    }

    /// A run of typing is one undo step, not one per keystroke.
    #[test]
    fn consecutive_typing_coalesces_into_one_step() {
        assert!(coalesces(&snap("ab", 2), &snap("abc", 3)));
        assert!(coalesces(&snap("", 0), &snap("a", 1)));
        // Typing in the middle still coalesces, as long as the caret follows.
        assert!(coalesces(&snap("ac", 1), &snap("abc", 2)));
    }

    #[test]
    fn deletions_and_pastes_each_begin_a_new_step() {
        // Shrinking.
        assert!(!coalesces(&snap("abc", 3), &snap("ab", 2)));
        // More than one character at once — a paste.
        assert!(!coalesces(&snap("a", 1), &snap("abcd", 4)));
        // Same length, different content.
        assert!(!coalesces(&snap("abc", 3), &snap("abd", 3)));
    }

    /// A caret that jumped means the user went somewhere else; the previous run
    /// is finished even though the next edit is still a single insertion.
    #[test]
    fn a_moved_caret_breaks_the_run() {
        // Typed at 1 while the previous caret was at 3.
        assert!(!coalesces(&snap("abc", 3), &snap("axbc", 2)));
        // Insertion at the caret, but the caret did not advance with it.
        assert!(!coalesces(&snap("ab", 2), &snap("abc", 1)));
    }

    /// Replacing a selection is destructive, so it starts its own step even
    /// when the result happens to be one character longer.
    #[test]
    fn replacing_a_selection_begins_a_new_step() {
        let selected = EditSnapshot {
            value: "abc".into(),
            state: EditState { anchor: 0, head: 2 },
        };
        assert!(!coalesces(&selected, &snap("abcd", 4)));
    }

    fn typed_code(start: &str, keys: &[&str], length: usize) -> (String, (usize, usize)) {
        let mut value = start.to_string();
        let n = value.chars().count();
        let mut sel = (n, n);
        for key in keys {
            if let Some((next, next_sel)) = code_transition(&value, sel, key, length, true) {
                value = next;
                sel = next_sel;
            }
        }
        (value, sel)
    }

    /// Filling left to right, and then what a *further* digit does — which is
    /// not what it looks like. `codeInsertReplacement` caps the caret at
    /// `length - 1`, so once the code is full the caret sits on the last slot
    /// and the next digit **replaces** it. Surprising, and it is the web
    /// target's behaviour; the Rust port follows it rather than inventing a
    /// second rule.
    #[test]
    fn digits_fill_left_to_right_then_overwrite_the_last_slot() {
        let (value, sel) = typed_code("", &["1", "2", "3"], 3);
        assert_eq!((value.as_str(), sel), ("123", (2, 2)));
        let (value, _) = typed_code("", &["1", "2", "3", "4"], 3);
        assert_eq!(value, "124");
    }

    /// The whole point of the caret: typing over a selected slot replaces that
    /// slot rather than appending.
    #[test]
    fn typing_into_a_selected_slot_replaces_it() {
        let (value, sel) = code_transition("1234", (1, 2), "9", 4, true).expect("ours");
        assert_eq!(value, "1934");
        assert_eq!(sel, (2, 2));
    }

    /// Consumed, not forwarded: a letter in a digits-only code must not fall
    /// through to whatever handles plain keys.
    #[test]
    fn letters_are_swallowed_by_a_numbers_only_code() {
        assert_eq!(
            code_transition("12", (2, 2), "a", 6, true)
                .map(|(v, _)| v)
                .as_deref(),
            Some("12")
        );
        assert_eq!(
            code_transition("12", (2, 2), "a", 6, false)
                .map(|(v, _)| v)
                .as_deref(),
            Some("12a")
        );
    }

    #[test]
    fn backspace_removes_before_the_caret_and_is_inert_at_the_start() {
        let (value, sel) = code_transition("123", (3, 3), "backspace", 6, true).expect("ours");
        assert_eq!((value.as_str(), sel), ("12", (2, 2)));
        // Mid-code, not just at the end.
        let (value, sel) = code_transition("123", (2, 2), "backspace", 6, true).expect("ours");
        assert_eq!((value.as_str(), sel), ("13", (1, 1)));
        // A selected slot is what gets removed.
        let (value, sel) = code_transition("123", (0, 1), "backspace", 6, true).expect("ours");
        assert_eq!((value.as_str(), sel), ("23", (0, 0)));
        let (value, _) = code_transition("", (0, 0), "backspace", 6, true).expect("ours");
        assert_eq!(value, "");
    }

    #[test]
    fn arrows_move_the_caret_without_touching_the_value() {
        let (value, sel) = code_transition("1234", (2, 2), "left", 4, true).expect("ours");
        assert_eq!((value.as_str(), sel), ("1234", (1, 1)));
        let (_, sel) = code_transition("1234", (0, 0), "left", 4, true).expect("ours");
        assert_eq!(sel, (0, 0), "clamped at the first slot");
        let (_, sel) = code_transition("12", (0, 0), "right", 4, true).expect("ours");
        assert_eq!(sel, (1, 1));
    }

    /// Escape only claims the key when there is something to clear, so an empty
    /// code still lets a dialog close on Escape.
    #[test]
    fn escape_clears_a_filled_code_and_passes_through_an_empty_one() {
        assert_eq!(
            code_transition("123", (3, 3), "escape", 6, true)
                .map(|(v, _)| v)
                .as_deref(),
            Some("")
        );
        assert!(code_transition("", (0, 0), "escape", 6, true).is_none());
    }

    #[test]
    fn multi_character_keys_are_not_ours() {
        assert!(code_transition("1", (1, 1), "enter", 6, true).is_none());
        assert!(code_transition("1", (1, 1), "tab", 6, true).is_none());
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
            selected_text(
                "héllo wörld",
                EditState {
                    anchor: 6,
                    head: 11
                }
            ),
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
        let (value, state) = type_keys("ac", &[("left", false, false), ("b", false, false)]);
        assert_eq!(value, "abc");
        assert_eq!(state, EditState { anchor: 2, head: 2 });
    }

    #[test]
    fn backspace_deletes_before_the_caret() {
        let (value, _) = type_keys(
            "abc",
            &[("left", false, false), ("backspace", false, false)],
        );
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
        let outcome = edit_transition(
            "abc",
            EditState { anchor: 0, head: 0 },
            "backspace",
            false,
            false,
        )
        .expect("backspace is ours even with nothing to delete");
        assert!(outcome.value.is_none());

        let outcome = edit_transition(
            "abc",
            EditState { anchor: 3, head: 3 },
            "delete",
            false,
            false,
        )
        .expect("delete is ours even at the end");
        assert!(outcome.value.is_none());
    }

    #[test]
    fn arrows_home_and_end_move_without_changing_the_value() {
        for key in ["left", "right", "home", "end"] {
            let outcome =
                edit_transition("abc", EditState { anchor: 1, head: 1 }, key, false, false)
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
            &[
                ("home", false, false),
                ("right", true, false),
                ("right", true, false),
            ],
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
        for (key, accel) in [
            ("enter", false),
            ("tab", false),
            ("escape", false),
            ("c", true),
        ] {
            assert!(
                edit_transition("abc", EditState { anchor: 3, head: 3 }, key, false, accel)
                    .is_none(),
                "{key} must fall through"
            );
        }
    }

    #[test]
    fn a_cursor_past_a_shortened_value_is_clamped() {
        // The host owns the value and can rewrite it between frames.
        let outcome = edit_transition(
            "ab",
            EditState { anchor: 9, head: 9 },
            "backspace",
            false,
            false,
        )
        .unwrap();
        assert_eq!(outcome.value.as_deref(), Some("a"));
    }

    #[test]
    fn multibyte_text_edits_by_character_not_byte() {
        let (value, _) = type_keys(
            "héllo",
            &[
                ("home", false, false),
                ("right", false, false),
                ("delete", false, false),
            ],
        );
        assert_eq!(value, "hllo");
    }
}

// ── Undo ────────────────────────────────────────────────────────────
//
// History is *ephemeral UI state*, like a blink phase or a scroll offset — it
// belongs to the field while it is on screen and means nothing once it is
// gone. So the backend keeps the stack, and this layer owns the only part that
// is a text-editing decision: what counts as one undoable step.

/// A value and where the caret was in it.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EditSnapshot {
    pub value: String,
    pub state: EditState,
}

/// Whether `next` should replace the top of the stack rather than push a new
/// entry.
///
/// Undoing one character at a time is nobody's idea of undo, so a continuous
/// run of typing collapses into a single step. A run ends when the edit stops
/// being "more text arriving at the caret": a deletion, a paste, or a caret
/// that jumped somewhere else all begin a new one.
pub fn coalesces(previous: &EditSnapshot, next: &EditSnapshot) -> bool {
    let prev_len = previous.value.chars().count();
    let next_len = next.value.chars().count();

    // Only growth coalesces, and only one character at a time — a paste
    // arriving as a single larger jump is its own step, because undoing it
    // whole is what a user expects.
    if next_len != prev_len + 1 {
        return false;
    }
    // Typing collapses a selection, so the previous step having one means this
    // edit replaced something: a destructive change worth its own entry.
    if previous.state.anchor != previous.state.head {
        return false;
    }
    // The insertion has to be *at* the previous caret, and leave the caret just
    // after it. Otherwise the caret moved between keystrokes and the run broke.
    if next.state.anchor != next.state.head || next.state.head != previous.state.head + 1 {
        return false;
    }
    // Everything before the caret must be unchanged, and everything after it.
    let cut = previous.state.head;
    let prev_before: String = previous.value.chars().take(cut).collect();
    let next_before: String = next.value.chars().take(cut).collect();
    let prev_after: String = previous.value.chars().skip(cut).collect();
    let next_after: String = next.value.chars().skip(cut + 1).collect();
    prev_before == next_before && prev_after == next_after
}

// ── Slotted codes ───────────────────────────────────────────────────
//
// A code input is one value shown across N slots. The contract's web target
// hides a real `<input>` behind the slots and lets the browser own typing; the
// Rust targets have no such input, so the same rules live here — once, for
// every target — rather than in each backend's key handler.

/// What a keystroke does to a slotted code, at the caret.
///
/// Returns the new value and caret, or `None` when the key is not ours (so the
/// host's submit/cancel handling still sees it). A key that *is* ours but
/// changes nothing returns the state unchanged, because it was still consumed.
///
/// Deletion follows the web target, which leaves backspace to the browser's
/// native `<input>`: remove the selection if there is one, otherwise the
/// character before the caret, shifting the rest left.
pub fn code_transition(
    value: &str,
    selection: (usize, usize),
    key: &str,
    length: usize,
    numbers_only: bool,
) -> Option<(String, (usize, usize))> {
    let chars: Vec<char> = value
        .chars()
        .filter(|c| !numbers_only || c.is_ascii_digit())
        .take(length)
        .collect();
    let len = chars.len();
    let (start, end) = (
        selection.0.min(len),
        selection.1.min(len).max(selection.0.min(len)),
    );

    let splice = |from: usize, to: usize| -> String {
        chars
            .iter()
            .take(from)
            .chain(chars.iter().skip(to))
            .collect()
    };

    match key {
        "backspace" => {
            if start != end {
                Some((splice(start, end), (start, start)))
            } else if start == 0 {
                // Consumed but inert: nothing before the caret to remove.
                Some((chars.into_iter().collect(), (0, 0)))
            } else {
                Some((splice(start - 1, start), (start - 1, start - 1)))
            }
        }
        "delete" => {
            if start != end {
                Some((splice(start, end), (start, start)))
            } else if start >= len {
                Some((chars.into_iter().collect(), (start, start)))
            } else {
                Some((splice(start, start + 1), (start, start)))
            }
        }
        "left" => Some((chars.into_iter().collect(), {
            let at = start.saturating_sub(1);
            (at, at)
        })),
        "right" => Some((chars.into_iter().collect(), {
            let at = (end + 1).min(length.saturating_sub(1)).min(len);
            (at, at)
        })),
        "home" => Some((chars.into_iter().collect(), (0, 0))),
        "end" => Some((chars.into_iter().collect(), (len, len))),
        // Clearing the whole code is worth a key of its own: with a fixed
        // number of slots, the alternative is one backspace per slot.
        "escape" if len > 0 => Some((String::new(), (0, 0))),
        _ => {
            let current: String = chars.into_iter().collect();
            let mut typed = key.chars();
            let (Some(_), None) = (typed.next(), typed.next()) else {
                return None;
            };
            match code_insert_replacement(&current, key, (start, end), length, numbers_only) {
                Some((next, caret)) => Some((next, (caret, caret))),
                // Sanitized away — a letter in a digits-only code. Consumed
                // deliberately, so it cannot fall through and submit.
                None => Some((current, (start, end))),
            }
        }
    }
}

/// What clicking slot `index` selects.
///
/// A faithful port of `codeSlotSelection` in `packages/core/src/code-input.ts`,
/// which the contract names as the authority: clicking a **filled** slot
/// selects that character so the next keystroke replaces it in place, while
/// clicking past the end of the value collapses at the end.
pub fn code_slot_selection(index: usize, value_len: usize) -> (usize, usize) {
    let start = index.min(value_len);
    let end = if index < value_len { index + 1 } else { start };
    (start, end)
}

/// Slot indices that end a visual group.
///
/// A faithful port of `codeGroupEndIndices` in
/// `packages/core/src/code-input.ts`: group lengths are accepted only as one
/// complete positive-integer partition of `length`. An omitted, single-group,
/// partial, or invalid pattern produces no breaks, keeping grouping
/// presentation-only and leaving value and caret behaviour untouched.
pub fn code_group_end_indices(length: usize, groups: &[usize]) -> Vec<usize> {
    if length == 0
        || groups.len() < 2
        || groups.iter().any(|group| *group == 0)
        || groups.iter().sum::<usize>() != length
    {
        return Vec::new();
    }
    let mut consumed = 0;
    groups[..groups.len() - 1]
        .iter()
        .map(|group| {
            consumed += group;
            consumed - 1
        })
        .collect()
}

/// Typing into a code with a selection: overwrite from `start`, extending the
/// replaced span to cover what was inserted, capped to `length`.
///
/// Ported from `codeInsertReplacement`. Returns `None` when the sanitized input
/// is empty — a letter typed into a digits-only code changes nothing, and the
/// caller decides whether to swallow the key.
pub fn code_insert_replacement(
    value: &str,
    data: &str,
    selection: (usize, usize),
    length: usize,
    numbers_only: bool,
) -> Option<(String, usize)> {
    let next: String = data
        .chars()
        .filter(|c| !numbers_only || c.is_ascii_digit())
        .collect();
    if next.is_empty() {
        return None;
    }

    let chars: Vec<char> = value.chars().collect();
    let (start, end) = selection;
    let start = start.min(chars.len());
    let replacement_end = end.max((start + next.chars().count()).min(chars.len()));

    let mut out: String = chars.iter().take(start).collect();
    out.push_str(&next);
    out.extend(chars.iter().skip(replacement_end));
    let out: String = out.chars().take(length).collect();

    let caret = (start + next.chars().count()).min(length.saturating_sub(1));
    Some((out, caret))
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
