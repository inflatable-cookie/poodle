//! Audio controls rendered exclusively from serializable visual state.
//!
//! Contract: `docs/architecture/008-audio-control-family.md` and the nine
//! component contracts under `docs/contracts/components/`.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeKind,
    NodePosition, NodeRole, NodeToggled,
};
use poodle_specs::{
    AudioMeterSpec, AudioMeterStyle, AudioSwitchSpec, ControlDensity, ControlSize,
    DragNumberFieldSpec, EnvelopeEditorSpec, FaderSpec, GainReductionMeterSpec, KnobSpec,
    Orientation, ValueReadoutSpec, XYPadSpec,
};

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};

fn audio_size_rem(size: ControlSize, values: [f32; 5]) -> f32 {
    values[match size {
        ControlSize::Xs => 0,
        ControlSize::Sm => 1,
        ControlSize::Md => 2,
        ControlSize::Lg => 3,
        ControlSize::Xl => 4,
    }]
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

pub fn knob(spec: &KnobSpec, theme: &dyn ThemeProvider) -> Node {
    let state = &spec.visual_state;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let size = rem_to_px(audio_size_rem(effective_size, [2.0, 2.5, 3.0, 3.5, 4.0]));
    let center = size / 2.0;
    let accent = theme.resolve_color("color.accent.base");
    let muted = theme.resolve_color("color.border.default");
    let surface = theme.resolve_color("color.background.elevated");
    let mut root = Node::container();
    root.id = Some("knob-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.style.descriptor.opacity = if state.enabled { 1.0 } else { 0.48 };
    a11y_value(
        &mut root,
        NodeRole::Slider,
        &spec.aria_label,
        &spec.value_text,
    );

    let segment_count = match spec.density {
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
    cap.style.descriptor.border.width = 1.0;
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

pub fn fader(spec: &FaderSpec, theme: &dyn ThemeProvider) -> Node {
    let state = &spec.visual_state;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let vertical = spec.orientation == Orientation::Vertical;
    let cross = rem_to_px(audio_size_rem(effective_size, [1.5, 1.75, 2.0, 2.25, 2.5]));
    let length = rem_to_px(audio_size_rem(effective_size, [7.0, 8.5, 10.0, 11.5, 13.0]));
    let (width, height) = if vertical {
        (cross, length)
    } else {
        (length, cross)
    };
    let track = theme.resolve_color("color.border.default");
    let accent = theme.resolve_color("color.accent.base");
    let thumb_color = theme.resolve_color("color.background.elevated");
    let mut root = Node::container();
    root.id = Some("fader-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.style.descriptor.opacity = if state.enabled { 1.0 } else { 0.48 };
    a11y_value(
        &mut root,
        NodeRole::Slider,
        &spec.aria_label,
        &spec.value_text,
    );

    let mut rail = Node::container();
    let rail_cross = density_metric(spec.density, [4.0, 6.0, 8.0]);
    let rail_w = if vertical { rail_cross } else { width };
    let rail_h = if vertical { height } else { rail_cross };
    rail.style.descriptor.layout.width = LayoutSizing::Fixed(rail_w);
    rail.style.descriptor.layout.height = LayoutSizing::Fixed(rail_h);
    rail.style.descriptor.background = Some(track);
    let r = 3.0;
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
        let norm = detent.clamp(0.0, 1.0) as f32;
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
    thumb.style.descriptor.border.width = 1.0;
    thumb.style.descriptor.border.color = accent;
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

fn meter_channel(
    value: f64,
    segments: usize,
    style: AudioMeterStyle,
    vertical: bool,
    size: ControlSize,
    density: ControlDensity,
    gain_reduction: bool,
    theme: &dyn ThemeProvider,
) -> Node {
    let accent = theme.resolve_color("color.status.success");
    let warning = theme.resolve_color("color.status.warning");
    let danger = theme.resolve_color("color.status.danger");
    let idle = theme.resolve_color("color.border.subtle");
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

pub fn audio_meter(spec: &AudioMeterSpec, theme: &dyn ThemeProvider) -> Node {
    let vertical = spec.orientation == Orientation::Vertical;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let mut root = Node::container();
    root.id = Some("audio-meter-root".into());
    root.style.descriptor.layout.direction = if vertical {
        LayoutDirection::Row
    } else {
        LayoutDirection::Column
    };
    root.style.descriptor.layout.spacing.gap = density_metric(spec.density, [2.0, 4.0, 6.0]);
    a11y_value(
        &mut root,
        NodeRole::ProgressIndicator,
        &spec.aria_label,
        &spec.value_text,
    );
    for channel in &spec.channels {
        root = root.child(meter_channel(
            channel.ballistic_value,
            spec.segments,
            spec.style,
            vertical,
            effective_size,
            spec.density,
            false,
            theme,
        ));
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
    theme: &dyn ThemeProvider,
) -> Node {
    let mut root = Node::text(text);
    root.style.descriptor.background = Some(theme.resolve_color("color.background.surface"));
    root.style.descriptor.text_color = Some(theme.resolve_color("color.text.primary"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = theme.resolve_color(if editing {
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

pub fn value_readout(spec: &ValueReadoutSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    readout(
        &spec.text,
        spec.aria_label.as_deref(),
        false,
        spec.visual_state.enabled,
        effective_size,
        spec.density,
        theme,
    )
}

pub fn drag_number_field(spec: &DragNumberFieldSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let mut root = readout(
        &spec.text,
        Some(&spec.aria_label),
        spec.visual_state.focus,
        spec.visual_state.enabled,
        effective_size,
        spec.density,
        theme,
    );
    root.id = Some("drag-number-field-root".into());
    root.a11y.role = Some(NodeRole::SpinButton);
    root.interaction.focusable = spec.visual_state.enabled;
    root.interaction.disabled = !spec.visual_state.enabled;
    root
}

pub fn envelope_editor(spec: &EnvelopeEditorSpec, theme: &dyn ThemeProvider) -> Node {
    let state = &spec.visual_state;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let width = rem_to_px(audio_size_rem(
        effective_size,
        [8.0, 10.0, 12.0, 14.0, 16.0],
    ));
    let height = rem_to_px(audio_size_rem(effective_size, [6.0, 8.0, 10.0, 12.0, 14.0]));
    let point_size = density_metric(spec.density, [6.0, 8.0, 10.0]);
    let accent = theme.resolve_color("color.accent.base");
    let mut root = Node::container();
    root.id = Some("envelope-editor-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    root.style.descriptor.background = Some(theme.resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = theme.resolve_color("color.border.default");
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
    for point in &state.points {
        let mut handle = Node::container();
        circle(
            &mut handle,
            if point.selected {
                point_size + 2.0
            } else {
                point_size
            },
        );
        handle.style.descriptor.background = Some(theme.resolve_color("color.background.elevated"));
        handle.style.descriptor.border.width = 2.0;
        handle.style.descriptor.border.color = accent;
        handle.a11y.role = Some(NodeRole::Slider);
        handle.a11y.label = Some(format!("Envelope point {}", point.id));
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

pub fn xy_pad(spec: &XYPadSpec, theme: &dyn ThemeProvider) -> Node {
    let state = &spec.visual_state;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let size = rem_to_px(audio_size_rem(effective_size, [6.0, 8.0, 10.0, 12.0, 14.0]));
    let thumb_size = density_metric(spec.density, [10.0, 12.0, 16.0]);
    let accent = theme.resolve_color("color.accent.base");
    let mut root = Node::container();
    root.id = Some("xy-pad-root".into());
    root.style.descriptor.layout.width = LayoutSizing::Fixed(size);
    root.style.descriptor.layout.height = LayoutSizing::Fixed(size);
    root.style.descriptor.background = Some(theme.resolve_color("color.background.surface"));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = theme.resolve_color("color.border.default");
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
    thumb.style.descriptor.background = Some(theme.resolve_color("color.background.elevated"));
    thumb.style.descriptor.border.width = 2.0;
    thumb.style.descriptor.border.color = accent;
    absolute(
        &mut thumb,
        state.x_norm as f32 * size - thumb_size / 2.0,
        (1.0 - state.y_norm as f32) * size - thumb_size / 2.0,
    );
    root.child(thumb)
}

pub fn audio_switch(spec: &AudioSwitchSpec, theme: &dyn ThemeProvider) -> Node {
    let state = &spec.visual_state;
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let mut root = Node::container();
    root.id = Some("audio-switch-root".into());
    root.style.descriptor.layout.direction = LayoutDirection::Row;
    root.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    root.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    root.style.descriptor.layout.spacing.gap = density_metric(spec.density, [4.0, 8.0, 12.0]);
    root.style.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(audio_size_rem(
        effective_size,
        [2.25, 2.625, 3.0, 3.375, 3.75],
    )));
    root.style.descriptor.layout.height = LayoutSizing::Fixed(rem_to_px(audio_size_rem(
        effective_size,
        [1.5, 1.75, 2.0, 2.25, 2.5],
    )));
    root.style.descriptor.background = Some(theme.resolve_color(if state.state > 0 {
        "color.accent.base"
    } else {
        "color.background.surface"
    }));
    root.style.descriptor.border.width = 1.0;
    root.style.descriptor.border.color = theme.resolve_color("color.border.default");
    root.interaction.focusable = state.enabled;
    root.interaction.disabled = !state.enabled;
    root.a11y.role = Some(NodeRole::Switch);
    root.a11y.label = Some(spec.aria_label.clone());
    root.a11y.toggled = Some(if state.state > 0 {
        NodeToggled::True
    } else {
        NodeToggled::False
    });
    let mut lamp = Node::container();
    circle(&mut lamp, 10.0);
    lamp.style.descriptor.background = Some(theme.resolve_color(if state.lamp_on {
        "color.status.success"
    } else {
        "color.border.subtle"
    }));
    let label = Node::text(format!("{} / {}", state.state + 1, state.state_count));
    root.child(lamp).child(label)
}

pub fn gain_reduction_meter(spec: &GainReductionMeterSpec, theme: &dyn ThemeProvider) -> Node {
    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let mut root = meter_channel(
        spec.visual_state.meter.ballistic_value,
        spec.segments,
        spec.style,
        spec.orientation == Orientation::Vertical,
        effective_size,
        spec.density,
        true,
        theme,
    );
    root.id = Some("gain-reduction-meter-root".into());
    a11y_value(
        &mut root,
        NodeRole::ProgressIndicator,
        &spec.aria_label,
        &spec.value_text,
    );
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::audio::{
        switch_visual_state, AudioMeterContext, AudioSwitchMode, AudioValueLaw,
        EnvelopeVisualPoint, EnvelopeVisualState,
    };

    struct Theme;
    impl ThemeProvider for Theme {
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
        let knob = knob(&KnobSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear), &Theme);
        assert_eq!(knob.a11y.role, Some(NodeRole::Slider));
        assert!(knob.a11y.label.as_deref().unwrap().contains("0.5"));
        let switch = audio_switch(
            &AudioSwitchSpec::new(
                switch_visual_state(AudioSwitchMode::Latch, 1, 2, false, None, true),
                AudioSwitchMode::Latch,
            ),
            &Theme,
        );
        assert_eq!(switch.a11y.toggled, Some(NodeToggled::True));
    }

    #[test]
    fn stereo_meter_has_two_visual_channels() {
        let visual = AudioMeterContext::default().visual_state();
        let mut spec = AudioMeterSpec::new(visual.clone());
        spec.channels.push(visual);
        assert_eq!(audio_meter(&spec, &Theme).children.len(), 2);
    }

    #[test]
    fn envelope_points_keep_independent_slider_semantics() {
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
        let node = envelope_editor(&EnvelopeEditorSpec::new(visual), &Theme);
        assert!(node
            .children
            .iter()
            .any(|child| child.a11y.role == Some(NodeRole::Slider)));
    }

    #[test]
    fn presentation_axes_change_geometry_without_changing_visual_state() {
        let mut small = KnobSpec::new(0.5, 0.0, 1.0, AudioValueLaw::Linear);
        small.size = ControlSize::Xs;
        let mut large = small.clone();
        large.size = ControlSize::Xl;
        assert_eq!(small.visual_state, large.visual_state);
        assert_ne!(
            knob(&small, &Theme).style.descriptor.layout.width,
            knob(&large, &Theme).style.descriptor.layout.width,
        );

        let mut compact = AudioMeterSpec::new(AudioMeterContext::default().visual_state());
        compact.density = ControlDensity::Compact;
        let mut comfortable = compact.clone();
        comfortable.density = ControlDensity::Comfortable;
        assert_eq!(compact.channels, comfortable.channels);
        assert_ne!(
            audio_meter(&compact, &Theme)
                .style
                .descriptor
                .layout
                .spacing
                .gap,
            audio_meter(&comfortable, &Theme)
                .style
                .descriptor
                .layout
                .spacing
                .gap,
        );
    }
}
