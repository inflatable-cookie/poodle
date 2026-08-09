//! ColorPicker specimen — trigger + open surface across modes, swatches,
//! alpha, preview-only, disabled, sizes, and densities.
//!
//! Mirrors the GPUI specimen's full contract coverage. Every open example
//! exercises the real surface build-out (saturation/value gradient pad, hue
//! slider, optional alpha slider, mode toggle `SegmentedControl`, channel
//! `NumberInput`s, and the preset swatch grid) — all from `js_color_picker`,
//! no hand-rolled boxes.

use crate::compat::js_color_picker;
use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use poodle_specs::{ColorInputMode, ColorPickerSpec, ControlDensity, ControlSize};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let swatches = vec![
        "#ef4444".to_string(),
        "#f97316".to_string(),
        "#eab308".to_string(),
        "#22c55e".to_string(),
        "#3b82f6".to_string(),
        "#6366f1".to_string(),
        "#8b5cf6".to_string(),
        "#ec4899".to_string(),
    ];

    let mut root = div().flex_col().gap(24.0).max_w(420.0);

    // ── Basic picker (open surface, hex mode) ──
    root = root.child(group(
        "Basic picker (open)",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new().with_value("#6366f1").with_open(true),
            theme,
        ),
    ));

    // ── With swatches (open + preset grid) ──
    root = root.child(group(
        "With swatches",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new()
                .with_value("#3b82f6")
                .with_open(true)
                .with_swatches(swatches.clone()),
            theme,
        ),
    ));

    // ── With alpha (open + alpha slider + alpha channel) ──
    root = root.child(group(
        "With alpha",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new()
                .with_value("#3b82f6")
                .with_open(true)
                .with_show_alpha(true),
            theme,
        ),
    ));

    // ── Default open, RGB mode (three channel inputs) ──
    root = root.child(group(
        "Default open, RGB mode",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new()
                .with_value("#22c55e")
                .with_open(true)
                .with_default_mode(ColorInputMode::Rgb),
            theme,
        ),
    ));

    // ── Preview only (no inline hex input) ──
    root = root.child(group(
        "Preview only (no input)",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new()
                .with_value("#6366f1")
                .with_open(true)
                .with_show_input(false),
            theme,
        ),
    ));

    // ── Closed trigger (no color / placeholder) ──
    root = root.child(group(
        "No color (closed trigger)",
        secondary,
        js_color_picker(&ColorPickerSpec::new(), theme),
    ));

    // ── Disabled ──
    root = root.child(group(
        "Disabled",
        secondary,
        js_color_picker(
            &ColorPickerSpec::new()
                .with_value("#22c55e")
                .with_disabled(true),
            theme,
        ),
    ));

    // ── Sizes (xs–xl): closed triggers per intrinsic size ──
    root = root.child(label("Sizes").text_color(secondary).text_size(11.0));
    for (name, size) in [
        ("XS", ControlSize::Xs),
        ("SM", ControlSize::Sm),
        ("MD", ControlSize::Md),
        ("LG", ControlSize::Lg),
        ("XL", ControlSize::Xl),
    ] {
        root = root.child(group(
            name,
            secondary,
            js_color_picker(
                &ColorPickerSpec::new().with_value("#6366f1").with_size(size),
                theme,
            ),
        ));
    }

    // ── Densities: closed triggers per density ──
    root = root.child(label("Densities").text_color(secondary).text_size(11.0));
    for (name, density) in [
        ("Compact", ControlDensity::Compact),
        ("Default", ControlDensity::Default),
        ("Comfortable", ControlDensity::Comfortable),
    ] {
        root = root.child(group(
            name,
            secondary,
            js_color_picker(
                &ColorPickerSpec::new()
                    .with_value("#6366f1")
                    .with_density(density),
                theme,
            ),
        ));
    }

    root
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
