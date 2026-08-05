//! Grid specimen — flex-wrap grid layouts (mirrors Svelte GridSpecimen).
//!
//! Covers all three contract §13 groups with real `js_surface` children:
//! Three columns, Mixed column widths, Auto-fit responsive.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_grid;
use crate::compat::js_surface;

use poodle_specs::{GridSpec, PaddingScale, SurfaceBorder, SurfaceSpec, SurfaceTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");
    let text_primary = resolve_color(theme, "color.text.primary");

    // Real Surface child — resolves all chrome from tokens via SurfaceSpec.
    let surface = |text: &str, padding: PaddingScale| {
        let spec = SurfaceSpec::new()
            .with_tone(SurfaceTone::Panel)
            .with_border(SurfaceBorder::Subtle)
            .with_padding(padding);
        js_surface(
            &spec,
            theme,
            vec![label(text).text_color(text_primary).text_size(13.0)],
        )
    };
    let surface_md = |l: &str| surface(l, PaddingScale::Md);
    let surface_sm = |l: &str| surface(l, PaddingScale::Sm);

    div()
        .flex_col()
        .gap(24.0)
        // Three columns — 1fr 1fr 1fr, gap md.
        .child(group(
            "Three columns",
            secondary,
            js_grid(
                &GridSpec::new()
                    .with_columns("1fr 1fr 1fr")
                    .with_gap(PaddingScale::Md),
                theme,
                vec![
                    surface_md("Column 1"),
                    surface_md("Column 2"),
                    surface_md("Column 3"),
                ],
            ),
        ))
        // Mixed column widths — 1fr 2fr, gap md.
        .child(group(
            "Mixed column widths",
            secondary,
            js_grid(
                &GridSpec::new()
                    .with_columns("1fr 2fr")
                    .with_gap(PaddingScale::Md),
                theme,
                vec![surface_md("Sidebar (1fr)"), surface_md("Main content (2fr)")],
            ),
        ))
        // Auto-fit responsive — repeat(auto-fit, minmax(8rem, 1fr)), gap sm.
        .child(group(
            "Auto-fit responsive",
            secondary,
            js_grid(
                &GridSpec::new()
                    .with_columns("repeat(auto-fit, minmax(8rem, 1fr))")
                    .with_gap(PaddingScale::Sm),
                theme,
                vec![
                    surface_sm("A"),
                    surface_sm("B"),
                    surface_sm("C"),
                    surface_sm("D"),
                    surface_sm("E"),
                ],
            ),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
