//! CodeInput — segmented code entry with visual digit slots.
//!
//! Contract: `docs/contracts/components/code-input.md`
//! Ported from: `packages/jetstream/components/src/code_input.rs`.
//!
//! Runtime gaps (noted): the real/hidden input, paste, autofill, one-time-code
//! autocomplete, slot-click caret placement and the text caret are host/engine
//! concerns. This builder renders the slot grid, the distributed value, the
//! active-slot highlight, and the error label.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, ShadowLayer,
    TextAlign,
};
use poodle_specs::{CodeInputSpec, ControlDensity, ValidationState};

use crate::color::with_alpha;
use crate::presentation::{
    code_input_slot_font_rem, code_input_slot_size_rem, rem_to_px, resolve_semantic_size,
};

/// Host callbacks.
///
/// `on_value_change` reports the sanitized code on every keystroke;
/// `on_complete` fires the moment it reaches `length`, which is the whole point
/// of a one-time-code field — the host submits without a button.
#[derive(Default)]
pub struct CodeInputHandlers {
    pub on_value_change: Option<poodle_node::TextChangeHandler>,
    pub on_complete: Option<poodle_node::TextChangeHandler>,
}

pub fn code_input(spec: &CodeInputSpec, theme: &dyn ThemeProvider) -> Node {
    code_input_with_handlers(spec, theme, CodeInputHandlers::default())
}

/// Render a code input that can actually be typed into.
///
/// The contract's web target hides a real `<input>` behind the slots and lets
/// the browser own typing. There is no such input here, so the slot row itself
/// takes focus and the keys, and the slots stay pure visuals — the same
/// division, reached differently.
pub fn code_input_with_handlers(
    spec: &CodeInputSpec,
    theme: &dyn ThemeProvider,
    handlers: CodeInputHandlers,
) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let validation = spec.effective_validation_state();
    let is_invalid = validation == ValidationState::Invalid;

    // ── Token resolution ──
    let surface = theme.resolve_color("color.background.surface");
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_default = theme.resolve_color("color.border.default");
    let accent_border = theme.resolve_color("color.accent.border");
    let focus_ring = theme.resolve_color("color.accent.focusRing");
    let danger = theme.resolve_color("color.status.danger");
    let radius = theme.resolve_radius("radius.control");

    // Contract §7: only the invalid case (or an error) changes slot colors.
    let slot_border = if is_invalid { danger } else { border_default };
    let active_border = if is_invalid { danger } else { accent_border };
    let active_ring = if is_invalid { danger } else { focus_ring };

    // ── Sizing (contract §7) ──
    // Slots are square at every size; font follows the slot font ladder.
    let slot_size = rem_to_px(code_input_slot_size_rem(effective_size));
    let font_size = rem_to_px(code_input_slot_font_rem(effective_size));
    let border_width = rem_to_px(0.0625); // 1px = 0.0625rem, contract border width
                                          // Inter-slot gap: compact space.inline.xs, default space.inline.sm,
                                          // comfortable space.inline.md.
    let gap = match spec.density {
        ControlDensity::Compact => theme.resolve_space("space.inline.xs"),
        ControlDensity::Default => theme.resolve_space("space.inline.sm"),
        ControlDensity::Comfortable => theme.resolve_space("space.inline.md"),
    };
    // Contract §7 split-after: fixed margin-right = space.inline.md at index 2
    // when length == 6 (3+3 grouping).
    let split_margin = theme.resolve_space("space.inline.md");

    // ── Distribute the sanitized value across slots (per numbers_only) ──
    let chars = spec.sanitized_chars();

    // ── Root container ──
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.spacing.gap = gap;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.interaction.focusable = true;

    // The old tier keeps the component label, slot row, and supporting copy
    // in one column. Keeping that wrapper is part of the visual contract.
    let active_idx = chars.len().min(spec.length.saturating_sub(1));

    // ── Visual slots ──
    for i in 0..spec.length {
        let ch = chars.get(i).copied();
        // Active slot = the next empty slot, or the final slot when complete.
        let is_active = i == active_idx && !spec.is_disabled;
        let display_text = match ch {
            Some(_) if spec.mask => "\u{2022}".to_string(),
            Some(c) => c.to_string(),
            None => String::new(),
        };

        let slot_text_color = if ch.is_some() {
            text_primary
        } else {
            text_secondary
        };

        let mut slot = Node::container();
        {
            let s = &mut slot.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(slot_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(slot_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = radius;
            c.top_right = radius;
            c.bottom_right = radius;
            c.bottom_left = radius;
            s.descriptor.background = Some(surface);
            s.descriptor.border.width = border_width;
            s.descriptor.border.color = if is_active {
                active_border
            } else {
                slot_border
            };
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            if is_active {
                s.shadow_layers = vec![ShadowLayer {
                    offset_x: 0.0,
                    offset_y: 0.0,
                    blur: 0.0,
                    spread: rem_to_px(0.125),
                    color: with_alpha(active_ring, active_ring.3 * 0.28),
                    inset: false,
                }];
            }
            // 3+3 split for 6-digit codes.
            if spec.length == 6 && i == 2 {
                s.descriptor.layout.spacing.margin.right = split_margin;
            }
        }

        // Slot value uses the code font family (contract §7).
        let mut value = Node::text(&display_text);
        {
            let s = &mut value.style;
            s.text_size = Some(font_size);
            s.descriptor.text_color = Some(slot_text_color);
            s.text_weight = Some(600);
            s.text_align = Some(TextAlign::Center);
        }

        row = row.child(slot.child(value));
    }

    // Keys land on the slot row, not on any one slot: the value is one string
    // and the active slot is derived from its length, so there is nothing
    // per-slot to dispatch to.
    if !spec.is_disabled
        && (handlers.on_value_change.is_some() || handlers.on_complete.is_some())
    {
        row.interaction.focusable = true;
        let value = spec.current_value().to_string();
        let length = spec.length;
        let numbers_only = spec.numbers_only;
        let on_value_change = handlers.on_value_change.clone();
        let on_complete = handlers.on_complete.clone();
        row.interaction.on_edit_key = Some(std::sync::Arc::new(move |key: &str, _mods| {
            let Some(next) =
                poodle_headless::text_input::code_transition(&value, key, length, numbers_only)
            else {
                return;
            };
            if next == value {
                return;
            }
            if let Some(handler) = &on_value_change {
                handler(&next);
            }
            // Completion is a distinct event, and it fires on the transition
            // *into* a full code — not on every keystroke that leaves it full,
            // which a full code cannot produce anyway.
            if next.chars().count() == length {
                if let Some(handler) = &on_complete {
                    handler(&next);
                }
            }
        }));
        // Paste arrives as content rather than a keystroke, and a one-time code
        // is far more often pasted than typed.
        let paste_value = spec.current_value().to_string();
        let on_value_change = handlers.on_value_change.clone();
        let on_complete = handlers.on_complete;
        row.interaction.on_edit_insert = Some(std::sync::Arc::new(move |text: &str| {
            let next = poodle_headless::text_input::code_paste(text, length, numbers_only);
            if next == paste_value {
                return;
            }
            if let Some(handler) = &on_value_change {
                handler(&next);
            }
            if next.chars().count() == length {
                if let Some(handler) = &on_complete {
                    handler(&next);
                }
            }
        }));
    }

    let mut outer = Node::container();
    {
        let s = &mut outer.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
    }
    if !spec.label.is_empty() {
        let mut label = Node::text(spec.label.as_str());
        label.style.text_size = Some(rem_to_px(crate::presentation::size_font_rem(
            effective_size,
        )));
        label.style.text_weight = Some(500);
        label.style.descriptor.text_color = Some(text_primary);
        outer = outer.child(label);
    }
    outer = outer.child(row);

    if let Some(ref hint) = spec.hint {
        if spec.error.is_none() {
            let mut hint_node = Node::text(hint.as_str());
            hint_node.style.text_size = Some(theme.resolve_space("typography.label.size"));
            hint_node.style.descriptor.text_color = Some(text_secondary);
            outer = outer.child(hint_node);
        }
    }
    if let Some(ref error) = spec.error {
        let mut error_label = Node::text(error.as_str());
        error_label.style.text_size = Some(theme.resolve_space("typography.label.size"));
        error_label.style.descriptor.text_color = Some(danger);
        outer = outer.child(error_label);
    }

    // ── Disabled state ──
    if spec.is_disabled {
        outer.style.descriptor.opacity = theme.resolve_opacity("state.opacity.disabled");
        outer.interaction.disabled = true;
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            outer.a11y.label = Some(label.to_string());
        }
    }
    outer
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn typed(spec: &CodeInputSpec, key: &str) -> (Vec<String>, Vec<String>) {
        let changes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let completes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let c = std::sync::Arc::clone(&changes);
        let k = std::sync::Arc::clone(&completes);
        let node = code_input_with_handlers(
            spec,
            &theme(),
            CodeInputHandlers {
                on_value_change: Some(std::sync::Arc::new(move |v: &str| {
                    c.lock().unwrap().push(v.to_string())
                })),
                on_complete: Some(std::sync::Arc::new(move |v: &str| {
                    k.lock().unwrap().push(v.to_string())
                })),
            },
        );
        let row = node
            .find(&|n| n.interaction.on_edit_key.is_some())
            .expect("something takes the keys");
        (row.interaction.on_edit_key.as_ref().unwrap())(key, poodle_node::NodeModifiers::default());
        let changes = changes.lock().unwrap().clone();
        let completes = completes.lock().unwrap().clone();
        (changes, completes)
    }

    /// The row takes focus and the keys; the slots stay visuals. Focusing a
    /// slot would mean six focus stops for one value.
    #[test]
    fn the_slot_row_takes_the_keys_and_no_slot_does() {
        let node = code_input_with_handlers(
            &CodeInputSpec::new().with_length(4),
            &theme(),
            CodeInputHandlers {
                on_value_change: Some(std::sync::Arc::new(|_| {})),
                ..CodeInputHandlers::default()
            },
        );
        fn count_keys(n: &Node) -> usize {
            usize::from(n.interaction.on_edit_key.is_some())
                + n.children.iter().map(count_keys).sum::<usize>()
        }
        assert_eq!(count_keys(&node), 1);
    }

    #[test]
    fn a_digit_appends_and_the_last_one_completes() {
        let spec = CodeInputSpec::new().with_length(4).with_value("12");
        let (changes, completes) = typed(&spec, "3");
        assert_eq!(changes, vec!["123".to_string()]);
        assert!(completes.is_empty(), "three of four is not complete");

        let spec = CodeInputSpec::new().with_length(4).with_value("123");
        let (changes, completes) = typed(&spec, "4");
        assert_eq!(changes, vec!["1234".to_string()]);
        assert_eq!(completes, vec!["1234".to_string()]);
    }

    /// A key that changes nothing reports nothing — a full code must not
    /// re-fire `onComplete` on every further keypress.
    #[test]
    fn keys_that_change_nothing_report_nothing() {
        let spec = CodeInputSpec::new().with_length(4).with_value("1234");
        let (changes, completes) = typed(&spec, "5");
        assert!(changes.is_empty() && completes.is_empty());

        let spec = CodeInputSpec::new().with_length(4);
        let (changes, _) = typed(&spec, "backspace");
        assert!(changes.is_empty(), "backspace on an empty code");
    }

    #[test]
    fn a_disabled_code_input_takes_no_keys() {
        let node = code_input_with_handlers(
            &CodeInputSpec::new().with_length(4).with_disabled(true),
            &theme(),
            CodeInputHandlers {
                on_value_change: Some(std::sync::Arc::new(|_| {})),
                ..CodeInputHandlers::default()
            },
        );
        assert!(node.find(&|n| n.interaction.on_edit_key.is_some()).is_none());
    }

    /// Paste is how a one-time code usually arrives, and it completes too.
    #[test]
    fn pasting_a_code_fills_it_and_completes() {
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&seen);
        let node = code_input_with_handlers(
            &CodeInputSpec::new().with_length(6),
            &theme(),
            CodeInputHandlers {
                on_complete: Some(std::sync::Arc::new(move |v: &str| {
                    sink.lock().unwrap().push(v.to_string())
                })),
                ..CodeInputHandlers::default()
            },
        );
        let row = node
            .find(&|n| n.interaction.on_edit_insert.is_some())
            .expect("paste target");
        (row.interaction.on_edit_insert.as_ref().unwrap())("123-456");
        assert_eq!(*seen.lock().unwrap(), vec!["123456".to_string()]);
    }
}
