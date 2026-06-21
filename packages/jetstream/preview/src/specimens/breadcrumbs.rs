//! Breadcrumbs specimen — contract §12 specimen groups: Basic, Deep path,
//! Collapsed (`max_visible_items=3`), Size ladder (xs..xl), Density ladder.
//!
//! Crumbs render statically here; click navigation + the `href` vs callback
//! distinction live in the preview event loop (component renders crumbs only).

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::breadcrumbs_comp::js_breadcrumbs;
use poodle_jetstream_components::theme_ext::*;
use poodle_specs::{BreadcrumbItem, BreadcrumbsSpec, ControlDensity, ControlSize};

fn basic_items() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::new("home", "Home"),
        BreadcrumbItem::new("projects", "Projects"),
        BreadcrumbItem::new("poodle", "Poodle").with_is_current(true),
    ]
}

fn deep_items() -> Vec<BreadcrumbItem> {
    vec![
        BreadcrumbItem::new("home", "Home"),
        BreadcrumbItem::new("workspace", "Workspace"),
        BreadcrumbItem::new("projects", "Projects"),
        BreadcrumbItem::new("poodle", "Poodle Design System"),
        BreadcrumbItem::new("primitives", "Primitives"),
        BreadcrumbItem::new("button", "Button").with_is_current(true),
    ]
}

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        // Contract §12 Basic — link-style intermediates + non-link current.
        .child(group("Basic", secondary,
            js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()), theme)
        ))
        // Contract §12 Deep path — full six-item trail.
        .child(group("Deep path", secondary,
            js_breadcrumbs(&BreadcrumbsSpec::new(deep_items()), theme)
        ))
        // Contract §12 Collapsed — Home > … > Primitives > Button.
        .child(group("Collapsed (max 3 visible)", secondary,
            js_breadcrumbs(
                &BreadcrumbsSpec::new(deep_items()).with_max_visible_items(3),
                theme,
            )
        ))
        // Contract §12 Sizes — xs..xl font + gap ladder.
        .child(group("Sizes", secondary,
            div().flex_col().gap(12.0)
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_size(ControlSize::Xs), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_size(ControlSize::Sm), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_size(ControlSize::Md), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_size(ControlSize::Lg), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_size(ControlSize::Xl), theme))
        ))
        // Contract §12 Densities — gap ladder (compact/default/comfortable).
        .child(group("Densities", secondary,
            div().flex_col().gap(12.0)
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_density(ControlDensity::Compact), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_density(ControlDensity::Default), theme))
                .child(js_breadcrumbs(&BreadcrumbsSpec::new(basic_items()).with_density(ControlDensity::Comfortable), theme))
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
