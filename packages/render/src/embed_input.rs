//! EmbedInput — URL input with parsed embed preview pills.
//!
//! Contract: `docs/contracts/components/embed-input.md`
//! Ported from: `packages/jetstream/components/src/embed_input.rs`.
//!
//! Composes the real `text_input` primitive (multiline, rows=3) and the real
//! `pill` provider chip — no hand-styled fakes.
//!
//! `onValueChange` is the component's: it owns the nested field, so it is the
//! only layer that can see an edit. Debouncing and `onParse` stay the host's —
//! the host does the parsing, and the spec arrives with the result already
//! resolved.

use poodle_adapter::ThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, Node};
use poodle_specs::{EmbedInputSpec, PillSize, PillSpec, PillTone, TextInputSpec};

use crate::pill::pill;
use crate::presentation::rem_to_px;
use crate::text_input::{text_input, text_input_with_change};

/// Host callbacks. `on_value_change` reports the URL as it is typed.
#[derive(Default)]
pub struct EmbedInputHandlers {
    pub on_value_change: Option<poodle_node::TextChangeHandler>,
}

pub fn embed_input(spec: &EmbedInputSpec, theme: &dyn ThemeProvider) -> Node {
    embed_input_with_handlers(spec, theme, EmbedInputHandlers::default())
}

/// Render an embed input whose field reports what is typed into it.
pub fn embed_input_with_handlers(
    spec: &EmbedInputSpec,
    theme: &dyn ThemeProvider,
    handlers: EmbedInputHandlers,
) -> Node {
    // Status colors split per contract: error = text-danger, success = text-success.
    let danger_color = theme.resolve_color("color.status.danger");
    let success_color = theme.resolve_color("color.status.success");
    let status_font = theme.resolve_space("typography.label.size");

    // Contract §7 spacing. Root gap 0.25rem → space.inline.xs; status
    // min-height 1.25rem → space.stack.lg. Status gap 0.375rem has no exact
    // named token — exact rem.
    let root_gap = theme.resolve_space("space.inline.xs");
    let status_min_h = theme.resolve_space("space.stack.lg");
    let status_gap = rem_to_px(0.375);

    let (parsed, error) = spec.resolved_parse_state();

    // Real multiline TextInput primitive (rows=3) — delegates input
    // semantics, sizing, token resolution, and disabled-opacity.
    let placeholder = spec
        .placeholder
        .clone()
        .unwrap_or_else(|| String::from("Paste a URL or embed code..."));
    let field = text_input_with_change(
        &TextInputSpec::new()
            .with_id("embed-input")
            .with_input_type("multiline")
            .with_rows(3)
            .with_value(spec.value.clone())
            .with_placeholder(placeholder)
            .with_disabled(spec.is_disabled)
            // The nested field inherits the composite's axes; without this it
            // always rendered at the default size whatever the host asked for.
            .with_size(poodle_specs::resolve_semantic_control_size(
                spec.size,
                spec.size_role,
            ))
            .with_density(spec.density),
        theme,
        handlers.on_value_change,
    );

    let mut wrapper = Node::container();
    {
        let s = &mut wrapper.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = root_gap;
    }
    let mut wrapper = wrapper.child(field);

    if error.is_some() || parsed.is_some() {
        let mut status_row = Node::container();
        {
            let s = &mut status_row.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = status_gap;
            s.min_height = Some(status_min_h);
        }

        if let Some(ref err) = error {
            // Error span — text-danger.
            let mut msg = Node::text(err);
            msg.style.descriptor.text_color = Some(danger_color);
            msg.style.text_size = Some(status_font);
            status_row = status_row.child(msg);
        } else if let Some(ref parsed) = parsed {
            // Success: real Pill (tone=Success; size Sm) + SuccessText.
            let mut success = Node::text("Embed detected");
            success.style.descriptor.text_color = Some(success_color);
            success.style.text_size = Some(status_font);
            status_row = status_row
                .child(pill(
                    &PillSpec::new()
                        .with_label(parsed.provider.clone())
                        .with_tone(PillTone::Success)
                        .with_size(PillSize::Sm),
                    theme,
                ))
                .child(success);
        }

        wrapper = wrapper.child(status_row);
    }

    wrapper
}
