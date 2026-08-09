//! Collapsible specimen — collapsible sections in open and closed states.

use crate::compat::js_collapsible;
use crate::compat::{rem_to_px, size_font_rem};
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{CollapsibleSpec, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_muted = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        // Open
        .child(group(
            "Open",
            secondary,
            div().w(300.0).child(js_collapsible(
                &CollapsibleSpec::new()
                    .with_title("Section Title")
                    .with_default_open(true),
                theme,
                Some(
                    label("This is the collapsible content that is visible when open.")
                        .text_color(text_muted)
                        .text_size(body_font),
                ),
            )),
        ))
        // Closed
        .child(group(
            "Closed",
            secondary,
            div().w(300.0).child(js_collapsible(
                &CollapsibleSpec::new()
                    .with_title("Collapsed Section")
                    .with_default_open(false),
                theme,
                Some(
                    label("Hidden content")
                        .text_color(text_muted)
                        .text_size(body_font),
                ),
            )),
        ))
        // Disabled
        .child(group(
            "Disabled",
            secondary,
            div().w(300.0).child(js_collapsible(
                &CollapsibleSpec::new()
                    .with_title("Disabled Section")
                    .with_default_open(false)
                    .with_disabled(true),
                theme,
                Some(
                    label("Cannot toggle")
                        .text_color(text_muted)
                        .text_size(body_font),
                ),
            )),
        ))
        // Highlighted — accent border (+ halo where supported) from highlight tokens.
        .child(group(
            "Highlighted",
            secondary,
            div().w(300.0).child(js_collapsible(
                &CollapsibleSpec::new()
                    .with_title("Focused section")
                    .with_highlighted(true)
                    .with_default_open(true),
                theme,
                Some(
                    label("Highlighted collapsibles draw attention to a matched section.")
                        .text_color(text_muted)
                        .text_size(body_font),
                ),
            )),
        ))
        // Sizes (xs–xl) — intrinsic dimensions resolve from the size token.
        .child(group(
            "Sizes",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(size_variant(theme, ControlSize::Xs, "xs"))
                .child(size_variant(theme, ControlSize::Sm, "sm"))
                .child(size_variant(theme, ControlSize::Md, "md"))
                .child(size_variant(theme, ControlSize::Lg, "lg"))
                .child(size_variant(theme, ControlSize::Xl, "xl")),
        ))
        // Densities — inline spacing only; height unchanged.
        .child(group(
            "Densities",
            secondary,
            div()
                .flex_col()
                .gap(12.0)
                .child(density_variant(theme, ControlDensity::Compact, "compact"))
                .child(density_variant(theme, ControlDensity::Default, "default"))
                .child(density_variant(
                    theme,
                    ControlDensity::Comfortable,
                    "comfortable",
                )),
        ))
}

fn variant_body(theme: &JetstreamThemeProvider) -> El {
    let body_font = rem_to_px(size_font_rem(ControlSize::Md));
    let text_muted = resolve_color(theme, "color.text.secondary");
    label("Resolves from the size/density tokens.")
        .text_color(text_muted)
        .text_size(body_font)
}

fn size_variant(theme: &JetstreamThemeProvider, size: ControlSize, label_text: &str) -> El {
    div().w(300.0).child(js_collapsible(
        &CollapsibleSpec::new()
            .with_title(format!("Collapsible at {label_text}"))
            .with_size(size)
            .with_default_open(true),
        theme,
        Some(variant_body(theme)),
    ))
}

fn density_variant(
    theme: &JetstreamThemeProvider,
    density: ControlDensity,
    label_text: &str,
) -> El {
    div().w(300.0).child(js_collapsible(
        &CollapsibleSpec::new()
            .with_title(format!("Collapsible at {label_text}"))
            .with_density(density)
            .with_default_open(true),
        theme,
        Some(variant_body(theme)),
    ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
