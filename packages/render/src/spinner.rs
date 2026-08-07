//! Spinner — animated loading indicator: ring, grid or dots.
//!
//! Contract: `docs/contracts/components/spinner.md`
//! Ported from: `packages/jetstream/components/src/spinner.rs`. The animation
//! is *declared* on the node (keyframes, duration, loop); driving the clock is
//! the backend's job, keyed by the animation's stable key so immediate-mode
//! rebuilds don't restart it.

use poodle_adapter::ThemeProvider;
use poodle_node::{
    AnimEasing, AnimKeyframe, AnimLoop, AnimProperty, ColorValue, CrossAxisAlignment,
    LayoutDirection, LayoutSizing, MainAxisAlignment, Node, NodeAnimation, NodeRole,
};
use poodle_specs::{SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::presentation::rem_to_px;

pub fn spinner(spec: &SpinnerSpec, theme: &dyn ThemeProvider) -> Node {
    let tone_color = match spec.tone {
        SpinnerTone::Current => theme.resolve_color("color.text.primary"),
        SpinnerTone::Accent => theme.resolve_color("color.accent.base"),
        SpinnerTone::Muted => theme.resolve_color("color.text.secondary"),
    };

    let mut root = match spec.variant {
        SpinnerVariant::Ring => build_ring(spec, tone_color),
        SpinnerVariant::Grid => build_grid(spec, tone_color),
        SpinnerVariant::Dots => build_dots(tone_color),
    };
    if let Some(label) = spec.aria_label.as_deref() {
        root.a11y.label = Some(label.to_string());
    }
    root.a11y.role = Some(NodeRole::Status);
    root
}

/// Ring: the same spinner SVG asset and rotation used by the old GPUI tier.
fn build_ring(spec: &SpinnerSpec, tone: ColorValue) -> Node {
    let mut el = Node::icon("spinner", spec.size_px());
    el.style.descriptor.text_color = Some(tone);
    // Contract: spinner-ring 0.8s linear infinite. One shared key: all ring
    // spinners share a clock and rotate in phase, like CSS keyframes.
    el.style.animation = Some(NodeAnimation::spin("poodle-spinner-ring", 0.8));
    el
}

/// Grid: 6 cells, 2×3, each pulsing opacity phase-shifted into a snake.
fn build_grid(spec: &SpinnerSpec, tone: ColorValue) -> Node {
    let width = rem_to_px(spec.grid_width_rem());
    let height = rem_to_px(spec.grid_height_rem());
    let cell_radius = rem_to_px(spec.cell_radius_rem());
    let gap = rem_to_px(spec.grid_gap_rem());
    let cell_w = (width - gap) / 2.0;
    let cell_h = (height - gap * 2.0) / 3.0;

    let floor = spec.opacity_floor();
    let span = spec.opacity_peak() - spec.opacity_floor();
    let phase = [1.0_f32, 0.7, 0.4, 0.85, 0.1, 0.55];

    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.width = LayoutSizing::Fixed(width);
        s.descriptor.layout.height = LayoutSizing::Fixed(height);
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.flex_wrap = true;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
    }

    for (i, &ph) in phase.iter().enumerate() {
        let mut cell = Node::container();
        {
            let s = &mut cell.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(cell_w);
            s.descriptor.layout.height = LayoutSizing::Fixed(cell_h);
            s.descriptor.corner_radii.top_left = cell_radius;
            s.descriptor.corner_radii.top_right = cell_radius;
            s.descriptor.corner_radii.bottom_right = cell_radius;
            s.descriptor.corner_radii.bottom_left = cell_radius;
            s.descriptor.background = Some(tone);
            s.animation = Some(cell_pulse(i, floor, span, ph));
        }
        root = root.child(cell);
    }
    root
}

/// Dots: three static dots, the quietest variant.
fn build_dots(tone: ColorValue) -> Node {
    let dot = rem_to_px(0.25);
    let gap = rem_to_px(0.1875);

    let mut row = Node::container();
    {
        let s = &mut row.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
    }
    for _ in 0..3 {
        let mut d = Node::container();
        {
            let s = &mut d.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(dot);
            s.descriptor.layout.height = LayoutSizing::Fixed(dot);
            s.descriptor.corner_radii.top_left = 999.0;
            s.descriptor.corner_radii.top_right = 999.0;
            s.descriptor.corner_radii.bottom_right = 999.0;
            s.descriptor.corner_radii.bottom_left = 999.0;
            s.descriptor.background = Some(tone);
        }
        row = row.child(d);
    }
    row
}

/// Looping opacity pulse: a sine sweep through the contract band, phase-shifted
/// per cell.
fn cell_pulse(index: usize, floor: f32, span: f32, phase: f32) -> NodeAnimation {
    let sample = |t: f32| -> f32 {
        let angle = (t + phase) * std::f32::consts::TAU;
        floor + span * (0.5 + 0.5 * angle.sin())
    };
    let keyframes = (0..=4)
        .map(|k| {
            let at = k as f32 / 4.0;
            AnimKeyframe {
                at,
                values: vec![(AnimProperty::Opacity, sample(at))],
            }
        })
        .collect();
    NodeAnimation {
        key: format!("poodle-spinner-cell-{index}"),
        keyframes,
        duration_secs: 1.2,
        easing: AnimEasing::Linear,
        loop_mode: AnimLoop::Loop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_uses_the_reference_svg_asset_and_rotation() {
        let theme =
            poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
        let node = spinner(&SpinnerSpec::new(), &theme);
        assert!(matches!(
            &node.kind,
            poodle_node::NodeKind::Icon { name, .. } if name == "spinner"
        ));
        let animation = node.style.animation.expect("ring rotation");
        assert_eq!(animation.duration_secs, 0.8);
        assert!(animation.keyframes.iter().any(|frame| frame
            .values
            .iter()
            .any(|(property, _)| *property == AnimProperty::Rotate)));
    }
}
