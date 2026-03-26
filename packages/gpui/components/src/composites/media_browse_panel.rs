//! MediaBrowsePanel — grid of selectable media items with loading/error/empty states.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::MediaBrowsePanelSpec;

use crate::theme_ext::{resolve_color, resolve_px};

pub struct MediaBrowsePanel {
    spec: MediaBrowsePanelSpec,
    theme: GpuiThemeProvider,
}

impl MediaBrowsePanel {
    pub fn from_spec(spec: MediaBrowsePanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone() }
    }
}

impl IntoElement for MediaBrowsePanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let body_size = resolve_px(theme, "semantic.typography.body.size");
        let label_size = resolve_px(theme, "semantic.typography.label.size");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let surface_bg = resolve_color(theme, "semantic.color.background.surface");
        let border_subtle = resolve_color(theme, "semantic.color.border.subtle");

        let mut panel = div().flex().flex_col().gap(px(8.0)).w_full();

        if spec.loading {
            panel = panel.child(
                div().text_size(body_size).text_color(text_secondary)
                    .child("Loading media...")
            );
        } else if let Some(ref error) = spec.error {
            panel = panel.child(
                div().text_size(body_size).text_color(resolve_color(theme, "semantic.color.status.danger"))
                    .child(error.clone())
            );
        } else if spec.items.is_empty() {
            panel = panel.child(
                div().text_size(body_size).text_color(text_secondary)
                    .child("No media items")
            );
        } else {
            let mut grid = div().flex().flex_wrap().gap(px(8.0));
            for item in &spec.items {
                grid = grid.child(
                    div()
                        .w(px(120.0)).h(px(80.0))
                        .rounded(px(6.0))
                        .border_1().border_color(Hsla { a: border_subtle.a * 0.5, ..border_subtle })
                        .bg(Hsla { a: surface_bg.a * 0.6, ..surface_bg })
                        .flex().flex_col().items_center().justify_center().gap(px(4.0))
                        .child(div().text_size(label_size).text_color(text_primary).child(item.label.clone()))
                        .child(div().text_size(px(10.0)).text_color(text_secondary).child(item.kind.clone()))
                );
            }
            panel = panel.child(grid);
        }

        panel.into_any_element()
    }
}
