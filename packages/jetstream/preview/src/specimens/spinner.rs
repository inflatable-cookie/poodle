//! Spinner specimen — ring + grid variants across the full size ladder, all
//! tones, a mixed-tone grid row, and host-context tone chips. Every spinner is
//! the REAL `js_spinner` builder; chips are plain host containers (the same
//! role the Svelte `.poodle-specimen__chip` plays), not fakes of the component.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;

use crate::compat::js_spinner;

use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

const SIZES: [SpinnerSize; 5] = [
    SpinnerSize::Xs,
    SpinnerSize::Sm,
    SpinnerSize::Md,
    SpinnerSize::Lg,
    SpinnerSize::Xl,
];

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let border = resolve_color(theme, "color.border.default");
    let surface = resolve_color(theme, "color.background.surface");

    // ── Ring: full size ladder ──
    let mut ring_row = div().flex_row().gap(16.0).items_center();
    for size in SIZES {
        ring_row = ring_row.child(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_size(size),
            theme,
        ));
    }

    // ── Grid: full size ladder ──
    let mut grid_row = div().flex_row().gap(16.0).items_center();
    for size in SIZES {
        grid_row = grid_row.child(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Grid)
                .with_size(size),
            theme,
        ));
    }

    // ── CLI grid with mixed tones (Svelte: xs=muted, md=accent, rest=current) ──
    let mut mixed_grid = div().flex_row().gap(16.0).items_center();
    for (i, size) in SIZES.iter().enumerate() {
        let tone = match i {
            0 => SpinnerTone::Muted,
            2 => SpinnerTone::Accent,
            _ => SpinnerTone::Current,
        };
        mixed_grid = mixed_grid.child(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Grid)
                .with_size(*size)
                .with_tone(tone),
            theme,
        ));
    }

    // ── Tones (ring) ──
    let tones_row = div().flex_row().gap(16.0).items_center()
        .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Current), theme))
        .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Accent), theme))
        .child(js_spinner(&SpinnerSpec::new().with_tone(SpinnerTone::Muted), theme));

    // ── Context tones: spinners hosted inside bordered surface chips ──
    // Jetstream has no CSS inheritance, so `current` always resolves to
    // text.primary (accepted delta); the chips show current/accent/muted on a
    // real surface background rather than faking a colour inversion.
    let chip = |inner: El| -> El {
        div()
            .min_w(rem_to_px(2.5))
            .min_h(rem_to_px(2.5))
            .p(rem_to_px(0.5))
            .flex_row()
            .items_center()
            .justify_center()
            .border_1()
            .border_color(border)
            .rounded(resolve_radius(theme, "radius.control"))
            .bg(surface)
            .child(inner)
    };
    let context_row = div().flex_row().gap(rem_to_px(0.75)).items_center()
        .child(chip(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_tone(SpinnerTone::Current),
            theme,
        )))
        .child(chip(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Ring)
                .with_tone(SpinnerTone::Accent),
            theme,
        )))
        .child(chip(js_spinner(
            &SpinnerSpec::new()
                .with_variant(SpinnerVariant::Grid)
                .with_tone(SpinnerTone::Muted),
            theme,
        )));

    div().flex_col().gap(24.0)
        .child(group("Ring", secondary, ring_row))
        .child(group("CLI grid", secondary, grid_row))
        .child(group("CLI grid (mixed tones)", secondary, mixed_grid))
        .child(group("Tones", secondary, tones_row))
        .child(group("Context tones", secondary, context_row))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
