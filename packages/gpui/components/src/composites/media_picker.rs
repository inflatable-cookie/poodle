//! MediaPicker — media selection/upload interface backed by MediaPickerSpec.

use crate::presentation::{
    control_space_x_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
    size_font_rem,
};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};
use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MediaPickerSpec;
use poodle_specs::{ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};
use std::rc::Rc;

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
    on_select: Option<Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_confirm: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_tab_change: Option<Rc<dyn Fn(MediaPickerTab, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for MediaPicker {
    type Target = MediaPickerSpec;
    fn deref(&self) -> &MediaPickerSpec {
        &self.spec
    }
}

impl MediaPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MediaPickerSpec::new("Select media"),
            theme: theme.clone(),
            active_tab: MediaPickerTab::Browse,
            thumbnails: Vec::new(),
            content: None,
            on_select: None,
            on_confirm: None,
            on_tab_change: None,
        }
    }
    pub fn from_spec(spec: MediaPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            active_tab: MediaPickerTab::Browse,
            thumbnails: Vec::new(),
            content: None,
            on_select: None,
            on_confirm: None,
            on_tab_change: None,
        }
    }
    pub fn with_content(mut self, c: impl IntoElement) -> Self {
        self.content = Some(c.into_any_element());
        self
    }
    pub fn with_active_tab(mut self, tab: MediaPickerTab) -> Self {
        self.active_tab = tab;
        self
    }
    pub fn with_thumbnail(mut self, thumb: MediaPickerItem) -> Self {
        self.thumbnails.push(thumb);
        self
    }
    pub fn with_thumbnails(mut self, thumbs: impl IntoIterator<Item = MediaPickerItem>) -> Self {
        self.thumbnails.extend(thumbs);
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    pub fn on_confirm(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_confirm = Some(Rc::new(handler));
        self
    }
    pub fn on_tab_change(
        mut self,
        handler: impl Fn(MediaPickerTab, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_change = Some(Rc::new(handler));
        self
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

impl IntoElement for MediaPicker {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        // When closed, render nothing — matches dialog-style composites
        // (confirm_action, etc.). Callers drive is_open via spec/state.
        if !self.spec.is_open {
            return div().into_any_element();
        }
        let theme = &self.theme;
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let pad_x = rem_to_px(panel_space_x_rem(self.spec.density));
        let pad_y = rem_to_px(panel_space_y_rem(self.spec.density));
        let item_gap = rem_to_px(control_space_x_rem(self.spec.density));

        let fill = resolve_color(theme, self.spec.fill_token());
        let radius = resolve_radius(theme, "radius.surface");
        let border_color = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let label_size = px(font_size);
        let label_tok = resolve_px(theme, "typography.label.size");
        let radius_control = resolve_radius(theme, "radius.control");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let gap_lg = resolve_px(theme, "space.inline.lg");

        // ── Dialog wrapper ───────────────────────────────────────
        let mut dialog = div()
            .bg(fill)
            .rounded(radius)
            .shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ])
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
            .px(px(pad_x))
            .py(px(pad_y))
            .border_b_1()
            .border_color(border_color)
            .child(
                div()
                    .text_color(text_primary)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.spec.title.clone()),
            )
            .child({
                let close_icon = Icon::from_spec(IconSpec::new("x").with_size(IconSize::Sm), theme)
                    .with_color(text_secondary);
                div().cursor_pointer().child(close_icon)
            });

        dialog = dialog.child(title_bar);

        // ── Tab header (Browse / Upload) ─────────────────────────
        let browse_active = self.active_tab == MediaPickerTab::Browse;

        let tab_item = |_label: &str, is_active: bool| -> Div {
            let base = div()
                .px(gap_lg)
                .py(gap_sm)
                .text_size(label_size)
                .font_weight(FontWeight::MEDIUM)
                .cursor_pointer();
            if is_active {
                base.text_color(accent).border_b_2().border_color(accent)
            } else {
                base.text_color(text_secondary)
            }
        };

        let browse_icon = Icon::from_spec(IconSpec::new("image").with_size(IconSize::Sm), theme)
            .with_color(if browse_active {
                accent
            } else {
                text_secondary
            });

        let upload_icon = Icon::from_spec(IconSpec::new("upload").with_size(IconSize::Sm), theme)
            .with_color(if !browse_active {
                accent
            } else {
                text_secondary
            });

        let on_tab_change = self.on_tab_change;

        let browse_tab = {
            let mut el = tab_item("Browse", browse_active)
                .id("media-picker-tab-browse")
                .flex()
                .items_center()
                .gap(gap_sm)
                .child(browse_icon)
                .child("Browse");
            if let Some(ref handler) = on_tab_change {
                let handler = handler.clone();
                el = el.on_click(move |ev, win, app| {
                    handler(MediaPickerTab::Browse, ev, win, app);
                });
            }
            el
        };

        let upload_tab = {
            let mut el = tab_item("Upload", !browse_active)
                .id("media-picker-tab-upload")
                .flex()
                .items_center()
                .gap(gap_sm)
                .child(upload_icon)
                .child("Upload");
            if let Some(ref handler) = on_tab_change {
                let handler = handler.clone();
                el = el.on_click(move |ev, win, app| {
                    handler(MediaPickerTab::Upload, ev, win, app);
                });
            }
            el
        };

        let tabs = div()
            .flex()
            .border_b_1()
            .border_color(border_color)
            .child(browse_tab)
            .child(upload_tab);

        dialog = dialog.child(tabs);

        // ── Search input area ────────────────────────────────────
        let search_icon = Icon::from_spec(IconSpec::new("search").with_size(IconSize::Sm), theme)
            .with_color(text_secondary);

        let search_bar = div()
            .flex()
            .items_center()
            .gap(gap_sm)
            .mx(gap_lg)
            .my(gap_md)
            .px(gap_md)
            .py(gap_sm)
            .rounded(radius_control)
            .border_1()
            .border_color(border_color)
            .child(search_icon)
            .child(
                div()
                    .flex_grow()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child("Search media\u{2026}"),
            );

        dialog = dialog.child(search_bar);

        // ── Accepted types hint ──────────────────────────────────
        if let Some(ref types) = self.spec.accepted_types {
            dialog = dialog.child(
                div()
                    .px(gap_lg)
                    .text_size(label_tok)
                    .text_color(text_secondary)
                    .child(format!("Accepted: {}", types)),
            );
        }

        // ── Thumbnail grid ───────────────────────────────────────
        let mut grid = div()
            .flex()
            .flex_wrap()
            .gap(px(item_gap))
            .px(px(pad_x))
            .py(px(pad_y))
            .overflow_hidden()
            .flex_grow();

        if self.thumbnails.is_empty() && self.content.is_none() {
            // Empty-state copy: prefer spec.empty_message when set,
            // fall back to the contract default ("No media items
            // found." per Svelte reference). This honours the
            // `emptyMessage` prop from the contract doc.
            let empty_text = self
                .spec
                .empty_message
                .clone()
                .unwrap_or_else(|| "No media items found.".to_string());
            grid = grid.child(
                div()
                    .w_full()
                    .py(gap_lg)
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .text_center()
                    .child(empty_text),
            );
        }

        let on_select = self.on_select;

        for thumb in &self.thumbnails {
            let selected_border = if thumb.is_selected {
                accent
            } else {
                border_color
            };
            let border_w = if thumb.is_selected { px(2.0) } else { px(1.0) };

            let item_id = SharedString::from(format!("media-picker-thumb-{}", thumb.id));

            let mut item = div()
                .id(item_id)
                .w(px(72.0))
                .h(px(72.0))
                .rounded(radius_control)
                .border(border_w)
                .border_color(selected_border)
                .bg(resolve_color(theme, "color.background.surface"))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(gap_sm)
                .cursor_pointer()
                .child({
                    let icon =
                        Icon::from_spec(IconSpec::new("image").with_size(IconSize::Md), theme)
                            .with_color(text_secondary);
                    icon
                })
                .child(
                    div()
                        .text_size(label_tok)
                        .text_color(text_secondary)
                        .max_w(px(80.0))
                        .overflow_x_hidden()
                        .child(thumb.label.clone()),
                );

            if let Some(ref handler) = on_select {
                let handler = handler.clone();
                let thumb_id = thumb.id.clone();
                item = item.on_click(move |ev, win, app| {
                    handler(&thumb_id, ev, win, app);
                });
            }

            grid = grid.child(item);
        }

        if let Some(c) = self.content {
            grid = grid.child(c);
        }

        dialog = dialog.child(grid);

        // ── Footer with selection count ──────────────────────────
        // is_multiple shapes the wording: single-select picker shows
        // "1 selected" / "None selected" (no pluralization ambiguity),
        // multi-select shows "N items selected".
        let selected = self.spec.selected_count;
        let is_multi = self.spec.is_multiple;
        let footer_text = if is_multi {
            format!(
                "{} item{} selected",
                selected,
                if selected == 1 { "" } else { "s" },
            )
        } else if selected == 0 {
            "None selected".to_string()
        } else {
            "1 selected".to_string()
        };
        let footer = div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(pad_x))
            .py(px(pad_y))
            .border_t_1()
            .border_color(border_color)
            .child(
                div()
                    .text_size(label_tok)
                    .text_color(text_secondary)
                    .when(selected > 0, |el| el.text_color(text_primary))
                    .child(footer_text),
            )
            .child({
                let mut confirm_btn = div()
                    .id("media-picker-confirm")
                    .px(gap_md)
                    .py(gap_sm)
                    .rounded(radius_control)
                    .bg(accent)
                    .text_color(gpui::white())
                    .text_size(label_size)
                    .font_weight(FontWeight::MEDIUM)
                    .cursor_pointer()
                    .child("Confirm");
                if let Some(ref handler) = self.on_confirm {
                    let handler = handler.clone();
                    confirm_btn = confirm_btn.on_click(move |ev, win, app| {
                        handler(ev, win, app);
                    });
                }
                confirm_btn
            });

        dialog = dialog.child(footer);

        dialog.into_any_element()
    }
}
