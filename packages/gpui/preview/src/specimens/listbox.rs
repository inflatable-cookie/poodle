use std::sync::Arc;

use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::listbox::{ListboxEffect, ListboxSelectionMode};
use poodle_node::{LayoutDirection, LayoutSizing, Node};
use poodle_render::{listbox_option_focus_id, listbox_with_rows, ListboxHandlers};
use poodle_specs::ListboxSpec;

const INSTANCE_ID: &str = "gpui-preview-listbox";

fn demo_spec(state: &AppState) -> ListboxSpec {
    let machine = &state.listbox.context;
    let mut spec = ListboxSpec::new(machine.items.clone())
        .with_selection_mode(machine.selection_mode)
        .with_orientation(machine.orientation)
        .with_aria_label("Sound library categories")
        .with_interaction_state(machine);
    if machine.selection_mode == ListboxSelectionMode::Multiple {
        spec = spec.with_values(machine.selected_values.clone());
    } else {
        spec = spec.with_value(machine.selected_values.first().cloned());
    }
    spec.with_disabled(machine.disabled)
}

fn row_content(
    item: &poodle_specs::ListboxItem,
    selected: bool,
    focused: bool,
    theme: &GpuiThemeProvider,
) -> Node {
    let primary = theme.resolve_color("color.text.primary");
    let secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");
    let surface = theme.resolve_color("color.background.elevated");
    let border = theme.resolve_color("color.border.subtle");

    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = poodle_node::CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = 12.0;
    row.style.descriptor.layout.spacing.padding.left = 12.0;
    row.style.descriptor.layout.spacing.padding.right = 12.0;
    row.style.descriptor.layout.spacing.padding.top = 8.0;
    row.style.descriptor.layout.spacing.padding.bottom = 8.0;
    row.style.descriptor.layout.height = LayoutSizing::Fixed(48.0);
    row.style.fill_width = true;
    row.style.min_width = Some(0.0);
    row.style.descriptor.border.width = 1.0;
    row.style.descriptor.border.color = if selected { accent } else { border };
    row.style.descriptor.corner_radii.top_left = 6.0;
    row.style.descriptor.corner_radii.top_right = 6.0;
    row.style.descriptor.corner_radii.bottom_left = 6.0;
    row.style.descriptor.corner_radii.bottom_right = 6.0;
    row.style.descriptor.background = if selected { Some(surface) } else { None };

    let mut marker = Node::text(if selected { "✓" } else { "•" });
    marker.style.text_size = Some(16.0);
    marker.style.descriptor.text_color = Some(if selected { accent } else { secondary });
    marker.style.flex_none = true;

    let mut text = Node::container();
    text.style.descriptor.layout.direction = LayoutDirection::Column;
    text.style.descriptor.layout.spacing.gap = 2.0;
    text.style.flex_fill = true;
    let mut title = Node::text(item.label.clone());
    title.style.text_size = Some(14.0);
    title.style.descriptor.text_color = Some(primary);
    let description = match item.value.as_str() {
        "ambient" => "Pads, textures and evolving beds",
        "drums" => "Acoustic and electronic grooves",
        "keys" => "Pianos, organs and synth layers",
        _ => "Unavailable in this collection",
    };
    let mut detail = Node::text(description);
    detail.style.text_size = Some(11.0);
    detail.style.descriptor.text_color = Some(secondary);
    text = text.child(title).child(detail);

    let mut posture = Node::text(if focused { "Focused" } else { "" });
    posture.style.text_size = Some(10.0);
    posture.style.descriptor.text_color = Some(secondary);
    row = row.child(marker).child(text).child(posture);
    row
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let _ = cx;
    let theme = &state.theme;
    let spec = demo_spec(state);
    let queue = Arc::clone(&state.node_events);
    let handlers = ListboxHandlers::new(INSTANCE_ID).on_transition(Arc::new(move |result| {
        for effect in &result.effects {
            if let ListboxEffect::Focus { value } = effect {
                poodle_gpui_node_backend::request_focus(&listbox_option_focus_id(
                    INSTANCE_ID,
                    value,
                ));
            }
        }
        queue
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Listbox(result));
    }));
    let node = listbox_with_rows(
        &spec,
        &crate::node_compat::preview_render_context(theme),
        &handlers,
        |item, selected, focused| row_content(item, selected, focused, theme),
    );
    let list = poodle_gpui_node_backend::to_gpui(&node);
    let selected = if state.listbox.context.selected_values.is_empty() {
        "None".to_string()
    } else {
        state.listbox.context.selected_values.join(", ")
    };
    let activation = state
        .listbox
        .last_activation
        .as_deref()
        .unwrap_or("None yet");

    let content = div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .w(px(420.0))
        .child(list)
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                .child(format!("Selected: {selected}")),
        )
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(theme.resolve_color("color.text.secondary")))
                .child(format!("Last activated: {activation}")),
        );

    specimen_layout(
        state,
        cx,
        "listbox",
        content.into_any_element(),
        SpecimenAxes::examples_only(),
    )
}
