//! TimeZoneSelect — Jetstream timezone select backed by TimeZoneSelectSpec.
//!
//! Contract: `docs/contracts/components/time-zone-select.md`
//! Reference: `packages/svelte/primitives/src/TimeZoneSelect.svelte`
//!
//! Renders a field-chrome shell with a select display and disclosure
//! chevron indicator.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::TimeZoneSelectSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_time_zone_select(spec: &TimeZoneSelectSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, "color.background.surface");
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");
    let icon_muted = resolve_color(theme, "color.icon.muted");
    let focus_ring = resolve_color(theme, "color.accent.focusRing");

    // Focus-within: border becomes focusRing color, shadow at 28% opacity
    let focus_ring_c: Color = focus_ring.into();
    let _focus_shadow = focus_ring_c.with_alpha(0.28);

    // Determine display
    let display_text = spec.trigger_text().unwrap_or("Select time zone");
    let has_value = spec.value.is_some();
    let display_color = if has_value { text_color } else { muted };

    let mut shell = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .h(height)
        .pl(pad_x)
        .pr(pad_x)
        .flex_row()
        .items_center()
        .gap(rem_to_px(0.375))
        .focusable()
        .cursor_pointer();

    // Value display
    shell = shell.child(
        ui_element::label(display_text)
            .text_color(display_color)
            .text_size(font_size)
            .grow(),
    );

    // Indicator chevron (decorative, pointer-events: none in CSS)
    shell = shell.child(
        ui_element::icon("chevron-down")
            .w(rem_to_px(0.75))
            .h(rem_to_px(0.75))
            .text_color(icon_muted),
    );

    // Disabled state
    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        shell = shell.opacity(opacity).disabled(true);
    }

    shell
}
