use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, Eyebrow, IconButton, ListGrid, Surface};
use poodle_specs::{
    ButtonSpec, ButtonVariant, EyebrowSpec, IconButtonSpec, ListGridSpec, ListGridVariant,
    PaddingScale, SurfaceBorder, SurfaceSpec,
};

fn tile(theme: &poodle_gpui::GpuiThemeProvider, title: &str, blurb: &str, text_muted: Hsla) -> Surface {
    Surface::from_spec(
        SurfaceSpec::new()
            .with_padding(PaddingScale::Md)
            .with_border(SurfaceBorder::Subtle),
        theme,
    )
    .with_content(
        div()
            .flex()
            .flex_col()
            .gap(px(theme.resolve_space("space.stack.sm")))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(title.to_string()),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(text_muted)
                    .child(blurb.to_string()),
            ),
    )
}

pub(crate) fn render(state: &AppState, _cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_muted = color_to_hsla(theme.resolve_color("color.text.muted"));
    let stack_lg = px(theme.resolve_space("space.stack.lg"));
    let stack_sm = px(theme.resolve_space("space.stack.sm"));

    let default_grid = ListGrid::from_spec(
        ListGridSpec::new().with_min_item_width_em(14.0),
        theme,
    )
    .with_child(tile(
        theme,
        "Alpha",
        "Auto-fill columns from min width.",
        text_muted,
    ))
    .with_child(tile(
        theme,
        "Bravo",
        "Resize the preview to see wrapping.",
        text_muted,
    ))
    .with_child(tile(theme, "Charlie", "Third tile.", text_muted));

    let actions_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(theme.resolve_space("space.inline.sm")))
        .child(
            Button::from_spec(
                ButtonSpec::new()
                    .with_variant(ButtonVariant::Secondary)
                    .with_label("Export"),
                theme,
            ),
        )
        .child(
            IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon("refresh-cw")
                    .with_aria_label("Refresh list")
                    .with_variant(ButtonVariant::Secondary),
                theme,
            ),
        );

    let with_actions = ListGrid::from_spec(
        ListGridSpec::new().with_min_item_width_em(16.0),
        theme,
    )
    .with_header(actions_row)
    .with_child(tile(theme, "Project A", "With header actions row.", text_muted))
    .with_child(tile(theme, "Project B", "Second tile.", text_muted));

    let compact = ListGrid::from_spec(
        ListGridSpec::new().with_variant(ListGridVariant::Compact),
        theme,
    )
    .with_child(tile(
        theme,
        "One",
        "Compact single-column stack.",
        text_muted,
    ))
    .with_child(tile(theme, "Two", "Tighter default gap.", text_muted));

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
                .child(
                    Eyebrow::from_spec(
                        EyebrowSpec::new().with_content("Default — flex-wrap grid"),
                        theme,
                    ),
                )
                .child(default_grid),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(
                    Eyebrow::from_spec(
                        EyebrowSpec::new().with_content("With header actions"),
                        theme,
                    ),
                )
                .child(with_actions),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(stack_sm)
                .child(
                    Eyebrow::from_spec(
                        EyebrowSpec::new().with_content("Compact — single column"),
                        theme,
                    ),
                )
                .child(compact),
        )
}
