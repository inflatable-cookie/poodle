use crate::app_state::AppState;
use crate::node_compat::{
    Button, Eyebrow, Icon, IconButton, IntoCompatNode, ListCard, ListGrid, Surface, Text,
};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_node::{ColorValue, CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, IconButtonSpec, IconSize, IconSpec, LeadingShape,
    ListCardSpec, ListGridSpec, ListGridVariant, PaddingScale, SurfaceBorder, SurfaceSpec,
    TextSpec, TextTone,
};

fn tile(
    theme: &poodle_gpui::GpuiThemeProvider,
    title: &str,
    blurb: &str,
    text_muted: ColorValue,
) -> Surface {
    let mut content = Node::container();
    content.style.descriptor.layout.direction = LayoutDirection::Column;
    content.style.descriptor.layout.spacing.gap = theme.resolve_space("space.stack.sm");
    let mut title_node = Node::text(title);
    title_node.style.text_size = Some(14.0);
    title_node.style.text_weight = Some(600);
    let mut blurb_node = Node::text(blurb);
    blurb_node.style.text_size = Some(12.0);
    blurb_node.style.descriptor.text_color = Some(text_muted);
    content = content.child(title_node).child(blurb_node);

    Surface::from_spec(
        SurfaceSpec::new()
            .with_padding(PaddingScale::Md)
            .with_border(SurfaceBorder::Subtle),
        theme,
    )
    .with_content(content)
}

fn action_row(theme: &poodle_gpui::GpuiThemeProvider, children: Vec<Node>) -> Node {
    let mut row = Node::container();
    row.style.descriptor.layout.direction = LayoutDirection::Row;
    row.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    row.style.descriptor.layout.spacing.gap = theme.resolve_space("space.inline.sm");
    children.into_iter().fold(row, Node::child)
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_muted_value = theme.resolve_color("color.text.muted");
    let text_muted = color_to_hsla(text_muted_value);
    let stack_lg = px(theme.resolve_space("space.stack.lg"));
    let stack_sm = px(theme.resolve_space("space.stack.sm"));

    let default_grid = ListGrid::from_spec(ListGridSpec::new().with_min_item_width_em(14.0), theme)
        .with_child(tile(
            theme,
            "Alpha",
            "Auto-fill columns from min width.",
            text_muted_value,
        ))
        .with_child(tile(
            theme,
            "Bravo",
            "Resize the preview to see wrapping.",
            text_muted_value,
        ))
        .with_child(tile(theme, "Charlie", "Third tile.", text_muted_value));

    let actions_row = action_row(
        theme,
        vec![
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Export"),
                theme,
            )
            .into_compat_node(),
            IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon("refresh-cw")
                    .with_aria_label("Refresh list")
                    .with_variant(ButtonVariant::Secondary),
                theme,
            )
            .into_compat_node(),
        ],
    );

    let with_actions = ListGrid::from_spec(ListGridSpec::new().with_min_item_width_em(16.0), theme)
        .with_header(actions_row)
        .with_child(tile(
            theme,
            "Project A",
            "With header actions row.",
            text_muted_value,
        ))
        .with_child(tile(theme, "Project B", "Second tile.", text_muted_value));

    let compact = ListGrid::from_spec(
        ListGridSpec::new().with_variant(ListGridVariant::Compact),
        theme,
    )
    .with_child(tile(
        theme,
        "One",
        "Compact single-column stack.",
        text_muted_value,
    ))
    .with_child(tile(theme, "Two", "Tighter default gap.", text_muted_value));

    // Grid of real ListCard tiles (contract: ListGrid hosts card collections).
    let card_tile = |title: &str, subtitle: &str, icon: &str| {
        ListCard::from_spec(
            ListCardSpec::new()
                .with_title(title)
                .with_subtitle(subtitle)
                .with_leading_shape(LeadingShape::RoundedSquare)
                .with_interactive(true),
            theme,
        )
        .with_leading(
            Icon::from_spec(IconSpec::new(icon).with_size(IconSize::Md), theme)
                .with_color(text_muted),
        )
    };

    let with_cards = ListGrid::from_spec(ListGridSpec::new().with_min_item_width_em(16.0), theme)
        .with_child(card_tile(
            "Design system",
            "Tokens, primitives, composites",
            "layers",
        ))
        .with_child(card_tile(
            "Media library",
            "Images, video, and audio",
            "image",
        ))
        .with_child(card_tile("Documentation", "Contracts and guides", "book"))
        .with_child(card_tile(
            "Analytics",
            "Usage and adoption metrics",
            "bar-chart",
        ));

    // Empty grid: header actions row with no items (contract: empty-state is
    // host-owned; the grid renders its header and an empty content region).
    let empty_grid = ListGrid::from_spec(ListGridSpec::new().with_min_item_width_em(16.0), theme)
        .with_header(action_row(
            theme,
            vec![Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("New item"),
                theme,
            )
            .into_compat_node()],
        ))
        .with_child({
            let mut empty = Node::container();
            empty.style.descriptor.layout.direction = LayoutDirection::Row;
            empty.style.fill_width = true;
            empty.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            empty.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            let padding = theme.resolve_space("space.stack.lg");
            empty.style.descriptor.layout.spacing.padding.top = padding;
            empty.style.descriptor.layout.spacing.padding.bottom = padding;
            empty.child(Text::node_from_spec(
                TextSpec::new("No items yet \u{2014} add one to populate the grid.")
                    .with_tone(TextTone::Secondary),
                theme,
            ))
        });

    div()
        .flex()
        .flex_col()
        .gap(stack_lg)
        .max_w(px(720.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Default — flex-wrap grid"),
                    theme,
                ))
                .child(default_grid),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With header actions"),
                    theme,
                ))
                .child(with_actions),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Compact — single column"),
                    theme,
                ))
                .child(compact),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("With ListCards"),
                    theme,
                ))
                .child(with_cards),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Empty — header with no items"),
                    theme,
                ))
                .child(empty_grid),
        )
}
