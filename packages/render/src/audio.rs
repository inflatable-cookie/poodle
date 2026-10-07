//! Audio controls rendered exclusively from serializable visual state.
//!
//! Contract: `docs/architecture/008-audio-control-family.md` and the twelve
//! component contracts under `docs/contracts/components/`.

use poodle_adapter::ThemeProvider as _;
use poodle_headless::audio::{
    format_value, AudioSwitchMode, AudioValueFormat, KeyboardKeyLayout, KeyboardOrientation,
};
use poodle_node::{
    ColorValue, CrossAxisAlignment, FocusRing, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeKind, NodePosition, NodeRole, NodeToggled,
};
use poodle_specs::{
    AudioMeterSpec, AudioMeterStyle, AudioSwitchSpec, ControlDensity, ControlSize,
    DragNumberFieldSpec, EnvelopeEditorSpec, FaderSpec, GainReductionMeterSpec, KeyboardSpec,
    KnobSpec, ModMatrixGridSpec, Orientation, ValueReadoutSpec, WaveformDisplaySpec, XYPadSpec,
};

use crate::color::{mix_srgb, with_alpha, WHITE};
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, size_font_rem};

fn audio_size_rem(size: ControlSize, values: [f32; 5]) -> f32 {
    values[match size {
        ControlSize::Xs => 0,
        ControlSize::Sm => 1,
        ControlSize::Md => 2,
        ControlSize::Lg => 3,
        ControlSize::Xl => 4,
    }]
}

pub(crate) fn keyboard_dimensions(
    size: ControlSize,
    orientation: KeyboardOrientation,
) -> (f32, f32) {
    let long = rem_to_px(audio_size_rem(size, [14.0, 18.0, 22.0, 26.0, 30.0]));
    let short = rem_to_px(audio_size_rem(size, [4.0, 5.5, 7.0, 8.5, 10.0]));
    match orientation {
        KeyboardOrientation::Horizontal => (long, short),
        KeyboardOrientation::Vertical => (short, long),
    }
}

fn density_metric(density: ControlDensity, values: [f32; 3]) -> f32 {
    values[match density {
        ControlDensity::Compact => 0,
        ControlDensity::Default => 1,
        ControlDensity::Comfortable => 2,
    }]
}

fn circle(node: &mut Node, size: f32) {
    node.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    node.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    let radii = &mut node.style.descriptor.corner_radii;
    radii.top_left = size / 2.0;
    radii.top_right = size / 2.0;
    radii.bottom_right = size / 2.0;
    radii.bottom_left = size / 2.0;
}

fn absolute(node: &mut Node, left: f32, top: f32) {
    node.position = NodePosition::Absolute {
        top: Some(top),
        left: Some(left),
        right: None,
        bottom: None,
    };
}

fn a11y_value(node: &mut Node, role: NodeRole, label: &str, value_text: &str) {
    node.a11y.role = Some(role);
    node.a11y.label = Some(format!("{label}: {value_text}"));
}

fn meter_semantics(node: &mut Node, label: &str, value: f64, min: f64, max: f64, value_text: &str) {
    node.a11y.role = Some(NodeRole::Meter);
    if !label.is_empty() {
        node.a11y.label = Some(label.to_owned());
    }
    node.a11y.value = Some(value);
    node.a11y.value_min = Some(min);
    node.a11y.value_max = Some(max);
    node.a11y.value_text = Some(value_text.to_owned());
}

fn meter_now_db(spec: &AudioMeterSpec) -> f64 {
    let Some(channel) = spec.channels.first() else {
        return spec.min_db;
    };
    spec.min_db + channel.ballistic_value.clamp(0.0, 1.0) * (spec.max_db - spec.min_db)
}

pub fn knob(spec: &KnobSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let size = rem_to_px(audio_size_rem(effective_size, [2.0, 2.5, 3.0, 3.5, 4.0]));
    let center = size / 2.0;
    let accent = ctx.theme().resolve_color("color.accent.base");
    let muted = ctx.theme().resolve_color("color.border.default");
    let surface = ctx.theme().resolve_color("color.background.elevated");
    let hairline = ctx.theme().resolve_border_width("border.width.default");
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    let mut root = Node::container();
    root.id = Some("knob-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.style.descriptor.opacity = if state.enabled { 1.0 } else { disabled_opacity };
    a11y_value(
        &mut root,
        NodeRole::Slider,
        &spec.aria_label,
        &spec.value_text,
    );

    let segment_count = match density {
        ControlDensity::Compact => 20,
        ControlDensity::Default => 28,
        ControlDensity::Comfortable => 36,
    };
    for index in 0..segment_count {
        let fraction = index as f64 / (segment_count - 1) as f64;
        let angle = (-135.0 + fraction * 270.0).to_radians();
        let mut segment = Node::container();
        circle(&mut segment, 3.0);
        segment.style.descriptor.background = Some(if fraction <= state.value_norm {
            accent
        } else {
            muted
        });
        absolute(
            &mut segment,
            center + angle.cos() as f32 * (center - 3.0),
            center + angle.sin() as f32 * (center - 3.0),
        );
        root = root.child(segment);
    }

    let mut cap = Node::container();
    circle(&mut cap, size * 0.68);
    cap.style.descriptor.background = Some(surface);
    cap.style.descriptor.border.width = hairline;
    cap.style.descriptor.border.color = muted;
    absolute(&mut cap, size * 0.16, size * 0.16);
    root = root.child(cap);

    let angle = (-135.0 + state.value_norm.clamp(0.0, 1.0) * 270.0).to_radians();
    let mut indicator = Node::container();
    circle(&mut indicator, 5.0);
    indicator.style.descriptor.background = Some(accent);
    absolute(
        &mut indicator,
        center + angle.cos() as f32 * size * 0.22 - 2.5,
        center + angle.sin() as f32 * size * 0.22 - 2.5,
    );
    root.child(indicator)
}

pub fn fader(spec: &FaderSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let vertical = spec.orientation == Orientation::Vertical;
    let cross = rem_to_px(audio_size_rem(effective_size, [1.5, 1.75, 2.0, 2.25, 2.5]));
    let length = rem_to_px(audio_size_rem(effective_size, [7.0, 8.5, 10.0, 11.5, 13.0]));
    let (width, height) = if vertical {
        (cross, length)
    } else {
        (length, cross)
    };
    let track = ctx.theme().resolve_color("color.border.default");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let thumb_color = ctx.theme().resolve_color("color.background.elevated");
    let hairline = ctx.theme().resolve_border_width("border.width.default");
    let disabled_opacity = ctx.theme().resolve_opacity("state.opacity.disabled");
    let mut root = Node::container();
    root.id = Some("fader-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.style.descriptor.opacity = if state.enabled { 1.0 } else { disabled_opacity };
    a11y_value(
        &mut root,
        NodeRole::Slider,
        &spec.aria_label,
        &spec.value_text,
    );

    let mut rail = Node::container();
    let rail_cross = density_metric(density, [4.0, 6.0, 8.0]);
    let rail_w = if vertical { rail_cross } else { width };
    let rail_h = if vertical { height } else { rail_cross };
    rail.style.descriptor.layout.width = LayoutSizing::Fixed(rail_w);
    rail.style.descriptor.layout.height = LayoutSizing::Fixed(rail_h);
    rail.style.descriptor.background = Some(track);
    let r = rail_cross / 2.0;
    rail.style.descriptor.corner_radii.top_left = r;
    rail.style.descriptor.corner_radii.top_right = r;
    rail.style.descriptor.corner_radii.bottom_left = r;
    rail.style.descriptor.corner_radii.bottom_right = r;
    absolute(
        &mut rail,
        if vertical {
            (width - rail_w) / 2.0
        } else {
            0.0
        },
        if vertical {
            0.0
        } else {
            (height - rail_h) / 2.0
        },
    );
    root = root.child(rail);

    for detent in &spec.detents {
        let norm = poodle_headless::audio::normalize_value(*detent, spec.min, spec.max, spec.law)
            .clamp(0.0, 1.0) as f32;
        let mut mark = Node::container();
        mark.style.descriptor.background = Some(accent);
        mark.style.descriptor.layout.width =
            LayoutSizing::Fixed(if vertical { width } else { 1.0 });
        mark.style.descriptor.layout.height =
            LayoutSizing::Fixed(if vertical { 1.0 } else { height });
        absolute(
            &mut mark,
            if vertical { 0.0 } else { norm * width },
            if vertical { (1.0 - norm) * height } else { 0.0 },
        );
        root = root.child(mark);
    }

    let thumb_w = if vertical { width } else { 12.0 };
    let thumb_h = if vertical { 12.0 } else { height };
    let mut thumb = Node::container();
    thumb.style.descriptor.layout.width = LayoutSizing::Fixed(thumb_w);
    thumb.style.descriptor.layout.height = LayoutSizing::Fixed(thumb_h);
    thumb.style.descriptor.background = Some(thumb_color);
    thumb.style.descriptor.border.width = hairline;
    thumb.style.descriptor.border.color = track;
    absolute(
        &mut thumb,
        if vertical {
            0.0
        } else {
            state.value_norm as f32 * (width - thumb_w)
        },
        if vertical {
            (1.0 - state.value_norm as f32) * (height - thumb_h)
        } else {
            0.0
        },
    );
    root.child(thumb)
}

#[expect(
    clippy::too_many_arguments,
    reason = "meter channel rendering keeps resolved state and token metrics explicit"
)]
fn meter_channel(
    value: f64,
    segments: usize,
    style: AudioMeterStyle,
    vertical: bool,
    size: ControlSize,
    density: ControlDensity,
    gain_reduction: bool,
    ctx: &RenderContext<'_>,
) -> Node {
    let accent = ctx.theme().resolve_color("color.status.success");
    let warning = ctx.theme().resolve_color("color.status.warning");
    let danger = ctx.theme().resolve_color("color.status.danger");
    let idle = ctx.theme().resolve_color("color.border.subtle");
    let length = rem_to_px(audio_size_rem(
        size,
        if gain_reduction {
            [5.5, 6.75, 8.0, 9.5, 11.0]
        } else {
            [7.0, 8.5, 10.0, 11.5, 13.0]
        },
    ));
    let cross = rem_to_px(audio_size_rem(size, [0.5, 0.625, 0.75, 0.875, 1.0]));
    let gap = density_metric(density, [1.0, 2.0, 3.0]);
    if style == AudioMeterStyle::Bar {
        let mut node = Node {
            kind: NodeKind::Progress {
                fraction: value.clamp(0.0, 1.0) as f32,
            },
            ..Node::default()
        };
        node.style.descriptor.layout.width =
            LayoutSizing::Fixed(if vertical { cross } else { length });
        node.style.descriptor.layout.height =
            LayoutSizing::Fixed(if vertical { length } else { cross });
        node.style.descriptor.background = Some(idle);
        node.style.descriptor.text_color = Some(accent);
        return node;
    }
    let count = segments.max(4);
    let mut root = Node::container();
    root.style.descriptor.layout.direction = if vertical {
        LayoutDirection::Column
    } else {
        LayoutDirection::Row
    };
    root.style.descriptor.layout.spacing.gap = gap;
    if vertical {
        root.children.reserve(count);
    }
    for display_index in 0..count {
        let index = if vertical {
            count - display_index - 1
        } else {
            display_index
        };
        let fraction = (index + 1) as f64 / count as f64;
        let mut segment = Node::container();
        let along = ((length - gap * (count.saturating_sub(1)) as f32) / count as f32).max(1.0);
        segment.style.descriptor.layout.width =
            LayoutSizing::Fixed(if vertical { cross } else { along });
        segment.style.descriptor.layout.height =
            LayoutSizing::Fixed(if vertical { along } else { cross });
        segment.style.descriptor.background = Some(if fraction <= value {
            if fraction > 0.9 {
                danger
            } else if fraction > 0.72 {
                warning
            } else {
                accent
            }
        } else {
            idle
        });
        root = root.child(segment);
    }
    root
}

pub fn audio_meter(spec: &AudioMeterSpec, ctx: &RenderContext<'_>) -> Node {
    let vertical = spec.orientation == Orientation::Vertical;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let mut root = Node::container();
    root.id = Some("audio-meter-root".into());
    root.style.descriptor.layout.direction = if vertical {
        LayoutDirection::Row
    } else {
        LayoutDirection::Column
    };
    root.style.descriptor.layout.spacing.gap = density_metric(density, [2.0, 4.0, 6.0]);
    let enabled = spec
        .channels
        .first()
        .map(|channel| channel.control.enabled)
        .unwrap_or(true);
    root.style.descriptor.opacity = if enabled {
        1.0
    } else {
        ctx.theme().resolve_opacity("state.opacity.disabled")
    };
    meter_semantics(
        &mut root,
        &spec.aria_label,
        meter_now_db(spec),
        spec.min_db,
        spec.max_db,
        &spec.value_text,
    );
    for channel in &spec.channels {
        let mut visual = meter_channel(
            channel.ballistic_value,
            spec.segments,
            spec.style,
            vertical,
            effective_size,
            density,
            false,
            ctx,
        );
        visual.a11y.hidden = Some(true);
        root = root.child(visual);
    }
    root
}

fn readout(
    text: &str,
    label: Option<&str>,
    editing: bool,
    enabled: bool,
    size: ControlSize,
    density: ControlDensity,
    ctx: &RenderContext<'_>,
) -> Node {
    let mut root = Node::text(text);
    root.style.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
    root.style.descriptor.text_color = Some(ctx.theme().resolve_color("color.text.primary"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color(if editing {
        "color.accent.base"
    } else {
        "color.border.default"
    });
    let padding_x = density_metric(density, [4.0, 6.0, 8.0]);
    let padding_y = density_metric(density, [2.0, 4.0, 6.0]);
    root.style.descriptor.layout.spacing.padding.left = padding_x;
    root.style.descriptor.layout.spacing.padding.right = padding_x;
    root.style.descriptor.layout.spacing.padding.top = padding_y;
    root.style.descriptor.layout.spacing.padding.bottom = padding_y;
    root.style.text_size = Some(rem_to_px(size_font_rem(size)));
    root.style.descriptor.opacity = if enabled { 1.0 } else { 0.48 };
    if let Some(label) = label {
        root.a11y.role = Some(NodeRole::Label);
        root.a11y.label = Some(format!("{label}: {text}"));
    }
    root
}

pub fn value_readout(spec: &ValueReadoutSpec, ctx: &RenderContext<'_>) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let mut root = readout(
        &spec.text,
        spec.aria_label.as_deref(),
        false,
        spec.visual_state.enabled,
        effective_size,
        density,
        ctx,
    );
    root.id = Some("value-readout-root".into());
    // Svelte `<output>`: optional accessible name, formatted text announced
    // once. The shared readout helper concatenates for drag-number; override.
    root.a11y.role = Some(NodeRole::Status);
    root.a11y.label = spec.aria_label.clone();
    root
}

pub fn drag_number_field(spec: &DragNumberFieldSpec, ctx: &RenderContext<'_>) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let mut root = readout(
        &spec.text,
        Some(&spec.aria_label),
        spec.visual_state.focus,
        spec.visual_state.enabled,
        effective_size,
        density,
        ctx,
    );
    root.id = Some("drag-number-field-root".into());
    root.a11y.role = Some(NodeRole::SpinButton);
    root.interaction.focusable = spec.visual_state.enabled;
    root.interaction.disabled = !spec.visual_state.enabled;
    root
}

/// Resolved envelope geometry: fixed width, height, and point-control size.
/// The bind reuses the exact painted dimensions for hit testing, so pointer
/// math matches the render instead of guessing its own length.
pub(crate) fn envelope_geometry(
    spec: &EnvelopeEditorSpec,
    ctx: &RenderContext<'_>,
) -> (f32, f32, f32) {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let width = rem_to_px(audio_size_rem(
        effective_size,
        [8.0, 10.0, 12.0, 14.0, 16.0],
    ));
    let height = rem_to_px(audio_size_rem(effective_size, [6.0, 8.0, 10.0, 12.0, 14.0]));
    (width, height, density_metric(density, [6.0, 8.0, 10.0]))
}

pub fn envelope_editor(spec: &EnvelopeEditorSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let (width, height, point_size) = envelope_geometry(spec, ctx);
    let accent = ctx.theme().resolve_color("color.accent.base");
    let mut root = Node::container();
    root.id = Some("envelope-editor-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    root.style.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
    root.style.descriptor.opacity = if state.enabled { 1.0 } else { 0.48 };
    root.a11y.role = Some(NodeRole::Group);
    root.a11y.label = Some(spec.aria_label.clone());

    for pair in state.points.windows(2) {
        let from = &pair[0];
        let to = &pair[1];
        for sample in 0..=24 {
            let t = sample as f64 / 24.0;
            let shaped = if from.curve == 0.0 {
                t
            } else if from.curve > 0.0 {
                t.powf(1.0 + from.curve * 4.0)
            } else {
                1.0 - (1.0 - t).powf(1.0 + from.curve.abs() * 4.0)
            };
            let x = from.x_norm + (to.x_norm - from.x_norm) * t;
            let y = from.y_norm + (to.y_norm - from.y_norm) * shaped;
            let mut dot = Node::container();
            circle(&mut dot, 2.0);
            dot.style.descriptor.background = Some(accent);
            absolute(
                &mut dot,
                x as f32 * width - 1.0,
                (1.0 - y as f32) * height - 1.0,
            );
            root = root.child(dot);
        }
    }
    for (index, point) in state.points.iter().enumerate() {
        let mut handle = Node::container();
        circle(
            &mut handle,
            if point.selected {
                point_size + 2.0
            } else {
                point_size
            },
        );
        handle.style.descriptor.background =
            Some(ctx.theme().resolve_color("color.background.elevated"));
        handle.style.descriptor.border.width = 2.0;
        handle.style.descriptor.border.color = accent;
        handle.a11y.role = Some(NodeRole::Slider);
        handle.a11y.label = Some(format!(
            "Point {}, X {} percent, Y {} percent, curve {:.2}",
            index + 1,
            (point.x_norm * 100.0).round() as i64,
            (point.y_norm * 100.0).round() as i64,
            point.curve
        ));
        handle.interaction.focusable = state.enabled;
        absolute(
            &mut handle,
            point.x_norm as f32 * width - point_size / 2.0,
            (1.0 - point.y_norm as f32) * height - point_size / 2.0,
        );
        root = root.child(handle);
    }
    root
}

pub fn envelope_editor_with_handlers(
    spec: &EnvelopeEditorSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::EnvelopeHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::EnvelopeLive>>,
) -> Node {
    let (width, height, _) = envelope_geometry(spec, ctx);
    let mut node = envelope_editor(spec, ctx);
    crate::audio_handlers::bind_envelope_editor(
        &mut node, spec, ctx, handlers, live, width, height,
    );
    node
}

pub fn xy_pad(spec: &XYPadSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let size = rem_to_px(audio_size_rem(effective_size, [6.0, 8.0, 10.0, 12.0, 14.0]));
    let thumb_size = density_metric(density, [10.0, 12.0, 16.0]);
    let accent = ctx.theme().resolve_color("color.accent.base");
    let mut root = Node::container();
    root.id = Some("xy-pad-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    root.style.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.a11y.role = Some(NodeRole::Group);
    root.a11y.label = Some(format!(
        "{}: X {}, Y {}",
        spec.aria_label, spec.x_value_text, spec.y_value_text
    ));
    let mut x_trace = Node::container();
    x_trace.style.descriptor.layout.width = LayoutSizing::Fixed(1.0);
    x_trace.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    x_trace.style.descriptor.background = Some(accent);
    absolute(&mut x_trace, state.x_norm as f32 * size, 0.0);
    root = root.child(x_trace);
    let mut y_trace = Node::container();
    y_trace.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    y_trace.style.descriptor.layout.height = LayoutSizing::Fixed(1.0);
    y_trace.style.descriptor.background = Some(accent);
    absolute(&mut y_trace, 0.0, (1.0 - state.y_norm as f32) * size);
    root = root.child(y_trace);
    let mut thumb = Node::container();
    circle(&mut thumb, thumb_size);
    thumb.style.descriptor.background =
        Some(ctx.theme().resolve_color("color.background.elevated"));
    thumb.style.descriptor.border.width = 2.0;
    thumb.style.descriptor.border.color = accent;
    absolute(
        &mut thumb,
        state.x_norm as f32 * size - thumb_size / 2.0,
        (1.0 - state.y_norm as f32) * size - thumb_size / 2.0,
    );
    root.child(thumb)
}

pub fn fader_with_handlers(
    spec: &FaderSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::FaderHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::FaderLive>>,
) -> Node {
    let mut node = fader(spec, ctx);
    crate::audio_handlers::bind_fader(&mut node, spec, ctx, handlers, live);
    node
}

pub fn knob_with_handlers(
    spec: &KnobSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::KnobHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::KnobLive>>,
) -> Node {
    let mut node = knob(spec, ctx);
    crate::audio_handlers::bind_knob(&mut node, spec, ctx, handlers, live);
    node
}

pub fn xy_pad_with_handlers(
    spec: &XYPadSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::XYPadHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::XYPadLive>>,
) -> Node {
    let mut node = xy_pad(spec, ctx);
    crate::audio_handlers::bind_xy_pad(&mut node, spec, ctx, handlers, live);
    node
}

pub fn drag_number_field_with_handlers(
    spec: &DragNumberFieldSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::DragNumberHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::DragNumberLive>>,
) -> Node {
    let mut node = drag_number_field(spec, ctx);
    crate::audio_handlers::bind_drag_number(&mut node, spec, ctx, handlers, live);
    node
}

pub fn audio_switch_with_handlers(
    spec: &AudioSwitchSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::AudioSwitchHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::AudioSwitchLive>>,
) -> Node {
    let mut node = audio_switch(spec, ctx);
    crate::audio_handlers::bind_audio_switch(&mut node, spec, ctx, handlers, live);
    node
}

pub fn audio_switch(spec: &AudioSwitchSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let mut root = Node::container();
    root.id = Some("audio-switch-root".into());
    root.style.descriptor.layout.direction = LayoutDirection::Row;
    root.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    root.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    root.style.descriptor.layout.spacing.gap = density_metric(density, [4.0, 8.0, 12.0]);
    root.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(audio_size_rem(
        effective_size,
        [2.25, 2.625, 3.0, 3.375, 3.75],
    )));
    root.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(audio_size_rem(
        effective_size,
        [1.5, 1.75, 2.0, 2.25, 2.5],
    )));
    root.style.descriptor.background = Some(ctx.theme().resolve_color(if state.state > 0 {
        "color.accent.base"
    } else {
        "color.background.surface"
    }));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
    root.style.descriptor.opacity = if state.enabled {
        1.0
    } else {
        ctx.theme().resolve_opacity("state.opacity.disabled")
    };
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.a11y.role = Some(NodeRole::Switch);
    let state_text = format!("State {} of {}", state.state + 1, state.state_count);
    root.a11y.label = Some(if spec.mode == AudioSwitchMode::Multi {
        format!("{}, {state_text}", spec.aria_label)
    } else {
        spec.aria_label.clone()
    });
    root.a11y.toggled = if spec.mode == AudioSwitchMode::Multi {
        None
    } else if state.state > 0 {
        Some(NodeToggled::True)
    } else {
        Some(NodeToggled::False)
    };
    let mut lamp = Node::container();
    circle(&mut lamp, 10.0);
    lamp.style.descriptor.background = Some(ctx.theme().resolve_color(if state.lamp_on {
        "color.status.success"
    } else {
        "color.border.subtle"
    }));
    lamp.a11y.hidden = Some(true);
    let mut label = Node::text(format!("{} / {}", state.state + 1, state.state_count));
    label.a11y.hidden = Some(true);
    root.child(lamp).child(label)
}

pub fn gain_reduction_meter(spec: &GainReductionMeterSpec, ctx: &RenderContext<'_>) -> Node {
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let mut root = meter_channel(
        spec.visual_state.meter.ballistic_value,
        spec.segments,
        spec.style,
        spec.orientation == Orientation::Vertical,
        effective_size,
        density,
        true,
        ctx,
    );
    root.id = Some("gain-reduction-meter-root".into());
    root.style.descriptor.opacity = if spec.visual_state.meter.control.enabled {
        1.0
    } else {
        ctx.theme().resolve_opacity("state.opacity.disabled")
    };
    let ballistic_db =
        spec.visual_state.meter.ballistic_value.clamp(0.0, 1.0) * spec.max_reduction_db;
    meter_semantics(
        &mut root,
        &spec.aria_label,
        ballistic_db,
        0.0,
        spec.max_reduction_db,
        &spec.value_text,
    );
    for child in &mut root.children {
        child.a11y.hidden = Some(true);
    }
    root
}

fn keyboard_recipe_color(ctx: &RenderContext<'_>, hook: &str, fallback: ColorValue) -> ColorValue {
    ctx.theme().try_resolve_color(hook).unwrap_or(fallback)
}

fn keyboard_recipe_opacity(ctx: &RenderContext<'_>, hook: &str, fallback: f32) -> f32 {
    ctx.theme().try_resolve_opacity(hook).unwrap_or(fallback)
}

pub fn keyboard(spec: &KeyboardSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let horizontal = state.orientation == KeyboardOrientation::Horizontal;
    let (width, height) = keyboard_dimensions(size, state.orientation);
    let accent = ctx.theme().resolve_color("color.accent.base");
    let white_idle = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.white-key",
        poodle_tokens::typed::primitives::COLOR_NEUTRAL_25,
    );
    let black_idle = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.black-key",
        poodle_tokens::typed::primitives::COLOR_NEUTRAL_900,
    );
    let white_held = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.white-key-held",
        mix_srgb(accent, WHITE, 0.60),
    );
    let black_held = keyboard_recipe_color(ctx, "recipe.keyboard.black-key-held", accent);
    let fill = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.fill",
        ctx.theme().resolve_color("color.background.surface"),
    );
    let container_border = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.border",
        ctx.theme().resolve_color("color.border.default"),
    );
    let key_border = keyboard_recipe_color(
        ctx,
        "recipe.keyboard.border",
        with_alpha(poodle_tokens::typed::primitives::COLOR_NEUTRAL_900, 0.42),
    );
    let external_ring = keyboard_recipe_color(ctx, "recipe.keyboard.external-ring", accent);
    let focus_ring = FocusRing {
        color: keyboard_recipe_color(
            ctx,
            "recipe.keyboard.focus-ring",
            ctx.theme().resolve_color("color.accent.focusRing"),
        ),
        width: rem_to_px(0.125),
        offset: rem_to_px(-0.1875),
    };
    let mut root = Node::container();
    root.id = Some("keyboard-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    if !horizontal && state.key_layout == KeyboardKeyLayout::EqualRows {
        root.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }
    root.style.descriptor.background = Some(fill);
    root.style.descriptor.border.width = density_metric(density, [0.5, 1.0, 2.0]);
    root.style.descriptor.border.color = container_border;
    root.a11y.role = Some(NodeRole::Toolbar);
    root.a11y.label = Some(spec.aria_label.clone());
    root.a11y.orientation = Some(if horizontal {
        "horizontal".into()
    } else {
        "vertical".into()
    });
    root.interaction.disabled = !state.enabled;
    root.interaction.focusable = state.enabled;
    root.a11y.tab_index = Some(-1);
    if state.enabled {
        root.style.focus_ring = Some(FocusRing {
            color: with_alpha(accent, 0.32),
            width: rem_to_px(0.1875),
            offset: 0.0,
        });
    } else {
        root.style.descriptor.opacity = keyboard_recipe_opacity(
            ctx,
            "recipe.keyboard.disabled-opacity",
            ctx.theme().resolve_opacity("state.opacity.disabled"),
        );
    }
    let first_note = state.keys.first().map(|key| key.note);
    let any_focused = state.keys.iter().any(|key| key.focused);
    for key in &state.keys {
        let mut visual = Node::container();
        visual.id = Some(format!("keyboard-root:visual-{}", key.note));
        let held = key.held || key.externally_held;
        visual.style.descriptor.background = Some(if held {
            if key.black {
                black_held
            } else {
                white_held
            }
        } else if key.black {
            black_idle
        } else {
            white_idle
        });
        visual.style.descriptor.border.width = density_metric(density, [0.0, 1.0, 2.0]);
        visual.style.descriptor.border.color = key_border;
        if key.externally_held {
            visual.style.descriptor.border.width = rem_to_px(0.125);
            visual.style.descriptor.border.color = external_ring;
        }
        if key.focused {
            visual.style.focus_ring = Some(focus_ring);
            visual.style.focus_ring_within = true;
        }
        if horizontal {
            visual.style.descriptor.layout.width =
                LayoutSizing::Fixed(key.length_norm as f32 * width);
            visual.style.descriptor.layout.height =
                LayoutSizing::Fixed(key.breadth_norm as f32 * height);
            absolute(&mut visual, key.start_norm as f32 * width, 0.0);
        } else if state.key_layout == KeyboardKeyLayout::EqualRows {
            let row_top = key
                .start_px
                .expect("equal-row vertical keys carry pixel geometry");
            let row_height = key
                .length_px
                .expect("equal-row vertical keys carry pixel geometry");
            visual.style.descriptor.layout.width =
                LayoutSizing::Fixed(key.breadth_norm as f32 * width);
            visual.style.descriptor.layout.height = LayoutSizing::Fixed(row_height as f32);
            absolute(&mut visual, 0.0, row_top as f32);
        } else {
            visual.style.descriptor.layout.width =
                LayoutSizing::Fixed(key.breadth_norm as f32 * width);
            visual.style.descriptor.layout.height =
                LayoutSizing::Fixed(key.length_norm as f32 * height);
            absolute(&mut visual, 0.0, key.start_norm as f32 * height);
        }
        let mut control = Node::container();
        control.id = Some(format!("keyboard-root:note-{}", key.note));
        control.a11y.role = Some(NodeRole::Button);
        control.a11y.label = Some(format_value(f64::from(key.note), AudioValueFormat::Note));
        control.a11y.toggled = Some(if key.held || key.externally_held {
            NodeToggled::True
        } else {
            NodeToggled::False
        });
        let tab_stop = key.focused || (!any_focused && first_note == Some(key.note));
        control.interaction.focusable = state.enabled;
        control.a11y.tab_index = if state.enabled {
            Some(if tab_stop { 0 } else { -1 })
        } else {
            None
        };
        control.style.descriptor.layout.width = LayoutSizing::Fixed(1.0);
        control.style.descriptor.layout.height = LayoutSizing::Fixed(1.0);
        absolute(&mut control, 0.0, 0.0);
        root = root.child(visual.child(control));
    }
    root
}

pub fn keyboard_with_handlers(
    spec: &KeyboardSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::KeyboardHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::KeyboardLive>>,
) -> Node {
    let mut node = keyboard(spec, ctx);
    crate::audio_handlers::bind_keyboard(&mut node, spec, ctx, handlers, live);
    node
}

pub fn waveform_display(spec: &WaveformDisplaySpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let width = rem_to_px(audio_size_rem(size, [12.0, 17.0, 22.0, 27.0, 32.0]));
    let height = rem_to_px(audio_size_rem(size, [3.0, 5.0, 7.0, 9.0, 11.0]));
    let mut root = Node::container();
    root.id = Some("waveform-display-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    root.style.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
    root.a11y.role = Some(NodeRole::Slider);
    let mut summary = format!(
        "{}, samples {} to {}",
        spec.aria_label, state.visible_start, state.visible_end
    );
    if let Some(cursor) = state.cursor_sample {
        summary.push_str(&format!(", cursor {cursor}"));
    }
    if let Some(selection) = state.selection {
        summary.push_str(&format!(
            ", selection {} to {}",
            selection.start, selection.end
        ));
    }
    root.a11y.label = Some(summary.clone());
    root.a11y.value_min = Some(state.visible_start as f64);
    root.a11y.value_max = Some(state.visible_end.saturating_sub(1).max(state.visible_start) as f64);
    root.a11y.value = Some(state.cursor_sample.unwrap_or(state.visible_start) as f64);
    root.a11y.value_text = Some(summary);
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    let gap = density_metric(density, [0.0, 0.5, 1.0]);
    let column_width = (width / state.columns.len().max(1) as f32 - gap).max(1.0);
    for (index, peak) in state.columns.iter().enumerate() {
        let mut column = Node::container();
        let amplitude = (peak.max - peak.min).clamp(0.0, 2.0) as f32;
        column.style.descriptor.layout.width = LayoutSizing::Fixed(column_width);
        column.style.descriptor.layout.height =
            LayoutSizing::Fixed((amplitude * height / 2.0).max(1.0));
        column.style.descriptor.background = Some(ctx.theme().resolve_color("color.accent.base"));
        absolute(
            &mut column,
            index as f32 * (column_width + gap),
            (1.0 - peak.max.clamp(-1.0, 1.0) as f32) * height / 2.0,
        );
        root = root.child(column);
    }
    let sample_span = (state.visible_end.saturating_sub(state.visible_start)).max(1) as f32;
    if let Some(selection) = state.selection {
        let mut overlay = Node::container();
        overlay.style.descriptor.layout.width = LayoutSizing::Fixed(
            ((selection.end.saturating_sub(selection.start) + 1) as f32 / sample_span * width)
                .max(1.0),
        );
        overlay.style.descriptor.layout.height = LayoutSizing::Fixed(height);
        overlay.style.descriptor.background = Some(ctx.theme().resolve_color("color.accent.base"));
        overlay.style.descriptor.opacity = 0.22;
        absolute(
            &mut overlay,
            selection.start.saturating_sub(state.visible_start) as f32 / sample_span * width,
            0.0,
        );
        root = root.child(overlay);
    }
    if let Some(cursor) = state.cursor_sample {
        let mut line = Node::container();
        line.style.descriptor.layout.width = LayoutSizing::Fixed(2.0);
        line.style.descriptor.layout.height = LayoutSizing::Fixed(height);
        line.style.descriptor.background = Some(ctx.theme().resolve_color("color.accent.base"));
        absolute(
            &mut line,
            cursor.saturating_sub(state.visible_start) as f32 / sample_span * width,
            0.0,
        );
        root = root.child(line);
    }
    root
}

pub fn mod_matrix_grid(spec: &ModMatrixGridSpec, ctx: &RenderContext<'_>) -> Node {
    let state = &spec.visual_state;
    let size = ctx.resolve_size(spec.size, spec.size_role);
    let density = ctx.resolve_density(spec.density);
    let cell_width = rem_to_px(audio_size_rem(size, [2.5, 3.0, 3.5, 4.0, 4.5]));
    let cell_height = rem_to_px(audio_size_rem(size, [1.25, 1.5, 1.75, 2.0, 2.25]));
    let gap = density_metric(density, [2.0, 4.0, 8.0]);
    let mut root = Node::container();
    root.id = Some("mod-matrix-grid-root".into());
    root.style.descriptor.layout.spacing.gap = gap;
    root.style.descriptor.background = Some(ctx.theme().resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
    root.a11y.role = Some(NodeRole::Grid);
    root.a11y.label = Some(spec.aria_label.clone());
    root.interaction.disabled = !state.enabled;
    let mut header = Node::container();
    header.style.descriptor.layout.direction = LayoutDirection::Row;
    header.style.descriptor.layout.spacing.gap = gap;
    let mut corner = Node::text("Source");
    corner.style.descriptor.layout.width = LayoutSizing::Fixed(cell_width * 1.8);
    header = header.child(corner);
    for destination in &state.destinations {
        let mut label = Node::text(destination.label.clone());
        label.style.descriptor.layout.width = LayoutSizing::Fixed(cell_width);
        header = header.child(label);
    }
    root = root.child(header);
    for source in &state.sources {
        let mut row = Node::container();
        row.style.descriptor.layout.direction = LayoutDirection::Row;
        row.style.descriptor.layout.spacing.gap = gap;
        row.a11y.role = Some(NodeRole::Row);
        row.a11y.label = Some(source.label.clone());
        let mut label = Node::text(source.label.clone());
        label.style.descriptor.layout.width = LayoutSizing::Fixed(cell_width * 1.8);
        row = row.child(label);
        for cell in state
            .cells
            .iter()
            .filter(|cell| cell.cell.source_id == source.id)
        {
            let destination_label = state
                .destinations
                .iter()
                .find(|destination| destination.id == cell.cell.destination_id)
                .map(|destination| destination.label.as_str())
                .unwrap_or(cell.cell.destination_id.as_str());
            let mut node = Node::container();
            node.style.descriptor.layout.width = LayoutSizing::Fixed(cell_width);
            node.style.descriptor.layout.height = LayoutSizing::Fixed(cell_height);
            node.style.descriptor.background =
                Some(ctx.theme().resolve_color(if cell.cell.enabled {
                    "color.background.elevated"
                } else {
                    "color.background.canvas"
                }));
            node.style.descriptor.border.width = if cell.focused { 2.0 } else { 1.0 };
            node.style.descriptor.border.color = ctx.theme().resolve_color("color.border.default");
            node.a11y.role = Some(NodeRole::Cell);
            node.a11y.label = Some(format!(
                "{} to {}, {}, {:.2}, range {:.2} to {:.2}",
                source.label,
                destination_label,
                if cell.cell.enabled {
                    "enabled"
                } else {
                    "disabled"
                },
                cell.cell.amount,
                cell.cell.parameters.min,
                cell.cell.parameters.max
            ));
            node.a11y.selected = Some(cell.cell.enabled);
            node.interaction.focusable = state.enabled;
            let mut zero = Node::container();
            zero.style.descriptor.layout.width = LayoutSizing::Fixed(1.0);
            zero.style.descriptor.layout.height = LayoutSizing::Fixed(cell_height * 0.64);
            zero.style.descriptor.background =
                Some(ctx.theme().resolve_color("color.border.default"));
            absolute(
                &mut zero,
                cell.zero_norm as f32 * cell_width,
                cell_height * 0.18,
            );
            node = node.child(zero);
            let amount_width = cell.fill_span_norm as f32 * cell_width;
            if amount_width > 0.0 {
                let mut amount = Node::container();
                amount.style.descriptor.layout.width = LayoutSizing::Fixed(amount_width);
                amount.style.descriptor.layout.height =
                    LayoutSizing::Fixed(density_metric(density, [2.0, 4.0, 6.0]));
                amount.style.descriptor.background = Some(ctx.theme().resolve_color(
                    if cell.amount_norm < cell.zero_norm {
                        "color.status.danger"
                    } else {
                        "color.accent.base"
                    },
                ));
                absolute(
                    &mut amount,
                    cell.fill_start_norm as f32 * cell_width,
                    cell_height / 2.0 - 2.0,
                );
                node = node.child(amount);
            }
            row = row.child(node);
        }
        root = root.child(row);
    }
    root
}

pub fn waveform_display_with_handlers(
    spec: &WaveformDisplaySpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::WaveformHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::WaveformLive>>,
) -> Node {
    let mut node = waveform_display(spec, ctx);
    crate::audio_handlers::bind_waveform_display(&mut node, spec, ctx, handlers, live);
    node
}

pub fn mod_matrix_grid_with_handlers(
    spec: &ModMatrixGridSpec,
    ctx: &RenderContext<'_>,
    handlers: &crate::audio_handlers::ModMatrixHandlers,
    live: &std::sync::Arc<std::sync::Mutex<crate::audio_handlers::ModMatrixLive>>,
) -> Node {
    let mut node = mod_matrix_grid(spec, ctx);
    crate::audio_handlers::bind_mod_matrix_grid(&mut node, spec, ctx, handlers, live);
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::audio::{
        switch_visual_state, AudioMeterContext, AudioSwitchMode, AudioValueLaw,
        EnvelopeVisualPoint, EnvelopeVisualState,
    };

    struct Theme;
    impl poodle_adapter::ThemeProvider for Theme {
        fn resolve_color(&self, _: &str) -> poodle_node::ColorValue {
            poodle_node::ColorValue(0.5, 0.5, 0.5, 1.0)
        }
        fn resolve_space(&self, _: &str) -> f32 {
            8.0
        }
        fn resolve_border_width(&self, _: &str) -> f32 {
            1.0
        }
        fn resolve_radius(&self, _: &str) -> f32 {
            4.0
        }
        fn resolve_opacity(&self, _: &str) -> f32 {
            1.0
        }
    }

    #[test]
    fn controls_expose_native_roles_and_value_text() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let knob = knob(&KnobSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear), &ctx);
        assert_eq!(knob.a11y.role, Some(NodeRole::Slider));
        assert!(knob.a11y.label.as_deref().unwrap().contains("0.5"));
        let switch = audio_switch(
            &AudioSwitchSpec::new(
                switch_visual_state(AudioSwitchMode::Latch, 1, 2, false, None, true),
                AudioSwitchMode::Latch,
            ),
            &ctx,
        );
        assert_eq!(switch.a11y.toggled, Some(NodeToggled::True));
        let meter = audio_meter(
            &AudioMeterSpec::new(AudioMeterContext::default().visual_state()),
            &ctx,
        );
        assert_eq!(meter.a11y.role, Some(NodeRole::Meter));
        assert_eq!(meter.a11y.value_min, Some(-60.0));
        assert_eq!(meter.a11y.value_max, Some(0.0));
        let reduction = gain_reduction_meter(
            &GainReductionMeterSpec::new(
                poodle_headless::audio::GainReductionContext::default().visual_state(),
                30.0,
            ),
            &ctx,
        );
        assert_eq!(reduction.a11y.role, Some(NodeRole::Meter));
        assert_eq!(reduction.a11y.value_min, Some(0.0));
        assert_eq!(reduction.a11y.value_max, Some(30.0));
        let readout = value_readout(
            &ValueReadoutSpec::new(
                poodle_headless::audio::AudioControlVisualState::from_value(
                    440.0,
                    20.0,
                    20_000.0,
                    AudioValueLaw::Linear,
                    true,
                ),
                "440 Hz",
            ),
            &ctx,
        );
        assert_eq!(readout.a11y.role, Some(NodeRole::Status));
        assert_eq!(readout.intrinsic_text(), Some("440 Hz"));
    }

    #[test]
    fn stereo_meter_has_two_visual_channels() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let visual = AudioMeterContext::default().visual_state();
        let mut spec = AudioMeterSpec::new(visual.clone());
        spec.channels.push(visual);
        assert_eq!(audio_meter(&spec, &ctx).children.len(), 2);
    }

    #[test]
    fn phase_three_renderers_expose_visual_state_semantics() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let keyboard_state = poodle_headless::audio::keyboard_visual_state(
            &poodle_headless::audio::KeyboardContext::default(),
        );
        let keyboard_node = keyboard(&KeyboardSpec::new(keyboard_state), &ctx);
        assert_eq!(keyboard_node.a11y.role, Some(NodeRole::Toolbar));
        assert!(keyboard_node
            .find(&|child| child.a11y.role == Some(NodeRole::Button))
            .is_some());

        let waveform_state = poodle_headless::audio::WaveformContext {
            pyramid: poodle_headless::audio::WaveformPeakPyramid {
                sample_count: 1,
                levels: vec![poodle_headless::audio::WaveformPeakLevel {
                    samples_per_peak: 1,
                    peaks: vec![poodle_headless::audio::WaveformPeakPair {
                        min: -0.5,
                        max: 0.5,
                    }],
                }],
            },
            visible_start: 0,
            visible_end: 1,
            column_count: 1,
            cursor_sample: Some(0),
            selection: None,
            selection_anchor: None,
            selecting: false,
            focus: true,
            disabled: false,
        }
        .visual_state();
        assert_eq!(
            waveform_display(&WaveformDisplaySpec::new(waveform_state), &ctx)
                .a11y
                .role,
            Some(NodeRole::Slider)
        );

        let matrix_state = poodle_headless::audio::ModMatrixContext::new(
            vec![poodle_headless::audio::ModMatrixHeader {
                id: "s".into(),
                label: "Source".into(),
            }],
            vec![poodle_headless::audio::ModMatrixHeader {
                id: "d".into(),
                label: "Destination".into(),
            }],
            vec![],
        )
        .visual_state();
        assert_eq!(
            mod_matrix_grid(&ModMatrixGridSpec::new(matrix_state), &ctx)
                .a11y
                .role,
            Some(NodeRole::Grid)
        );
    }

    #[test]
    fn envelope_points_keep_independent_slider_semantics() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let visual = EnvelopeVisualState {
            points: vec![EnvelopeVisualPoint {
                id: "attack".into(),
                x_norm: 0.2,
                y_norm: 0.8,
                curve: 0.0,
                selected: true,
                dragging: false,
            }],
            hover_point_id: None,
            focus: true,
            enabled: true,
        };
        let node = envelope_editor(&EnvelopeEditorSpec::new(visual), &ctx);
        assert!(node
            .children
            .iter()
            .any(|child| child.a11y.role == Some(NodeRole::Slider)));
    }

    #[test]
    fn presentation_axes_change_geometry_without_changing_visual_state() {
        let theme = Theme;
        let ctx = RenderContext::new(&theme);
        let mut small = KnobSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear);
        small.size = Some(ControlSize::Xs);
        let mut large = small.clone();
        large.size = Some(ControlSize::Xl);
        assert_eq!(small.visual_state, large.visual_state);
        assert_ne!(
            knob(&small, &ctx).style.descriptor.layout.width,
            knob(&large, &ctx).style.descriptor.layout.width,
        );

        let mut compact = AudioMeterSpec::new(AudioMeterContext::default().visual_state());
        compact.density = Some(ControlDensity::Compact);
        let mut comfortable = compact.clone();
        comfortable.density = Some(ControlDensity::Comfortable);
        assert_eq!(compact.channels, comfortable.channels);
        assert_ne!(
            audio_meter(&compact, &ctx)
                .style
                .descriptor
                .layout
                .spacing
                .gap,
            audio_meter(&comfortable, &ctx)
                .style
                .descriptor
                .layout
                .spacing
                .gap,
        );
    }
}
