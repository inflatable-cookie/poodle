//! VideoPlayer — Jetstream video player backed by VideoPlayerSpec.
use jetstream_runtime::ui_element::{self, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_components::VideoPlayerSpec;

use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::resolve_radius;

/// Format seconds as m:ss.
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    let m = total / 60;
    let s = total % 60;
    format!("{m}:{s:02}")
}

pub fn js_video_player(spec: &VideoPlayerSpec, theme: &JetstreamThemeProvider) -> JsEl {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let radius = resolve_radius(theme, "radius.surface");

    // Size-driven button dimensions from contract
    let btn_size = rem_to_px(match effective_size {
        poodle_components::ControlSize::Xs => 1.25,
        poodle_components::ControlSize::Sm => 1.5,
        poodle_components::ControlSize::Md => 1.75,
        poodle_components::ControlSize::Lg => 2.125,
        poodle_components::ControlSize::Xl => 2.25,
    });
    let icon_size = rem_to_px(match effective_size {
        poodle_components::ControlSize::Xs => 0.75,
        poodle_components::ControlSize::Sm => 0.875,
        poodle_components::ControlSize::Md => 0.875,
        poodle_components::ControlSize::Lg => 1.0,
        poodle_components::ControlSize::Xl => 1.125,
    });
    let volume_width = rem_to_px(match effective_size {
        poodle_components::ControlSize::Xs => 2.5,
        poodle_components::ControlSize::Sm => 3.0,
        poodle_components::ControlSize::Md => 3.5,
        poodle_components::ControlSize::Lg => 4.0,
        poodle_components::ControlSize::Xl => 4.5,
    });
    let time_font = rem_to_px(match effective_size {
        poodle_components::ControlSize::Xs => 0.5625,
        poodle_components::ControlSize::Sm => 0.625,
        poodle_components::ControlSize::Md => 0.6875,
        poodle_components::ControlSize::Lg => 0.75,
        poodle_components::ControlSize::Xl => 0.8125,
    });
    let big_play_size = rem_to_px(match effective_size {
        poodle_components::ControlSize::Xs => 3.0,
        poodle_components::ControlSize::Sm => 3.5,
        poodle_components::ControlSize::Md => 4.0,
        poodle_components::ControlSize::Lg => 4.5,
        poodle_components::ControlSize::Xl => 5.0,
    });

    // Density-driven spacing from contract
    let bar_gap = rem_to_px(match spec.density {
        poodle_components::ControlDensity::Compact => 0.25,
        poodle_components::ControlDensity::Default => 0.375,
        poodle_components::ControlDensity::Comfortable => 0.5,
    });

    // Video player uses black background regardless of theme
    let black = glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
    let white_90 = glam::Vec4::new(1.0, 1.0, 1.0, 0.9);
    let white_80 = glam::Vec4::new(1.0, 1.0, 1.0, 0.8);
    let white_20 = glam::Vec4::new(1.0, 1.0, 1.0, 0.2);
    let accent = crate::theme_ext::resolve_color(theme, "color.accent.base");

    let track_height = rem_to_px(0.25);

    // Root container
    let mut el = ui_element::div()
        .bg(black).rounded(radius).overflow_hidden()
        .flex_col().self_stretch();

    // Video area (placeholder since JsEl doesn't render video natively)
    let mut video_area = ui_element::div()
        .bg(black).grow().items_center().justify_center()
        .flex_col();

    // Big play button (when paused at start)
    if !spec.is_playing && spec.current_time <= 0.0 {
        video_area = video_area.child(
            ui_element::button("Play video")
                .w(big_play_size).h(big_play_size)
                .text_color(white_90)
                .rounded(999.0)
                .focusable()
        );
    }

    el = el.child(video_area);

    // Controls overlay
    let mut controls = ui_element::div().flex_col();

    // Progress bar
    let progress = spec.progress();
    let progress_bar = ui_element::div()
        .min_h(track_height).self_stretch().rounded(999.0)
        .bg(white_20)
        .child(
            ui_element::div()
                .min_h(track_height).rounded(999.0)
                .bg(accent)
                .w((progress * 100.0) as f32)
        );
    controls = controls.child(progress_bar);

    // Control bar
    let mut bar = ui_element::div().flex_row().items_center().gap(bar_gap);

    // Play/pause button
    let play_label = if spec.is_playing { "Pause" } else { "Play" };
    bar = bar.child(
        ui_element::button(play_label)
            .w(btn_size).h(btn_size)
            .text_color(white_90).text_size(icon_size)
            .focusable()
    );

    // Mute button
    bar = bar.child(
        ui_element::button("Mute")
            .w(btn_size).h(btn_size)
            .text_color(white_90).text_size(icon_size)
            .focusable()
    );

    // Volume slider
    bar = bar.child(
        ui_element::div()
            .min_h(track_height).rounded(999.0)
            .bg(white_20)
            .w(volume_width)
    );

    // Time display
    let time_text = format!("{} / {}", format_time(spec.current_time), format_time(spec.duration));
    bar = bar.child(
        ui_element::label(&time_text)
            .text_color(white_80).text_size(time_font)
    );

    // Spacer
    bar = bar.child(ui_element::div().grow());

    // Fullscreen button
    let fs_label = if spec.is_fullscreen { "Exit fullscreen" } else { "Fullscreen" };
    bar = bar.child(
        ui_element::button(fs_label)
            .w(btn_size).h(btn_size)
            .text_color(white_90).text_size(icon_size)
            .focusable()
    );

    controls = controls.child(bar);
    el = el.child(controls);

    el
}
