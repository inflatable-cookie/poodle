//! MediaPicker — media grid behind Browse | Upload tabs.
//!
//! Contract: `docs/contracts/components/media-picker.md`
//! Ported from: `packages/jetstream/components/src/media_picker.rs`.
//!
//! Single-select, select-and-close model. Thumbnail bitmaps are host-owned —
//! `MediaPickerItem::has_thumbnail` only drives the placeholder-vs-image
//! anatomy split. No `on_upload`: the upload tab composes `file_upload`,
//! whose drop zone needs file drops the runtime does not raise.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeRole, StylePatch,
};
use poodle_specs::{
    ControlDensity, ControlSize, DialogSpec, FileUploadSpec, MediaPickerItem, MediaPickerSpec,
    TextInputSpec,
};

use crate::color::TRANSPARENT;
use crate::context::RenderContext;
use crate::dialog::dialog;
use crate::file_upload::file_upload;
use crate::presentation::{rem_to_px, size_font_rem};
use crate::text_input::{text_input_with_handlers, TextInputHandlers};

/// Thumbnail square size in rem per size (contract §8 size table).
fn thumb_size_rem(size: ControlSize) -> f32 {
    match size {
        ControlSize::Xs => 3.5,
        ControlSize::Sm => 4.25,
        ControlSize::Md => 4.5,
        ControlSize::Lg => 5.0,
        ControlSize::Xl => 5.5,
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

/// Handlers mirror the GPUI target's names.
#[derive(Default)]
pub struct MediaPickerHandlers {
    /// Fires with the chosen item's id.
    pub on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Fires with the tab's value when one is pressed.
    pub on_tab_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// Reports the internal search query so the host can rebuild the filtered picker.
    pub on_search_change: Option<poodle_node::TextChangeHandler>,
    /// Reports the search caret/selection so controlled edits keep their insertion point.
    pub on_search_selection_change: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
    /// Reports dialog visibility changes (dismissal and selection request close).
    pub on_open_change: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

pub fn media_picker(
    spec: &MediaPickerSpec,
    ctx: &RenderContext<'_>,
    handlers: MediaPickerHandlers,
) -> Node {
    if !spec.is_open {
        return Node::container();
    }

    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let label_size = ctx.theme().resolve_space("typography.label.size");

    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let accent = ctx.theme().resolve_color("color.accent.base");
    let gap = ctx.theme().resolve_space("space.stack.sm");

    // ── Root: dialog body content ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = gap;
        s.min_height = Some(rem_to_px(20.0));
    }

    // ── Tabs: Browse | Upload (active reflects spec.active_tab) ──
    let browsing = spec.is_browsing();
    let tab = |text: &str, value: &'static str, active: bool| -> Node {
        let mut el = Node::button(text);
        {
            let s = &mut el.style;
            s.text_size = Some(font_size);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = rem_to_px(0.5);
            pad.right = rem_to_px(0.5);
            pad.bottom = rem_to_px(0.25);
            if active {
                s.descriptor.text_color = Some(accent);
                s.border_bottom_width = Some(2.0);
                s.descriptor.border.color = accent;
            } else {
                s.descriptor.text_color = Some(text_secondary);
            }
        }
        el.interaction.focusable = true;
        if let Some(handler) = &handlers.on_tab_change {
            let handler = Arc::clone(handler);
            el.style.descriptor.cursor = CursorHint::Pointer;
            el.interaction.on_activate = Some(Arc::new(move || handler(value)));
        }
        el
    };
    let mut tabs = Node::container();
    tabs.style.descriptor.layout.direction = LayoutDirection::Row;
    tabs.style.descriptor.layout.spacing.gap = rem_to_px(0.5);
    root = root.child(
        tabs.child(tab("Browse", "browse", browsing))
            .child(tab("Upload", "upload", !browsing)),
    );

    if browsing {
        // ── Search field (contract §2 search; a real input) ──
        let mut search_spec = TextInputSpec::new()
            .with_value(spec.search_query.clone())
            .with_placeholder("Search media...")
            .with_aria_label("Search media")
            .with_id("media-picker-search")
            .with_size(effective_size)
            .with_size_role(spec.size_role)
            .with_density(density)
            .with_show_clear_button(false);
        search_spec.selection_start = spec.search_selection.0;
        search_spec.selection_end = spec.search_selection.1;
        let search = text_input_with_handlers(
            &search_spec,
            ctx,
            TextInputHandlers {
                on_change: handlers.on_search_change.clone(),
                on_selection_change: handlers.on_search_selection_change.clone(),
                ..TextInputHandlers::default()
            },
        );
        let mut wrap = Node::container();
        // Explicit Row (see switch.rs).
        wrap.style.descriptor.layout.direction = LayoutDirection::Row;
        wrap.style.descriptor.layout.spacing.margin.top = rem_to_px(0.25);
        root = root.child(wrap.child(search));

        // ── Grid OR empty state ──
        let query = spec.search_query.to_lowercase();
        let visible_items: Vec<_> = spec
            .items
            .iter()
            .filter(|item| item.label.to_lowercase().contains(&query))
            .collect();
        if !visible_items.is_empty() {
            let (grid_gap, item_pad) = grid_gap_and_pad_rem(density);
            let thumb_size = rem_to_px(thumb_size_rem(effective_size));
            // Contract: the media grid is a `listbox` of selectable `option`s.
            let mut grid = Node::container();
            grid.id = Some("media-picker-grid".to_string());
            grid.runtime_id = grid.id.clone();
            grid.a11y.role = Some(NodeRole::ListBox);
            grid.a11y.label = Some("Media items".to_string());
            {
                let s = &mut grid.style;
                s.descriptor.layout.direction = LayoutDirection::Row;
                s.flex_wrap = true;
                s.descriptor.layout.spacing.gap = rem_to_px(grid_gap);
                s.max_height = Some(rem_to_px(20.0));
                s.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
                s.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
            }
            for item in visible_items {
                let mut cell = grid_item(item, spec, ctx, thumb_size, item_pad, label_size);
                let on_select = handlers.on_select.clone();
                let on_open_change = handlers.on_open_change.clone();
                let id = item.id.clone();
                cell.interaction.on_activate = Some(Arc::new(move || {
                    if let Some(handler) = &on_select {
                        handler(&id);
                    }
                    if let Some(handler) = &on_open_change {
                        handler(false);
                    }
                }));
                grid = grid.child(cell);
            }
            root = root.child(grid);
        } else {
            let empty_text = spec
                .empty_message
                .clone()
                .unwrap_or_else(|| "No media items found.".to_string());
            let mut copy = Node::text(&empty_text);
            copy.style.descriptor.text_color = Some(text_secondary);
            copy.style.text_size = Some(rem_to_px(0.875));
            let mut state = Node::container();
            {
                let s = &mut state.style;
                s.min_height = Some(rem_to_px(10.0));
                s.self_stretch = true;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            }
            root = root.child(state.child(copy));
        }
    } else {
        // ── Upload tab: compose the real FileUpload dropzone ──
        let mut upload_spec = FileUploadSpec::new()
            .with_multiple(true)
            .with_size(base_size)
            .with_size_role(spec.size_role)
            .with_density(density);
        if let Some(ref accept) = spec.accept {
            upload_spec = upload_spec.with_accept(accept.clone());
        }
        if let Some(max) = spec.max_file_size {
            upload_spec = upload_spec.with_max_size(max);
        }
        let mut wrap = Node::container();
        // Explicit Row (see switch.rs).
        wrap.style.descriptor.layout.direction = LayoutDirection::Row;
        wrap.style.descriptor.layout.spacing.margin.top = rem_to_px(0.25);
        wrap.style.self_stretch = true;
        root = root.child(wrap.child(file_upload(&upload_spec, ctx, None)));
    }

    let on_request_close = handlers
        .on_open_change
        .map(|handler| Arc::new(move || handler(false)) as Arc<dyn Fn() + Send + Sync>);
    dialog(
        &DialogSpec::new()
            .with_open(true)
            .with_title(spec.title.clone()),
        ctx,
        vec![root],
        None,
        on_request_close,
    )
}

/// One selectable browse-grid item: thumbnail (image surface or placeholder)
/// + truncated label.
fn grid_item(
    item: &MediaPickerItem,
    spec: &MediaPickerSpec,
    ctx: &RenderContext<'_>,
    thumb_size: f32,
    item_pad: f32,
    label_size: f32,
) -> Node {
    let item_radius = ctx.theme().resolve_radius(spec.item_radius_token());
    let label_color = ctx.theme().resolve_color(spec.label_token());
    let placeholder_color = ctx.theme().resolve_color(spec.placeholder_icon_token());
    let hover_fill = ctx.theme().resolve_color(spec.item_hover_fill_token());
    let hover_border = ctx.theme().resolve_color(spec.item_border_token());

    let all_radius = |node: &mut Node, r: f32| {
        let c = &mut node.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
    };

    // Thumbnail: real bitmap is host-owned. With a thumbnail, paint the panel
    // surface frame (image overlay = preview-loop); without, render the
    // centered placeholder image glyph.
    let thumb = if item.has_thumbnail {
        let mut t = Node::container();
        {
            let s = &mut t.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
            s.descriptor.background = Some(hover_fill);
        }
        all_radius(&mut t, rem_to_px(0.25));
        t
    } else {
        let mut t = Node::container();
        {
            let s = &mut t.style;
            s.descriptor.layout.width = LayoutSizing::Fixed(thumb_size);
            s.descriptor.layout.height = LayoutSizing::Fixed(thumb_size);
            s.descriptor.background = Some(hover_fill);
            s.descriptor.layout.direction = LayoutDirection::Column;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        }
        all_radius(&mut t, rem_to_px(0.25));
        let mut glyph = Node::icon(
            match item.kind {
                poodle_specs::MediaKind::Image => "image",
                poodle_specs::MediaKind::Audio => "music",
                poodle_specs::MediaKind::Video => "play",
                poodle_specs::MediaKind::Document => "file-text",
                poodle_specs::MediaKind::Embed => "external-link",
            },
            rem_to_px(1.5),
        );
        glyph.style.descriptor.text_color = Some(placeholder_color);
        t.child(glyph)
    };

    // The explicit label node below owns the tile caption. A non-empty
    // `Node::button` label would make the GPUI backend emit a second caption.
    let mut cell = Node::button("");
    cell.id = Some(format!("media-picker-item:{}", item.id));
    cell.runtime_id = cell.id.clone();
    cell.a11y.role = Some(NodeRole::ListBoxOption);
    cell.a11y.label = Some(item.label.clone());
    cell.a11y.selected = Some(false);
    {
        let s = &mut cell.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = rem_to_px(0.25);
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = rem_to_px(item_pad);
        pad.right = rem_to_px(item_pad);
        pad.top = rem_to_px(item_pad);
        pad.bottom = rem_to_px(item_pad);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = TRANSPARENT;
        s.descriptor.background = Some(TRANSPARENT);
        s.descriptor.cursor = CursorHint::Pointer;
        s.hover = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
        s.focus = Some(StylePatch {
            background: Some(hover_fill),
            border_color: Some(hover_border),
            text_color: None,
            opacity: None,
        });
    }
    all_radius(&mut cell, item_radius);
    cell.interaction.focusable = true;

    let mut label = Node::text(&item.label);
    {
        let s = &mut label.style;
        s.text_size = Some(label_size);
        s.descriptor.text_color = Some(label_color);
        s.max_width = Some(thumb_size);
        s.no_wrap = true;
        s.text_ellipsis = true;
    }

    cell.child(thumb).child(label)
}
