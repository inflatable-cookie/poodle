//! MediaBrowsePanel — grid of selectable media items with loading/error/empty states.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MediaBrowsePanelSpec;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::theme_ext::{resolve_color, resolve_px};

pub struct MediaBrowsePanel {
    spec: MediaBrowsePanelSpec,
    theme: GpuiThemeProvider,
}

impl MediaBrowsePanel {
    pub fn from_spec(spec: MediaBrowsePanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for MediaBrowsePanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_size = px(rem_to_px(size_font_rem(effective_size)));
        let grid_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.375,
            ControlDensity::Default => control_space_x_rem(spec.density),
            ControlDensity::Comfortable => 0.75,
        }));
        let item_gap = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.25,
            ControlDensity::Default => 0.375,
            ControlDensity::Comfortable => 0.5,
        }));
        let item_pad = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 0.75,
            ControlDensity::Comfortable => 0.875,
        }));
        let grid_min_w = px(rem_to_px(match effective_size {
            ControlSize::Xs => 8.5,
            ControlSize::Sm => 10.0,
            ControlSize::Md => 11.0,
            ControlSize::Lg => 12.5,
            ControlSize::Xl => 14.0,
        }));
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface_bg = resolve_color(theme, "color.background.panel");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_focus = resolve_color(theme, "color.accent.focusRing");
        let callout_danger = resolve_color(theme, "color.status.danger");
        let radius = resolve_px(theme, "radius.surface");
        let control_radius = resolve_px(theme, "radius.control");

        let mut panel = div().flex().flex_col().w_full().min_h(px(rem_to_px(18.0)));

        if spec.loading && spec.items.is_empty() {
            panel = panel.child(
                div()
                    .min_h(px(rem_to_px(18.0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child("Loading media..."),
            );
        } else if let Some(ref error) = spec.error {
            panel = panel.child(
                div()
                    .min_h(px(rem_to_px(18.0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(body_size)
                    .text_color(callout_danger)
                    .child(error.clone()),
            );
        } else if spec.items.is_empty() {
            panel = panel.child(
                div()
                    .min_h(px(rem_to_px(18.0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .child(spec.empty_message.clone()),
            );
        } else {
            let mut grid = div().flex().flex_wrap().gap(grid_gap).w_full();
            for item in &spec.items {
                grid =
                    grid.child(
                        div()
                            .min_w(grid_min_w)
                            .flex_grow()
                            .rounded(radius)
                            .border_1()
                            .border_color(Hsla {
                                a: border_subtle.a * 0.5,
                                ..border_subtle
                            })
                            .bg(Hsla {
                                a: surface_bg.a * 0.92,
                                ..surface_bg
                            })
                            .flex()
                            .flex_col()
                            .gap(item_gap)
                            .p(item_pad)
                            .child(div().min_h(px(rem_to_px(6.0))).rounded(control_radius).bg(
                                Hsla {
                                    a: elevated_bg.a * 0.72,
                                    ..elevated_bg
                                },
                            ))
                            .child(
                                div()
                                    .text_size(body_size)
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(text_primary)
                                    .child(item.label.clone()),
                            )
                            .child(
                                div()
                                    .text_size(resolve_px(theme, "typography.label.size"))
                                    .text_color(text_secondary)
                                    .child(item.meta.clone().unwrap_or_else(|| item.kind.clone())),
                            ),
                    );
            }
            panel = panel.child(grid);

            if spec.has_more {
                let label = if spec.loading {
                    "Loading..."
                } else {
                    spec.load_more_label.as_str()
                };

                panel = panel.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .mt(grid_gap)
                        .child(
                            div()
                                .px(px(rem_to_px(0.75)))
                                .py(px(rem_to_px(0.375)))
                                .rounded(control_radius)
                                .border_1()
                                .border_color(border_focus)
                                .text_size(resolve_px(theme, "typography.label.size"))
                                .text_color(text_primary)
                                .child(label.to_string()),
                        ),
                );
            }
        }

        panel.into_any_element()
    }
}
