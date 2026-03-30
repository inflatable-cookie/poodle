//! DateTimeRangePicker — Jetstream date+time range picker trigger backed by DateTimeRangePickerSpec.
//!
//! Contract: `docs/contracts/foundation/date-time-range-picker.md`
//! Reference: `packages/svelte/primitives/src/DateTimeRangePicker.svelte`
//!
//! Renders the trigger button with value/placeholder display and disclosure
//! indicator.  The range-calendar+paired-time-fields surface is overlay-managed
//! at runtime.

use jetstream_runtime::game_ui::Color;
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_primitives::DateTimeRangePickerSpec;

use crate::presentation::{
    control_height_rem, control_space_x_rem, rem_to_px, resolve_semantic_size,
    resolve_supporting_visual_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub fn js_date_time_range_picker(
    spec: &DateTimeRangePickerSpec,
    theme: &JetstreamThemeProvider,
) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let height = rem_to_px(control_height_rem(effective_size));
    let font_size = rem_to_px(size_font_rem(effective_size));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));
    let icon_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(effective_size)));

    let fill = resolve_color(theme, "semantic.color.background.surface");
    let elevated = resolve_color(theme, "semantic.color.background.elevated");
    let border_color = resolve_color(theme, "semantic.color.border.default");
    let radius = resolve_radius(theme, "semantic.radius.control");
    let text_color = resolve_color(theme, "semantic.color.text.primary");
    let muted = resolve_color(theme, "semantic.color.text.secondary");
    let icon_muted = resolve_color(theme, "semantic.color.icon.muted");

    // Hover: 86% surface blended with elevated
    let fill_c: Color = fill.into();
    let elevated_c: Color = elevated.into();
    let hover_bg = fill_c.mix(elevated_c, 0.14);

    // Determine display text
    let val = spec.current_value();
    let start_has = val.start.date.is_some() || val.start.time.is_some();
    let end_has = val.end.date.is_some() || val.end.time.is_some();
    let has_value = start_has || end_has;
    let display = if has_value {
        let start_str = format!(
            "{} {}",
            val.start.date.as_deref().unwrap_or("..."),
            val.start.time.as_deref().unwrap_or("")
        );
        let end_str = format!(
            "{} {}",
            val.end.date.as_deref().unwrap_or("..."),
            val.end.time.as_deref().unwrap_or("")
        );
        format!("{} - {}", start_str.trim(), end_str.trim())
    } else {
        spec.placeholder.clone()
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
        let opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        root = root.opacity(opacity).disabled(true);
    }

    root
}
