//! ThemeSelect — Jetstream theme picker backed by ThemeSelectSpec.
//!
//! Contract: `docs/contracts/components/theme-select.md`
//! Reference: `packages/svelte/components/src/ThemeSelect.svelte`
//!
//! A trigger showing the current theme's swatch opens a popover grid of theme
//! swatch tiles. Swatch colors are literal per-theme hex (converted sRGB→linear);
//! chrome resolves from tokens. Selection + applying the theme is host/controller
//! work — render-only, build/probe-verified.
use glam::Vec4;
use jetstream_ui::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{ControlSize, ThemeOption, ThemeSelectSpec};

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{hex_to_rgb255, resolve_color, resolve_opacity, resolve_radius};

fn to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Parse a `#rrggbb` swatch color into a linear-space Vec4, or a fallback.
fn hex_vec4(hex: &str, fallback: Vec4) -> Vec4 {
    match hex_to_rgb255(hex) {
        Some(c) => Vec4::new(
            to_linear(c.r as f32 / 255.0),
            to_linear(c.g as f32 / 255.0),
            to_linear(c.b as f32 / 255.0),
            c.a,
        ),
        None => fallback,
    }
}

/// Mini theme preview: canvas fill + surface card + accent dot + text bar.
fn swatch(option: &ThemeOption, theme: &JetstreamThemeProvider, w: f32, h: f32, selected: bool) -> JsEl {
    let fallback = resolve_color(theme, "color.background.surface");
    let color = |hex: &str| hex_vec4(hex, fallback);
    let border = resolve_color(theme, "color.border.subtle");
    let accent = resolve_color(theme, "color.accent.base");

    ui_element::div()
        .relative()
        .w(rem_to_px(w))
        .h(rem_to_px(h))
        .rounded(rem_to_px(0.375))
        .border_1()
        .border_color(if selected { accent } else { border })
        .bg(color(&option.swatch.canvas))
        .child(
            ui_element::div()
                .absolute()
                .left(rem_to_px(w * 0.14))
                .bottom(0.0)
                .w(rem_to_px(w * 0.72))
                .h(rem_to_px(h * 0.52))
                .bg(color(&option.swatch.surface)),
        )
        .child(
            ui_element::div()
                .absolute()
                .top(rem_to_px(h * 0.18))
                .left(rem_to_px(w * 0.16))
                .w(rem_to_px(h * 0.26))
                .h(rem_to_px(h * 0.26))
                .rounded(rem_to_px(h * 0.13))
                .bg(color(&option.swatch.accent)),
        )
        .child(
            ui_element::div()
                .absolute()
                .top(rem_to_px(h * 0.24))
                .right(rem_to_px(w * 0.16))
                .w(rem_to_px(w * 0.34))
                .h(rem_to_px(0.125))
                .rounded(rem_to_px(0.0625))
                .bg(color(&option.swatch.text)),
        )
}

pub fn js_theme_select(spec: &ThemeSelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let trigger_h = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.25,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.25,
    });

    let text_primary = resolve_color(theme, spec.field_text_token());
    let text_secondary = resolve_color(theme, spec.label_color_token());
    let border = resolve_color(theme, spec.field_border_token());
    let surface = resolve_color(theme, spec.field_fill_token());
    let elevated = resolve_color(theme, spec.surface_fill_token());
    let item_border = resolve_color(theme, spec.item_border_token());
    let accent = resolve_color(theme, spec.accent_token());
    let radius = resolve_radius(theme, spec.radius_token());
    let surface_radius = resolve_radius(theme, spec.surface_radius_token());

    // ── Trigger ─────────────────────────────────────────────────────────────
    let mut trigger = ui_element::div()
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.5))
        .min_h(trigger_h)
        .pl(rem_to_px(0.75))
        .pr(rem_to_px(0.75))
        .rounded(radius)
        .border_1()
        .border_color(border)
        .bg(surface);

    if let Some(current) = spec.current_option() {
        trigger = trigger.child(swatch(current, theme, 1.25, 1.25, false));
    }
    if spec.show_label {
        trigger = trigger.child(
            ui_element::label(&spec.trigger_label())
                .text_color(text_primary)
                .text_size(rem_to_px(0.8125)),
        );
    }
    trigger = trigger.child(ui_element::label("▾").text_color(text_secondary));

    let mut root = ui_element::div().flex_col().gap(rem_to_px(0.375)).child(trigger);

    // ── Popover grid (rendered inline when open) ──────────────────────────────
    if spec.is_open {
        let mut grid = ui_element::div()
            .flex_row()
            .flex_wrap()
            .gap(rem_to_px(0.5))
            .max_w(rem_to_px(22.0));
        for option in spec.themes.iter() {
            let selected = spec.is_selected(option);
            grid = grid.child(
                ui_element::div()
                    .flex_col()
                    .items_center()
                    .gap(rem_to_px(0.375))
                    .w(rem_to_px(4.5))
                    .pl(rem_to_px(0.375))
                    .pr(rem_to_px(0.375))
                    .pt(rem_to_px(0.375))
                    .pb(rem_to_px(0.375))
                    .rounded(radius)
                    .border_1()
                    .border_color(if selected { accent } else { Vec4::new(0.0, 0.0, 0.0, 0.0) })
                    .child(swatch(option, theme, 2.75, 2.0, selected))
                    .child(
                        ui_element::label(&option.label)
                            .text_color(text_primary)
                            .text_size(rem_to_px(0.71875)),
                    ),
            );
        }

        root = root.child(
            ui_element::div()
                .rounded(surface_radius)
                .border_1()
                .border_color(item_border)
                .bg(elevated)
                .pl(rem_to_px(0.5))
                .pr(rem_to_px(0.5))
                .pt(rem_to_px(0.5))
                .pb(rem_to_px(0.5))
                .child(grid),
        );
    }

    if spec.is_disabled {
        root = root.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_specs::ThemeSwatch;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn themes() -> Vec<ThemeOption> {
        vec![
            ThemeOption::new("eclipse", "Eclipse", ThemeSwatch::new("#0e1012", "#15181b", "#f0b24d", "#eef2f6", "#333")),
            ThemeOption::new("nord", "Nord", ThemeSwatch::new("#2e3440", "#3b4252", "#88c0d0", "#eceff4", "#4c566a")),
        ]
    }

    #[test]
    fn trigger_shows_current_label() {
        let el = js_theme_select(
            &ThemeSelectSpec::new().with_themes(themes()).with_value("nord"),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 320.0, 80.0);
        assert!(tree.has_text("Nord"), "trigger label missing: {:?}", tree.texts());
    }

    #[test]
    fn open_grid_lists_all_themes() {
        let el = js_theme_select(
            &ThemeSelectSpec::new().with_themes(themes()).with_value("eclipse").with_open(true),
            &theme(),
        );
        let tree = crate::render_probe::probe(&el, 360.0, 240.0);
        assert!(tree.has_text("Eclipse") && tree.has_text("Nord"), "tiles missing: {:?}", tree.texts());
    }
}
