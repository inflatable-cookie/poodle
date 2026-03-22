//! MediaPicker — media selection/upload interface backed by MediaPickerSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec};
use pug_composites::MediaPickerSpec;
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_radius};

/// Which tab is active in the media picker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MediaPickerTab {
    #[default]
    Browse,
    Upload,
}

/// A thumbnail item shown in the picker grid.
#[derive(Clone, Debug)]
pub struct MediaPickerItem {
    pub id: String,
    pub label: String,
    pub is_selected: bool,
}

pub struct MediaPicker {
    spec: MediaPickerSpec,
    theme: GpuiThemeProvider,
    active_tab: MediaPickerTab,
    thumbnails: Vec<MediaPickerItem>,
    content: Option<AnyElement>,
}

impl std::ops::Deref for MediaPicker {
    type Target = MediaPickerSpec;
    fn deref(&self) -> &MediaPickerSpec { &self.spec }
}

impl MediaPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MediaPickerSpec::new("Select media"),
            theme: theme.clone(),
            active_tab: MediaPickerTab::Browse,
            thumbnails: Vec::new(),
            content: None,
        }
    }
    pub fn from_spec(spec: MediaPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            active_tab: MediaPickerTab::Browse,
            thumbnails: Vec::new(),
            content: None,
        }
    }
    pub fn with_content(mut self, c: impl IntoElement) -> Self {
        self.content = Some(c.into_any_element()); self
    }
    pub fn with_active_tab(mut self, tab: MediaPickerTab) -> Self {
        self.active_tab = tab; self
    }
    pub fn with_thumbnail(mut self, thumb: MediaPickerItem) -> Self {
        self.thumbnails.push(thumb); self
    }
    pub fn with_thumbnails(mut self, thumbs: impl IntoIterator<Item = MediaPickerItem>) -> Self {
        self.thumbnails.extend(thumbs); self
    }
}

impl IntoElement for MediaPicker {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let fill = resolve_color(theme, self.spec.fill_token());
        let radius = resolve_radius(theme, "semantic.radius.surface");
        let border_color = resolve_color(theme, "semantic.color.border.subtle");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let accent = resolve_color(theme, "semantic.color.accent.base");

        // ── Dialog wrapper ───────────────────────────────────────
        let mut dialog = div()
            .bg(fill)
            .rounded(radius)
            .shadow_lg()
            .flex()
            .flex_col()
            .min_w(px(480.0))
            .max_h(px(520.0))
            .overflow_hidden()
            .border_1()
            .border_color(border_color);

        // ── Title bar ────────────────────────────────────────────
        let title_bar = div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(20.0))
            .py(px(12.0))
            .border_b_1()
            .border_color(border_color)
            .child(
                div()
                    .text_color(text_primary)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.spec.title.clone()),
            )
            .child({
                let close_icon = Icon::from_spec(
                    IconSpec::new("x").with_size(IconSize::Sm),
                    theme,
                ).with_color(text_secondary);
                div().cursor_pointer().child(close_icon)
            });

        dialog = dialog.child(title_bar);

        // ── Tab header (Browse / Upload) ─────────────────────────
        let browse_active = self.active_tab == MediaPickerTab::Browse;

        let tab_item = |label: &str, is_active: bool| -> Div {
            let base = div()
                .px(px(16.0))
                .py(px(8.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer();
            if is_active {
                base.text_color(accent)
                    .border_b_2()
                    .border_color(accent)
            } else {
                base.text_color(text_secondary)
            }
        };

        let browse_icon = Icon::from_spec(
            IconSpec::new("image").with_size(IconSize::Sm),
            theme,
        ).with_color(if browse_active { accent } else { text_secondary });

        let upload_icon = Icon::from_spec(
            IconSpec::new("upload").with_size(IconSize::Sm),
            theme,
        ).with_color(if !browse_active { accent } else { text_secondary });

        let tabs = div()
            .flex()
            .border_b_1()
            .border_color(border_color)
            .child(
                tab_item("Browse", browse_active)
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(browse_icon)
                    .child("Browse"),
            )
            .child(
                tab_item("Upload", !browse_active)
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(upload_icon)
                    .child("Upload"),
            );

        dialog = dialog.child(tabs);

        // ── Search input area ────────────────────────────────────
        let search_icon = Icon::from_spec(
            IconSpec::new("search").with_size(IconSize::Sm),
            theme,
        ).with_color(text_secondary);

        let search_bar = div()
            .flex()
            .items_center()
            .gap(px(6.0))
            .mx(px(16.0))
            .my(px(10.0))
            .px(px(10.0))
            .py(px(6.0))
            .rounded(px(6.0))
            .border_1()
            .border_color(border_color)
            .child(search_icon)
            .child(
                div()
                    .flex_grow()
                    .text_size(px(13.0))
                    .text_color(text_secondary)
                    .child("Search media\u{2026}"),
            );

        dialog = dialog.child(search_bar);

        // ── Accepted types hint ──────────────────────────────────
        if let Some(ref types) = self.spec.accepted_types {
            dialog = dialog.child(
                div()
                    .px(px(16.0))
                    .text_size(px(11.0))
                    .text_color(text_secondary)
                    .child(format!("Accepted: {}", types)),
            );
        }

        // ── Thumbnail grid ───────────────────────────────────────
        let mut grid = div()
            .flex()
            .flex_wrap()
            .gap(px(8.0))
            .px(px(16.0))
            .py(px(12.0))
            .overflow_hidden()
            .flex_grow();

        if self.thumbnails.is_empty() && self.content.is_none() {
            grid = grid.child(
                div()
                    .w_full()
                    .py(px(24.0))
                    .text_size(px(13.0))
                    .text_color(text_secondary)
                    .text_center()
                    .child("No media items"),
            );
        }

        for thumb in &self.thumbnails {
            let selected_border = if thumb.is_selected { accent } else { border_color };
            let border_w = if thumb.is_selected { px(2.0) } else { px(1.0) };

            let item = div()
                .w(px(96.0))
                .h(px(96.0))
                .rounded(px(6.0))
                .border(border_w)
                .border_color(selected_border)
                .bg(resolve_color(theme, "semantic.color.background.surface"))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(px(4.0))
                .cursor_pointer()
                .child({
                    let icon = Icon::from_spec(
                        IconSpec::new("image").with_size(IconSize::Md),
                        theme,
                    ).with_color(text_secondary);
                    icon
                })
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(text_secondary)
                        .max_w(px(80.0))
                        .overflow_x_hidden()
                        .child(thumb.label.clone()),
                );

            grid = grid.child(item);
        }

        if let Some(c) = self.content { grid = grid.child(c); }

        dialog = dialog.child(grid);

        // ── Footer with selection count ──────────────────────────
        let selected = self.spec.selected_count;
        let footer = div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(16.0))
            .py(px(10.0))
            .border_t_1()
            .border_color(border_color)
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(text_secondary)
                    .when(selected > 0, |el| el.text_color(text_primary))
                    .child(format!(
                        "{} item{} selected",
                        selected,
                        if selected == 1 { "" } else { "s" },
                    )),
            )
            .child(
                div()
                    .px(px(14.0))
                    .py(px(6.0))
                    .rounded(px(6.0))
                    .bg(accent)
                    .text_color(gpui::white())
                    .text_size(px(13.0))
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .child("Confirm"),
            );

        dialog = dialog.child(footer);

        dialog.into_any_element()
    }
}
