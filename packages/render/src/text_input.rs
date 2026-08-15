//! TextInput — a text field: affixes, icons, validation, char count.
//!
//! Contract: `docs/contracts/components/text-input.md`
//! Ported from: `packages/gpui/components/src/primitives/text_input.rs`.
//! The node declares the current value and replacement-text callback; the
//! backend owns key dispatch and the eventual native editor/IME integration.

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, Node, NodeRole, StylePatch,
    TextChangeHandler,
};
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, SpinnerSize, SpinnerSpec, SpinnerTone,
    SpinnerVariant, TextInputSpec, ValidationState,
};

use crate::color::with_alpha;
use crate::icon::icon;
use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_height_offset_rem, size_padding_x_offset_rem,
};
use crate::spinner::spinner;

/// Render a text input without an editing callback.
pub fn text_input(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
) -> Node {
    text_input_with_handlers(
        spec,
        theme,
        TextInputHandlers {
            on_clear,
            ..TextInputHandlers::default()
        },
    )
}

const SELECTION_ALPHA: f32 = 0.30;

/// Host callbacks for an editable field.
///
/// `on_change` reports the new value; `on_selection_change` reports where the
/// caret or selection moved to. The host stores both — the caret is spec state,
/// exactly like `TreeSpec::focused_value`, because the Rust targets have no
/// native editor to own it for them.
#[derive(Default)]
pub struct TextInputHandlers {
    pub on_change: Option<TextChangeHandler>,
    pub on_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    /// Fires when the field is activated (clicked or entered). The host stores
    /// it so the next render knows to draw a caret. The backend reports both
    /// directions: it owns focus, so it is the only layer that can see a blur.
    pub on_focus_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_submit: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_cancel: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_clear: Option<Arc<dyn Fn() + Send + Sync>>,
}

/// Render a text input with host-owned replacement-text updates.
pub fn text_input_with_change(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    on_change: Option<TextChangeHandler>,
) -> Node {
    text_input_with_handlers(
        spec,
        theme,
        TextInputHandlers {
            on_change,
            ..TextInputHandlers::default()
        },
    )
}

/// Render a text input that can actually be edited: the caret moves, keys
/// insert where it is, and selections replace.
pub fn text_input_with_handlers(
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
    handlers: TextInputHandlers,
) -> Node {
    let on_change = handlers.on_change.clone();
    let effective_size = resolve_semantic_size(spec.size.unwrap_or_default(), spec.size_role);
    let control_height = theme.resolve_space(spec.control_height_token())
        + rem_to_px(size_height_offset_rem(effective_size));
    let density = spec.density.unwrap_or_default();
    let density_offset_rem = match density {
        ControlDensity::Compact => -0.125,
        ControlDensity::Default => 0.0,
        ControlDensity::Comfortable => 0.125,
    };
    let inline_padding = theme.resolve_space(spec.horizontal_padding_token())
        + rem_to_px(size_padding_x_offset_rem(effective_size) + density_offset_rem);
    let inline_gap = theme.resolve_space(spec.inline_gap_token());
    let radius = theme.resolve_radius(spec.radius_token());
    let body_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.75,
        ControlSize::Sm => 0.8125,
        ControlSize::Md => 0.875,
        ControlSize::Lg => 0.9375,
        ControlSize::Xl => 1.0,
    });
    let body_line_height = theme.resolve_space(spec.body_line_height_token()) / body_size;
    let selection_fill = theme.resolve_color("color.accent.base");

    let border_default = theme.resolve_color(spec.border_token());
    let surface_raw = theme.resolve_color(spec.fill_token());
    let surface_bg = with_alpha(surface_raw, surface_raw.3 * 0.82);
    let border = with_alpha(border_default, border_default.3 * 0.72);
    let hover_border = with_alpha(border_default, border_default.3 * 0.92);
    let effective_border = match spec.validation_state {
        ValidationState::Invalid => theme.resolve_color("color.status.danger"),
        ValidationState::Valid => theme.resolve_color("color.status.success"),
        ValidationState::Pending => theme.resolve_color("color.accent.base"),
        ValidationState::None => border,
    };
    let text_primary = theme.resolve_color(spec.text_color_token());
    let text_secondary = theme.resolve_color(spec.placeholder_color_token());
    let icon_color = theme.resolve_color(spec.icon_color_token());

    let current_value = spec.current_value();
    let display_color = if current_value.is_empty() {
        text_secondary
    } else {
        text_primary
    };

    let mut inner = Node::container();
    {
        let s = &mut inner.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = inline_gap;
        s.fill_width = true;
        s.fill_height = true;
    }

    if let Some(prefix) = &spec.prefix {
        let mut el = affix(prefix, true, inline_gap, spec, theme);
        if let Some(text) = el.children.first_mut() {
            text.id = Some("text-input-prefix".to_owned());
        }
        inner = inner.child(el);
    }

    let leading_name = spec.leading_icon.clone().or_else(|| {
        (spec.input_type == "search").then(|| "search".to_owned())
    });
    if let Some(name) = &leading_name {
        let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
        glyph.id = Some("text-input-leading".to_owned());
        glyph.style.descriptor.text_color = Some(icon_color);
        inner = inner.child(glyph);
    }

    // A STABLE, distinct id per field. Backends key element state by it —
    // gpui stores focus and the editing cursor there — so two unnamed inputs
    // sharing one id share a caret and steal each other's focus. Falling back
    // to the field's own descriptive text keeps unnamed fields apart without
    // inventing per-frame identity, which would break clicks entirely.
    let field_id = match spec.id.as_deref() {
        Some(id) => format!("poodle-input-{id}"),
        None => {
            let descriptor = [
                spec.aria_label.as_deref(),
                spec.placeholder.as_deref(),
                spec.name.as_deref(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("-");
            if descriptor.is_empty() {
                "poodle-input".to_string()
            } else {
                format!("poodle-input-{descriptor}")
            }
        }
    };

    // The value is one input node, not a stack of text runs: the caret and
    // the selection are drawn by the backend at *measured* positions, because
    // mapping a character index to an x offset means shaping glyphs, and no
    // target-independent layer can do that. The component supplies the index
    // and the colours; the backend supplies the pixels, and decides when to
    // draw them at all — it is the only thing that knows what holds focus.
    // A text node, not a nested input: the field root already carries the
    // TextInput role and the accessible name, and an input inside an input is
    // one control announced twice. The caret channel is what makes the backend
    // measure it.
    // The node draws the placeholder when the value is empty, and *says so* via
    // the caret channel. Without that flag the two are indistinguishable one
    // layer down: selection indices counted into the placeholder, and undo
    // recorded it as a value — so undoing an empty field typed the placeholder
    // into the field for real.
    //
    // A flag rather than a separate placeholder node: an absolutely-positioned
    // sibling renders correctly in GPUI and overlaps in Jetstream, and the
    // vocabulary is meant to describe intent, not to encode one backend's
    // layout quirks.
    let showing_placeholder = current_value.is_empty();
    let display: String = if showing_placeholder {
        spec.placeholder.as_deref().unwrap_or("").to_string()
    } else {
        current_value.to_string()
    };
    let mut value = Node::text(display);
    // Derived from the field's id, because the backend caches this node's
    // *measured* line under it to answer "which character did I click?".
    value.id = Some(format!("{field_id}-value"));
    {
        let s = &mut value.style;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
        s.descriptor.text_color = Some(display_color);
        s.text_ellipsis = true;
        s.no_wrap = true;
        s.min_width = Some(0.0);
    }
    let mut value_caret = None;
    let mut value_select: Option<
        Arc<dyn Fn(usize, usize, poodle_node::SelectGranularity) + Send + Sync>,
    > = None;
    if !spec.is_disabled {
        let caret_color = if spec.is_read_only {
            // Read-only fields still select and still show where the selection
            // is; what they do not do is show an insertion point.
            with_alpha(text_primary, 0.0)
        } else {
            text_primary
        };
        value = value.with_caret(
            spec.selection_range(),
            caret_color,
            with_alpha(selection_fill, selection_fill.3 * SELECTION_ALPHA),
        );
        if let Some(caret) = &mut value.caret {
            caret.showing_placeholder = showing_placeholder;
        }
        value_caret = value.caret;
        if let Some(on_selection_change) = handlers.on_selection_change.clone() {
            // The backend names the granularity because it counts the clicks;
            // resolving it needs to know what a word is, which is a text rule
            // and therefore shared, not per-backend.
            let text = spec.current_value().to_string();
            let handler: Arc<dyn Fn(usize, usize, poodle_node::SelectGranularity) + Send + Sync> =
                Arc::new(
                    move |start: usize, end: usize, granularity: poodle_node::SelectGranularity| {
                        let (start, end) = match granularity {
                            poodle_node::SelectGranularity::Character => (start, end),
                            poodle_node::SelectGranularity::Word => {
                                let (a, _) =
                                    poodle_headless::text_input::word_range_at(&text, start);
                                let (_, b) = poodle_headless::text_input::word_range_at(&text, end);
                                (a, b)
                            }
                            poodle_node::SelectGranularity::Line => (0, text.chars().count()),
                        };
                        on_selection_change(start, end);
                    },
                );
            value.interaction.on_select_range = Some(Arc::clone(&handler));
            value_select = Some(handler);
        }
        if !spec.is_read_only {
            let text = spec.current_value().to_string();
            let (start, end) = spec.selection_range();
            let on_selection = handlers.on_selection_change.clone();
            let change = on_change.clone();
            value.interaction.on_edit_insert = Some(Arc::new(move |inserted: &str| {
                let outcome = poodle_headless::text_input::insert_transition(
                    &text,
                    poodle_headless::text_input::EditState {
                        anchor: start,
                        head: end,
                    },
                    inserted,
                );
                if let Some(next) = outcome.value {
                    if let Some(change) = &change {
                        change(&next);
                    }
                }
                if let Some(on_selection) = &on_selection {
                    on_selection(outcome.state.anchor, outcome.state.head);
                }
            }));
        }
    }
    inner = inner.child(value);

    if let Some(name) = &spec.trailing_icon {
        let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
        glyph.id = Some("text-input-trailing".to_owned());
        glyph.style.descriptor.text_color = Some(icon_color);
        inner = inner.child(glyph);
    }

    let can_clear = spec.input_type == "search"
        && spec.show_clear_button
        && !spec.is_disabled
        && !spec.is_read_only
        && !current_value.is_empty();
    if can_clear {
        let mut clear = Node::button("");
        clear.id = Some("text-input-clear".to_owned());
        clear.a11y.role = Some(NodeRole::Button);
        clear.a11y.label = Some("Clear search query".to_owned());
        clear.interaction.focusable = true;
        {
            let s = &mut clear.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(theme.resolve_space("size.icon.sm"));
            s.descriptor.layout.height = LayoutSizing::Fixed(theme.resolve_space("size.icon.sm"));
        }
        let change = on_change.clone();
        let on_clear = handlers.on_clear.clone();
        clear.interaction.on_activate = Some(Arc::new(move || {
            if let Some(change) = &change {
                change("");
            }
            if let Some(on_clear) = &on_clear {
                on_clear();
            }
        }));
        let mut glyph = icon(&IconSpec::new("x").with_size(IconSize::Sm), theme);
        glyph.style.descriptor.text_color = Some(icon_color);
        inner = inner.child(clear.child(glyph));
    }

    if spec.shows_validation_status {
        match spec.validation_state {
            ValidationState::Valid | ValidationState::Invalid => {
                let name = if spec.validation_state == ValidationState::Valid {
                    "check"
                } else {
                    "x"
                };
                let mut glyph = icon(&IconSpec::new(name).with_size(IconSize::Sm), theme);
                glyph.id = Some("text-input-validation".to_owned());
                glyph.style.descriptor.text_color =
                    Some(theme.resolve_color(spec.validation_indicator_color_token()));
                inner = inner.child(glyph);
            }
            ValidationState::Pending => {
                let mut pending = spinner(
                    &SpinnerSpec::new()
                        .with_variant(SpinnerVariant::Ring)
                        .with_size(SpinnerSize::Sm)
                        .with_tone(SpinnerTone::Accent),
                    theme,
                );
                pending.id = Some("text-input-validation".to_owned());
                pending.style.descriptor.text_color =
                    Some(theme.resolve_color(spec.validation_indicator_color_token()));
                inner = inner.child(pending);
            }
            ValidationState::None => {}
        }
    }

    if spec.show_char_count {
        let len = current_value.chars().count();
        let over = spec.max_length.is_some_and(|max| len > max);
        let mut count = Node::text(match spec.max_length {
            Some(max) => format!("{len}/{max}"),
            None => len.to_string(),
        });
        count.id = Some("text-input-char-count".to_owned());
        count.style.descriptor.text_color = Some(theme.resolve_color(if over {
            spec.char_count_over_color_token()
        } else {
            spec.char_count_color_token()
        }));
        count.style.text_size = Some(theme.resolve_space(spec.char_count_font_size_token()));
        count.style.no_wrap = true;
        inner = inner.child(count);
    }

    if let Some(suffix) = &spec.suffix {
        let mut el = affix(suffix, false, inline_gap, spec, theme);
        if let Some(text) = el.children.first_mut() {
            text.id = Some("text-input-suffix".to_owned());
        }
        inner = inner.child(el);
    }

    let mut root = Node::input(current_value, spec.placeholder.as_deref().unwrap_or(""));
    root.id = Some(field_id.clone());
    {
        let s = &mut root.style;
        s.descriptor.background = Some(surface_bg);
        // The native reference uses GPUI's `border_1()` directly. The token
        // can resolve to zero on the compact axis, which removes both the
        // stroke and its one-pixel content inset.
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = effective_border;
        s.descriptor.corner_radii.top_left = radius;
        s.descriptor.corner_radii.top_right = radius;
        s.descriptor.corner_radii.bottom_right = radius;
        s.descriptor.corner_radii.bottom_left = radius;
        s.descriptor.layout.height = LayoutSizing::Fixed(control_height);
        s.descriptor.layout.spacing.padding.left = inline_padding;
        s.descriptor.layout.spacing.padding.right = inline_padding;
        s.descriptor.text_color = Some(text_primary);
        s.text_size = Some(body_size);
        s.line_height = Some(body_line_height);
        s.fill_width = true;
        s.hover = Some(StylePatch {
            background: None,
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
    }
    root.interaction.focusable = true;
    // Contract §States focus-visible: the field rings in the accent focus
    // colour. Without it a clicked field looked exactly like an idle one —
    // The caret says where typing lands; the ring says the field will receive
    // it at all, and is the only signal a keyboard-focused field gets.
    if !spec.is_disabled {
        root.style.focus = Some(StylePatch {
            background: None,
            border_color: Some(theme.resolve_color("color.accent.focusRing")),
            text_color: None,
            opacity: None,
        });
    }
    if !spec.is_disabled && !spec.is_read_only {
        // Keys go through the shared editing model rather than the backend's:
        // the caret lives in this spec, so only the component can say what a
        // keystroke means. Backends just report which key arrived.
        let value = spec.current_value().to_string();
        let (start, end) = spec.selection_range();
        let on_selection = handlers.on_selection_change.clone();
        let change = on_change.clone();
        root.interaction.on_edit_key = Some(Arc::new(move |key, mods| {
            let state = poodle_headless::text_input::EditState {
                anchor: start,
                head: end,
            };
            let Some(outcome) = poodle_headless::text_input::edit_transition(
                &value, state, key, mods.shift, mods.accel,
            ) else {
                return;
            };
            if let Some(next) = outcome.value {
                if let Some(change) = &change {
                    change(&next);
                }
            }
            if let Some(on_selection) = &on_selection {
                on_selection(outcome.state.anchor, outcome.state.head);
            }
        }));
        // Paste and cut arrive as content, not as a keystroke, so they go
        // through the same edit model by a different door.
        {
            let value = spec.current_value().to_string();
            let (start, end) = spec.selection_range();
            let on_selection = handlers.on_selection_change.clone();
            let change = on_change.clone();
            root.interaction.on_edit_insert = Some(Arc::new(move |text: &str| {
                let outcome = poodle_headless::text_input::insert_transition(
                    &value,
                    poodle_headless::text_input::EditState {
                        anchor: start,
                        head: end,
                    },
                    text,
                );
                if let Some(next) = outcome.value {
                    if let Some(change) = &change {
                        change(&next);
                    }
                }
                if let Some(on_selection) = &on_selection {
                    on_selection(outcome.state.anchor, outcome.state.head);
                }
            }));
        }
        // The root carries the caret too. It never *draws* one — it has
        // children, so the backend does not render its intrinsic value — but
        // key events arrive at the focusable root, and copy/cut need to know
        // what is selected without hunting through the subtree for it.
        root.caret = value_caret;
        // The root also reports selection, which the pointer path never uses
        // (it has no measured text of its own) — undo does, because restoring a
        // snapshot moves the caret as well as the value, and undo arrives as a
        // keystroke at the focusable root.
        root.interaction.on_select_range = value_select.clone();
        root.interaction.on_text_change = on_change;
        // Focus is reported by the backend, which is the only thing that knows
        // it. An earlier pass latched this on activation, so it could report a
        // gain and never a loss — a field kept its caret after focus moved on.
        root.interaction.on_focus_change = handlers.on_focus_change.clone();
    }
    if !spec.is_disabled {
        root.interaction.on_submit = handlers.on_submit.clone();
        root.interaction.on_cancel = handlers.on_cancel.clone();
    }
    if spec.is_disabled {
        root.style.descriptor.opacity = theme.resolve_opacity(spec.disabled_opacity_token());
        root.style.descriptor.cursor = CursorHint::NotAllowed;
        root.interaction.disabled = true;
    }
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::TextInput);
    root.roles.insert(
        "size".to_owned(),
        format!("{effective_size:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "density".to_owned(),
        format!("{density:?}").to_ascii_lowercase(),
    );
    root.roles.insert(
        "validation".to_owned(),
        format!("{:?}", spec.validation_state).to_ascii_lowercase(),
    );
    root.roles.insert("type".to_owned(), spec.input_type.clone());
    root.child(inner)
}

fn affix(
    text: &str,
    prefix: bool,
    inline_gap: f32,
    spec: &TextInputSpec,
    theme: &dyn ThemeProvider,
) -> Node {
    let separator_base = theme.resolve_color(spec.affix_separator_color_token());
    let separator = with_alpha(separator_base, separator_base.3 * 0.52);
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.text_color = Some(theme.resolve_color(spec.affix_color_token()));
        s.no_wrap = true;
        if prefix {
            s.descriptor.layout.spacing.padding.right = inline_gap;
            s.descriptor.layout.spacing.margin.right = inline_gap;
            s.border_right_width = Some(1.0);
        } else {
            s.descriptor.layout.spacing.padding.left = inline_gap;
            s.descriptor.layout.spacing.margin.left = inline_gap;
            s.border_left_width = Some(1.0);
        }
        s.descriptor.border.color = separator;
    }
    el.child(Node::text(text))
}

#[cfg(test)]
mod tests {

    /// The value is a single text node carrying its caret, not a stack of
    /// text runs and not a nested input. Runs positioned the caret without
    /// measuring anything, which is why they could not answer a click:
    /// `closest_index_for_x` needs one shaped line, not a boundary between
    /// two. A nested input would announce the field twice.
    #[test]
    fn the_value_is_one_text_node_carrying_the_caret_and_its_colors() {
        let theme = theme();
        let node = text_input_with_handlers(
            &TextInputSpec::new()
                .with_id("stock")
                .with_value("hello")
                .with_selection(1, 4),
            &theme,
            TextInputHandlers::default(),
        );
        let value = node
            .find(&|n| n.id.as_deref() == Some("poodle-input-stock-value"))
            .expect("the value node is keyed off the field id");
        let NodeKind::Text { content } = &value.kind else {
            panic!("the value node is text, so the field root is the only input");
        };
        assert_eq!(content, "hello");
        // Exactly one node in the tree carries the TextInput role.
        fn count_inputs(n: &Node) -> usize {
            usize::from(matches!(n.kind, NodeKind::Input { .. }))
                + n.children.iter().map(count_inputs).sum::<usize>()
        }
        assert_eq!(count_inputs(&node), 1);
        let caret = value.caret.expect("the value node carries the caret");
        assert_eq!(caret.selection, (1, 4));
    }

    /// Pointer selection lands on the value node, because that is the node
    /// whose text gets measured. On the field root it would have to hit-test
    /// against affixes and icons as if they were characters.
    #[test]
    fn pointer_selection_is_reported_from_the_value_node_only() {
        let theme = theme();
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = std::sync::Arc::clone(&seen);
        let node = text_input_with_handlers(
            &TextInputSpec::new().with_id("stock").with_value("hello"),
            &theme,
            TextInputHandlers {
                on_selection_change: Some(Arc::new(move |a, b| sink.lock().unwrap().push((a, b)))),
                ..TextInputHandlers::default()
            },
        );
        let value = node
            .find(&|n| n.id.as_deref() == Some("poodle-input-stock-value"))
            .expect("value node");
        let select = value
            .interaction
            .on_select_range
            .as_ref()
            .expect("the value node reports pointer selection");
        select(1, 3, poodle_node::SelectGranularity::Character);
        assert_eq!(*seen.lock().unwrap(), vec![(1, 3)]);

        // A double click reports where it landed; the word around it is the
        // component's to work out, because only it knows what a word is.
        select(1, 1, poodle_node::SelectGranularity::Word);
        assert_eq!(seen.lock().unwrap().last().copied(), Some((0, 5)));
        select(2, 2, poodle_node::SelectGranularity::Line);
        assert_eq!(seen.lock().unwrap().last().copied(), Some((0, 5)));
        // The root carries the same channel — undo restores a caret and arrives
        // as a keystroke there — but it never resolves a pointer position,
        // because only the value node's text is measured. Same handler, so a
        // report from either reaches the same host.
        assert!(
            node.interaction.on_select_range.is_some(),
            "the root reports selection for undo"
        );
        assert!(
            node.caret.is_some() && value.caret.is_some(),
            "both carry the caret: the root for clipboard, the value node to draw it"
        );
    }

    /// A disabled field draws no caret at all: `with_caret` is what turns the
    /// caret on, so a disabled field must never reach it.
    #[test]
    fn a_disabled_field_carries_no_caret() {
        let theme = theme();
        let node = text_input_with_handlers(
            &TextInputSpec::new()
                .with_id("stock")
                .with_value("hello")
                .with_disabled(true),
            &theme,
            TextInputHandlers::default(),
        );
        let value = node
            .find(&|n| n.id.as_deref() == Some("poodle-input-stock-value"))
            .expect("value node");
        assert_eq!(value.caret, None);
    }

    use super::*;
    use poodle_adapter::ThemeProvider;
    use poodle_node::NodeKind;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn root_is_a_full_width_input_with_inline_count_and_validation() {
        let spec = TextInputSpec::new()
            .with_id("email")
            .with_value("bad@")
            .with_max_length(12)
            .with_show_char_count(true)
            .with_validation_state(ValidationState::Invalid);
        let node = text_input(&spec, &theme(), None);

        assert!(matches!(node.kind, NodeKind::Input { .. }));
        assert_eq!(node.id.as_deref(), Some("poodle-input-email"));
        assert!(node.style.fill_width);
        assert_eq!(node.style.descriptor.border.width, 1.0);
        assert_eq!(node.children.len(), 1, "one inline content row");
        assert_eq!(node.children[0].children.len(), 3, "value + count + status");
        assert_eq!(
            node.style.descriptor.border.color,
            theme().resolve_color("color.status.danger")
        );
    }

    #[test]
    fn editable_callback_lives_on_the_input_but_read_only_suppresses_it() {
        let callback: TextChangeHandler = Arc::new(|_| {});
        let editable =
            text_input_with_change(&TextInputSpec::new(), &theme(), Some(callback.clone()));
        assert!(editable.interaction.on_text_change.is_some());

        let read_only = text_input_with_change(
            &TextInputSpec::new().with_read_only(true),
            &theme(),
            Some(callback),
        );
        assert!(read_only.interaction.on_text_change.is_none());
    }

    #[test]
    fn affixes_keep_separator_inside_the_inline_row() {
        let node = text_input(
            &TextInputSpec::new().with_prefix("$").with_suffix("/mo"),
            &theme(),
            None,
        );
        let children = &node.children[0].children;
        assert_eq!(children[0].style.border_right_width, Some(1.0));
        assert_eq!(children[2].style.border_left_width, Some(1.0));
    }
}
