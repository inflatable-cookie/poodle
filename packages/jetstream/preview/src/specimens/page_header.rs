//! PageHeader specimen — page-level title and action region.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::button::js_button;
use poodle_jetstream_components::page_header::{js_page_header, js_page_header_with_slots};
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{ButtonSpec, ButtonVariant, ControlSize, PageHeaderSpec, StatusTone};

fn actions(theme: &JetstreamThemeProvider) -> JsEl {
    div()
        .flex_row()
        .items_center()
        .gap(6.0)
        .child(js_button(
            &ButtonSpec::new()
                .with_label("View source")
                .with_variant(ButtonVariant::Secondary)
                .with_size(ControlSize::Sm),
            theme,
        ))
        .child(js_button(
            &ButtonSpec::new()
                .with_label("Edit")
                .with_size(ControlSize::Sm),
            theme,
        ))
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Basic (title + subtitle)",
            secondary,
            js_page_header(
                &PageHeaderSpec::new("Components")
                    .with_subtitle("Browse and manage your component library."),
                theme,
            ),
        ))
        .child(group(
            "Eyebrow + count + actions",
            secondary,
            js_page_header_with_slots(
                &PageHeaderSpec::new("Button")
                    .with_eyebrow("Primitive")
                    .with_count(12)
                    .with_subtitle("Primary interactive control for triggering actions."),
                theme,
                None,
                Some(actions(theme)),
                None,
            ),
        ))
        .child(group(
            "Section + back link + banner",
            secondary,
            js_page_header(
                &PageHeaderSpec::new("Media Library")
                    .with_section("Workspace")
                    .with_back("/workspace", "Back to Workspace")
                    .with_back_is_contextual(true)
                    .with_banner("You have unsynced changes.", StatusTone::Warning),
                theme,
            ),
        ))
        .child(group(
            "Title only",
            secondary,
            js_page_header(&PageHeaderSpec::new("Settings"), theme),
        ))
        .child(group(
            "Sizes (xs–xl)",
            secondary,
            div().flex_col().gap(12.0).children(
                [
                    ControlSize::Xs,
                    ControlSize::Sm,
                    ControlSize::Md,
                    ControlSize::Lg,
                    ControlSize::Xl,
                ]
                .into_iter()
                .map(|size| {
                    js_page_header(
                        &PageHeaderSpec::new("Media Library")
                            .with_subtitle("Sized header sample.")
                            .with_size(size),
                        theme,
                    )
                }),
            ),
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
