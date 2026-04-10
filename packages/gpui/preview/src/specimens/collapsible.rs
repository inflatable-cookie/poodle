use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_primitives::{CollapsibleSpec, ControlDensity, ControlSize, EyebrowSpec};
use poodle_gpui_components::{Collapsible, Eyebrow};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // ── Group: Default (closed) ──────────────────────────────────────
    let closed_spec = CollapsibleSpec::new()
        .with_title("Project settings")
        .with_description("Configure build options and deploy targets.");

    let is_closed_open = state.specimens.is_on("collapsible-closed");
    let closed_spec = if is_closed_open {
        closed_spec.with_open(true)
    } else {
        closed_spec
    };

    let closed_collapsible = Collapsible::from_spec(closed_spec, theme)
        .with_id("specimen-closed")
        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
            this.state.specimens.toggle("collapsible-closed");
            cx.notify();
        }))
        .with_content(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Build target: production".to_string())
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Output directory: dist/".to_string())
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Source maps: enabled".to_string())
                )
        );

    // ── Group: Default open ──────────────────────────────────────────
    let open_toggled = state.specimens.is_on("collapsible-open-toggled");
    let open_spec = CollapsibleSpec::new()
        .with_title("Advanced options")
        .with_open(!open_toggled);

    let open_collapsible = Collapsible::from_spec(open_spec, theme)
        .with_id("specimen-open")
        .on_toggle(cx.listener(|this, _open: &bool, _w, cx| {
            this.state.specimens.toggle("collapsible-open-toggled");
            cx.notify();
        }))
        .with_content(
            div().flex().flex_col().gap(px(4.0))
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Cache TTL: 3600s".to_string())
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Retry count: 3".to_string())
                )
                .child(
                    div().text_xs().text_color(color_to_hsla(text_secondary))
                        .child("Timeout: 30s".to_string())
                )
        );

    // ── Group: Disabled ──────────────────────────────────────────────
    let disabled_spec = CollapsibleSpec::new()
        .with_title("Locked section")
        .with_description("Requires admin access.")
        .with_disabled(true);

    let disabled_collapsible = Collapsible::from_spec(disabled_spec, theme)
        .with_id("specimen-disabled")
        .with_content(
            div().text_xs().text_color(color_to_hsla(text_secondary))
                .child("This content is hidden behind a disabled collapsible.".to_string())
        );

    div().flex().flex_col().gap(px(24.0))
        // --- Default (closed) ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default (closed)"), theme))
                .child(closed_collapsible)
        )
        // --- Default open ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Default open"), theme))
                .child(open_collapsible)
        )
        // --- Disabled ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Disabled"), theme))
                .child(disabled_collapsible)
        )
        // --- Sizes ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Sizes"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Xs size"),
                                theme,
                            ).with_id("size-xs").size(ControlSize::Xs)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Sm size"),
                                theme,
                            ).with_id("size-sm").size(ControlSize::Sm)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Md size"),
                                theme,
                            ).with_id("size-md").size(ControlSize::Md)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Lg size"),
                                theme,
                            ).with_id("size-lg").size(ControlSize::Lg)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Xl size"),
                                theme,
                            ).with_id("size-xl").size(ControlSize::Xl)
                        )
                )
        )
        // --- Densities ---
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(Eyebrow::from_spec(EyebrowSpec::new().with_content("Densities"), theme))
                .child(
                    div().flex().flex_col().gap(px(8.0))
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Compact"),
                                theme,
                            ).with_id("density-compact").with_density(ControlDensity::Compact)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Default"),
                                theme,
                            ).with_id("density-default").with_density(ControlDensity::Default)
                        )
                        .child(
                            Collapsible::from_spec(
                                CollapsibleSpec::new().with_title("Comfortable"),
                                theme,
                            ).with_id("density-comfortable").with_density(ControlDensity::Comfortable)
                        )
                )
        )
}
