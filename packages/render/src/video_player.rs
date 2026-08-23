//! VideoPlayer — black surface with transport overlay.
//!
//! Contract: `docs/contracts/components/video-player.md`
//! Ported from: `packages/jetstream/components/src/video_player.rs`.
//!
//! Fixed white-on-black chrome regardless of theme (contract §8); the seek
//! bar is a Progress node relying on the widget's default accent fill.

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow,
    LayoutSizing, MainAxisAlignment, Node, NodeKind,
};
use poodle_specs::VideoPlayerSpec;

use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Format seconds as m:ss (contract `.video-player__time`).
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}

pub fn video_player(spec: &VideoPlayerSpec, ctx: &RenderContext<'_>) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let radius = ctx.theme().resolve_radius("radius.surface");

    // ── Size/density geometry (contract §7/§8 rem ladders via spec helpers) ─
    let btn_size = rem_to_px(VideoPlayerSpec::button_size_rem(effective_size));
    let icon_size = rem_to_px(VideoPlayerSpec::icon_size_rem(effective_size));
    let volume_width = rem_to_px(VideoPlayerSpec::volume_width_rem(effective_size));
    let time_font = rem_to_px(VideoPlayerSpec::time_font_rem(effective_size));
    let big_play_size = rem_to_px(VideoPlayerSpec::big_play_size_rem(effective_size));
    let track_height = rem_to_px(VideoPlayerSpec::track_height_rem());
    let volume_thumb = rem_to_px(VideoPlayerSpec::volume_thumb_rem());
    let bar_gap = rem_to_px(VideoPlayerSpec::bar_gap_rem(density));
    let pill = VideoPlayerSpec::pill_radius_rem();

    // ── Fixed colors (contract §8: white-on-black regardless of theme) ──────
    let black = ColorValue(0.0, 0.0, 0.0, 1.0);
    let white = ColorValue(1.0, 1.0, 1.0, 1.0);
    let white_90 = ColorValue(1.0, 1.0, 1.0, 0.9);
    let white_80 = ColorValue(1.0, 1.0, 1.0, 0.8);
    let white_50 = ColorValue(1.0, 1.0, 1.0, 0.5);
    let white_20 = ColorValue(1.0, 1.0, 1.0, 0.2);
    let overlay = ColorValue(0.0, 0.0, 0.0, 0.7);

    let all_pill = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // ── Transport icon button helper ────────────────────────────────────────
    let transport = |name: &'static str| -> Node {
        // Icon-only: named after what the control does.
        let action = match name {
            "play" => "Play",
            "pause" => "Pause",
            "skip-back" => "Skip back",
            "skip-forward" => "Skip forward",
            "volume-2" => "Mute",
            "volume-x" => "Unmute",
            "maximize" => "Full screen",
            "minimize" => "Exit full screen",
            other => other,
        };
        let mut b = Node::button("");
        b.a11y.label = Some(action.to_string());
        {
            let s = &mut b.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(btn_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(btn_size);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.cursor = CursorHint::Pointer;
        }
        b.interaction.focusable = true;
        let mut glyph = Node::icon(name, icon_size);
        glyph.style.descriptor.text_color = Some(white_90);
        b.child(glyph)
    };

    // ── Root container — black surface, token radius ───────────────────────
    //
    // The chrome has a contract minimum height (220px @ 16px base) and pins its
    // controls to the bottom edge; without both, the viewport collapses to zero
    // and the controls ride up out of the black surface.
    let mut el = Node::container();
    {
        let s = &mut el.style;
        s.descriptor.background = Some(black);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.main = MainAxisAlignment::End;
        s.min_height = Some(rem_to_px(13.75));
        s.fill_width = true;
        s.self_stretch = true;
    }
    all_pill(&mut el, radius);

    // ── Video area (placeholder — no native video) ─────────────────────────
    let mut video_area = Node::container();
    {
        let s = &mut video_area.style;
        s.descriptor.background = Some(black);
        s.descriptor.layout.width = LayoutSizing::Grow;
        // Grow on the column's main axis too, so the viewport takes the slack
        // above the controls rather than shrinking to its content.
        s.descriptor.layout.height = LayoutSizing::Grow;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.direction = LayoutDirection::Column;
    }

    // Big play button — only when paused at currentTime=0.
    let video_area = if !spec.is_playing && spec.current_time <= 0.0 {
        let mut big = Node::button("");
        big.a11y.label = Some("Play".to_string());
        {
            let s = &mut big.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(big_play_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(big_play_size);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.descriptor.cursor = CursorHint::Pointer;
        }
        all_pill(&mut big, pill);
        // The big play affordance reads as an outlined circle; the glyph sits
        // inside it.
        big.style.descriptor.border.width = rem_to_px(0.125);
        big.style.descriptor.border.color = white_90;
        big.interaction.focusable = true;
        let mut glyph = Node::icon("play", big_play_size * 0.5);
        glyph.style.descriptor.text_color = Some(white_90);
        video_area.child(big.child(glyph))
    } else {
        video_area
    };

    let mut el = el.child(video_area);

    // ── Controls overlay ───────────────────────────────────────────────────
    let mut controls = Node::container();
    {
        let s = &mut controls.style;
        s.descriptor.background = Some(overlay);
        s.descriptor.layout.direction = LayoutDirection::Column;
        // Asymmetric inset: the tall top pad is what lifts the controls clear
        // of the video content behind the overlay.
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.top = rem_to_px(1.5);
        pad.left = rem_to_px(0.5);
        pad.right = rem_to_px(0.5);
        pad.bottom = rem_to_px(0.375);
    }

    // Progress / seek bar — Progress node (widget default accent fill).
    let mut progress_bar = Node::container();
    progress_bar.kind = NodeKind::Progress {
        fraction: spec.progress() as f32,
    };
    {
        let s = &mut progress_bar.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(track_height);
        s.self_stretch = true;
        s.descriptor.layout.spacing.margin.bottom = rem_to_px(0.375);
        s.descriptor.background = Some(white_20);
        // `text_color` is the channel the backend reads for a Progress node's
        // filled portion; without it the played region renders white on a white
        // track and the seek position is invisible.
        s.descriptor.text_color = Some(ctx.theme().resolve_color("color.accent.base"));
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
    }
    all_pill(&mut progress_bar, pill);
    let mut controls = controls.child(progress_bar);

    // Control bar.
    let mut bar = Node::container();
    {
        let s = &mut bar.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = bar_gap;
    }

    // Play/pause + mute (icon swaps).
    let mut bar = bar
        .child(transport(if spec.is_playing { "pause" } else { "play" }))
        .child(transport(if spec.volume <= 0.0 {
            "volume-x"
        } else {
            "volume-2"
        }));

    // Volume slider — [filled-track | thumb | rest-track] across fixed width.
    let vol_frac = (spec.volume as f32).clamp(0.0, 1.0);
    let usable = (volume_width - volume_thumb).max(0.0);
    let filled_w = usable * vol_frac;
    let rest_w = usable - filled_w;
    let track_seg = |w: f32| -> Node {
        let mut seg = Node::container();
        {
            let s = &mut seg.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.width = LayoutSizing::Fixed(w);
            s.min_height = Some(track_height);
            s.descriptor.background = Some(white_50);
        }
        all_pill(&mut seg, rem_to_px(0.125));
        seg
    };
    let mut thumb = Node::container();
    {
        let s = &mut thumb.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Fixed(volume_thumb);
        s.descriptor.layout.height = LayoutSizing::Fixed(volume_thumb);
        s.descriptor.background = Some(white);
    }
    all_pill(&mut thumb, pill);
    let mut volume_slider = Node::container();
    {
        let s = &mut volume_slider.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Fixed(volume_width);
    }
    bar = bar.child(
        volume_slider
            .child(track_seg(filled_w))
            .child(thumb)
            .child(track_seg(rest_w)),
    );

    // Time display (m:ss / m:ss) — monospace.
    let mut time = Node::text(format!(
        "{} / {}",
        format_time(spec.current_time),
        format_time(spec.duration)
    ));
    {
        let s = &mut time.style;
        s.descriptor.text_color = Some(white_80);
        s.text_size = Some(time_font);
        s.font_family = Some(FontFamily::Mono);
    }
    bar = bar.child(time);

    // Spacer pushes fullscreen to the right edge.
    let mut spacer = Node::container();
    {
        let s = &mut spacer.style;
        // Explicit Row (see switch.rs).
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.width = LayoutSizing::Grow;
    }
    bar = bar.child(spacer);

    // Fullscreen (icon swaps).
    bar = bar.child(transport(if spec.is_fullscreen {
        "minimize-2"
    } else {
        "maximize-2"
    }));

    controls = controls.child(bar);
    el = el.child(controls);

    if let Some(label) = spec.aria_label.as_deref() {
        if !label.is_empty() {
            el.a11y.label = Some(label.to_string());
        }
    }
    el
}
