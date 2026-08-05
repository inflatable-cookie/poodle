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
use crate::presentation::rem_to_px;

/// Host callbacks: change and rerun, each carrying the step's value.
#[derive(Default)]
pub struct StepperHandlers {
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_rerun: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn stepper(
    spec: &StepperSpec,
    theme: &dyn ThemeProvider,
    handlers: StepperHandlers,
) -> Node {
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

        // The marker carries index, glyph or spinner by status. Pending is
        // the only case that shows a number.
        let marker: Node = match step.status {
            StepStatus::Complete | StepStatus::Failed | StepStatus::Running => {
                let name = match step.status {
                    StepStatus::Complete => "check",
                    StepStatus::Failed => "x",
                    _ => "loader",
                };
                let mut m = Node::icon(name, marker_font_size);
                m.style.descriptor.text_color = Some(marker_color);
                m
            }
            StepStatus::Pending => {
                let mut m = Node::text(format!("{}", index + 1));
                m.style.text_size = Some(marker_font_size);
                m.style.text_weight = Some(700);
                m.style.descriptor.text_color = Some(marker_color);
                m
            }
        };

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
            let mut glyph = Node::icon("refresh-cw", marker_font_size);
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
