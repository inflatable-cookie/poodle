//! InlineRemediation — GPUI specimen rendering the contract anatomy with real,
//! token-resolved primitives.
//!
//! Contract: `docs/contracts/components/inline-remediation.md`
//! Reference: there is NO Svelte component (contract §8 Known Delta: Svelte
//! composes inline recovery from the `Callout` primitive). The contract is the
//! sole authority; the Jetstream `js_inline_remediation` component is the
//! cross-target proof of the same anatomy.
//!
//! There is no GPUI `InlineRemediation` component yet (the contract's open
//! decision is whether to build one or fold into `Callout`). Until that lands,
//! this specimen materializes the contract anatomy directly:
//!   Root <aside> → tone-colored LEFT border → Content [Title?, Message,
//!   referenced-field hint?] → optional Action (real Button primitive).
//! Every color, gap, padding, radius, and border weight resolves from the
//! token system via `theme.resolve_*` + the spec's `border_token()`/`gap_token()`
//! — zero hand-coded visual values (CLAUDE.md "no fakes").

use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Button, Eyebrow};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, InlineRemediationSpec, RemediationAction, StatusTone,
};

/// Render one InlineRemediation from a spec, resolving all visuals from tokens.
fn remediation(
    spec: &InlineRemediationSpec,
    theme: &GpuiThemeProvider,
    cx: &mut Context<PreviewRoot>,
) -> AnyElement {
    // ── Colors (token-resolved) ──
    // Contract §6 Border: tone → color.status.* (from border_token()).
    let border = color_to_hsla(theme.resolve_color(spec.border_token()));
    let text_primary = color_to_hsla(theme.resolve_color("color.text.primary"));
    // Contract §2 Message → text-secondary.
    let text_secondary = color_to_hsla(theme.resolve_color("color.text.secondary"));

    // ── Dimensions (token-resolved) ──
    // Contract §6 Root gap: title-to-message vertical gap = space.stack.sm
    // (from gap_token()).
    let content_gap = px(theme.resolve_space(spec.gap_token()));
    let row_gap = px(theme.resolve_space(spec.gap_token()));
    let pad_x = px(theme.resolve_space("space.panel.x"));
    let pad_y = px(theme.resolve_space("space.panel.y"));
    let accent_w = px(theme.resolve_border_width("border.width.default"));
    let title_size = px(theme.resolve_space("typography.label.size"));
    let message_size = px(theme.resolve_space("typography.body.size"));
    let hint_size = px(theme.resolve_space("typography.label.size"));

    // ── Content column: Title? + Message + referenced-field hint? ──
    let mut content = div()
        .flex()
        .flex_col()
        .gap(content_gap)
        .flex_1()
        .min_w(px(0.0));

    if let Some(ref title) = spec.title {
        content = content.child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(title.clone()),
        );
    }

    content = content.child(
        div()
            .text_size(message_size)
            .text_color(text_secondary)
            .child(spec.message.clone()),
    );

    // Referenced-field hint: ambient context for the aria-describedby host
    // wiring described in contract §5 (the wiring itself lives in the form).
    if spec.reference_count() > 0 {
        let n = spec.reference_count();
        let hint = format!("{} field{} affected", n, if n == 1 { "" } else { "s" });
        content = content.child(
            div()
                .text_size(hint_size)
                .text_color(text_secondary)
                .child(hint),
        );
    }

    // ── Root <aside>: tone-colored LEFT border only (contract §2). No fill
    // tint — contract §6 lists no background-fill token. ──
    let mut root = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(row_gap)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y)
        .border_l(accent_w)
        .border_color(border)
        .child(content);

    // ── Action (contract §2 "delegates to Button primitive") ──
    if let Some(ref action) = spec.action {
        let action_id = action.id.clone();
        root = root.child(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(action.variant)
                    .with_label(action.label.clone())
                    .with_disabled(action.is_disabled),
                theme,
            )
            .with_id(format!("inline-remediation-action-{}", action.id))
            .on_click(cx.listener(move |_this, _e: &ClickEvent, _w, cx| {
                // Action click is owned by the caller's recovery logic; the
                // specimen just acknowledges it by re-rendering.
                let _ = &action_id;
                cx.notify();
            })),
        );
    }

    root.into_any_element()
}

/// An `Eyebrow`-labeled group wrapper.
fn group(label: &str, theme: &GpuiThemeProvider, child: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(child)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // ── Tones (info / warning / danger) ──
        .child(group(
            "Tones",
            theme,
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(remediation(
                    &InlineRemediationSpec::new(
                        "Double-check this value before continuing.",
                    )
                    .with_tone(StatusTone::Info)
                    .with_title("Info"),
                    theme,
                    cx,
                ))
                .child(remediation(
                    &InlineRemediationSpec::new(
                        "This setting may slow down syncing on large workspaces.",
                    )
                    .with_tone(StatusTone::Warning)
                    .with_title("Warning"),
                    theme,
                    cx,
                ))
                .child(remediation(
                    &InlineRemediationSpec::new(
                        "The end date must fall after the start date.",
                    )
                    .with_tone(StatusTone::Danger)
                    .with_title("Danger"),
                    theme,
                    cx,
                )),
        ))
        // ── Actionless (pure messaging) ──
        .child(group(
            "Actionless",
            theme,
            remediation(
                &InlineRemediationSpec::new(
                    "Slugs must be lowercase and contain no spaces.",
                )
                .with_tone(StatusTone::Info),
                theme,
                cx,
            ),
        ))
        // ── With action (delegates to Button primitive) ──
        .child(group(
            "With action",
            theme,
            remediation(
                &InlineRemediationSpec::new("Upload failed — the file may be too large.")
                    .with_tone(StatusTone::Danger)
                    .with_title("Upload failed")
                    .with_action(
                        RemediationAction::new("retry", "Retry")
                            .with_variant(ButtonVariant::Secondary),
                    ),
                theme,
                cx,
            ),
        ))
        // ── Referenced fields (cross-field attribution hint) ──
        .child(group(
            "Referenced fields",
            theme,
            remediation(
                &InlineRemediationSpec::new(
                    "Password and confirmation must match.",
                )
                .with_tone(StatusTone::Warning)
                .with_title("Mismatch")
                .with_referenced_field_ids(vec![
                    "password".to_string(),
                    "confirm-password".to_string(),
                ])
                .with_action(
                    RemediationAction::new("clear", "Clear both")
                        .with_variant(ButtonVariant::Ghost),
                ),
                theme,
                cx,
            ),
        ))
        // ── Disabled action ──
        .child(group(
            "Disabled action",
            theme,
            remediation(
                &InlineRemediationSpec::new("Reconnect to retry — currently offline.")
                    .with_tone(StatusTone::Danger)
                    .with_title("Offline")
                    .with_action(
                        RemediationAction::new("retry", "Retry")
                            .with_variant(ButtonVariant::Secondary)
                            .with_disabled(true),
                    ),
                theme,
                cx,
            ),
        ))
}
