//! Stepper — the route through a wizard process.
//!
//! Contract: `docs/contracts/components/stepper.md`
//! Ported from: `packages/jetstream/components/src/stepper.rs`.
//!
//! Status is read from the step, never from its index — a step that ran and
//! failed renders as failed wherever it sits. The rerun control sits OUTSIDE
//! the trigger (re-running spends whatever the step costs, so it must not be
//! reachable by clicking to look at a finished step; see `stepper.md` §2).

use std::sync::Arc;

use poodle_adapter::ThemeProvider;
use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, LayoutDirection, LayoutSizing, MainAxisAlignment,
    Node, NodeRole, StylePatch,
};
use poodle_specs::{Orientation, StepStatus, StepperSpec};

use crate::color::with_alpha;
use crate::presentation::{rem_to_px, resolve_supporting_visual_size, size_font_rem};

/// Host callbacks: change and rerun, each carrying the step's value, plus the
/// summary's collapse toggle carrying the new state.
#[derive(Default)]
pub struct StepperHandlers {
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_rerun: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_collapsed_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn stepper(spec: &StepperSpec, theme: &dyn ThemeProvider, handlers: StepperHandlers) -> Node {
    let row_height = rem_to_px(spec.row_height_rem());
    let marker_size = rem_to_px(spec.marker_size_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let marker_font_size = rem_to_px(spec.marker_font_size_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());
    let radius = theme.resolve_radius(spec.radius_token());
    // Contract §8: a hairline divider, stated as an absolute.
    let hairline = rem_to_px(0.0625);

    let border = theme.resolve_color(spec.border_token());
    let panel = theme.resolve_color(spec.surface_token());
    let label_color = theme.resolve_color(spec.label_token());
    let active_label = theme.resolve_color(spec.active_label_token());
    let accent = theme.resolve_color(spec.accent_token());
    let danger = theme.resolve_color(spec.danger_token());
    let disabled_opacity = theme.resolve_opacity(spec.disabled_opacity_token());

    let current = spec.current_value().map(str::to_owned);

    let mut root = Node::container();
    root.a11y.role = Some(NodeRole::List);
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = if spec.orientation == Orientation::Vertical {
            LayoutDirection::Column
        } else {
            LayoutDirection::Row
        };
        s.descriptor.border.width = hairline;
        s.descriptor.border.color = border;
        let c = &mut s.descriptor.corner_radii;
        c.top_left = radius;
        c.top_right = radius;
        c.bottom_right = radius;
        c.bottom_left = radius;
        s.descriptor.background = Some(with_alpha(panel, panel.3 * 0.92));
        s.descriptor.layout.overflow_x = poodle_node::LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = poodle_node::LayoutOverflow::Hidden;
    }
    if let Some(aria) = &spec.aria_label {
        root.a11y.label = Some(aria.clone());
    }

    // Collapse folds the stepper, not step content: expanding reveals the same
    // rows that were always there. Vertical only — see `stepper.md` §3.
    if spec.shows_summary() {
        root = root.child(summary_row(
            spec,
            theme,
            &handlers,
            SummaryColors {
                border,
                accent,
                danger,
                label: label_color,
                active_label,
            },
            hairline,
            disabled_opacity,
        ));
    }

    if spec.is_collapsed_now() {
        // Omitted, not hidden: hidden triggers are still stops in the tab order.
        return root;
    }

    let last = spec.steps.len().saturating_sub(1);

    for (index, step) in spec.steps.iter().enumerate() {
        let is_current = current.as_deref() == Some(step.value.as_str());
        let has_rerun = spec.show_rerun && step.status == StepStatus::Complete;
        let is_disabled = spec.is_disabled || step.is_disabled;

        let marker_color = match step.status {
            StepStatus::Failed => danger,
            StepStatus::Complete | StepStatus::Running => accent,
            StepStatus::Pending if is_current => accent,
            StepStatus::Pending => label_color,
        };

        // Failed wins over current: the breakage is the more urgent fact.
        let text_color = match step.status {
            StepStatus::Failed => danger,
            StepStatus::Complete | StepStatus::Running => active_label,
            StepStatus::Pending if is_current => active_label,
            StepStatus::Pending => label_color,
        };

        let marker_glyph = match step.status {
            StepStatus::Complete => "✓".to_string(),
            StepStatus::Failed => "✕".to_string(),
            StepStatus::Running => "◌".to_string(),
            StepStatus::Pending => format!("{}", index + 1),
        };
        let mut marker = Node::text(marker_glyph);
        marker.style.text_size = Some(marker_font_size);
        marker.style.descriptor.text_color = Some(marker_color);

        let mut marker_box = Node::container();
        {
            let s = &mut marker_box.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(marker_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(marker_size);
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.flex_shrink_zero = true;
            s.descriptor.border.width = hairline;
            s.descriptor.border.color = marker_color;
            let c = &mut s.descriptor.corner_radii;
            c.top_left = 999.0;
            c.top_right = 999.0;
            c.bottom_right = 999.0;
            c.bottom_left = 999.0;
        }
        let marker_box = marker_box.child(marker);

        let status_word = match step.status {
            StepStatus::Running => ", running",
            StepStatus::Complete => ", complete",
            StepStatus::Failed => ", failed",
            StepStatus::Pending => "",
        };

        let mut trigger = Node::button("");
        // Status reaches assistive technology through the name; colour and
        // glyph do not. `pending` is omitted as the unremarkable case.
        trigger.a11y.label = Some(format!("{}{}", step.label, status_word));
        trigger.a11y.role = Some(NodeRole::Button);
        {
            let s = &mut trigger.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.spacing.gap = gap;
            s.descriptor.layout.width = LayoutSizing::Grow;
            if spec.orientation == Orientation::Horizontal {
                s.flex_basis = Some(0.0);
            }
            s.min_width = Some(0.0);
            s.min_height = Some(row_height);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = pad_x;
            pad.right = if has_rerun { 0.0 } else { pad_x };
            pad.top = pad_y;
            pad.bottom = pad_y;
            s.descriptor.background = Some(ColorValue(0.0, 0.0, 0.0, 0.0));
            s.text_size = Some(font_size);
            s.descriptor.text_color = Some(text_color);
        }
        trigger.interaction.focusable = true;
        let mut label = Node::text(step.label.clone());
        label.style.text_size = Some(font_size);
        label.style.descriptor.text_color = Some(text_color);
        label.style.min_width = Some(0.0);
        let mut trigger = trigger.child(marker_box).child(label);

        if is_disabled {
            trigger.style.descriptor.opacity = disabled_opacity;
            trigger.interaction.disabled = true;
        } else {
            trigger.style.descriptor.cursor = CursorHint::Pointer;
            if let Some(handler) = &handlers.on_change {
                let handler = Arc::clone(handler);
                let value = step.value.clone();
                trigger.interaction.on_activate = Some(Arc::new(move || handler(&value)));
            }
        }

        // The tint belongs to the whole column (spans trigger + rerun); the
        // hit target is unchanged.
        let mut cell = Node::container();
        cell.a11y.role = Some(NodeRole::ListItem);
        {
            let s = &mut cell.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            // items_stretch is the flex default — silence is correct here.
            s.min_width = Some(0.0);
            s.descriptor.layout.width = LayoutSizing::Grow;
            if spec.orientation == Orientation::Horizontal {
                s.flex_basis = Some(0.0);
            }
            if is_current {
                s.descriptor.background = Some(with_alpha(accent, accent.3 * 0.10));
            }
            if !is_disabled {
                let hover_fill = if is_current {
                    with_alpha(accent, accent.3 * 0.16)
                } else {
                    with_alpha(active_label, active_label.3 * 0.06)
                };
                s.hover = Some(StylePatch {
                    background: Some(hover_fill),
                    border_color: None,
                    text_color: None,
                    opacity: None,
                });
            }
        }
        let mut cell = cell.child(trigger);

        // Deliberately outside the trigger (see module docs).
        if has_rerun {
            let mut rerun = Node::button("");
            rerun.a11y.label = Some(format!("{}: {}", spec.rerun_label, step.label));
            rerun.a11y.role = Some(NodeRole::Button);
            {
                let s = &mut rerun.style;
                s.descriptor.layout.width = LayoutSizing::Fixed(marker_size);
                s.descriptor.layout.height = LayoutSizing::Fixed(marker_size);
                // Room on both sides so the icon reads as a deliberate action.
                s.descriptor.layout.spacing.margin.left = gap;
                s.descriptor.layout.spacing.margin.right = pad_x;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
                s.flex_shrink_zero = true;
            }
            rerun.interaction.focusable = true;
            let mut glyph = Node::text("⟳");
            glyph.style.text_size = Some(font_size);
            glyph.style.descriptor.text_color = Some(label_color);
            let mut rerun = rerun.child(glyph);

            if is_disabled {
                rerun.style.descriptor.opacity = disabled_opacity;
                rerun.interaction.disabled = true;
            } else {
                rerun.style.descriptor.cursor = CursorHint::Pointer;
                // Its own handler, inert when unwired: clicks bubble to the
                // nearest handler, so an unwired rerun would select the step.
                if let Some(handler) = &handlers.on_rerun {
                    let handler = Arc::clone(handler);
                    let value = step.value.clone();
                    rerun.interaction.on_activate = Some(Arc::new(move || handler(&value)));
                } else {
                    rerun.interaction.on_activate = Some(Arc::new(|| {}));
                }
            }
            cell = cell.child(rerun);
        }

        // Dividers inside the shared track; vertical moves them to the
        // bottom edge.
        if index < last {
            if spec.orientation == Orientation::Vertical {
                cell.style.border_bottom_width = Some(1.0);
            } else {
                cell.style.border_right_width = Some(1.0);
            }
            cell.style.descriptor.border.color = border;
        }

        root = root.child(cell);
    }

    root
}

/// The colours the summary row shares with the step rows.
struct SummaryColors {
    border: ColorValue,
    accent: ColorValue,
    danger: ColorValue,
    label: ColorValue,
    active_label: ColorValue,
}

/// The collapsed one-line form: chevron, rail, current step label, `n/m`.
fn summary_row(
    spec: &StepperSpec,
    theme: &dyn ThemeProvider,
    handlers: &StepperHandlers,
    colors: SummaryColors,
    hairline: f32,
    disabled_opacity: f32,
) -> Node {
    let row_height = rem_to_px(spec.row_height_rem());
    let font_size = rem_to_px(spec.font_size_rem());
    let pad_y = rem_to_px(spec.padding_block_rem());
    let pad_x = rem_to_px(spec.padding_inline_rem());
    let gap = rem_to_px(spec.gap_rem());
    let rail_gap = rem_to_px(spec.rail_gap_rem());
    let segment_thickness = rem_to_px(spec.rail_thickness_rem());
    let chevron_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        spec.resolved_size(),
    )));
    let count_color = theme.resolve_color(spec.count_token());
    let rail_pending = theme.resolve_color(spec.rail_pending_token());

    let is_collapsed = spec.is_collapsed_now();
    let completed = spec.completed_count();
    let total = spec.steps.len();
    let current_label = spec
        .current_step()
        .map(|step| step.label.clone())
        .unwrap_or_default();

    let mut summary = Node::button("");
    // Jetstream routes clicks by node id rather than by reading `on_activate`,
    // so the summary carries one — otherwise the toggle is reachable on GPUI
    // and inert on Jetstream.
    summary.id = Some("poodle-stepper-summary".to_string());
    // The visible `n/m` is decorative — "five slash five" is not a sentence, so
    // the name spells it out. Chevron and rail restate the same facts.
    summary.a11y.label = Some(format!(
        "{current_label}, {completed} of {total} steps complete"
    ));
    summary.a11y.role = Some(NodeRole::Button);
    summary.a11y.expanded = Some(!is_collapsed);
    summary.interaction.focusable = true;
    {
        let s = &mut summary.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap;
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
        s.min_height = Some(row_height);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = pad_x;
        pad.right = pad_x;
        pad.top = pad_y;
        pad.bottom = pad_y;
        s.descriptor.background = Some(ColorValue(0.0, 0.0, 0.0, 0.0));
        s.text_size = Some(font_size);
        // Expanded, the summary sits above the list and draws the same divider
        // the steps draw between themselves. Collapsed it is the last row.
        if !is_collapsed {
            s.border_bottom_width = Some(hairline);
            s.descriptor.border.color = colors.border;
        }
    }

    if spec.is_disabled {
        summary.style.descriptor.opacity = disabled_opacity;
        summary.interaction.disabled = true;
    } else {
        summary.style.descriptor.cursor = CursorHint::Pointer;
        summary.style.hover = Some(StylePatch {
            background: Some(with_alpha(
                colors.active_label,
                colors.active_label.3 * 0.06,
            )),
            border_color: None,
            text_color: None,
            opacity: None,
        });
        if let Some(handler) = &handlers.on_collapsed_change {
            let handler = Arc::clone(handler);
            let next = !is_collapsed;
            summary.interaction.on_activate = Some(Arc::new(move || handler(next)));
        }
    }

    let mut chevron = Node::icon(
        if is_collapsed {
            "chevron-right"
        } else {
            "chevron-down"
        },
        chevron_size,
    );
    chevron.style.flex_shrink_zero = true;
    chevron.style.descriptor.text_color = Some(colors.label);

    // Two codes on two channels: colour is status, length is position. At dash
    // size one mark cannot hold two colour codes legibly, so the current step
    // keeps the full length and every other step draws half.
    let current_value = spec.current_value().map(str::to_owned);
    let mut rail = Node::container();
    {
        let s = &mut rail.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rail_gap;
        s.flex_shrink_zero = true;
    }
    for step in &spec.steps {
        let is_current = current_value.as_deref() == Some(step.value.as_str());
        let segment_width = rem_to_px(spec.rail_segment_width_rem(is_current));
        let fill = match step.status {
            StepStatus::Complete => colors.accent,
            // The same hue as complete, dimmer: running is on its way to it.
            StepStatus::Running => with_alpha(colors.accent, colors.accent.3 * 0.55),
            StepStatus::Failed => colors.danger,
            StepStatus::Pending => rail_pending,
        };
        let mut segment = Node::container();
        {
            let s = &mut segment.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(segment_width);
            s.descriptor.layout.height = LayoutSizing::Fixed(segment_thickness);
            s.flex_shrink_zero = true;
            s.descriptor.background = Some(fill);
            let c = &mut s.descriptor.corner_radii;
            c.top_left = 999.0;
            c.top_right = 999.0;
            c.bottom_right = 999.0;
            c.bottom_left = 999.0;
        }
        rail = rail.child(segment);
    }

    let mut label = Node::text(current_label);
    label.style.text_size = Some(font_size);
    label.style.descriptor.text_color = Some(colors.active_label);
    label.style.descriptor.layout.width = LayoutSizing::Grow;
    label.style.flex_basis = Some(0.0);
    label.style.min_width = Some(0.0);

    // Trailing edge, so a stack of collapsed steppers right-aligns its counts
    // regardless of how long the labels are.
    let mut count = Node::text(format!("{completed}/{total}"));
    count.style.text_size = Some(font_size);
    count.style.descriptor.text_color = Some(count_color);
    count.style.flex_shrink_zero = true;

    summary.child(chevron).child(rail).child(label).child(count)
}
