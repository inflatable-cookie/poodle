use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, Popover};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{EyebrowSpec, OverlayPlacement, PopoverSpec, PopoverSurfaceWidth};
use std::sync::Arc;

/// One labelled popover example. The popover renders through the shared
/// poodle-render composition; the toggle handler delivers through the
/// preview's node-event queue (node handlers carry no window context, so the
/// queue is drained in the render loop), keeping the trigger flow
/// interactive.
fn popover_case(
    state: &AppState,
    theme: &GpuiThemeProvider,
    key: &'static str,
    spec: PopoverSpec,
    trigger_label: &str,
    content: Node,
) -> Popover {
    let is_open = state.specimens.is_on(key);
    let queue = std::sync::Arc::clone(&state.node_events);
    Popover::from_spec(spec.with_open(is_open), theme)
        .with_instance_id(key)
        .on_open_change(Arc::new(move |open| {
            queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
                key: key.to_owned(),
                value: open,
            });
        }))
        // The composition owns the interactive trigger (role button,
        // focusable); the specimen only supplies its label.
        .with_trigger(Node::text(trigger_label))
        .with_content(content)
}

fn labelled(theme: &GpuiThemeProvider, label: &str, body: Popover) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(body)
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_primary = theme.resolve_color("color.text.primary");

    // Heading + paragraph content block (matches the Svelte specimen body).
    let heading_paragraph = |heading: &str, body: &str| -> Node {
        let mut content = Node::container();
        content.style.descriptor.layout.direction = LayoutDirection::Column;
        content.style.descriptor.layout.spacing.gap = 4.0;
        let mut title = Node::text(heading);
        title.style.text_size = Some(14.0);
        title.style.text_weight = Some(700);
        title.style.descriptor.text_color = Some(text_primary);
        let mut paragraph = Node::text(body);
        paragraph.style.text_size = Some(12.0);
        paragraph.style.descriptor.text_color = Some(text_secondary);
        content.child(title).child(paragraph)
    };

    let paragraph = |body: &str| -> Node {
        let mut paragraph = Node::text(body);
        paragraph.style.text_size = Some(12.0);
        paragraph.style.descriptor.text_color = Some(text_secondary);
        paragraph
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default (bottom-start) — contract §13 ---
        .child(labelled(
            theme,
            "Default (bottom-start)",
            popover_case(
                state,
                theme,
                "popover-default",
                PopoverSpec::new().with_aria_label("Quick settings"),
                "Open popover",
                heading_paragraph(
                    "Quick settings",
                    "Adjust your display preferences or notification settings from this panel.",
                ),
            ),
        ))
        // --- Top placement — contract §13 ---
        .child(labelled(
            theme,
            "Top placement",
            popover_case(
                state,
                theme,
                "popover-top",
                PopoverSpec::new()
                    .with_placement(OverlayPlacement::Top)
                    .with_aria_label("Help tip"),
                "Show help",
                paragraph("Popovers can be anchored to any side of their trigger element."),
            ),
        ))
        // --- Placements (left / right) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Placement (left / right)"),
                    theme,
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(48.0))
                        .child(popover_case(
                            state,
                            theme,
                            "popover-left",
                            PopoverSpec::new()
                                .with_placement(OverlayPlacement::Left)
                                .with_aria_label("Left popover"),
                            "Left",
                            paragraph("Anchored to the left of the trigger."),
                        ))
                        .child(popover_case(
                            state,
                            theme,
                            "popover-right",
                            PopoverSpec::new()
                                .with_placement(OverlayPlacement::Right)
                                .with_aria_label("Right popover"),
                            "Right",
                            paragraph("Anchored to the right of the trigger."),
                        )),
                ),
        )
        // --- Surface width: trigger ---
        .child(labelled(
            theme,
            "Surface width: trigger",
            popover_case(
                state,
                theme,
                "popover-surface-trigger",
                PopoverSpec::new()
                    .with_surface_width(PopoverSurfaceWidth::Trigger)
                    .with_aria_label("Trigger-width popover"),
                "Match trigger width",
                paragraph("The surface stretches to the trigger's width."),
            ),
        ))
        // --- Surface width: fixed (min/max overrides) ---
        .child(labelled(
            theme,
            "Surface width: fixed",
            popover_case(
                state,
                theme,
                "popover-surface-fixed",
                PopoverSpec::new()
                    .with_surface_min_width(poodle_specs::Dimension::new("20rem"))
                    .with_surface_max_width(poodle_specs::Dimension::new("20rem"))
                    .with_aria_label("Fixed-width popover"),
                "Fixed 20rem",
                paragraph("Surface min-width and max-width are pinned to 20rem."),
            ),
        ))
        // --- Disabled (trigger cannot open) ---
        .child(labelled(
            theme,
            "Disabled",
            popover_case(
                state,
                theme,
                "popover-disabled",
                PopoverSpec::new()
                    .with_disabled(true)
                    .with_aria_label("Disabled popover"),
                "Disabled trigger",
                paragraph("A disabled trigger blocks opening."),
            ),
        ))
}
