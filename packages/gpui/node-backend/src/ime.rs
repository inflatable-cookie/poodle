//! Platform text input for fields: dead keys, IME composition, and the
//! services macOS expects a text control to answer.
//!
//! An earlier note in this repo claimed this needed an `EntityInputHandler`
//! bound to an entity, and that a `&Node -> AnyElement` backend therefore could
//! not have it. That was wrong. `Window::handle_input` takes
//! `impl InputHandler` — a plain public trait with no entity requirement, of
//! which `ElementInputHandler` is merely the entity-backed implementation gpui
//! ships. This is the direct implementation.
//!
//! **Three encodings meet here.** The vocabulary counts *characters*, the text
//! system counts *bytes*, and this platform boundary counts *UTF-16 code
//! units*, because that is what macOS speaks. Every conversion is explicit and
//! lives in this file; nothing above it needs to know UTF-16 exists.

use std::ops::Range;
use std::sync::Arc;

use gpui::{App, Bounds, InputHandler, Pixels, Point, UTF16Selection, Window};

/// What the field looks like right now, refreshed every paint.
///
/// The handler is registered once per frame with current values rather than
/// holding a borrow: the component rebuilds the whole tree each frame, so
/// anything cached here would be stale by the time the platform asked.
pub(crate) struct NodeInputHandler {
    pub id: String,
    pub value: String,
    /// Character indices, as the vocabulary expresses them.
    pub selection: (usize, usize),
    pub insert: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub select: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
}

fn utf16_len(text: &str) -> usize {
    text.chars().map(char::len_utf16).sum()
}

/// Character index -> UTF-16 offset.
fn utf16_for_char(text: &str, char_index: usize) -> usize {
    text.chars().take(char_index).map(char::len_utf16).sum()
}

/// UTF-16 offset -> character index, clamped, never landing inside a surrogate
/// pair.
fn char_for_utf16(text: &str, utf16: usize) -> usize {
    let mut seen = 0;
    for (chars, c) in text.chars().enumerate() {
        if seen >= utf16 {
            return chars;
        }
        seen += c.len_utf16();
    }
    text.chars().count()
}

impl InputHandler for NodeInputHandler {
    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<UTF16Selection> {
        let (a, b) = self.selection;
        let (start, end) = if a <= b { (a, b) } else { (b, a) };
        Some(UTF16Selection {
            range: utf16_for_char(&self.value, start)..utf16_for_char(&self.value, end),
            reversed: b < a,
        })
    }

    fn marked_text_range(&mut self, _window: &mut Window, _cx: &mut App) -> Option<Range<usize>> {
        super::input_text::marked_range(&self.id)
            .map(|(s, e)| utf16_for_char(&self.value, s)..utf16_for_char(&self.value, e))
    }

    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        adjusted: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<String> {
        let len = utf16_len(&self.value);
        let start = range_utf16.start.min(len);
        let end = range_utf16.end.min(len).max(start);
        if start != range_utf16.start || end != range_utf16.end {
            *adjusted = Some(start..end);
        }
        let from = char_for_utf16(&self.value, start);
        let to = char_for_utf16(&self.value, end);
        Some(self.value.chars().skip(from).take(to - from).collect())
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        text: &str,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        // A commit ends any composition.
        super::input_text::clear_marked(&self.id);
        if let Some(range) = range_utf16 {
            // The platform is replacing a range that is not the selection —
            // move the caret there first, so the shared insert rule (which
            // replaces the selection) does the right thing.
            let start = char_for_utf16(&self.value, range.start);
            let end = char_for_utf16(&self.value, range.end);
            if let Some(select) = &self.select {
                select(start, end);
            }
        }
        if let Some(insert) = &self.insert {
            insert(text);
        }
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _new_selected_range: Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut App,
    ) {
        // Composition in progress: the text is provisional and will be
        // replaced again as the user keeps typing. It still goes into the value
        // so it is visible, and the marked range records what to replace next.
        let (start, end) = match range_utf16 {
            Some(range) => (
                char_for_utf16(&self.value, range.start),
                char_for_utf16(&self.value, range.end),
            ),
            None => super::input_text::marked_range(&self.id).unwrap_or(self.selection),
        };
        if let Some(select) = &self.select {
            select(start, end);
        }
        if let Some(insert) = &self.insert {
            insert(new_text);
        }
        let marked_end = start + new_text.chars().count();
        if new_text.is_empty() {
            super::input_text::clear_marked(&self.id);
        } else {
            super::input_text::set_marked(&self.id, (start, marked_end));
        }
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut App) {
        super::input_text::clear_marked(&self.id);
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<Bounds<Pixels>> {
        // Where the candidate window should appear. Answered from the last
        // painted line, the same cache click-to-position uses.
        let from = char_for_utf16(&self.value, range_utf16.start);
        let to = char_for_utf16(&self.value, range_utf16.end);
        super::input_text::bounds_for_chars(&self.id, from, to)
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Option<usize> {
        let chars = super::input_text::char_index_for_position(&self.id, point)?;
        Some(utf16_for_char(&self.value, chars))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole reason this file exists: three encodings, and a conversion
    /// that must never land inside a surrogate pair.
    #[test]
    fn utf16_offsets_round_trip_through_astral_characters() {
        // '🎈' is one char, four bytes, and *two* UTF-16 code units.
        let text = "a🎈b";
        assert_eq!(utf16_len(text), 4);
        assert_eq!(utf16_for_char(text, 0), 0);
        assert_eq!(utf16_for_char(text, 1), 1);
        assert_eq!(utf16_for_char(text, 2), 3, "the balloon takes two units");
        assert_eq!(utf16_for_char(text, 3), 4);

        for chars in 0..=text.chars().count() {
            assert_eq!(char_for_utf16(text, utf16_for_char(text, chars)), chars);
        }
    }

    /// An offset pointing into the middle of a surrogate pair resolves to a
    /// whole character rather than splitting it.
    #[test]
    fn an_offset_inside_a_surrogate_pair_does_not_split_it() {
        let text = "a🎈b";
        // Unit 2 is the low half of the balloon.
        let at = char_for_utf16(text, 2);
        assert_eq!(at, 2, "resolves past the whole character");
        assert!(text.chars().nth(at).is_some());
    }

    #[test]
    fn offsets_past_the_end_clamp() {
        let text = "ab";
        assert_eq!(char_for_utf16(text, 99), 2);
        assert_eq!(utf16_for_char(text, 99), 2);
        assert_eq!(char_for_utf16("", 3), 0);
    }
}
