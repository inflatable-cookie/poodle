//! MediaPicker — dialog-based media selection composite backed by
//! MediaPickerSpec.
//!
//! Contract: `docs/contracts/components/media-picker.md`
//! Reference: `packages/svelte/components/src/MediaPicker.svelte`
//!
//! Single-select, select-and-close model (no multi-select / confirm footer).
//! Composes the real `Tabs`, `TextInput`, `MediaThumbnail`, and `FileUpload`
//! primitives — no hand-coded chrome. Browse grid renders the real
//! `spec.items`; the upload tab renders the real FileUpload dropzone.

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size,
};
use crate::primitives::{FileUpload, Tabs, TextInput};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AspectRatio, ControlDensity, ControlSize, FileUploadSpec, MediaKind, MediaPickerItem,
    MediaPickerSpec, MediaPickerTab, MediaThumbnailSpec, SemanticControlSizeRole, TabDefinition,
    TabsSpec, TextInputSpec,
};
use std::rc::Rc;

use super::media_thumbnail::MediaThumbnail;

/// Browse-grid minimum column width in rem per size (contract §8 size table).
fn grid_min_col_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 4.75,
        ControlSize::Sm => 5.25,
        ControlSize::Md => 5.5,
        ControlSize::Lg => 6.0,
        ControlSize::Xl => 6.5,
    }
}

/// Browse-grid gap + item padding in rem per density (contract §8).
fn grid_gap_and_pad_rem(density: ControlDensity) -> (f32, f32) {
    match density {
        ControlDensity::Compact => (0.25, 0.25),
        ControlDensity::Default => (0.375, 0.375),
        ControlDensity::Comfortable => (0.5, 0.5),
    }
}

/// Map a media item kind to the thumbnail's MediaKind.
fn to_media_kind(kind: MediaKind) -> MediaKind {
    kind
}

pub struct MediaPicker {
    spec: MediaPickerSpec,
    theme: GpuiThemeProvider,
    on_select: Option<Rc<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
    on_tab_change: Option<Rc<dyn Fn(MediaPickerTab, &mut Window, &mut App) + 'static>>,
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
            on_select: None,
            on_tab_change: None,
        }
    }
    pub fn from_spec(spec: MediaPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_select: None,
            on_tab_change: None,
        }
    }
    pub fn with_active_tab(mut self, tab: MediaPickerTab) -> Self {
        self.spec.active_tab = tab;
        self
    }
    pub fn with_items(mut self, items: Vec<MediaPickerItem>) -> Self {
        self.spec.items = items;
        self
    }
    pub fn with_item(mut self, item: MediaPickerItem) -> Self {
        self.spec.items.push(item);
        self
    }
    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
    pub fn on_tab_change(
        mut self,
        handler: impl Fn(MediaPickerTab, &mut Window, &mut App) + 'static,
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
        // When closed, render nothing — callers drive is_open via spec/state.
        if !self.spec.is_open {
            return div().into_any_element();
        }
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        let stack_gap = px(rem_to_px(0.5));

        let fill = resolve_color(theme, spec.fill_token());
        let radius = resolve_radius(theme, "radius.surface");
        let border_color = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");

        // ── Dialog chrome (elevated surface + dialog shadow) ──
        let mut dialog = div()
            .bg(fill)
            .rounded(radius)
            .shadow(crate::theme_ext::elevation_dialog_shadow())
            .flex()
            .flex_col()
            .min_w(px(rem_to_px(30.0)))
            .max_h(px(rem_to_px(32.5)))
            .overflow_hidden()
            .border_1()
            .border_color(border_color);

        // ── Title bar ──
        let close_icon =
            crate::primitives::Icon::from_spec(poodle_specs::IconSpec::new("x"), theme)
                .with_px_size(rem_to_px(1.0))
                .with_color(text_secondary);
        dialog = dialog.child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .px(pad_x)
                .py(pad_y)
                .border_b_1()
                .border_color(border_color)
                .child(
                    div()
                        .text_color(text_primary)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(spec.title.clone()),
                )
                .child(div().cursor_pointer().child(close_icon)),
        );

        // ── Tabs (Browse | Upload) with content panels ──
        let active_value = if spec.is_browsing() { "browse" } else { "upload" };
        let tab_defs = vec![
            TabDefinition::new("browse", "Browse"),
            TabDefinition::new("upload", "Upload"),
        ];

        let browse_panel = self.browse_panel(theme, effective_size);
        let upload_panel = self.upload_panel(theme);

        let mut tabs = Tabs::from_spec(
            TabsSpec::new(tab_defs)
                .with_value(active_value)
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density),
            theme,
        )
        .with_id("media-picker")
        .with_content("browse", browse_panel)
        .with_content("upload", upload_panel);

        if let Some(handler) = self.on_tab_change.clone() {
            tabs = tabs.on_change(move |value, win, app| {
                let tab = if value == "upload" {
                    MediaPickerTab::Upload
                } else {
                    MediaPickerTab::Browse
                };
                handler(tab, win, app);
            });
        }

        dialog = dialog.child(
            div()
                .flex()
                .flex_col()
                .gap(stack_gap)
                .px(pad_x)
                .py(pad_y)
                .child(tabs),
        );

        dialog.into_any_element()
    }
}

impl MediaPicker {
    /// Browse tab: search input + grid of real items, or empty state.
    fn browse_panel(&self, theme: &GpuiThemeProvider, effective_size: ControlSize) -> AnyElement {
        let spec = &self.spec;
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let stack_gap = px(rem_to_px(0.5));

        let search = TextInput::from_spec(
            TextInputSpec::new()
                .with_placeholder("Search media...")
                .with_size(spec.size)
                .with_size_role(spec.size_role)
                .with_density(spec.density),
            theme,
        )
        .with_id("media-picker-search");

        let mut panel = div()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(div().mt(px(rem_to_px(0.25))).child(search));

        if spec.has_items() {
            let (grid_gap, item_pad) = grid_gap_and_pad_rem(spec.density);
            let grid_min = px(rem_to_px(grid_min_col_rem(effective_size)));
            let item_radius = resolve_radius(theme, spec.item_radius_token());
            let label_size = resolve_px(theme, "typography.label.size");
            let label_color = resolve_color(theme, spec.label_token());
            let border_focus = resolve_color(theme, spec.item_border_token());
            let item_hover_bg = resolve_color(theme, spec.item_hover_fill_token());
            let on_select = self.on_select.clone();

            let mut grid = div()
                .id("media-picker-grid")
                .flex()
                .flex_wrap()
                .gap(px(rem_to_px(grid_gap)))
                .max_h(px(rem_to_px(20.0)))
                .overflow_y_scroll();

            for item in &spec.items {
                let item_id = SharedString::from(format!("media-picker-item-{}", item.id));
                // Thumbnail — real MediaThumbnail (compact, square, no caption).
                let thumb = MediaThumbnail::from_spec(
                    MediaThumbnailSpec::new(to_media_kind(item.kind))
                        .with_aspect_ratio(AspectRatio::Square)
                        .with_show_caption(false),
                    theme,
                );

                let mut cell = div()
                    .id(item_id)
                    .focusable()
                    .min_w(grid_min)
                    .flex_grow()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(rem_to_px(0.25)))
                    .p(px(rem_to_px(item_pad)))
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .rounded(item_radius)
                    .cursor_pointer()
                    // Contract §8 :hover / :focus-visible → border-focus + panel bg.
                    .hover(move |s| s.border_color(border_focus).bg(item_hover_bg))
                    .focus(move |s| s.border_color(border_focus).bg(item_hover_bg))
                    .child(thumb)
                    .child(
                        div()
                            .max_w_full()
                            .text_size(label_size)
                            .text_color(label_color)
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(item.label.clone()),
                    );

                if let Some(ref handler) = on_select {
                    let handler = handler.clone();
                    let id = item.id.clone();
                    cell = cell.on_click(move |ev, win, app| handler(&id, ev, win, app));
                }

                grid = grid.child(cell);
            }

            panel = panel.child(grid);
        } else {
            // Empty state — centered message, min-height 10rem.
            let empty_text = spec
                .empty_message
                .clone()
                .unwrap_or_else(|| "No media items found.".to_string());
            panel = panel.child(
                div()
                    .min_h(px(rem_to_px(10.0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(rem_to_px(0.875)))
                    .text_color(text_secondary)
                    .child(empty_text),
            );
        }

        panel.into_any_element()
    }

    /// Upload tab: real FileUpload dropzone (multi-file).
    fn upload_panel(&self, theme: &GpuiThemeProvider) -> AnyElement {
        let spec = &self.spec;
        let mut upload_spec = FileUploadSpec::new()
            .with_multiple(true)
            .with_size(spec.size)
            .with_size_role(spec.size_role)
            .with_density(spec.density);
        if let Some(ref accept) = spec.accept {
            upload_spec = upload_spec.with_accept(accept.clone());
        }
        if let Some(max) = spec.max_file_size {
            upload_spec = upload_spec.with_max_size(max);
        }
        div()
            .mt(px(rem_to_px(0.25)))
            .child(FileUpload::from_spec(upload_spec, theme).with_id("media-picker"))
            .into_any_element()
    }
}
