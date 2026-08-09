//! Popover specimen — contract §13 groups plus surface-width + disabled.
//!
//! The Jetstream popover component (`js_popover`) renders only the open surface
//! (the panel at its current state); placement anchoring, outside-dismiss, and
//! open/close live in the preview event loop, not the component (accepted
//! render-only delta — see `docs/parity/popover.md`). Each group pairs a real
//! `js_button` trigger with the real `js_popover` open surface rendered inline,
//! so the surface anatomy (header / body / footer regions, separator, width
//! overrides) is exercised through the real component + tokens — no hand-rolled
//! surface boxes, dividers, or raw padding.

use crate::compat::js_button;
use crate::compat::js_popover;
use crate::compat::js_separator;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, PopoverSpec, RuleTone, SeparatorSpec};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));

    // Heading + paragraph body, matching the Svelte popover content.
    let heading = |t: &str| label(t).text_color(text_primary).text_size(body_font);
    let para = |t: &str| label(t).text_color(secondary).text_size(body_font);

    div().flex_col().gap(24.0)
        // Default (bottom-start) — contract §13: secondary trigger, heading + paragraph.
        .child(group("Default (bottom-start)", secondary,
            trigger_and_surface(
                theme,
                "Open popover",
                js_popover(
                    &PopoverSpec::new().with_aria_label("Quick settings").with_open(true),
                    theme,
                    Some(div().flex_col().gap(4.0)
                        .child(heading("Quick settings"))
                        .child(para("Adjust your display preferences or notification settings from this panel."))),
                ),
            )
        ))
        // Top placement — contract §13: secondary trigger, paragraph body.
        .child(group("Top placement", secondary,
            trigger_and_surface(
                theme,
                "Show help",
                js_popover(
                    &PopoverSpec::new()
                        .with_placement(poodle_specs::OverlayPlacement::Top)
                        .with_aria_label("Help tip"),
                    theme,
                    Some(div()
                        .child(para("Popovers can be anchored to any side of their trigger element."))),
                ),
            )
        ))
        // Rich content — header / body / footer regions split by a real separator.
        .child(group("Header / body / footer", secondary,
            trigger_and_surface(
                theme,
                "Settings",
                js_popover(
                    &PopoverSpec::new().with_aria_label("Settings"),
                    theme,
                    Some(div().flex_col().gap(8.0).self_stretch()
                        .child(heading("Settings"))
                        .child(para("Adjust your preferences below."))
                        .child(js_separator(&SeparatorSpec::new().with_tone(RuleTone::Subtle), theme))
                        .child(para("Option A"))
                        .child(para("Option B"))),
                ),
            )
        ))
        // Surface width: fixed — surfaceMinWidth/surfaceMaxWidth pinned to 20rem.
        .child(group("Surface width: fixed (20rem)", secondary,
            trigger_and_surface(
                theme,
                "Fixed 20rem",
                js_popover(
                    &PopoverSpec::new()
                        .with_surface_min_width_rem(20.0)
                        .with_surface_max_width_rem(20.0)
                        .with_aria_label("Fixed-width popover"),
                    theme,
                    Some(div().child(para("Surface min-width and max-width are pinned to 20rem."))),
                ),
            )
        ))
        // Disabled — a disabled trigger blocks opening (no surface).
        .child(group("Disabled", secondary,
            js_button(
                &ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_disabled(true)
                    .with_label("Disabled trigger"),
                theme,
            )
        ))
}

/// Render a real Button trigger above the real open popover surface. Jetstream's
/// popover is render-only, so the open surface is shown inline rather than
/// anchored by the trigger (placement/anchoring = preview-loop concern).
fn trigger_and_surface(theme: &JetstreamThemeProvider, trigger_label: &str, surface: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(js_button(
            &ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label(trigger_label),
            theme,
        ))
        .child(surface)
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
