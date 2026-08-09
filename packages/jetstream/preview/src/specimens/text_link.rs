//! TextLink specimen — inline text link, backed by `js_text_link` + `TextLinkSpec`.
//!
//! Contract: `docs/contracts/components/text-link.md`
//! Reference: `packages/svelte/preview/src/specimens/TextLinkSpecimen.svelte`
//!
//! Every link resolves through the real `js_text_link` builder and the token
//! system — no hand-styled labels. Covers the representable contract surface:
//! inline-prose usage, all three tones (accent / secondary / inherit), button
//! action (no href), and disabled (anchor + button paths).
//!
//! NOT shown — no spec/runtime channel exists, so a real specimen is impossible:
//!   - leading/trailing icon (external-link): icons are out of scope per the
//!     contract; `TextLinkSpec` has no icon field.
//!   - underline treatment + focus ring: El exposes no text-decoration or
//!     focus-ring affordance (runtime gap, noted in `docs/parity/text-link.md`).
//!   - size variants: `TextLinkSpec` has no size prop; the link inherits the
//!     surrounding text size.

use crate::compat::js_text_link;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{TextLinkSpec, TextLinkTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let primary = resolve_color(theme, "color.text.primary");

    div()
        .flex_col()
        .gap(24.0)
        // ── Inline prose ──
        .child(group(
            "Inline prose",
            secondary,
            div()
                .flex_row()
                .flex_wrap()
                .gap(4.0)
                .items_center()
                .child(label("Review the").text_color(primary).text_size(13.0))
                .child(js_text_link(
                    &TextLinkSpec::new("component contract").with_href("#contract"),
                    theme,
                ))
                .child(
                    label("before wiring the production route.")
                        .text_color(primary)
                        .text_size(13.0),
                ),
        ))
        // ── Tones ──
        .child(group(
            "Tones",
            secondary,
            div()
                .flex_row()
                .flex_wrap()
                .gap(16.0)
                .items_center()
                .child(js_text_link(
                    &TextLinkSpec::new("Accent link").with_href("#accent"),
                    theme,
                ))
                .child(js_text_link(
                    &TextLinkSpec::new("Secondary link")
                        .with_href("#secondary")
                        .with_tone(TextLinkTone::Secondary),
                    theme,
                ))
                .child(js_text_link(
                    &TextLinkSpec::new("Inherited link")
                        .with_href("#inherit")
                        .with_tone(TextLinkTone::Inherit),
                    theme,
                )),
        ))
        // ── Button action (no href) ──
        .child(group(
            "Button action",
            secondary,
            div().child(js_text_link(
                &TextLinkSpec::new("Open inline action"),
                theme,
            )),
        ))
        // ── Disabled (anchor + button paths) ──
        .child(group(
            "Disabled",
            secondary,
            div()
                .flex_row()
                .flex_wrap()
                .gap(16.0)
                .items_center()
                .child(js_text_link(
                    &TextLinkSpec::new("Disabled anchor")
                        .with_href("#disabled")
                        .with_disabled(true),
                    theme,
                ))
                .child(js_text_link(
                    &TextLinkSpec::new("Disabled action").with_disabled(true),
                    theme,
                )),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
