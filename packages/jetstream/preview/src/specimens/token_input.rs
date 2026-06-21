//! TokenInput specimen — tokenizing multi-value text entry.
//!
//! Contract: `docs/contracts/components/token-input.md`.
//! Reference: `packages/svelte/preview/src/specimens/TokenInputSpecimen.svelte`.
//!
//! Full contract-state coverage with the REAL `js_token_input` builder: each
//! committed token is a real `js_pill` chip with its `×` remove affordance and
//! the live draft is a real `js_text_input` — nothing hand-rolled. Covers the
//! default populated field, multiple separators, narrow + long-value wrapping,
//! empty (placeholder only), max-length draft, read-only, disabled, plus the
//! size (xs–xl) and density scales. Entry/commit/remove wiring lives in the
//! preview event loop (render-only here, per the parity doc).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::token_input::js_token_input;
use poodle_specs::{ControlDensity, ControlSize, TokenInputSpec};

/// Convenience: a populated token-input bound to an id with a placeholder.
fn field(id: &str, values: &[&str], placeholder: &str) -> TokenInputSpec {
    let mut spec = TokenInputSpec::new()
        .with_values(values.iter().map(|s| s.to_string()).collect())
        .with_placeholder(placeholder);
    spec.id = id.to_string();
    spec
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    // ── Default: committed tokens render as pills before the draft input. ──
    let mut default_spec = field("blog-tags", &["ifrs", "tax", "audit"], "Type a tag…");
    default_spec.name = Some("tags".into());

    // ── Multiple separators: comma + semicolon both commit a draft. ──
    let mut multi_sep = field("workflow-tags", &["draft", "review"], "draft; review; live");
    multi_sep.separators = vec![",".into(), ";".into()];

    // ── Empty: only the placeholder draft is visible (no committed tokens). ──
    let empty = field("empty-tags", &[], "Type a tag…");

    // ── Max length: the live draft caps individual token length. ──
    let mut max_len = field("short-tags", &["abbr"], "Max 8 chars…");
    max_len.max_length = Some(8);

    // ── Long / narrow: long tokens wrap inside the field chrome. ──
    let long_tags = field(
        "long-tags",
        &[
            "legacy-migration",
            "a-very-long-tag-that-should-wrap-inside-the-token-without-breaking-the-field",
        ],
        "Type a label…",
    );

    // ── Read-only: values stay visible, remove affordance + editing removed. ──
    let read_only = TokenInputSpec::new()
        .with_values(vec!["premium".into(), "acca".into(), "2027".into()])
        .with_read_only(true);

    // ── Disabled: dimmed, non-editable, remove buttons hidden. ──
    let disabled = TokenInputSpec::new()
        .with_values(vec!["finance".into(), "ethics".into()])
        .with_disabled(true);

    div()
        .flex_col()
        .gap(24.0)
        .max_w(640.0)
        .child(group(
            "Default",
            secondary,
            js_token_input(&default_spec, theme),
        ))
        .child(group(
            "Multiple separators (comma + semicolon)",
            secondary,
            js_token_input(&multi_sep, theme),
        ))
        .child(group(
            "Empty (placeholder only)",
            secondary,
            js_token_input(&empty, theme),
        ))
        .child(group(
            "Max length draft (8 chars)",
            secondary,
            js_token_input(&max_len, theme),
        ))
        .child(
            // Narrow container so the long token must wrap.
            group(
                "Narrow and long values",
                secondary,
                div()
                    .max_w(288.0)
                    .child(js_token_input(&long_tags, theme)),
            ),
        )
        .child(group(
            "Read only",
            secondary,
            js_token_input(&read_only, theme),
        ))
        .child(group(
            "Disabled",
            secondary,
            js_token_input(&disabled, theme),
        ))
        .child(group("Sizes", secondary, sizes(theme)))
        .child(group("Densities", secondary, densities(theme)))
}

/// Size scale (xs–xl): size drives padding, font, and pill size.
fn sizes(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    [
        (ControlSize::Xs, "xs"),
        (ControlSize::Sm, "sm"),
        (ControlSize::Md, "md"),
        (ControlSize::Lg, "lg"),
        (ControlSize::Xl, "xl"),
    ]
    .into_iter()
    .fold(div().flex_col().gap(12.0), |col, (size, name)| {
        let mut spec = TokenInputSpec::new()
            .with_values(vec!["alpha".into(), "beta".into()])
            .with_size(size)
            .with_placeholder(name.to_uppercase());
        spec.id = format!("size-{name}");
        col.child(
            div()
                .flex_row()
                .gap(12.0)
                .items_center()
                .child(label(name).text_color(secondary).text_size(11.0).min_w(40.0))
                .child(
                    div()
                        .grow()
                        .max_w(360.0)
                        .child(js_token_input(&spec, theme)),
                ),
        )
    })
}

/// Density scale: density drives the inline wrap gap.
fn densities(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");
    [
        (ControlDensity::Compact, "compact"),
        (ControlDensity::Default, "default"),
        (ControlDensity::Comfortable, "comfortable"),
    ]
    .into_iter()
    .fold(div().flex_col().gap(12.0), |col, (density, name)| {
        let mut spec = TokenInputSpec::new()
            .with_values(vec!["audit".into(), "reporting".into()])
            .with_density(density)
            .with_placeholder("Type a tag…");
        spec.id = format!("density-{name}");
        col.child(
            div()
                .flex_row()
                .gap(12.0)
                .items_center()
                .child(label(name).text_color(secondary).text_size(11.0).min_w(80.0))
                .child(
                    div()
                        .grow()
                        .max_w(360.0)
                        .child(js_token_input(&spec, theme)),
                ),
        )
    })
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
