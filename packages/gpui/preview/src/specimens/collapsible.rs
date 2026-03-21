use gpui::*;
use pug_adapter::ThemeProvider;
use pug_primitives::CollapsibleSpec;
use pug_gpui_components::Collapsible;
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");

    // ── Group: Default (closed) ──────────────────────────────────────
    // Contract: title="Project settings", description="Configure build options and deploy targets."
    // Collapsed by default, showing title + description in trigger, chevron down, content hidden.
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
    // Contract: title="Advanced options", defaultOpen
    // Expanded showing title, chevron rotated 180deg, content visible with 0.5rem gap.
    // Toggle key starts false (not toggled), so invert: open = !toggled
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
    // Contract: title="Locked section", description="Requires admin access.", isDisabled
    // Collapsed with reduced opacity, cursor not-allowed, clicking does not toggle.
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

    div().flex().flex_col().gap(px(16.0))
        .child(section_label("DEFAULT (CLOSED)", text_secondary))
        .child(closed_collapsible)

        .child(section_label("DEFAULT OPEN", text_secondary))
        .child(open_collapsible)

        .child(section_label("DISABLED", text_secondary))
        .child(disabled_collapsible)
}

fn section_label(label: &str, color: pug_tokens::typed::ColorValue) -> Div {
    div()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color_to_hsla(color))
        .child(label.to_string())
        .mb(px(2.0))
}
