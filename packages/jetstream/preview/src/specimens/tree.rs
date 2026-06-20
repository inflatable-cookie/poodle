//! Tree specimen — hierarchical file-explorer disclosure.
//!
//! The first tree is interactive: it renders from `AppState.tree` and responds
//! to clicks (via `tree:` / `tree-twisty:` token routing in the shell) and to
//! keyboard navigation (handled in the main event loop). The remaining trees are
//! static scaling demos.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::*;
use poodle_jetstream_components::tree::js_tree;
use poodle_specs::{ControlDensity, ControlSize, TreeNode, TreeSpec};

use crate::app_state::{demo_file_tree, AppState};

fn expanded() -> Vec<String> {
    vec!["src".into(), "src/components".into()]
}

pub fn render(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "File explorer — ↑/↓ ←/→ Enter · F2 rename · right-click menu",
            secondary,
            js_tree(&state.tree_spec(), theme),
        ))
        .child(group(
            "Checkbox cascade — click a checkbox",
            secondary,
            js_tree(
                &TreeSpec::new(demo_file_tree())
                    .with_expanded_values(expanded())
                    .with_show_checkboxes(true)
                    .with_checked_values(state.tree.checked.clone()),
                theme,
            ),
        ))
        .child(group(
            "Lazy / async children (loading row)",
            secondary,
            js_tree(
                &TreeSpec::new(vec![TreeNode::new("remote", "remote")
                    .with_icon("folder")
                    .with_branch(true)])
                .with_expanded_values(vec!["remote".into()])
                .with_loading_values(vec!["remote".into()]),
                theme,
            ),
        ))
        .child(group(
            "No guides, no icons",
            secondary,
            js_tree(
                &TreeSpec::new(demo_file_tree())
                    .with_expanded_values(expanded())
                    .with_show_guides(false)
                    .with_show_icons(false),
                theme,
            ),
        ))
        .child(group(
            "Sizes (xs · sm · md · lg · xl)",
            secondary,
            size_row(theme),
        ))
        .child(group(
            "Densities (compact · default · comfortable)",
            secondary,
            density_row(theme),
        ))
        .when(state.tree.menu_value.is_some(), |el| {
            el.child(menu_overlay(state, theme))
        })
}

/// Floating context menu for the tree, anchored at the right-click position.
fn menu_overlay(state: &AppState, theme: &JetstreamThemeProvider) -> JsEl {
    let (x, y) = state.tree.menu_pos;
    let surface = resolve_color(theme, "color.background.elevated");
    let border = resolve_color(theme, "color.border.default");
    let text = resolve_color(theme, "color.text.primary");

    let item = |label_text: &str, token: &str| {
        div()
            .id(token.to_string())
            .flex_row()
            .items_center()
            .px(12.0)
            .py(6.0)
            .focusable()
            .cursor_pointer()
            .child(label(label_text).text_color(text).text_size(12.0))
    };

    div()
        .overlay()
        .absolute()
        .left(x)
        .top(y)
        .flex_col()
        .min_w(140.0)
        .bg(surface)
        .border(1.0)
        .border_color(border)
        .rounded(6.0)
        .child(item("Rename", "tree-menu:rename"))
        .child(item("Delete", "tree-menu:delete"))
}

fn size_row(theme: &JetstreamThemeProvider) -> JsEl {
    let mut row = div().flex_row().gap(16.0);
    for size in [
        ControlSize::Xs,
        ControlSize::Sm,
        ControlSize::Md,
        ControlSize::Lg,
        ControlSize::Xl,
    ] {
        row = row.child(js_tree(
            &TreeSpec::new(demo_file_tree())
                .with_expanded_values(expanded())
                .with_selected_values(vec!["src/components/Tree.svelte".into()])
                .with_size(size),
            theme,
        ));
    }
    row
}

fn density_row(theme: &JetstreamThemeProvider) -> JsEl {
    let mut row = div().flex_row().gap(16.0);
    for density in [
        ControlDensity::Compact,
        ControlDensity::Default,
        ControlDensity::Comfortable,
    ] {
        row = row.child(js_tree(
            &TreeSpec::new(demo_file_tree())
                .with_expanded_values(expanded())
                .with_selected_values(vec!["src/components/Tree.svelte".into()])
                .with_density(density),
            theme,
        ));
    }
    row
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
