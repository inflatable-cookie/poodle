use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Collapsible, Eyebrow};
use crate::specimens::specimen_layout::specimen_layout;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{LayoutDirection, Node};
use poodle_specs::{CollapsibleSpec, EyebrowSpec};
use std::sync::Arc;

fn toggle(state: &AppState, key: &'static str) -> Arc<dyn Fn(bool) + Send + Sync> {
    let events = state.node_events.clone();
    Arc::new(move |_| {
        events
            .lock()
            .unwrap()
            .push(NodeSpecimenEvent::Toggle(key.to_string()));
    })
}

fn content(lines: &[&str], color: poodle_node::ColorValue) -> Node {
    let mut root = Node::container();
    root.style.descriptor.layout.direction = LayoutDirection::Column;
    root.style.descriptor.layout.spacing.gap = 4.0;
    for line in lines {
        let mut text = Node::text(*line);
        text.style.text_size = Some(12.0);
        text.style.descriptor.text_color = Some(color);
        root = root.child(text);
    }
    root
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    // ── Group: Default (closed) ──────────────────────────────────────
    let closed_spec = CollapsibleSpec::new()
        .with_title("Project settings")
        .with_description("Configure build options and deploy targets.");

    let is_closed_open = state.specimens.is_on("collapsible-closed");
    let closed_spec = if is_closed_open {
        closed_spec.with_open(true)
    } else {
        closed_spec
    };

    let closed_collapsible = Collapsible::from_spec(closed_spec, theme)
        .with_id("specimen-closed")
        .on_toggle(toggle(state, "collapsible-closed"))
        .with_content(content(
            &[
                "Build target: production",
                "Output directory: dist/",
                "Source maps: enabled",
            ],
            text_secondary,
        ));

    // ── Group: Default open ──────────────────────────────────────────
    let open_toggled = state.specimens.is_on("collapsible-open-toggled");
    let open_spec = CollapsibleSpec::new()
        .with_title("Advanced options")
        .with_open(!open_toggled);

    let open_collapsible = Collapsible::from_spec(open_spec, theme)
        .with_id("specimen-open")
        .on_toggle(toggle(state, "collapsible-open-toggled"))
        .with_content(content(
            &["Cache TTL: 3600s", "Retry count: 3", "Timeout: 30s"],
            text_secondary,
        ));

    // ── Group: Disabled ──────────────────────────────────────────────
    let disabled_spec = CollapsibleSpec::new()
        .with_title("Locked section")
        .with_description("Requires admin access.")
        .with_disabled(true);

    let disabled_collapsible = Collapsible::from_spec(disabled_spec, theme)
        .with_id("specimen-disabled")
        .with_content(content(
            &["This content is hidden behind a disabled collapsible."],
            text_secondary,
        ));

    // ── Group: Highlighted ───────────────────────────────────────────
    // highlighted=true applies the accent border + halo from the spec's
    // highlight tokens. Open by default so the highlighted container reads.
    let highlighted_toggled = state.specimens.is_on("collapsible-highlighted-toggled");
    let highlighted_spec = CollapsibleSpec::new()
        .with_title("Focused section")
        .with_highlighted(true)
        .with_open(!highlighted_toggled);

    let highlighted_collapsible = Collapsible::from_spec(highlighted_spec, theme)
        .with_id("specimen-highlighted")
        .on_toggle(toggle(state, "collapsible-highlighted-toggled"))
        .with_content(content(
            &["Highlighted collapsibles draw attention to a matched or focused section."],
            text_secondary,
        ));

    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Default (closed) ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default (closed)"),
                    theme,
                ))
                .child(closed_collapsible),
        )
        // --- Default open ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default open"),
                    theme,
                ))
                .child(open_collapsible),
        )
        // --- Disabled ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Disabled"),
                    theme,
                ))
                .child(disabled_collapsible),
        )
        // --- Highlighted ---
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Highlighted"),
                    theme,
                ))
                .child(highlighted_collapsible),
        )
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "collapsible",
        examples,
        |size, theme: &GpuiThemeProvider| {
            Collapsible::from_spec(CollapsibleSpec::new().with_title("Section"), theme)
                .with_id(format!("specimen-size-{:?}", size))
                .size(size)
                .into_any_element()
        },
        |density, theme: &GpuiThemeProvider| {
            Collapsible::from_spec(CollapsibleSpec::new().with_title("Section"), theme)
                .with_id(format!("specimen-density-{:?}", density))
                .with_density(density)
                .into_any_element()
        },
    )
}
