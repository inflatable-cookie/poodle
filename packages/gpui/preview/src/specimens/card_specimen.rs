//! Card specimen — g12.019 node-tier migration.
//!
//! Card paint and slot layout render through `poodle_render::card` and the
//! GPUI node backend. The specimen keeps only its outer gallery layout in
//! direct GPUI elements.

use crate::app_state::AppState;
use crate::node_compat::Eyebrow;
use crate::specimens::specimen_axes::density_key;
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{ColorValue, LayoutDirection, LayoutSizing, Node};
use poodle_render::presentation::rem_to_px;
use poodle_specs::{CardLayout, CardSpec, CardVariant, ControlDensity, EyebrowSpec};

fn text(content: &str, size: f32, color: ColorValue, weight: Option<u16>) -> Node {
    let mut node = Node::text(content);
    node.style.text_size = Some(size);
    node.style.text_weight = weight;
    node.style.descriptor.text_color = Some(color);
    node
}

fn header_slot(content: Node) -> Node {
    let mut slot = Node::container();
    slot.style.descriptor.layout.direction = LayoutDirection::Row;
    slot.style.flex_shrink_zero = true;
    slot.child(content)
}

fn body_slot(content: Node) -> Node {
    let mut slot = Node::container();
    slot.style.descriptor.layout.direction = LayoutDirection::Row;
    slot.style.flex_grow = Some(1.0);
    slot.child(content)
}

fn footer_slot(spec: &CardSpec, theme: &GpuiThemeProvider, content: Node) -> Node {
    let subtle = theme.resolve_color("color.border.subtle");
    let mut slot = Node::container();
    slot.style.descriptor.layout.direction = LayoutDirection::Row;
    slot.style.flex_shrink_zero = true;
    slot.style.border_top_width = Some(1.0);
    slot.style.border_color_top = Some(ColorValue(subtle.0, subtle.1, subtle.2, subtle.3 * 0.52));
    slot.style.descriptor.layout.spacing.padding.top = rem_to_px(spec.footer_padding_top_rem());
    slot.child(content)
}

fn node_card(
    id: &str,
    spec: CardSpec,
    theme: &GpuiThemeProvider,
    media: Option<Node>,
    header: Option<Node>,
    body: Option<Node>,
    footer: Option<Node>,
) -> AnyElement {
    let mut children = Vec::new();
    if let Some(media) = media {
        children.push(media);
    }
    if let Some(header) = header {
        children.push(header_slot(header));
    }
    if let Some(body) = body {
        children.push(body_slot(body));
    }
    if let Some(footer) = footer {
        children.push(footer_slot(&spec, theme, footer));
    }
    let mut node = poodle_render::card(&spec, theme, children);
    node.id = Some(id.to_string());
    poodle_gpui_node_backend::to_gpui(&node)
}

fn media_block(width: Option<f32>, height: f32, color: ColorValue) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Row;
    if let Some(width) = width {
        node.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    } else {
        node.style.fill_width = true;
    }
    node.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    node.style.descriptor.background = Some(color);
    node
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let primary = theme.resolve_color("color.text.primary");
    let secondary = theme.resolve_color("color.text.secondary");
    let accent = theme.resolve_color("color.accent.base");

    let title = |content: &str| text(content, 16.0, primary, Some(600));
    let body = |content: &str| text(content, 14.0, secondary, None);
    let meta = |content: &str| text(content, 12.0, secondary, None);
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            theme,
            "Default variant",
            div()
                .flex()
                .gap(px(16.0))
                .flex_wrap()
                .child(div().w(px(280.0)).child(node_card(
                    "card-default-1",
                    CardSpec::new().with_aria_label("Project card"),
                    theme,
                    None,
                    Some(title("Project Alpha")),
                    Some(body(
                        "A design system component library for building consistent interfaces.",
                    )),
                    Some(meta("Updated 2 days ago")),
                )))
                .child(div().w(px(280.0)).child(node_card(
                    "card-default-2",
                    CardSpec::new().with_aria_label("Stats card"),
                    theme,
                    None,
                    Some(title("Monthly report")),
                    Some(body("48 components shipped across 3 packages this month.")),
                    None,
                ))),
        ))
        .child(group(
            theme,
            "Outlined variant",
            div().w(px(280.0)).child(node_card(
                "card-outlined",
                CardSpec::new()
                    .with_variant(CardVariant::Outlined)
                    .with_aria_label("Outlined card"),
                theme,
                None,
                Some(title("Outlined card")),
                Some(body("This card uses a subtle border instead of elevation.")),
                None,
            )),
        ))
        .child(group(
            theme,
            "Elevated variant",
            div().w(px(280.0)).child(node_card(
                "card-elevated",
                CardSpec::new()
                    .with_variant(CardVariant::Elevated)
                    .with_aria_label("Elevated card"),
                theme,
                None,
                Some(title("Elevated card")),
                Some(body("This card uses a drop shadow for visual prominence.")),
                None,
            )),
        ))
        .child(group(
            theme,
            "Selected",
            div().w(px(280.0)).child(node_card(
                "card-selected",
                CardSpec::new().selected().with_aria_label("Selected card"),
                theme,
                None,
                Some(title("Selected card")),
                Some(body(
                    "Selected cards carry an accent border and accent ring.",
                )),
                None,
            )),
        ))
        .child(group(
            theme,
            "Interactive",
            div().w(px(280.0)).child(node_card(
                "card-interactive",
                CardSpec::new()
                    .interactive()
                    .with_aria_label("Clickable card"),
                theme,
                None,
                Some(title("Interactive card")),
                Some(body(
                    "Hover to see the interactive state. Cursor changes to pointer.",
                )),
                None,
            )),
        ))
        .child(group(
            theme,
            "Media slot",
            div().w(px(280.0)).child(node_card(
                "card-media",
                CardSpec::new()
                    .with_media(true)
                    .with_aria_label("Media card"),
                theme,
                Some(media_block(None, 120.0, accent)),
                Some(title("Media card")),
                Some(body(
                    "The media region is overflow-clipped with an inset radius.",
                )),
                None,
            )),
        ))
        .child(group(
            theme,
            "Horizontal layout",
            div().w(px(400.0)).child(node_card(
                "card-horizontal",
                CardSpec::new()
                    .with_layout(CardLayout::Horizontal)
                    .with_media(true)
                    .with_aria_label("Horizontal card"),
                theme,
                Some(media_block(Some(96.0), 96.0, accent)),
                None,
                Some({
                    let mut content = Node::container();
                    content.style.descriptor.layout.direction = LayoutDirection::Column;
                    content.style.descriptor.layout.spacing.gap = 4.0;
                    content.child(title("Horizontal card")).child(body(
                        "Media occupies the leading column; content fills the rest.",
                    ))
                }),
                None,
            )),
        ))
        .child(group(
            theme,
            "Compact layout",
            div().w(px(280.0)).child(node_card(
                "card-compact-layout",
                CardSpec::new()
                    .with_layout(CardLayout::Compact)
                    .with_aria_label("Compact card"),
                theme,
                None,
                Some(title("Compact layout")),
                Some(body("Reduced padding and gap via the compact layout.")),
                None,
            )),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "card",
        examples,
        SpecimenAxes::examples_only().with_densities(|density, theme: &GpuiThemeProvider| {
            density_card(theme, density).into_any_element()
        }),
    )
}

fn density_card(theme: &GpuiThemeProvider, density: ControlDensity) -> Div {
    let label = density_key(density);
    let primary = theme.resolve_color("color.text.primary");
    let secondary = theme.resolve_color("color.text.secondary");
    let id = format!("card-density-{label}");
    div().w(px(240.0)).child(node_card(
        &id,
        CardSpec::new()
            .with_density(density)
            .with_aria_label(format!("{label} density card")),
        theme,
        None,
        Some(text(label, 16.0, primary, Some(600))),
        Some(text(
            "A design system component library for building consistent interfaces.",
            14.0,
            secondary,
            None,
        )),
        Some(text("Updated 2 days ago", 12.0, secondary, None)),
    ))
}

fn group(theme: &GpuiThemeProvider, label: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(content)
}
