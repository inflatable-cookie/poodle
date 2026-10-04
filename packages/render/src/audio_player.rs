//! AudioPlayer — transport row with seek/volume bars.
//!
//! Contract: `docs/contracts/components/audio-player.md`
//! Ported from: `packages/jetstream/components/src/audio_player.rs`.
//!
//! Seek and volume are Progress nodes (true proportional fills); transport
//! and mute are icon circles. Click/drag wiring is host-owned.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, FocusRing, FontFamily, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeKey, NodeKind, NodeModifiers, NodeRole, ScrubAxis, ScrubPhase, StylePatch, TextAlign,
};
use poodle_specs::{AudioPlayerSpec, ControlDensity, ControlSize};

use crate::color::mix_srgb;
use crate::context::RenderContext;
use crate::presentation::rem_to_px;

/// Host-owned media actions for a native AudioPlayer instance.
///
/// The component contract has no outward events: these handlers are the
/// renderer's internal bridge to a target's audio API and controlled state.
#[derive(Clone, Default)]
pub struct AudioPlayerHandlers {
    instance_id: Option<String>,
    pub on_playing_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_seek: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_muted_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub on_volume_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
    pub on_rate_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
}

impl AudioPlayerHandlers {
    pub fn new(instance_id: impl Into<String>) -> Self {
        let instance_id = instance_id.into();
        assert!(
            !instance_id.trim().is_empty(),
            "AudioPlayerHandlers requires a non-empty lifetime-stable instance_id"
        );
        Self {
            instance_id: Some(instance_id),
            ..Self::default()
        }
    }
}

fn stamp(node: &mut Node, handlers: &AudioPlayerHandlers, part: &str) {
    if let Some(instance_id) = &handlers.instance_id {
        let id = format!("audio-player:{instance_id}:{part}");
        node.id = Some(id.clone());
        node.runtime_id = Some(id);
    }
}

fn snapped_value(raw: f64, max: f64, step: f64) -> f64 {
    if max <= 0.0 {
        return 0.0;
    }
    ((raw.clamp(0.0, max) / step).round() * step).clamp(0.0, max)
}

fn rate_at(rates: &[f64], current: f64) -> usize {
    rates
        .iter()
        .position(|rate| (*rate - current).abs() < f64::EPSILON)
        .unwrap_or(2)
}

/// Format seconds as m:ss.
fn format_time(seconds: f64) -> String {
    let total = seconds.max(0.0) as u64;
    format!("{}:{:02}", total / 60, total % 60)
}

pub fn audio_player(spec: &AudioPlayerSpec, ctx: &RenderContext<'_>) -> Node {
    audio_player_with_handlers(spec, ctx, &AudioPlayerHandlers::default())
}

pub fn audio_player_with_handlers(
    spec: &AudioPlayerSpec,
    ctx: &RenderContext<'_>,
    handlers: &AudioPlayerHandlers,
) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    // Contract §"CurrentTime/TotalTime": the time labels are label-size type,
    // not the control's own size ladder.
    let font_size = ctx.theme().resolve_space("typography.label.size");

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

    // Density-driven spacing (contract §"Density Overrides"). `gap` and `pad-y`
    // share one ladder; `pad-x` has its own and is NOT the generic
    // `control_space_x_rem` — comfortable is 0.875rem here, not 1rem.
    let density = ctx.resolve_density(spec.density);
    let gap = rem_to_px(match density {
        ControlDensity::Compact => 0.375,
        ControlDensity::Default => 0.5,
        ControlDensity::Comfortable => 0.625,
    });
    let pad_y = gap;
    let pad_x = rem_to_px(match density {
        ControlDensity::Compact => 0.5,
        ControlDensity::Default => 0.75,
        ControlDensity::Comfortable => 0.875,
    });

    let fill = ctx.theme().resolve_color(spec.fill_token());
    let border = ctx.theme().resolve_color("color.border.default");
    let radius = ctx.theme().resolve_radius("radius.surface");
    let text_primary = ctx.theme().resolve_color(spec.control_color_token());
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");

    // Contract §"SeekSlider track"/"VolumeSlider track": both tracks are
    // 0.25rem tall with a 0.125rem radius — a contract-exact geometry, not the
    // pill radius the surrounding controls use.
    let track_height = rem_to_px(0.25);
    let track_radius = rem_to_px(0.125);
    let pill = ctx.theme().resolve_radius("radius.pill");
    let border_w = rem_to_px(0.0625);
    let accent = ctx.theme().resolve_color("color.accent.base");
    // Transport hover tint, matching the other controls' accent-into-surface
    // hover treatment.
    let hover_fill = mix_srgb(accent, fill, 0.12);

    // Transport icon button (sized circle + tinted glyph).
    let icon_btn = |name: &'static str,
                    label: &'static str,
                    part: &'static str,
                    action: Option<Arc<dyn Fn() + Send + Sync>>|
     -> Node {
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
        b.interaction.on_activate = action;
        b.a11y.role = Some(NodeRole::Button);
        b.a11y.label = Some(label.to_owned());
        b.a11y.tab_index = Some(0);
        b.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: border_w,
            offset: rem_to_px(0.0625),
        });
        b.style.hover = Some(StylePatch {
            background: Some(hover_fill),
            ..StylePatch::default()
        });
        let mut glyph = Node::icon(name, icon_size);
        glyph.style.descriptor.text_color = Some(text_primary);
        b = b.child(glyph);
        stamp(&mut b, handlers, part);
        b
    };

    let time_label = |t: f64| -> Node {
        let mut l = Node::text(format_time(t));
        let s = &mut l.style;
        s.descriptor.text_color = Some(text_secondary);
        s.text_size = Some(font_size);
        s.font_family = Some(FontFamily::Mono);
        s.min_width = Some(time_width);
        s.text_align = Some(TextAlign::Center);
        l
    };

    // Root: flex row container.
    let mut el = Node::container();
    el.a11y.role = Some(NodeRole::Group);
    el.a11y.label = Some(
        spec.aria_label
            .as_deref()
            .filter(|label| !label.is_empty())
            .unwrap_or("Audio player")
            .to_owned(),
    );
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
    let next_playing = !spec.is_playing;
    let play_action = handlers.on_playing_change.as_ref().map(|on_change| {
        let on_change = Arc::clone(on_change);
        Arc::new(move || on_change(next_playing)) as Arc<dyn Fn() + Send + Sync>
    });
    let mut el = el.child(icon_btn(
        if spec.is_playing { "pause" } else { "play" },
        if spec.is_playing { "Pause" } else { "Play" },
        "play",
        play_action,
    ));

    // Current time (m:ss) — monospace.
    el = el.child(time_label(spec.current_time));

    // Seek slider — proportional fill via the Progress node.
    let mut seek = Node::container();
    seek.kind = NodeKind::Progress {
        fraction: spec.progress() as f32,
    };
    {
        let s = &mut seek.style;
        // Fixed track height, NOT `self_stretch` — stretching made the track
        // fill the whole transport row instead of reading as a 0.25rem rail.
        s.descriptor.layout.height = LayoutSizing::Fixed(track_height);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = track_radius;
        c.top_right = track_radius;
        c.bottom_right = track_radius;
        c.bottom_left = track_radius;
        // Track base per contract; `text_color` is the channel the backend
        // reads for a Progress node's filled portion.
        s.descriptor.background = Some(text_primary);
        s.descriptor.text_color = Some(accent);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(rem_to_px(4.0));
    }
    bind_range(
        &mut seek,
        ctx,
        handlers,
        "seek",
        "Seek",
        spec.current_time,
        spec.duration.max(0.0),
        0.1,
        handlers.on_seek.clone(),
    );
    el = el.child(seek);

    // Total time (m:ss) — monospace.
    el = el.child(time_label(spec.duration));

    // Mute / unmute (icon).
    let next_muted = !spec.is_muted;
    let mute_action = handlers.on_muted_change.as_ref().map(|on_change| {
        let on_change = Arc::clone(on_change);
        Arc::new(move || on_change(next_muted)) as Arc<dyn Fn() + Send + Sync>
    });
    el = el.child(icon_btn(
        if spec.is_muted {
            "volume-x"
        } else {
            "volume-2"
        },
        if spec.is_muted { "Unmute" } else { "Mute" },
        "mute",
        mute_action,
    ));

    // Volume slider — proportional fill; base = accent tinted (contract).
    let vol_frac = if spec.is_muted {
        0.0
    } else {
        spec.volume.clamp(0.0, 1.0)
    };
    let mut volume = Node::container();
    volume.kind = NodeKind::Progress {
        fraction: vol_frac as f32,
    };
    {
        let s = &mut volume.style;
        s.descriptor.layout.height = LayoutSizing::Fixed(track_height);
        let c = &mut s.descriptor.corner_radii;
        c.top_left = track_radius;
        c.top_right = track_radius;
        c.bottom_right = track_radius;
        c.bottom_left = track_radius;
        // The contract's solid accent track assumes a native range thumb marks
        // the value. This tier draws a proportional fill instead, so the base
        // is accent mixed into the surface — a solid accent track under an
        // accent fill would carry no information.
        s.descriptor.background = Some(mix_srgb(accent, fill, 0.30));
        s.descriptor.text_color = Some(accent);
        s.descriptor.layout.width = LayoutSizing::Fixed(volume_width);
    }
    bind_range(
        &mut volume,
        ctx,
        handlers,
        "volume",
        "Volume",
        vol_frac,
        1.0,
        0.01,
        handlers.on_volume_change.clone(),
    );
    el = el.child(volume);

    // Speed selector (optional) — its host owns the actual audio-rate API.
    if spec.show_speed_control {
        let rates = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
        let rate_live = Arc::new(AtomicU64::new(spec.rate.to_bits()));
        let on_rate_change = handlers.on_rate_change.clone();
        let rates_for_activate = rates;
        let live_for_activate = Arc::clone(&rate_live);
        let activate_change = on_rate_change.clone();
        let rate_activate = on_rate_change.as_ref().map(|_| {
            Arc::new(move || {
                let current = f64::from_bits(live_for_activate.load(Ordering::SeqCst));
                let index = (rate_at(&rates_for_activate, current) + 1) % rates_for_activate.len();
                let next = rates_for_activate[index];
                live_for_activate.store(next.to_bits(), Ordering::SeqCst);
                if let Some(handler) = &activate_change {
                    handler(next);
                }
            }) as Arc<dyn Fn() + Send + Sync>
        });
        let rates_for_key = rates;
        let live_for_key = Arc::clone(&rate_live);
        let key_change = on_rate_change.clone();
        let rate_key = handlers.on_rate_change.as_ref().map(|_| {
            Arc::new(move |key: NodeKey, _mods: NodeModifiers| {
                let current = f64::from_bits(live_for_key.load(Ordering::SeqCst));
                let index = rate_at(&rates_for_key, current);
                let next_index = match key {
                    NodeKey::ArrowLeft | NodeKey::ArrowDown => index.saturating_sub(1),
                    NodeKey::ArrowRight | NodeKey::ArrowUp => {
                        (index + 1).min(rates_for_key.len() - 1)
                    }
                    _ => return None,
                };
                let next = rates_for_key[next_index];
                live_for_key.store(next.to_bits(), Ordering::SeqCst);
                if let Some(handler) = &key_change {
                    handler(next);
                }
                None
            }) as Arc<dyn Fn(NodeKey, NodeModifiers) -> Option<String> + Send + Sync>
        });
        let mut rate = Node::text(spec.rate_label());
        rate.a11y.role = Some(NodeRole::ComboBox);
        rate.a11y.label = Some("Playback speed".to_owned());
        rate.a11y.value_text = Some(spec.rate_label());
        rate.a11y.tab_index = Some(0);
        rate.interaction.focusable = true;
        rate.interaction.on_activate = rate_activate;
        rate.interaction.on_key = rate_key;
        rate.style.descriptor.text_color = Some(text_secondary);
        rate.style.text_size = Some(font_size);
        rate.style.descriptor.border.width = border_w;
        rate.style.descriptor.border.color = border;
        rate.style.descriptor.background = Some(fill);
        let rate_radius = ctx.theme().resolve_radius("radius.control");
        rate.style.descriptor.corner_radii.top_left = rate_radius;
        rate.style.descriptor.corner_radii.top_right = rate_radius;
        rate.style.descriptor.corner_radii.bottom_right = rate_radius;
        rate.style.descriptor.corner_radii.bottom_left = rate_radius;
        rate.style.descriptor.layout.height =
            LayoutSizing::Fixed(ctx.theme().resolve_space("size.control.height"));
        rate.style.descriptor.layout.spacing.padding.left = rem_to_px(0.25);
        rate.style.descriptor.layout.spacing.padding.right = rem_to_px(0.25);
        rate.style.descriptor.layout.spacing.padding.top = rem_to_px(0.125);
        rate.style.descriptor.layout.spacing.padding.bottom = rem_to_px(0.125);
        rate.style.focus_ring = Some(FocusRing {
            color: ctx.theme().resolve_color("color.accent.focusRing"),
            width: border_w,
            offset: rem_to_px(0.125),
        });
        stamp(&mut rate, handlers, "speed");
        el = el.child(rate);
    }

    stamp(&mut el, handlers, "root");
    el
}

fn bind_range(
    node: &mut Node,
    ctx: &RenderContext<'_>,
    handlers: &AudioPlayerHandlers,
    part: &str,
    label: &str,
    value: f64,
    max: f64,
    step: f64,
    on_change: Option<Arc<dyn Fn(f64) + Send + Sync>>,
) {
    let max = max.max(0.0);
    node.a11y.role = Some(NodeRole::Slider);
    node.a11y.label = Some(label.to_owned());
    node.a11y.value = Some(value);
    node.a11y.value_min = Some(0.0);
    node.a11y.value_max = Some(max);
    node.a11y.orientation = Some("horizontal".to_owned());
    node.a11y.tab_index = Some(0);
    node.interaction.focusable = true;
    node.style.focus_ring = Some(FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: rem_to_px(0.0625),
        offset: rem_to_px(0.125),
    });

    if let Some(on_change) = on_change {
        let live = Arc::new(AtomicU64::new(value.to_bits()));
        let scrub_live = Arc::clone(&live);
        let scrub_change = Arc::clone(&on_change);
        node.interaction.scrub_axis = ScrubAxis::Horizontal;
        node.interaction.on_scrub = Some(Arc::new(move |fraction, phase| {
            if matches!(
                phase,
                ScrubPhase::Press | ScrubPhase::Drag | ScrubPhase::Release
            ) {
                let next = snapped_value(fraction as f64 * max, max, step);
                scrub_live.store(next.to_bits(), Ordering::SeqCst);
                scrub_change(next);
            }
        }));

        let key_live = Arc::clone(&live);
        node.interaction.on_key = Some(Arc::new(move |key, _mods| {
            let current = f64::from_bits(key_live.load(Ordering::SeqCst));
            let next = match key {
                NodeKey::ArrowLeft | NodeKey::ArrowDown => (current - step).max(0.0),
                NodeKey::ArrowRight | NodeKey::ArrowUp => (current + step).min(max),
                NodeKey::Home => 0.0,
                NodeKey::End => max,
                _ => return None,
            };
            key_live.store(next.to_bits(), Ordering::SeqCst);
            on_change(next);
            None
        }));
    }
    stamp(node, handlers, part);
}
