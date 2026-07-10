//! Pagination — nav/page/ellipsis button builders.
//!
//! Split out of `pagination/mod.rs` (god-file decomposition); behavior
//! unchanged.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_specs::{
    IconSize, IconSpec,
};

use crate::primitives::icon::Icon;
use crate::presentation::rem_to_px;

use super::Pagination;

impl Pagination {
    pub(super) fn render_nav_button(
        &self,
        icon_name: &str,
        disabled: bool,
        target_page: usize,
        id: &str,
    ) -> AnyElement {
        let theme = &self.theme;
        let fill = self.button_fill;
        let border = self.button_border;
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let focus_ring = self.focus_ring;
        let radius = self.radius;
        let button_height = self.button_height;
        let disabled_opacity = self.disabled_opacity;

        let mut btn = div()
            .id(SharedString::from(id.to_string()))
            .focusable()
            .flex()
            .items_center()
            .justify_center()
            // Contract: min-width = control-height
            .min_w(self.button_min_width)
            .h(button_height)
            .px(self.button_padding)
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            // Focus ring
            .focus(move |s| s.border_color(focus_ring))
            .when(disabled, |el| {
                el.opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed)
            })
            .when(!disabled, |el| {
                el.cursor_pointer().hover(|style| style.bg(hover_fill))
            })
            .child(
                Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Sm), theme)
                    .with_color(text_color),
            );

        // Wire click handler for navigation
        if !disabled {
            if let Some(ref handler) = self.on_page_change {
                let handler = handler.clone();
                btn = btn.on_click(move |_event, window, cx| {
                    handler(&target_page, window, cx);
                });
            }
        }

        btn.into_any_element()
    }

    /// Text-label variant of the nav button used by the Simple variant
    /// ("Prev" / "Next" instead of chevron icons).
    pub(super) fn render_text_nav_button(
        &self,
        label: &'static str,
        disabled: bool,
        target_page: usize,
        id: &str,
    ) -> AnyElement {
        let fill = self.button_fill;
        let border = self.button_border;
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let focus_ring = self.focus_ring;
        let radius = self.radius;
        let button_height = self.button_height;
        let disabled_opacity = self.disabled_opacity;

        let mut btn = div()
            .id(SharedString::from(id.to_string()))
            .focusable()
            .flex()
            .items_center()
            .justify_center()
            .h(button_height)
            .px(self.button_padding)
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_size(crate::theme_ext::resolve_px(
                &self.theme,
                "typography.label.size",
            ))
            .text_color(text_color)
            .focus(move |s| s.border_color(focus_ring))
            .when(disabled, |el| {
                el.opacity(disabled_opacity)
                    .cursor(CursorStyle::OperationNotAllowed)
            })
            .when(!disabled, |el| {
                el.cursor_pointer().hover(|style| style.bg(hover_fill))
            })
            .child(label);

        if !disabled {
            if let Some(ref handler) = self.on_page_change {
                let handler = handler.clone();
                btn = btn.on_click(move |_event, window, cx| {
                    handler(&target_page, window, cx);
                });
            }
        }

        btn.into_any_element()
    }

    pub(super) fn render_page_button(&self, page: usize) -> AnyElement {
        let is_current = page == self.spec.current_page;
        let fill = if is_current {
            self.current_fill
        } else {
            self.button_fill
        };
        let border = if is_current {
            self.current_border
        } else {
            self.button_border
        };
        let text_color = self.button_text;
        let hover_fill = self.hover_fill;
        let focus_ring = self.focus_ring;
        let radius = self.radius;
        let button_height = self.button_height;

        let page_id = SharedString::from(format!("poodle-pg-page-{}", page));

        let mut btn = div()
            .id(page_id)
            .flex()
            .items_center()
            .justify_center()
            .min_w(self.button_min_width) // contract: min-width = control-height
            .h(button_height)
            .px(self.button_padding)
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_color(text_color)
            // Contract: label font per effective size
            .text_size(self.font_size)
            .font_weight(FontWeight::SEMIBOLD)
            // Focus ring
            .focus(move |s| s.border_color(focus_ring))
            .when(is_current, |el| el.font_weight(FontWeight::BOLD))
            .when(!is_current, |el| {
                el.cursor_pointer().hover(|style| style.bg(hover_fill))
            })
            .child(page.to_string());

        // Wire click handler for page selection
        if !is_current {
            if let Some(ref handler) = self.on_page_change {
                let handler = handler.clone();
                btn = btn.on_click(move |_event, window, cx| {
                    handler(&page, window, cx);
                });
            }
        }

        btn.into_any_element()
    }

    pub(super) fn render_ellipsis(&self) -> AnyElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            // Contract: ellipsis min-width 1.5rem
            .min_w(px(rem_to_px(1.5)))
            .h(self.button_height)
            .text_color(self.ellipsis_color)
            // Contract: same font as buttons
            .text_size(self.font_size)
            .font_weight(FontWeight::SEMIBOLD)
            .child("...")
            .into_any_element()
    }
}
