//! PageHeader — real GPUI component backed by PageHeaderSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_composites::{PageHeaderAlign, PageHeaderSpec};
use poodle_primitives::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::presentation::{resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem, rem_to_px};
use crate::theme_ext::{resolve_color, resolve_px};

/// A real GPUI page header component backed by `PageHeaderSpec`.
///
/// Renders a standardized title and action region with optional breadcrumbs,
/// eyebrow text, subtitle, and actions slot.
pub struct PageHeader {
    spec: PageHeaderSpec,
    theme: GpuiThemeProvider,
    breadcrumbs_slot: Option<AnyElement>,
    actions_slot: Option<AnyElement>,
    meta_slot: Option<AnyElement>,
    /// Fired when the back link is clicked. The spec's `back_href` is
    /// passed to the handler so callers can route it (GPUI has no
    /// built-in navigation concept — routing is app-level).
    on_back: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for PageHeader {
    type Target = PageHeaderSpec;
    fn deref(&self) -> &PageHeaderSpec { &self.spec }
}

impl PageHeader {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: PageHeaderSpec::new(title),
            theme: theme.clone(),
            breadcrumbs_slot: None,
            actions_slot: None,
            meta_slot: None,
            on_back: None,
        }
    }

    pub fn from_spec(spec: PageHeaderSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            breadcrumbs_slot: None,
            actions_slot: None,
            meta_slot: None,
            on_back: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self { self.spec.subtitle = Some(v.into()); self }
    pub fn eyebrow(mut self, v: impl Into<String>) -> Self { self.spec.eyebrow = Some(v.into()); self }
    pub fn align(mut self, v: PageHeaderAlign) -> Self { self.spec.align = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn with_size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }


    pub fn with_breadcrumbs(mut self, breadcrumbs: impl IntoElement) -> Self {
        self.breadcrumbs_slot = Some(breadcrumbs.into_any_element());
        self
    }

    pub fn with_actions(mut self, actions: impl IntoElement) -> Self {
        self.actions_slot = Some(actions.into_any_element());
        self
    }

    /// Metadata slot rendered between the header row and any banner.
    /// Typically a MetaBar with pills / timestamps.
    pub fn with_meta(mut self, meta: impl IntoElement) -> Self {
        self.meta_slot = Some(meta.into_any_element());
        self
    }

    // ── New spec builders ────────────────────────────────────
    pub fn section(mut self, v: impl Into<String>) -> Self { self.spec.section = Some(v.into()); self }
    pub fn count(mut self, v: u32) -> Self { self.spec.count = Some(v); self }
    pub fn back(mut self, href: impl Into<String>, label: impl Into<String>) -> Self {
        self.spec.back_href = Some(href.into());
        self.spec.back_label = Some(label.into());
        self
    }
    pub fn back_is_contextual(mut self, v: bool) -> Self { self.spec.back_is_contextual = v; self }
    pub fn banner(mut self, message: impl Into<String>, tone: poodle_primitives::StatusTone) -> Self {
        self.spec.banner_message = Some(message.into());
        self.spec.banner_tone = tone;
        self
    }

    /// Fired when the back link is clicked. Receives the back_href
    /// string so the caller can route navigation however it likes.
    pub fn on_back(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_back = Some(Box::new(handler));
        self
    }
}

impl IntoElement for PageHeader {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let font_size = rem_to_px(size_font_rem(effective_size));
        let _panel_px = rem_to_px(panel_space_x_rem(spec.density));
        let _panel_py = rem_to_px(panel_space_y_rem(spec.density));

        let gap = resolve_px(theme, spec.gap_token());
        let body_size = px(font_size);
        let heading_size = resolve_px(theme, "typography.heading.size");
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());

        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());
        let eyebrow_color = resolve_color(theme, spec.eyebrow_color_token());
        let back_color = resolve_color(theme, spec.back_color_token());
        let count_color = resolve_color(theme, spec.count_color_token());
        let banner_color = resolve_color(theme, spec.banner_color_token());
        let panel_radius = resolve_px(theme, "radius.control");

        let mut wrapper = div().w_full().flex().flex_col().py(padding_y);

        // Back-navigation link (above everything else).
        if spec.has_back_link() {
            let label = spec.back_label.clone().unwrap_or_default();
            let href = spec.back_href.clone().unwrap_or_default();
            let back_text_size = if spec.back_is_contextual {
                resolve_px(theme, "typography.caption.size")
            } else {
                body_size
            };
            let inline_xs = resolve_px(theme, "space.inline.sm");
            let mut back_row = div()
                .id("poodle-page-header-back")
                .w_full()
                .mb(gap)
                .flex()
                .items_center()
                .gap(inline_xs)
                .text_size(back_text_size)
                .text_color(back_color)
                .cursor_pointer()
                .child(div().child("\u{2190}"))
                .child(div().child(format!("Back to {label}")));

            if let Some(handler) = self.on_back {
                back_row = back_row.on_click(move |_event, window, cx| {
                    handler(&href, window, cx);
                });
            }

            wrapper = wrapper.child(back_row);
        }

        // Optional breadcrumbs above
        if let Some(breadcrumbs) = self.breadcrumbs_slot {
            wrapper = wrapper.child(
                div().w_full().mb(gap).child(breadcrumbs),
            );
        }

        // Section label (rendered above the main row, distinct from eyebrow)
        if let Some(ref section) = spec.section {
            wrapper = wrapper.child(
                div()
                    .w_full()
                    .mb(gap)
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(eyebrow_color)
                    .child(section.to_uppercase()),
            );
        }

        // Main row: title block + actions
        let justify = match spec.align {
            PageHeaderAlign::Between => true,
            PageHeaderAlign::Start => false,
        };

        let mut main_row = div()
            .w_full()
            .flex()
            .items_center()
            .gap(header_gap);

        if justify {
            main_row = main_row.justify_between();
        }

        // Title block: eyebrow, title row (+count), subtitle stacked
        let mut title_block = div().flex().flex_col().gap(gap);

        if let Some(ref eyebrow) = spec.eyebrow {
            title_block = title_block.child(
                div()
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(eyebrow_color)
                    .child(eyebrow.clone()),
            );
        }

        // Title row: title + optional count badge
        let mut title_row = div().flex().items_center().gap(px(8.0));
        title_row = title_row.child(
            div()
                .text_size(heading_size)
                .font_weight(FontWeight::BOLD)
                .text_color(title_color)
                .child(spec.title.clone()),
        );

        if let Some(count) = spec.count {
            title_row = title_row.child(
                div()
                    .text_size(body_size)
                    .text_color(count_color)
                    .child(format!("{count}")),
            );
        }

        title_block = title_block.child(title_row);

        if let Some(ref subtitle) = spec.subtitle {
            title_block = title_block.child(
                div()
                    .text_size(body_size)
                    .text_color(subtitle_color)
                    .child(subtitle.clone()),
            );
        }

        main_row = main_row.child(title_block);

        if let Some(actions) = self.actions_slot {
            main_row = main_row.child(
                div().flex().items_center().flex_shrink_0().gap(px(6.0)).child(actions),
            );
        }

        wrapper = wrapper.child(main_row);

        // Metadata row (MetaBar or similar) below the main header row.
        if let Some(meta) = self.meta_slot {
            wrapper = wrapper.child(
                div().w_full().mt(gap).child(meta),
            );
        }

        // Banner row
        if let Some(ref message) = spec.banner_message {
            let banner_bg = Hsla { a: banner_color.a * 0.12, ..banner_color };
            let banner_border = Hsla { a: banner_color.a * 0.38, ..banner_color };
            wrapper = wrapper.child(
                div()
                    .w_full()
                    .mt(gap)
                    .px(px(12.0))
                    .py(px(8.0))
                    .bg(banner_bg)
                    .border_1()
                    .border_color(banner_border)
                    .rounded(panel_radius)
                    .text_size(body_size)
                    .text_color(banner_color)
                    .child(message.clone()),
            );
        }

        wrapper.into_any_element()
    }
}
