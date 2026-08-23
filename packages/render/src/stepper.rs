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

use poodle_node::{
    ColorValue, CrossAxisAlignment, CursorHint, FocusRing, LayoutDirection, LayoutSizing,
    MainAxisAlignment, Node, NodeRole, StylePatch,
};
use poodle_specs::{Orientation, StepStatus, StepperSpec};

use crate::color::with_alpha;
use crate::context::RenderContext;
use crate::presentation::{rem_to_px, resolve_supporting_visual_size, size_font_rem};

/// The contracted focus ring (§4/§8): `border-width-focus` of
/// `accent-focusRing`. The trigger and rerun draw it 2px (0.125rem) outside
/// the border box; the summary draws it inset. Declaring it is also what
/// gives each control a tracked native focus handle, so keyboard entry no
/// longer depends on a prior pointer press.
fn stepper_focus_ring(ctx: &RenderContext<'_>, offset: f32) -> FocusRing {
    FocusRing {
        color: ctx.theme().resolve_color("color.accent.focusRing"),
        width: ctx.theme().resolve_border_width("border.width.focus"),
        offset,
    }
}

/// Host callbacks: change and rerun, each carrying the step's value, plus the
/// summary's collapse toggle carrying the new state.
#[derive(Default)]
pub struct StepperHandlers {
    pub on_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_rerun: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub on_collapsed_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn stepper(spec: &StepperSpec, ctx: &RenderContext<'_>, handlers: StepperHandlers) -> Node {
    // The spec's size ladders apply the size role internally, so they take
    // the context-resolved BASE size; the density ladders take the resolved
    // density.
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let row_height = rem_to_px(spec.row_height_rem(base_size));
    let marker_size = rem_to_px(spec.marker_size_rem(base_size));
    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let marker_font_size = rem_to_px(spec.marker_font_size_rem(base_size));
    let pad_y = rem_to_px(spec.padding_block_rem(base_size));
    let pad_x = rem_to_px(spec.padding_inline_rem(density));
    let gap = rem_to_px(spec.gap_rem(density));
    let radius = ctx.theme().resolve_radius(spec.radius_token());
    // Contract §8: a hairline divider, stated as an absolute.
    let hairline = rem_to_px(0.0625);

    let border = ctx.theme().resolve_color(spec.border_token());
    let panel = ctx.theme().resolve_color(spec.surface_token());
    let label_color = ctx.theme().resolve_color(spec.label_token());
    let active_label = ctx.theme().resolve_color(spec.active_label_token());
    let accent = ctx.theme().resolve_color(spec.accent_token());
    let danger = ctx.theme().resolve_color(spec.danger_token());
    let disabled_opacity = ctx.theme().resolve_opacity(spec.disabled_opacity_token());

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
            ctx,
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
        // A stable identity per step value, like the summary's: the focus
        // ring makes the backend track a real handle under this id, which is
        // what lets keyboard entry reach the control without a pointer press.
        trigger.id = Some(format!("poodle-stepper:trigger:{}", step.value));
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
            // Contract §8: the trigger's focus ring is cornered at
            // `radius-control` so the first and last steps' rings do not
            // render square inside the track's rounded corners. The resting
            // background is transparent, so the radius is invisible until the
            // ring draws.
            let ring_radius = ctx.theme().resolve_radius("radius.control");
            let c = &mut s.descriptor.corner_radii;
            c.top_left = ring_radius;
            c.top_right = ring_radius;
            c.bottom_right = ring_radius;
            c.bottom_left = ring_radius;
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
            // Contract §6: Tab moves between the trigger and its rerun
            // control — each is a sequential focus stop (the web buttons'
            // implicit tabindex=0).
            trigger.a11y.tab_index = Some(0);
            // Contract §8 trigger focus ring: `border-width-focus` of
            // `accent-focusRing`, 2px (0.125rem) outside the border box.
            trigger.style.focus_ring = Some(stepper_focus_ring(ctx, rem_to_px(0.125)));
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
            rerun.id = Some(format!("poodle-stepper:rerun:{}", step.value));
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
                rerun.a11y.tab_index = Some(0);
                // Contract §4: keyboard focus on the rerun control draws the
                // same focus ring the trigger draws.
                rerun.style.focus_ring = Some(stepper_focus_ring(ctx, rem_to_px(0.125)));
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
    ctx: &RenderContext<'_>,
    handlers: &StepperHandlers,
    colors: SummaryColors,
    hairline: f32,
    disabled_opacity: f32,
) -> Node {
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let row_height = rem_to_px(spec.row_height_rem(base_size));
    let font_size = rem_to_px(spec.font_size_rem(base_size));
    let pad_y = rem_to_px(spec.padding_block_rem(base_size));
    let pad_x = rem_to_px(spec.padding_inline_rem(density));
    let gap = rem_to_px(spec.gap_rem(density));
    let rail_gap = rem_to_px(spec.rail_gap_rem(density));
    let segment_thickness = rem_to_px(spec.rail_thickness_rem(base_size));
    let chevron_size = rem_to_px(size_font_rem(resolve_supporting_visual_size(
        spec.resolved_size(base_size),
    )));
    let count_color = ctx.theme().resolve_color(spec.count_token());
    let rail_pending = ctx.theme().resolve_color(spec.rail_pending_token());

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
        // Contract §8: the summary's resting `radius-control` rounds the
        // full-width row so the hover fill and the inset focus ring do not
        // render square inside the track's rounded corners.
        let row_radius = ctx.theme().resolve_radius("radius.control");
        let c = &mut s.descriptor.corner_radii;
        c.top_left = row_radius;
        c.top_right = row_radius;
        c.bottom_right = row_radius;
        c.bottom_left = row_radius;
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
        // Contract §6: the summary is the FIRST stop when collapsible.
        summary.a11y.tab_index = Some(0);
        // Contract §8 summary focus ring: the same ring, drawn 2px
        // (-0.125rem) INSIDE the border box — the row spans the track edge
        // to edge, so an outset ring would clip against the track.
        summary.style.focus_ring = Some(stepper_focus_ring(ctx, rem_to_px(-0.125)));
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
        let segment_width = rem_to_px(spec.rail_segment_width_rem(base_size, is_current));
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use poodle_adapter::ThemeProvider;
    use poodle_specs::StepperStep;

    use super::*;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    /// The Soundcheck arrangement, with one completed step carrying a rerun
    /// and one step disabled.
    fn steps() -> Vec<StepperStep> {
        vec![
            StepperStep::new("state", "Current state").with_status(StepStatus::Complete),
            StepperStep::new("categories", "Categories"),
            StepperStep::new("apply", "Apply and verify").with_disabled(true),
        ]
    }

    /// A recorder that names which callback fired and with what, so the three
    /// actions cannot be mistaken for each other.
    #[derive(Default)]
    struct Emissions {
        changes: Arc<Mutex<Vec<String>>>,
        reruns: Arc<Mutex<Vec<String>>>,
        collapses: Arc<Mutex<Vec<bool>>>,
    }

    impl Emissions {
        fn handlers(&self) -> StepperHandlers {
            let changes = Arc::clone(&self.changes);
            let reruns = Arc::clone(&self.reruns);
            let collapses = Arc::clone(&self.collapses);
            StepperHandlers {
                on_change: Some(Arc::new(move |value: &str| {
                    changes.lock().unwrap().push(value.to_string())
                })),
                on_rerun: Some(Arc::new(move |value: &str| {
                    reruns.lock().unwrap().push(value.to_string())
                })),
                on_collapsed_change: Some(Arc::new(move |collapsed: bool| {
                    collapses.lock().unwrap().push(collapsed)
                })),
            }
        }

        fn changes(&self) -> Vec<String> {
            self.changes.lock().unwrap().clone()
        }

        fn reruns(&self) -> Vec<String> {
            self.reruns.lock().unwrap().clone()
        }

        fn collapses(&self) -> Vec<bool> {
            self.collapses.lock().unwrap().clone()
        }
    }

    /// The step cells, in order: the list items directly under the root.
    fn cells(root: &Node) -> Vec<&Node> {
        root.children
            .iter()
            .filter(|child| child.a11y.role == Some(NodeRole::ListItem))
            .collect()
    }

    /// A cell's trigger and its rerun control, which are deliberately
    /// different nodes (contract §2).
    fn trigger(cell: &Node) -> &Node {
        &cell.children[0]
    }

    fn rerun(cell: &Node) -> Option<&Node> {
        cell.children.get(1)
    }

    /// g15.052: the three contracted controls declare the focus ring and a
    /// stable identity — the declaration is what makes the backend track a
    /// real focus handle, so keyboard entry no longer needs a pointer press.
    /// A disabled step is unfocusable and declares no ring, matching the web.
    #[test]
    fn the_contracted_controls_declare_the_focus_ring_and_stable_identities() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let ring_color = theme.resolve_color("color.accent.focusRing");
        let root = stepper(
            &StepperSpec::new(steps())
                .with_value("categories")
                .with_show_rerun(true),
            &ctx,
            StepperHandlers::default(),
        );
        let cells = cells(&root);

        let first_trigger = trigger(cells[0]);
        let ring = first_trigger.style.focus_ring.expect("the trigger declares a ring");
        assert_eq!(ring.color, ring_color);
        assert_eq!(ring.width, 2.0);
        assert_eq!(ring.offset, 2.0, "the trigger ring draws 2px outside");
        assert_eq!(
            first_trigger.id.as_deref(),
            Some("poodle-stepper:trigger:state"),
            "a stable focus identity keyed by the step value",
        );

        let rerun = rerun(cells[0]).expect("a completed step carries the rerun control");
        let ring = rerun.style.focus_ring.expect("the rerun control declares a ring");
        assert_eq!(ring.color, ring_color);
        assert_eq!(ring.offset, 2.0);
        assert_eq!(rerun.id.as_deref(), Some("poodle-stepper:rerun:state"));

        let disabled = trigger(cells[2]);
        assert!(
            disabled.style.focus_ring.is_none(),
            "a disabled step is unfocusable and declares no ring",
        );

        // The summary's ring is INSET (contract §8: -0.125rem) — the row
        // spans the track edge to edge, so an outset ring would clip.
        let collapsible = stepper(
            &StepperSpec::new(steps())
                .with_orientation(Orientation::Vertical)
                .with_collapsible(true)
                .with_value("categories"),
            &ctx,
            StepperHandlers::default(),
        );
        let summary = collapsible
            .find(&|node| node.id.as_deref() == Some("poodle-stepper-summary"))
            .expect("a collapsible vertical stepper has a summary row");
        let ring = summary.style.focus_ring.expect("the summary declares a ring");
        assert_eq!(ring.color, ring_color);
        assert_eq!(ring.width, 2.0);
        assert_eq!(ring.offset, -2.0);
    }

    #[test]
    fn an_enabled_trigger_emits_its_own_value_and_a_disabled_one_emits_nothing() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let emissions = Emissions::default();
        let root = stepper(
            &StepperSpec::new(steps()).with_value("categories"),
            &ctx,
            emissions.handlers(),
        );
        let cells = cells(&root);
        assert_eq!(cells.len(), 3);

        trigger(cells[0]).interaction.on_activate.as_ref().unwrap()();
        trigger(cells[1]).interaction.on_activate.as_ref().unwrap()();
        assert_eq!(emissions.changes(), ["state", "categories"]);

        let disabled = trigger(cells[2]);
        assert!(disabled.interaction.disabled);
        assert!(
            disabled.interaction.on_activate.is_none(),
            "a disabled step carries no activation at all — suppression is not \
             a guard inside the handler",
        );
        assert_eq!(emissions.changes().len(), 2);
    }

    #[test]
    fn rerun_is_a_separate_control_that_never_selects_its_step() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let emissions = Emissions::default();
        let root = stepper(
            &StepperSpec::new(steps())
                .with_value("categories")
                .with_show_rerun(true),
            &ctx,
            emissions.handlers(),
        );
        let cells = cells(&root);

        let rerun = rerun(cells[0]).expect("a completed step carries the rerun control");
        assert!(
            !std::ptr::eq(rerun, trigger(cells[0])),
            "rerun and trigger are different nodes",
        );
        rerun.interaction.on_activate.as_ref().unwrap()();
        assert_eq!(emissions.reruns(), ["state"]);
        assert!(
            emissions.changes().is_empty(),
            "re-running a finished step must not also navigate to it",
        );

        assert_eq!(
            rerun.a11y.label.as_deref(),
            Some("Re-run step: Current state"),
            "each rerun names its own step, not a row of identical buttons",
        );
    }

    #[test]
    fn rerun_appears_only_for_a_completed_step_that_asked_for_it() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let without = stepper(
            &StepperSpec::new(steps()).with_value("categories"),
            &ctx,
            StepperHandlers::default(),
        );
        for cell in cells(&without) {
            assert!(
                rerun(cell).is_none(),
                "no rerun control until the host shows it",
            );
        }

        let with = stepper(
            &StepperSpec::new(steps())
                .with_value("categories")
                .with_show_rerun(true),
            &ctx,
            StepperHandlers::default(),
        );
        let cells = cells(&with);
        assert!(rerun(cells[0]).is_some(), "complete");
        assert!(rerun(cells[1]).is_none(), "pending");
        assert!(rerun(cells[2]).is_none(), "pending and disabled");
    }

    #[test]
    fn an_unwired_rerun_still_swallows_its_activation() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let emissions = Emissions::default();
        let mut handlers = emissions.handlers();
        handlers.on_rerun = None;
        let root = stepper(
            &StepperSpec::new(steps())
                .with_value("categories")
                .with_show_rerun(true),
            &ctx,
            handlers,
        );
        let cells = cells(&root);

        let rerun = rerun(cells[0]).expect("the control is still painted");
        rerun.interaction.on_activate.as_ref().unwrap()();
        assert!(
            emissions.changes().is_empty(),
            "an unwired rerun stays inert rather than bubbling into selection",
        );
        assert!(emissions.reruns().is_empty());
    }

    #[test]
    fn collapse_is_vertical_only_and_independent_of_the_other_two() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let emissions = Emissions::default();
        let collapsed = stepper(
            &StepperSpec::new(steps())
                .with_orientation(Orientation::Vertical)
                .with_collapsible(true)
                .with_collapsed(true)
                .with_value("categories"),
            &ctx,
            emissions.handlers(),
        );
        let summary = collapsed
            .find(&|node| node.id.as_deref() == Some("poodle-stepper-summary"))
            .expect("a collapsible vertical stepper has a summary row");
        summary.interaction.on_activate.as_ref().unwrap()();
        assert_eq!(emissions.collapses(), [false], "the toggle carries the next state");
        assert!(emissions.changes().is_empty());
        assert!(emissions.reruns().is_empty());
        assert!(
            cells(&collapsed).is_empty(),
            "collapsed omits the step rows rather than hiding them",
        );

        let horizontal = stepper(
            &StepperSpec::new(steps())
                .with_collapsible(true)
                .with_collapsed(true)
                .with_value("categories"),
            &ctx,
            StepperHandlers::default(),
        );
        assert!(
            horizontal
                .find(&|node| node.id.as_deref() == Some("poodle-stepper-summary"))
                .is_none(),
            "collapse is ignored in horizontal orientation",
        );
        assert_eq!(cells(&horizontal).len(), 3, "the full track still renders");
    }
}
