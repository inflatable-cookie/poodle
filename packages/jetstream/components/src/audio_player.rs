//! AudioPlayer — Jetstream audio player backed by AudioPlayerSpec.
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::AudioPlayerSpec;

use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{resolve_color, resolve_radius, tint};

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
        poodle_specs::ControlSize::Xs => 1.5,
        poodle_specs::ControlSize::Sm => 1.75,
        poodle_specs::ControlSize::Md => 2.0,
        poodle_specs::ControlSize::Lg => 2.25,
        poodle_specs::ControlSize::Xl => 2.5,
    });
    let icon_size = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 0.875,
        poodle_specs::ControlSize::Sm => 1.0,
        poodle_specs::ControlSize::Md => 1.0,
        poodle_specs::ControlSize::Lg => 1.125,
        poodle_specs::ControlSize::Xl => 1.25,
    });
    let time_width = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 2.0,
        poodle_specs::ControlSize::Sm => 2.5,
        poodle_specs::ControlSize::Md => 2.5,
        poodle_specs::ControlSize::Lg => 2.75,
        poodle_specs::ControlSize::Xl => 3.0,
    });
    let volume_width = rem_to_px(match effective_size {
        poodle_specs::ControlSize::Xs => 3.0,
        poodle_specs::ControlSize::Sm => 4.0,
        poodle_specs::ControlSize::Md => 4.0,
        poodle_specs::ControlSize::Lg => 4.5,
        poodle_specs::ControlSize::Xl => 5.0,
    });

    // Density-driven spacing from contract
    let gap = rem_to_px(match spec.density {
        poodle_specs::ControlDensity::Compact => 0.375,
        poodle_specs::ControlDensity::Default => 0.5,
        poodle_specs::ControlDensity::Comfortable => 0.625,
    });
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let fill = resolve_color(theme, spec.fill_token());
    let border = resolve_color(theme, "color.border.default");
    let radius = resolve_radius(theme, "radius.surface");
    let text_primary = resolve_color(theme, spec.control_color_token());
    let text_secondary = resolve_color(theme, "color.text.secondary");

    let track_height = rem_to_px(0.25);
    let pill = resolve_radius(theme, "radius.pill");
    let border_w = rem_to_px(0.0625);
    let accent = resolve_color(theme, "color.accent.base");

    // Transport icon button (sized circle + tinted glyph).
    let icon_btn = |name: &'static str| -> JsEl {
        ui_element::div()
            .w(button_size)
            .h(button_size)
            .rounded(pill)
            .flex_row()
            .items_center()
            .justify_center()
            .focusable()
            .child(
                ui_element::icon(name)
                    .size(icon_size)
                    .text_color(text_primary),
            )
    };

    // Root: flex row container
    let mut el = ui_element::div()
        .bg(fill)
        .border(border_w)
        .border_color(border)
        .rounded(radius)
        .flex_row()
        .items_center()
        .gap(gap)
        .pl(pad_x)
        .pr(pad_x)
        .pt(pad_y)
        .pb(pad_y);

    // Play / pause (icon, not text).
    el = el.child(icon_btn(if spec.is_playing { "pause" } else { "play" }));

    // Current time (m:ss) — monospace (contract CurrentTime).
    el = el.child(
        ui_element::label(format_time(spec.current_time))
            .text_color(text_secondary)
            .text_size(font_size)
            .font_family(FontFamily::Mono)
            .min_w(time_width),
    );

    // Seek slider — proportional fill via the runtime ProgressBar widget
    // (a `.w(frac * 100)` child renders a fixed sliver; JsEl has no percent sizing).
    el = el.child(
        ui_element::progress(spec.progress() as f32)
            .min_h(track_height)
            .self_stretch()
            .rounded(pill)
            .bg(text_primary)
            .grow()
            .min_w(rem_to_px(4.0)),
    );

    // Total time (m:ss) — monospace (contract TotalTime).
    el = el.child(
        ui_element::label(format_time(spec.duration))
            .text_color(text_secondary)
            .text_size(font_size)
            .font_family(FontFamily::Mono)
            .min_w(time_width),
    );

    // Mute / unmute (icon).
    el = el.child(icon_btn(if spec.is_muted { "volume-x" } else { "volume-2" }));

    // Volume slider — proportional fill; base = accent-base tinted (contract).
    let vol_frac = if spec.is_muted { 0.0 } else { spec.volume };
    el = el.child(
        ui_element::progress(vol_frac as f32)
            .min_h(track_height)
            .rounded(pill)
            .bg(tint(accent, 0.30))
            .w(volume_width),
    );

    // Speed select (optional) — shows the spec rate (e.g. "1x"/"1.5x").
    if spec.show_speed_control {
        el = el.child(
            ui_element::label(spec.rate_label())
                .text_color(text_secondary)
                .text_size(font_size),
        );
    }

    crate::aria::with_aria_label(el, spec.aria_label.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn renders_icon_transport_not_text() {
        use crate::render_probe::probe;
        let el = js_audio_player(
            &AudioPlayerSpec::new("a.mp3").with_duration(125.0).with_current_time(5.0),
            &theme(),
        );
        let tree = probe(&el, 400.0, 48.0);
        // m:ss times, no "Play"/"Mute" text labels, and a ProgressBar seek/volume.
        assert!(tree.has_text("0:05") && tree.has_text("2:05"), "m:ss times missing: {:?}", tree.texts());
        assert!(!tree.has_text("Play") && !tree.has_text("Mute"), "text-label transport leaked");
        assert!(tree.count_kind("ProgressBar") >= 1, "seek/volume not ProgressBar widgets");
    }

    #[test]
    fn speed_select_shows_rate() {
        use crate::render_probe::probe;
        let el = js_audio_player(
            &AudioPlayerSpec::new("a.mp3").with_show_speed_control(true).with_rate(1.5),
            &theme(),
        );
        let tree = probe(&el, 400.0, 48.0);
        assert!(tree.has_text("1.5x"), "rate label missing: {:?}", tree.texts());
    }
}
