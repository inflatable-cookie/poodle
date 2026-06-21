//! PageHeader — real GPUI component backed by PageHeaderSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ControlDensity, ControlSize, IconSize, IconSpec, PillAppearance, PillSize, PillSpec, PillTone,
    SemanticControlSizeRole,
};
use poodle_specs::{PageHeaderAlign, PageHeaderSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, resolve_supporting_visual_size, size_font_rem,
};
use crate::primitives::{Icon, Pill};
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// Map the resolved control size to the supporting-visual `PillSize` the count
/// badge renders at (matches Svelte `resolveSupportingVisualSize`).
fn count_pill_size(size: ControlSize) -> PillSize {
    match resolve_supporting_visual_size(size) {
        ControlSize::Xs => PillSize::Xs,
        ControlSize::Sm => PillSize::Sm,
        ControlSize::Md => PillSize::Md,
        ControlSize::Lg => PillSize::Lg,
        ControlSize::Xl => PillSize::Xl,
    }
}

/// Per-level heading scale over the base heading-size token (level 2 = base,
/// level 1 larger, levels 3–6 compact). Mirrors the Svelte title-size ladder.
fn level_scale(level: u8) -> f32 {
    match level {
        1 => 1.143,
        2 => 1.0,
        _ => 0.714,
    }
}

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
    fn deref(&self) -> &PageHeaderSpec {
        &self.spec
    }
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
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn subtitle(mut self, v: impl Into<String>) -> Self {
        self.spec.subtitle = Some(v.into());
        self
    }
    pub fn eyebrow(mut self, v: impl Into<String>) -> Self {
        self.spec.eyebrow = Some(v.into());
        self
    }
    pub fn align(mut self, v: PageHeaderAlign) -> Self {
        self.spec.align = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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
    pub fn section(mut self, v: impl Into<String>) -> Self {
        self.spec.section = Some(v.into());
        self
    }
    pub fn count(mut self, v: u32) -> Self {
        self.spec.count = Some(v);
        self
    }
    pub fn level(mut self, v: u8) -> Self {
        self.spec.level = v.clamp(1, 6);
        self
    }
    pub fn back(mut self, href: impl Into<String>, label: impl Into<String>) -> Self {
        self.spec.back_href = Some(href.into());
        self.spec.back_label = Some(label.into());
        self
    }
    pub fn back_is_contextual(mut self, v: bool) -> Self {
        self.spec.back_is_contextual = v;
        self
    }
    pub fn banner(mut self, message: impl Into<String>, tone: poodle_specs::StatusTone) -> Self {
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

        let gap = resolve_px(theme, spec.gap_token());
        let body_size = px(font_size);
        // Title size: scale the heading-size token per level (level 2 = base).
        let heading_base = resolve_px(theme, spec.heading_size_token());
        let heading_size = heading_base * level_scale(spec.level);
        let header_gap = resolve_px(theme, spec.header_gap_token());
        let title_block_gap = resolve_px(theme, spec.title_block_gap_token());
        let title_gap = resolve_px(theme, spec.title_gap_token());
        let actions_gap = resolve_px(theme, spec.actions_gap_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());
        let banner_radius = resolve_px(theme, spec.banner_radius_token());

        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());
        let eyebrow_color = resolve_color(theme, spec.eyebrow_color_token());
        let section_color = resolve_color(theme, spec.section_color_token());
        let back_color = resolve_color(theme, spec.back_color_token());
        let context_dot = resolve_color(theme, spec.context_dot_color_token());
        let banner_color = resolve_color(theme, spec.banner_color_token());
        let surface = resolve_color(theme, "color.background.surface");

        let primary_title = spec.primary_title();
        let resolved_subtitle = spec.resolved_subtitle();

        let mut wrapper = div().w_full().flex().flex_col().gap(gap).py(padding_y);

        // ── Title block: eyebrow, section, title (+count Pill), subtitle ──────
        let mut title_block = div().flex().flex_col().gap(title_block_gap);

        if let Some(ref eyebrow) = spec.eyebrow {
            title_block = title_block.child(
                div()
                    .text_size(px(rem_to_px(0.6875)))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(eyebrow_color)
                    .child(eyebrow.to_uppercase()),
            );
        }

        // Section row — distinct from eyebrow (only in default posture w/ split).
        if spec.has_section_title_split() && !spec.is_entity_detail_posture() {
            if let Some(ref section) = spec.section {
                title_block = title_block.child(
                    div()
                        .text_size(px(rem_to_px(0.75)))
                        .font_weight(FontWeight::BOLD)
                        .text_color(section_color)
                        .child(section.to_uppercase()),
                );
            }
        }

        // Title row: heading + optional count Pill.
        let mut title_row = div().flex().items_center().gap(title_gap);
        title_row = title_row.child(
            div()
                .text_size(heading_size)
                .font_weight(FontWeight::BOLD)
                .text_color(title_color)
                .child(primary_title),
        );
        if let Some(count) = spec.count {
            title_row = title_row.child(
                Pill::from_spec(
                    PillSpec::new()
                        .with_label(format!("{count}"))
                        .with_tone(PillTone::Neutral)
                        .with_appearance(PillAppearance::Subtle)
                        .with_size(count_pill_size(effective_size)),
                    theme,
                ),
            );
        }
        title_block = title_block.child(title_row);

        if let Some(ref subtitle) = resolved_subtitle {
            title_block = title_block.child(
                div()
                    .text_size(body_size)
                    .text_color(subtitle_color)
                    .child(subtitle.clone()),
            );
        }

        // ── Actions row: back link (left) + actions cluster (right) ──────────
        let has_actions_row = spec.has_back_link() || self.actions_slot.is_some();
        let actions_row = if has_actions_row {
            let mut row = div()
                .flex()
                .items_center()
                .flex_shrink_0()
                .gap(header_gap);

            if spec.has_back_link() {
                let href = spec.back_href.clone().unwrap_or_default();
                let display = spec.back_display_label();
                // Back link: arrow-left icon + stripped display label + dot.
                let mut back_row = div()
                    .id("poodle-page-header-back")
                    .flex()
                    .items_center()
                    .gap(px(rem_to_px(0.35)))
                    .text_size(px(rem_to_px(0.8125)))
                    .text_color(back_color)
                    .cursor_pointer()
                    .child(
                        Icon::from_spec(
                            IconSpec::new("arrow-left").with_size(IconSize::Sm),
                            theme,
                        )
                        .with_color(back_color),
                    )
                    .child(div().child(display));

                if spec.back_is_contextual {
                    back_row = back_row.child(
                        div()
                            .w(px(rem_to_px(0.375)))
                            .h(px(rem_to_px(0.375)))
                            .rounded(px(rem_to_px(0.1875)))
                            .flex_shrink_0()
                            .bg(context_dot),
                    );
                }

                if let Some(handler) = self.on_back {
                    back_row = back_row.on_click(move |_event, window, cx| {
                        handler(&href, window, cx);
                    });
                }
                row = row.child(back_row);
            }

            if let Some(actions) = self.actions_slot {
                row = row.child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_center()
                        .gap(actions_gap)
                        .child(actions),
                );
            }
            Some(row)
        } else {
            None
        };

        // ── Top row: title block + actions row ───────────────────────────────
        let mut top_row = div().w_full().flex().items_start().gap(header_gap);
        if matches!(spec.align, PageHeaderAlign::Between) {
            top_row = top_row.justify_between();
        }
        top_row = top_row.child(title_block);
        if let Some(actions_row) = actions_row {
            top_row = top_row.child(actions_row);
        }
        wrapper = wrapper.child(top_row);

        // ── Secondary content: breadcrumbs / meta ────────────────────────────
        if let Some(breadcrumbs) = self.breadcrumbs_slot {
            wrapper = wrapper.child(div().w_full().child(breadcrumbs));
        }
        if let Some(meta) = self.meta_slot {
            wrapper = wrapper.child(div().w_full().child(meta));
        }

        // ── Banner row ───────────────────────────────────────────────────────
        if let Some(ref message) = spec.banner_message {
            // Tinted fill / border derived from the tone color mixed into surface.
            let banner_bg = color_mix(banner_color, surface, 0.12);
            let banner_border = color_mix(banner_color, surface, 0.38);
            wrapper = wrapper.child(
                div()
                    .w_full()
                    .px(px(rem_to_px(0.75)))
                    .py(px(rem_to_px(0.5)))
                    .bg(banner_bg)
                    .border_l_2()
                    .border_color(banner_border)
                    .rounded(banner_radius)
                    .text_size(body_size)
                    .text_color(banner_color)
                    .child(message.clone()),
            );
        }

        wrapper.into_any_element()
    }
}
