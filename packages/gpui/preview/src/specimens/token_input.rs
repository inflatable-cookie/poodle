//! TokenInput specimen — tokenizing multi-value text entry.
//!
//! Contract: `docs/contracts/components/token-input.md`.
//! Reference: `packages/svelte/preview/src/specimens/TokenInputSpecimen.svelte`.
//!
//! Full contract-state coverage with the REAL `TokenInput` component: each
//! committed token is a real `Pill` chip with its `x` remove affordance and the
//! draft is a real `TextInput`. Examples pane covers default, multiple
//! separators, narrow + long-value wrapping, empty (placeholder only), max
//! length, read-only, and disabled. Sizes/Densities tabs exercise the full size
//! (xs–xl) and density scales. Commit/remove wiring is consumer-owned (the
//! preview event loop); the specimen renders the resting state.

use crate::app_state::AppState;
use crate::node_compat::{Eyebrow, TokenInput};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, EyebrowSpec, TokenInputSpec};

/// A labelled example row (eyebrow caption above the control).
fn example(label: &str, theme: &GpuiThemeProvider, control: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label.to_string()),
            theme,
        ))
        .child(control)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    // ── Default: populated field with placeholder draft. ──
    let mut default_spec = TokenInputSpec::new()
        .with_values(vec!["ifrs".into(), "tax".into(), "audit".into()])
        .with_placeholder("Type a tag...");
    default_spec.id = "blog-tags".into();
    default_spec.name = Some("tags".into());

    // ── Multiple separators: comma + semicolon. ──
    let mut multi_sep = TokenInputSpec::new()
        .with_values(vec!["draft".into(), "review".into()])
        .with_placeholder("draft; review; live");
    multi_sep.separators = vec![",".into(), ";".into()];

    // ── Empty: only the placeholder draft is visible. ──
    let empty = TokenInputSpec::new().with_placeholder("Type a tag...");

    // ── Max length: draft caps individual token length. ──
    let mut max_len = TokenInputSpec::new()
        .with_values(vec!["abbr".into()])
        .with_placeholder("Max 8 chars...");
    max_len.max_length = Some(8);

    // ── Long / narrow: long token wraps inside the field chrome. ──
    let long_tags = TokenInputSpec::new()
        .with_values(vec![
            "legacy-migration".into(),
            "a-very-long-tag-that-should-wrap-inside-the-token-without-breaking-the-field".into(),
        ])
        .with_placeholder("Type a label...");

    // ── Read-only: values visible, remove + editing removed. ──
    let read_only = TokenInputSpec::new()
        .with_values(vec!["premium".into(), "acca".into(), "2027".into()])
        .with_read_only(true);

    // ── Disabled: dimmed, non-editable, remove buttons hidden. ──
    let disabled = TokenInputSpec::new()
        .with_values(vec!["finance".into(), "ethics".into()])
        .with_disabled(true);

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(20.0))
        .max_w(px(560.0))
        .child(example(
            "Default",
            theme,
            TokenInput::from_spec(default_spec, theme),
        ))
        .child(example(
            "Multiple separators (comma + semicolon)",
            theme,
            TokenInput::from_spec(multi_sep, theme),
        ))
        .child(example(
            "Empty (placeholder only)",
            theme,
            TokenInput::from_spec(empty, theme),
        ))
        .child(example(
            "Max length draft (8 chars)",
            theme,
            TokenInput::from_spec(max_len, theme),
        ))
        .child(example(
            "Narrow and long values",
            theme,
            div()
                .max_w(px(288.0))
                .child(TokenInput::from_spec(long_tags, theme)),
        ))
        .child(example(
            "Read only",
            theme,
            TokenInput::from_spec(read_only, theme),
        ))
        .child(example(
            "Disabled",
            theme,
            TokenInput::from_spec(disabled, theme),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "token-input",
        examples,
        |size, theme: &GpuiThemeProvider| {
            TokenInput::from_spec(
                TokenInputSpec::new()
                    .with_values(vec!["alpha".into(), "beta".into()])
                    .with_size(size)
                    .with_placeholder(size_label(size)),
                theme,
            )
            .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            TokenInput::from_spec(
                TokenInputSpec::new()
                    .with_values(vec!["audit".into(), "reporting".into()])
                    .with_density(density)
                    .with_placeholder("Type a tag..."),
                theme,
            )
            .into_any_element()
        },
    )
}

fn size_label(size: ControlSize) -> String {
    match size {
        ControlSize::Xs => "XS",
        ControlSize::Sm => "SM",
        ControlSize::Md => "MD",
        ControlSize::Lg => "LG",
        ControlSize::Xl => "XL",
    }
    .to_string()
}
