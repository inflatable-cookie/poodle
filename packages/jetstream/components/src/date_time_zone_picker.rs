//! DateTimeZonePicker — Jetstream date+time+zone picker trigger backed by DateTimeZonePickerSpec.
//!
//! Contract: `docs/contracts/components/date-time-zone-picker.md`
//! Reference: `packages/svelte/components/src/DateTimeZonePicker.svelte`
//!
//! Renders the trigger button with value/placeholder display and disclosure
//! indicator.  The calendar+time-field+timezone-select surface is overlay-managed
//! at runtime.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::DateTimeZonePickerSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_date_time_zone_picker(
    spec: &DateTimeZonePickerSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    let fill = resolve_color(theme, "color.background.surface");
    let elevated = resolve_color(theme, "color.background.elevated");
    let border_color = resolve_color(theme, spec.border_token());
    let radius = resolve_radius(theme, "radius.control");
    let text_color = resolve_color(theme, "color.text.primary");
    let muted = resolve_color(theme, "color.text.secondary");
    let icon_muted = resolve_color(theme, "color.icon.muted");

    // Hover: 86% surface blended with elevated
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix(elevated_c, 0.14);

    // Determine display text
    let has_value = spec.value.is_some() || spec.time_zone.is_some();
    let display = if has_value {
        let mut parts = Vec::new();
        if let Some(ref v) = spec.value {
            parts.push(v.as_str());
        }
        if let Some(ref tz) = spec.time_zone {
            parts.push(tz.as_str());
        }
        parts.join(" ")
    } else {
        "Select date, time, and zone".to_string()
    };
    let display_color = if has_value { text_color } else { muted };

    let mut trigger = ui_element::div()
        .bg(fill)
        .border(1.0)
        .border_color(border_color)
        .rounded(radius)
        .h(height)
        .pl(pad_x)
        .pr(pad_x)
        .flex_row()
        .items_center()
        .justify_between()
        .gap(rem_to_px(0.75))
        .focusable()
        .cursor_pointer()
        .hover(|s| s.bg(hover_bg));

    // Value / placeholder
    trigger = trigger.child(
        ui_element::label(&display)
            .text_color(display_color)
            .text_size(font_size)
            .grow(),
    );

    // Disclosure indicator
    trigger = trigger.child(
        ui_element::icon("chevron-down")
            .w(icon_size)
            .h(icon_size)
            .text_color(icon_muted),
    );

    // Root wrapper with min-width (18rem per contract)
    let mut root = ui_element::div()
        .min_w(rem_to_px(18.0))
        .child(trigger);

    if spec.is_disabled {
        let opacity = resolve_opacity(theme, "state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}
