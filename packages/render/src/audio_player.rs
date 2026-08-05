//! AudioPlayer — transport row with seek/volume bars.
//!
//! Contract: `docs/contracts/components/audio-player.md`
//! Ported from: `packages/jetstream/components/src/audio_player.rs`.
//!
//! Seek and volume are Progress nodes (true proportional fills); transport
//! and mute are icon circles. Click/drag wiring is host-owned.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, FontFamily, LayoutDirection, LayoutSizing, MainAxisAlignment, Node,
    NodeKind,
};
use poodle_specs::{AudioPlayerSpec, ControlDensity, ControlSize};

use crate::color::with_alpha;
use crate::presentation::{
    control_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};

/// Format seconds as m:ss.
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}

pub fn audio_player(spec: &AudioPlayerSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));

    // Size-driven dimensions from contract.
    let button_size = rem_to_px(match effective_size {
        ControlSize::Xs => 1.5,
        ControlSize::Sm => 1.75,
        ControlSize::Md => 2.0,
        ControlSize::Lg => 2.25,
        ControlSize::Xl => 2.5,
    });
    let icon_size = rem_to_px(match effective_size {
        ControlSize::Xs => 0.875,
        ControlSize::Sm | ControlSize::Md => 1.0,
        ControlSize::Lg => 1.125,
        ControlSize::Xl => 1.25,
    });
    let time_width = rem_to_px(match effective_size {
        ControlSize::Xs => 2.0,
        ControlSize::Sm | ControlSize::Md => 2.5,
        ControlSize::Lg => 2.75,
        ControlSize::Xl => 3.0,
    });
    let volume_width = rem_to_px(match effective_size {
        ControlSize::Xs => 3.0,
        ControlSize::Sm | ControlSize::Md => 4.0,
        ControlSize::Lg => 4.5,
        ControlSize::Xl => 5.0,
    });

    // Density-driven spacing from contract.
    let gap = rem_to_px(match spec.density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    });
    let pad_y = rem_to_px(panel_space_y_rem(spec.density));
    let pad_x = rem_to_px(control_space_x_rem(spec.density));

    let fill = theme.resolve_color(spec.fill_token());
    let border = theme.resolve_color("color.border.default");
    let radius = theme.resolve_radius("radius.surface");
    let text_primary = theme.resolve_color(spec.control_color_token());
    let text_secondary = theme.resolve_color("color.text.secondary");

    let track_height = rem_to_px(0.25);
    let pill = theme.resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);
    let accent = theme.resolve_color("color.accent.base");

    // Transport icon button (sized circle + tinted glyph).
    let icon_btn = |name: &'static str| -> Node {
        let mut b = Node::container();
        {
            let s = &mut b.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(button_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(button_size);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = pill;
            c.top_right = pill;
            c.bottom_right = pill;
            c.bottom_left = pill;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        b.interaction.focusable = true;
        let mut glyph = Node::icon(name, icon_size);
        glyph.style.descriptor.text_color = Some(text_primary);
        b.child(glyph)
    };

    let time_label = |t: f64| -> Node {
        let mut l = Node::text(format_time(t));
        let s = &mut l.style;
        s.descriptor.text_color = Some(text_secondary);
        s.text_size = Some(font_size);
        s.font_family = Some(FontFamily::Mono);
        s.min_width = Some(time_width);
        l
    };

    // Root: flex row container.
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(fill);
        s.descriptor.border.width = border_w;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
    }

    // Play / pause (icon, not text).
    let mut el = el.child(icon_btn(if spec.is_playing { "pause" } else { "play" }));

    // Current time (m:ss) — monospace.
    el = el.child(time_label(spec.current_time));

    // Seek slider — proportional fill via the Progress node.
    let mut seek = Node::container();
    seek.kind = NodeKind::Progress {
        fraction: spec.progress() as f32,
    };
    {
        let s = &mut seek.style;
        s.min_height = Some(track_height);
        s.self_stretch = true;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = pill;
        c.top_right = pill;
        c.bottom_right = pill;
        c.bottom_left = pill;
        s.descriptor.background = Some(text_primary);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(rem_to_px(4.0));
    }
    el = el.child(seek);

    // Total time (m:ss) — monospace.
    el = el.child(time_label(spec.duration));

    // Mute / unmute (icon).
    el = el.child(icon_btn(if spec.is_muted {
        "volume-x"
    } else {
        "volume-2"
    }));

    // Volume slider — proportional fill; base = accent tinted (contract).
    let vol_frac = if spec.is_muted { 0.0 } else { spec.volume };
    let mut volume = Node::container();
    volume.kind = NodeKind::Progress {
        fraction: vol_frac as f32,
    };
    {
        let s = &mut volume.style;
        s.min_height = Some(track_height);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = pill;
        c.top_right = pill;
        c.bottom_right = pill;
        c.bottom_left = pill;
        s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.30));
        s.descriptor.layout.width = LayoutSizing::Fixed(volume_width);
    }
    el = el.child(volume);

    // Speed select (optional) — shows the spec rate (e.g. "1x"/"1.5x").
    if spec.show_speed_control {
        let mut rate = Node::text(spec.rate_label());
        rate.style.descriptor.text_color = Some(text_secondary);
        rate.style.text_size = Some(font_size);
        el = el.child(rate);
    }

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
