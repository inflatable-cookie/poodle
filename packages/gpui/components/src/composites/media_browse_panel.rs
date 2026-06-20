//! MediaBrowsePanel — grid of selectable media items with loading/error/empty states.
//!
//! Composes the real primitives per the contract (§2 Anatomy):
//!   - error state → `Callout` (danger tone)
//!   - each grid item's media shell → `MediaThumbnail` (compact, square)
//!   - load-more action → `Button` (secondary variant, disabled while loading)

use gpui::*;
use poodle_specs::{
    AspectRatio, ButtonSpec, ButtonVariant, CallOutSpec, MediaKind, MediaThumbnailSpec,
    StatusTone,
};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MediaBrowsePanelSpec;
use poodle_specs::{ControlDensity, ControlSize, SemanticControlSizeRole};

use crate::composites::media_thumbnail::MediaThumbnail;
use crate::presentation::{control_space_x_rem, rem_to_px, resolve_semantic_size, size_font_rem};
use crate::primitives::{Button, Callout};
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

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

/// Map the item's free-form `kind` string to a `MediaKind` (contract `toMediaKind`:
/// defaults to `image`). Svelte falls back to `image`; Jetstream falls back to
/// `embed`. We follow the contract/Svelte default of `image`.
fn to_media_kind(kind: &str) -> MediaKind {
    match kind {
        "image" => MediaKind::Image,
        "audio" => MediaKind::Audio,
        "video" => MediaKind::Video,
        "document" => MediaKind::Document,
        "embed" => MediaKind::Embed,
        _ => MediaKind::Image,
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
        // Contract §8 Size Adjustments: xs 8.5 / sm 10 / md 11 / lg 12 / xl 13.
        let grid_min_w = px(rem_to_px(match effective_size {
            ControlSize::Xs => 8.5,
            ControlSize::Sm => 10.0,
            ControlSize::Md => 11.0,
            ControlSize::Lg => 12.0,
            ControlSize::Xl => 13.0,
        }));
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let surface_bg = resolve_color(theme, "color.background.panel");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_focus = resolve_color(theme, "color.accent.focusRing");
        let radius = resolve_px(theme, "radius.surface");
        // Contract §8 Item / Meta font-size = 0.8125rem (13px). Resolved from the
        // caption typography token rather than a literal.
        let meta_size = resolve_px(theme, "typography.caption.size");

        // Contract §8: item border `0.0625rem solid border-subtle` (full alpha);
        // background `color-mix(in srgb, background-panel 92%, transparent)`.
        // color-mix against transparent reduces alpha proportionally.
        let item_bg = color_mix(surface_bg, gpui::transparent_black(), 0.92);
        // Item :hover / :focus-visible → border-focus, bg color-mix(elevated 90%, transparent).
        let item_hover_bg = color_mix(elevated_bg, gpui::transparent_black(), 0.90);

        let mut panel = div().flex().flex_col().w_full().min_h(px(rem_to_px(18.0)));

        if spec.loading && spec.items.is_empty() {
            // Contract §2: loading state is centered <p> copy.
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
            // Contract §2/§6: error state uses the Callout primitive (danger tone).
            panel = panel.child(
                div()
                    .w_full()
                    .min_h(px(rem_to_px(18.0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Callout::from_spec(
                        CallOutSpec::new()
                            .with_tone(StatusTone::Danger)
                            .with_content(error.clone())
                            .with_size(spec.size)
                            .with_size_role(spec.size_role)
                            .with_density(spec.density),
                        theme,
                    )),
            );
        } else if spec.items.is_empty() {
            // Contract §2: empty state is centered <p> copy.
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
            let mut grid = div().flex().flex_wrap().gap(grid_gap).w_full().mt(grid_gap);
            for item in &spec.items {
                let item_id = SharedString::from(format!("poodle-media-browse-item-{}", item.id));
                let meta_text = item.meta.clone().unwrap_or_else(|| item.kind.clone());
                grid = grid.child(
                    div()
                        .id(item_id)
                        .focusable()
                        .min_w(grid_min_w)
                        .flex_grow()
                        .rounded(radius)
                        .border_1()
                        .border_color(border_subtle)
                        .bg(item_bg)
                        .flex()
                        .flex_col()
                        .gap(item_gap)
                        .p(item_pad)
                        .cursor_pointer()
                        // Contract §8 :hover / :focus-visible.
                        .hover(move |s| s.border_color(border_focus).bg(item_hover_bg))
                        .focus(move |s| s.border_color(border_focus).bg(item_hover_bg))
                        // Thumbnail — real MediaThumbnail primitive (compact = no caption,
                        // square aspect ratio) per contract §2/§7.
                        .child(
                            MediaThumbnail::from_spec(
                                MediaThumbnailSpec::new(to_media_kind(&item.kind))
                                    .with_aspect_ratio(AspectRatio::Square)
                                    .with_show_caption(false)
                                    .with_title(item.label.clone()),
                                theme,
                            ),
                        )
                        // Label
                        .child(
                            div()
                                .text_size(body_size)
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(text_primary)
                                .child(item.label.clone()),
                        )
                        // Meta
                        .child(
                            div()
                                .text_size(meta_size)
                                .text_color(text_secondary)
                                .child(meta_text),
                        ),
                );
            }
            panel = panel.child(grid);

            if spec.has_more {
                // Contract §2: load-more is the Button primitive (secondary variant,
                // disabled while loading; label switches to "Loading...").
                let label = if spec.loading {
                    "Loading...".to_string()
                } else {
                    spec.load_more_label.clone()
                };

                panel = panel.child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_center()
                        .mt(grid_gap)
                        .child(
                            Button::from_spec(
                                ButtonSpec::new()
                                    .with_variant(ButtonVariant::Secondary)
                                    .with_size(spec.size)
                                    .with_size_role(spec.size_role)
                                    .with_density(spec.density)
                                    .with_label(label)
                                    .with_disabled(spec.loading),
                                theme,
                            )
                            .with_id("media-browse-load-more"),
                        ),
                );
            }
        }

        panel.into_any_element()
    }
}
