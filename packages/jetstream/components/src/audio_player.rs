//! AudioPlayer — Jetstream audio player backed by AudioPlayerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_composites::AudioPlayerSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius};

/// Format seconds as m:ss.
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    let m = total / 60;
    let s = total % 60;
    format!("{m}:{s:02}")
}

pub fn js_audio_player(spec: &AudioPlayerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));

    // Size-driven dimensions from contract
    let button_size = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 1.5,
        poodle_primitives::ControlSize::Sm => 1.75,
        poodle_primitives::ControlSize::Md => 2.0,
        poodle_primitives::ControlSize::Lg => 2.25,
        poodle_primitives::ControlSize::Xl => 2.5,
    });
    let icon_size = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 0.875,
        poodle_primitives::ControlSize::Sm => 1.0,
        poodle_primitives::ControlSize::Md => 1.0,
        poodle_primitives::ControlSize::Lg => 1.125,
        poodle_primitives::ControlSize::Xl => 1.25,
    });
    let time_width = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 2.0,
        poodle_primitives::ControlSize::Sm => 2.5,
        poodle_primitives::ControlSize::Md => 2.5,
        poodle_primitives::ControlSize::Lg => 2.75,
        poodle_primitives::ControlSize::Xl => 3.0,
    });
    let volume_width = rem_to_px(match effective_size {
        poodle_primitives::ControlSize::Xs => 3.0,
        poodle_primitives::ControlSize::Sm => 4.0,
        poodle_primitives::ControlSize::Md => 4.0,
        poodle_primitives::ControlSize::Lg => 4.5,
        poodle_primitives::ControlSize::Xl => 5.0,
    });

    // Density-driven spacing from contract
    let gap = rem_to_px(match spec.density {
        poodle_primitives::ControlDensity::Compact => 0.375,
        poodle_primitives::ControlDensity::Default => 0.5,
        poodle_primitives::ControlDensity::Comfortable => 0.625,
    });
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, spec.control_color_token());
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let track_height = rem_to_px(0.25);

    // Root: flex row container
    let mut el = ui_element::div()
        .bg(fill)
        .border(1.0).border_color(border)
        .rounded(radius)
        .flex_row().items_center().gap(gap)
        .pl(pad_x).pr(pad_x).pt(pad_y).pb(pad_y);

    // Play button
    let play_label = if spec.is_playing { "Pause" } else { "Play" };
    el = el.child(
        ui_element::button(play_label)
            .w(button_size).h(button_size)
            .text_color(text_primary).text_size(icon_size)
            .rounded(999.0)
            .focusable()
    );

    // Current time
    el = el.child(
        ui_element::label(&format_time(spec.current_time))
            .text_color(text_secondary).text_size(font_size)
            .min_w(time_width)
    );

    // Seek slider (represented as a track bar)
    let seek_progress = spec.progress();
    let accent = resolve_color(theme, "color.accent.base");
    let seek_track = ui_element::div()
        .min_h(track_height).self_stretch().rounded(999.0)
        .bg(text_primary)
        .grow()
        .min_w(rem_to_px(4.0))
        .child(
            ui_element::div()
                .min_h(track_height).rounded(999.0)
                .bg(accent)
                .w((seek_progress * 100.0) as f32)
        );
    el = el.child(seek_track);

    // Total time
    el = el.child(
        ui_element::label(&format_time(spec.duration))
            .text_color(text_secondary).text_size(font_size)
            .min_w(time_width)
    );

    // Mute button
    let mute_label = if spec.is_muted { "Unmute" } else { "Mute" };
    el = el.child(
        ui_element::button(mute_label)
            .w(button_size).h(button_size)
            .text_color(text_primary).text_size(icon_size)
            .rounded(999.0)
            .focusable()
    );

    // Volume slider
    let vol_frac = if spec.is_muted { 0.0 } else { spec.volume };
    let vol_track = ui_element::div()
        .min_h(track_height).rounded(999.0)
        .bg(accent)
        .w(volume_width)
        .child(
            ui_element::div()
                .min_h(track_height).rounded(999.0)
                .bg(accent)
                .w((vol_frac * volume_width as f64) as f32)
        );
    el = el.child(vol_track);

    // Speed control (optional)
    if spec.show_speed_control {
        el = el.child(
            ui_element::label("1x")
                .text_color(text_secondary).text_size(font_size)
        );
    }

    el
}
