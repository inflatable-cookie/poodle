use gpui::*;
use pug_adapter::ThemeProvider;
use pug_gpui_primitives::{BadgeSpec, BadgeVariant, CheckboxSpec};
use pug_gpui_components::{PugBadge, PugCheckbox};
use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("semantic.color.text.secondary");
    let sort_by = state.specimens.selected("picker-sort");

    let sorts = ["Name ↑", "Date", "Status"];
    let mut sort_row = div().flex().items_center().gap(px(6.0))
        .child(div().text_xs().text_color(color_to_hsla(text_secondary)).child("Sort by:"));

    for (i, label) in sorts.iter().enumerate() {
        let is_active = sort_by == i;

        let mut badge_spec = BadgeSpec::new()
            .with_variant(if is_active { BadgeVariant::Accent } else { BadgeVariant::Muted });
        badge_spec.content = Some(label.to_string());

        let btn = div()
            .id(SharedString::from(format!("sort-{}", i)))
            .cursor_pointer()
            .hover(|s| s.opacity(0.8))
            .child(PugBadge::new(badge_spec, theme))
            .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                this.state.specimens.select("picker-sort", i);
                cx.notify();
            }));
        sort_row = sort_row.child(btn);
    }

    let checked = state.specimens.is_on("picker-item");
    div().flex().flex_col().gap(px(6.0))
        .child(sort_row)
        .child(
            PugCheckbox::new(
                CheckboxSpec::new()
                    .with_checked(checked)
                    .with_label("Selected item"),
                theme,
            )
            .with_id("picker-item")
            .on_change(cx.listener(|this, _checked: &bool, _w, cx| {
                this.state.specimens.toggle("picker-item");
                cx.notify();
            }))
        )
}
