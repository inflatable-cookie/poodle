//! The value of an input, with a caret and selection drawn at measured
//! positions.
//!
//! Everything else in this backend maps a `Node` onto a `div`. This does not,
//! because a caret cannot be laid out: putting it at character 4 means knowing
//! how wide the first four glyphs are in this font at this size, and only the
//! text system knows that. gpui's own `examples/input.rs` is the reference —
//! `shape_line` to measure, `x_for_index` to place the caret, and
//! `closest_index_for_x` to turn a click back into a character index.
//!
//! Selection and caret are **painted quads**, not layout participants. An
//! earlier pass split the value into sibling text runs so the caret could sit
//! between them; that positions a caret without measuring, but it re-shapes
//! the text at every run boundary (so glyphs shift as the caret moves through
//! kerned pairs), and it cannot answer "which character did I click on?" at
//! all.
//!
//! The character/byte boundary lives here. The vocabulary speaks character
//! indices; `ShapedLine` speaks bytes. Conversions happen at the edges of this
//! file and nowhere else.

use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use gpui::{
    fill, point, px, relative, size, App, Bounds, Element, ElementId, GlobalElementId, Hsla,
    LayoutId, PaintQuad, Pixels, Point, ShapedLine, Style, TextRun, Window,
};

/// Full blink cycle. Matches the platform default closely enough that a caret
/// next to a native field does not look wrong.
const BLINK_PERIOD: Duration = Duration::from_millis(1060);

/// What the last paint measured, kept so a *later* mouse event can ask where a
/// character is. Mouse-down carries a position and nothing else; without the
/// previous frame's line there is no way to resolve it. Same trick as the gpui
/// example's `last_layout`/`last_bounds`.
struct Measured {
    line: ShapedLine,
    bounds: Bounds<Pixels>,
    /// The x the text was actually painted at, which is the field's left edge
    /// minus any scroll.
    origin_x: Pixels,
    /// The text the line was shaped from, so index conversions use the same
    /// string the measurement did.
    text: String,
}

thread_local! {
    static MEASURED: RefCell<HashMap<String, Measured>> = RefCell::new(HashMap::new());
    /// When each field's caret last moved or its value changed. A caret is
    /// solid immediately after you touch it and only then starts blinking —
    /// blinking straight through a keystroke reads as a dropped character.
    static BLINK_EPOCH: RefCell<HashMap<String, (Instant, String)>> = RefCell::new(HashMap::new());
    /// How far each focused field has scrolled its text left to keep the caret
    /// visible. Cleared on blur, so a field re-read from the start.
    static SCROLL: RefCell<HashMap<String, Pixels>> = RefCell::new(HashMap::new());
}

/// Character index -> byte offset, clamped to the string.
fn byte_for_char(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map(|(byte, _)| byte)
        .unwrap_or(text.len())
}

/// Byte offset -> character index, the inverse of [`byte_for_char`].
fn char_for_byte(text: &str, byte: usize) -> usize {
    text.char_indices().take_while(|(b, _)| *b < byte).count()
}

/// Where a pointer x lands in a field's value, as a character index.
///
/// Answers from the last painted line, so it is only meaningful once the field
/// has been painted at least once — true for any field the user can click.
/// Returns `None` when this id has never been measured.
pub(crate) fn char_index_for_position(id: &str, position: Point<Pixels>) -> Option<usize> {
    MEASURED.with(|m| {
        let m = m.borrow();
        let measured = m.get(id)?;
        if measured.text.is_empty() {
            return Some(0);
        }
        let byte = measured
            .line
            .closest_index_for_x(position.x - measured.origin_x);
        Some(char_for_byte(&measured.text, byte))
    })
}

/// Drop a field's cached measurement. Called when a field loses focus so a
/// stale line cannot answer a click on whatever replaced it.
pub(crate) fn forget(id: &str) {
    MEASURED.with(|m| {
        m.borrow_mut().remove(id);
    });
    BLINK_EPOCH.with(|b| {
        b.borrow_mut().remove(id);
    });
    SCROLL.with(|s| {
        s.borrow_mut().remove(id);
    });
}

pub(crate) struct InputText {
    pub id: String,
    /// The text to draw: the value, or the placeholder when the value is empty.
    pub display: String,
    /// The value itself, which is what selection indices count into. Differs
    /// from `display` only when the placeholder is showing, and then the
    /// selection is necessarily empty.
    pub value: String,
    pub color: Hsla,
    /// `None` when unfocused: no caret, no selection, no blink.
    pub selection: Option<(usize, usize)>,
    pub caret_color: Hsla,
    pub selection_color: Hsla,
    pub focused: bool,
}

pub(crate) struct PrepaintState {
    line: Option<ShapedLine>,
    caret: Option<PaintQuad>,
    selection: Option<PaintQuad>,
    blinking: bool,
    /// Where the text is actually drawn, once the caret has been scrolled into
    /// view. Hit-testing has to use the same origin or clicks land on the
    /// wrong character in a scrolled field.
    origin_x: Pixels,
}

impl gpui::IntoElement for InputText {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for InputText {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let run = TextRun {
            len: self.display.len(),
            font: style.font(),
            color: self.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let line =
            window
                .text_system()
                .shape_line(self.display.clone().into(), font_size, &[run], None);

        // The caret is the height of the text, not the field — a field-height
        // caret in a padded input looks like a divider.
        let caret_h = font_size * 1.2;
        let caret_top = bounds.top() + (bounds.size.height - caret_h) / 2.0;

        // Scroll the caret into view. A value wider than its field is clipped,
        // and without this the caret walks off the right edge and typing
        // continues somewhere you cannot see.
        let width = bounds.size.width;
        let scroll = if let Some((_, head)) = self.selection.filter(|_| self.focused) {
            let caret_x = line.x_for_index(byte_for_char(&self.value, head));
            let previous = SCROLL.with(|s| s.borrow().get(&self.id).copied().unwrap_or(px(0.0)));
            // Keep a character of context either side, so the caret never sits
            // flush against the edge it just reached.
            let margin = px(8.0);
            let scroll = if caret_x - previous > width - margin {
                caret_x - width + margin
            } else if caret_x - previous < margin {
                (caret_x - margin).max(px(0.0))
            } else {
                previous
            };
            // Never scroll past the end of the text: short values stay put.
            let scroll = scroll.min((line.width - width).max(px(0.0))).max(px(0.0));
            SCROLL.with(|s| {
                s.borrow_mut().insert(self.id.clone(), scroll);
            });
            scroll
        } else {
            SCROLL.with(|s| {
                s.borrow_mut().remove(&self.id);
            });
            px(0.0)
        };
        let origin_x = bounds.left() - scroll;

        let mut caret = None;
        let mut selection = None;
        let mut blinking = false;

        if let Some((start, end)) = self.selection.filter(|_| self.focused) {
            let (start, end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            if start == end {
                let x = line.x_for_index(byte_for_char(&self.value, start));
                // Restart the blink whenever the value or the caret moved, so
                // typing never hides the caret mid-keystroke.
                let signature = format!("{}\u{0}{start}", self.value);
                let now = Instant::now();
                let epoch = BLINK_EPOCH.with(|b| {
                    let mut b = b.borrow_mut();
                    let entry = b
                        .entry(self.id.clone())
                        .or_insert_with(|| (now, signature.clone()));
                    if entry.1 != signature {
                        *entry = (now, signature.clone());
                    }
                    entry.0
                });
                let phase = now.duration_since(epoch).as_secs_f32()
                    % BLINK_PERIOD.as_secs_f32();
                blinking = true;
                if phase < BLINK_PERIOD.as_secs_f32() / 2.0 {
                    caret = Some(fill(
                        Bounds::new(point(origin_x + x, caret_top), size(px(1.0), caret_h)),
                        self.caret_color,
                    ));
                }
            } else {
                let x0 = line.x_for_index(byte_for_char(&self.value, start));
                let x1 = line.x_for_index(byte_for_char(&self.value, end));
                selection = Some(fill(
                    Bounds::from_corners(
                        // Clamped to the field: a selection running past the
                        // visible run must not paint over the padding or the
                        // affixes beside it.
                        point((origin_x + x0).max(bounds.left()), caret_top),
                        point((origin_x + x1).min(bounds.right()), caret_top + caret_h),
                    ),
                    self.selection_color,
                ));
            }
        }

        PrepaintState {
            line: Some(line),
            caret,
            selection,
            blinking,
            origin_x,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection);
        }
        let line = prepaint.line.take().expect("prepaint shapes the line");
        let text_top = bounds.top() + (bounds.size.height - window.line_height()) / 2.0;
        let origin_x = prepaint.origin_x;
        let _ = line.paint(
            point(origin_x, text_top),
            window.line_height(),
            window,
            cx,
        );
        if let Some(caret) = prepaint.caret.take() {
            window.paint_quad(caret);
        }
        if prepaint.blinking {
            // Nothing else invalidates on a timer, so the blink has to ask for
            // the next frame itself. Only while focused: an idle window with an
            // unfocused field still parks at zero repaints.
            window.request_animation_frame();
        }

        MEASURED.with(|m| {
            m.borrow_mut().insert(
                self.id.clone(),
                Measured {
                    line,
                    bounds,
                    origin_x,
                    text: self.display.clone(),
                },
            );
        });
    }
}

/// Convenience for the backend: build the element from a node's input kind.
#[allow(clippy::too_many_arguments)]
pub(crate) fn input_text(
    id: String,
    display: String,
    value: String,
    color: Hsla,
    selection: Option<(usize, usize)>,
    caret_color: Hsla,
    selection_color: Hsla,
    focused: bool,
) -> InputText {
    InputText {
        id,
        display,
        value,
        color,
        selection,
        caret_color,
        selection_color,
        focused,
    }
}

/// Shared drag-select state: which field is mid-drag, and the anchor the drag
/// started from. Selection needs an anchor that survives across move events.
thread_local! {
    static SELECTING: RefCell<Option<(String, usize)>> = const { RefCell::new(None) };
}

pub(crate) fn begin_select(id: &str, anchor: usize) {
    SELECTING.with(|s| *s.borrow_mut() = Some((id.to_string(), anchor)));
}

pub(crate) fn drag_anchor(id: &str) -> Option<usize> {
    SELECTING.with(|s| {
        s.borrow()
            .as_ref()
            .filter(|(active, _)| active == id)
            .map(|(_, anchor)| *anchor)
    })
}

pub(crate) fn end_select() {
    SELECTING.with(|s| *s.borrow_mut() = None);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vocabulary counts characters; `ShapedLine` counts bytes. Every
    /// caret position crosses that boundary, so a multibyte value is where a
    /// naive `&value[..index]` puts the caret inside a glyph — or panics.
    #[test]
    fn char_and_byte_offsets_round_trip_through_multibyte_text() {
        let text = "héllo wörld";
        for char_index in 0..=text.chars().count() {
            let byte = byte_for_char(text, char_index);
            assert!(text.is_char_boundary(byte), "byte {byte} split a glyph");
            assert_eq!(char_for_byte(text, byte), char_index);
        }
    }

    #[test]
    fn char_offsets_past_the_end_clamp_to_the_length() {
        let text = "ab";
        assert_eq!(byte_for_char(text, 2), 2);
        assert_eq!(byte_for_char(text, 99), 2);
        assert_eq!(char_for_byte(text, 99), 2);
    }

    /// An emoji is one `char` in some places and several bytes everywhere;
    /// the caret must land after the whole thing, not inside it.
    #[test]
    fn a_caret_after_an_emoji_lands_on_a_boundary() {
        let text = "a🎈b";
        let byte = byte_for_char(text, 2);
        assert!(text.is_char_boundary(byte));
        assert_eq!(&text[..byte], "a🎈");
    }

    /// A drag anchor belongs to one field. Without the id check, a drag begun
    /// in one input would keep extending a selection over another.
    #[test]
    fn drag_anchors_are_scoped_to_the_field_that_started_the_drag() {
        begin_select("field-a", 3);
        assert_eq!(drag_anchor("field-a"), Some(3));
        assert_eq!(drag_anchor("field-b"), None);
        end_select();
        assert_eq!(drag_anchor("field-a"), None);
    }
}
