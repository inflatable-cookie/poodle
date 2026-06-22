//! ListCard — real GPUI component backed by ListCardSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{LeadingFill, LeadingShape, ListCardLayout, ListCardSpec, SelectionIndicator};

use crate::presentation::rem_to_px;
use crate::theme_ext::{
    color_mix, parse_hex_color, resolve_color, resolve_opacity, resolve_px, resolve_radius,
};

/// A real GPUI list-card component backed by `ListCardSpec`.
pub struct ListCard {
    spec: ListCardSpec,
    theme: GpuiThemeProvider,
    leading: Option<AnyElement>,
    corner: Option<AnyElement>,
    footer: Option<AnyElement>,
    trailing: Option<AnyElement>,
    _icon_color: Hsla,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ListCard {
    type Target = ListCardSpec;
    fn deref(&self) -> &ListCardSpec {
        &self.spec
    }
}

impl ListCard {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = ListCardSpec::default();
        Self {
            spec,
            theme: theme.clone(),
            leading: None,
            corner: None,
            footer: None,
            trailing: None,
            _icon_color: gpui::white(),
            on_click: None,
        }
    }

    pub fn from_spec(spec: ListCardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            leading: None,
            corner: None,
            footer: None,
            trailing: None,
            _icon_color: gpui::white(),
            on_click: None,
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
    pub fn interactive(mut self, v: bool) -> Self {
        self.spec.is_interactive = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn href(mut self, v: impl Into<String>) -> Self {
        self.spec.href = Some(v.into());
        self.spec.is_interactive = true;
        self
    }
    pub fn selectable(mut self, v: bool) -> Self {
        self.spec.is_selectable = v;
        self
    }
    pub fn selected(mut self, v: bool) -> Self {
        self.spec.is_selected = v;
        self
    }
    pub fn reorder_handle(mut self, v: bool) -> Self {
        self.spec.show_reorder_handle = v;
        self
    }

    pub fn with_leading(mut self, element: impl IntoElement) -> Self {
        self.leading = Some(element.into_any_element());
        self
    }

    /// Supplementary header-corner content (contract §2 Corner) — rendered
    /// top-right in the header row, alongside the title, at the tertiary text
    /// color. Mirrors the `corner` snippet in the header-accessories cluster.
    pub fn with_corner(mut self, element: impl IntoElement) -> Self {
        self.corner = Some(element.into_any_element());
        self
    }

    pub fn with_footer(mut self, element: impl IntoElement) -> Self {
        self.footer = Some(element.into_any_element());
        self
    }

    pub fn with_trailing(mut self, element: impl IntoElement) -> Self {
        self.trailing = Some(element.into_any_element());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ListCard {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve tokens ──────────────────────────────────────────
        let surface = resolve_color(theme, spec.fill_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        // Svelte: fill = color-mix(surface 88%, text-primary); hover = 82%
        let fill = color_mix(surface, text_primary, 0.88);
        let hover_fill = color_mix(surface, text_primary, 0.82);
        let border_subtle = resolve_color(theme, spec.border_token());
        let border_default = resolve_color(theme, spec.hover_border_token());
        // Svelte: border = color-mix(border-subtle 18%, transparent)
        let border = Hsla { a: border_subtle.a * 0.18, ..border_subtle };
        // Svelte: hover border = color-mix(border-default 52%, transparent)
        let hover_border = Hsla { a: border_default.a * 0.52, ..border_default };
        let radius = resolve_radius(theme, spec.radius_token());
        let title_color = resolve_color(theme, spec.title_color_token());
        let subtitle_color = resolve_color(theme, spec.subtitle_color_token());
        let meta_color = resolve_color(theme, spec.meta_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring_color = resolve_color(theme, spec.focus_ring_color_token());

        let body_size = resolve_px(theme, "typography.body.size");
        let label_size = resolve_px(theme, "typography.label.size");
        let icon_md = resolve_px(theme, spec.selection_indicator_size_token());
        let radius_control = resolve_radius(theme, spec.leading_radius_token());
        // accentColor (contract §3) overrides the theme accent for the leading
        // background and icon; otherwise the resolved accent token wins.
        let theme_accent = resolve_color(theme, spec.accent_base_token());
        let accent = spec
            .accent_color
            .as_deref()
            .and_then(parse_hex_color)
            .unwrap_or(theme_accent);
        let on_accent = resolve_color(theme, spec.on_accent_color_token());
        // Svelte: leading tint = color-mix(accent 12%, transparent)
        let leading_tint_bg = Hsla { a: accent.a * spec.leading_tint_ratio(), ..accent };

        // Leading edge from the contract size ladder (circle 2rem / square 2.75rem,
        // compact shrinks one step). Radius: pill for circle, control for square.
        let leading_size = px(rem_to_px(spec.leading_size_rem()));
        let leading_radius = match spec.leading_shape {
            LeadingShape::Circle => leading_size * 0.5,
            LeadingShape::RoundedSquare => radius_control,
        };
        let leading_font = px(rem_to_px(spec.leading_font_size_rem()));

        let is_disabled = spec.is_disabled;
        let is_not_live = spec.is_not_live;
        let is_interactive = spec.is_interactive && !is_disabled;
        let is_stacked = spec.layout == ListCardLayout::Stacked;

        // ── Leading slot ────────────────────────────────────────────
        let leading_el = {
            let (bg, icon_color) = match spec.leading_fill {
                LeadingFill::Tint => (leading_tint_bg, accent),
                LeadingFill::Solid => (accent, on_accent),
            };

            let mut container = div()
                .w(leading_size)
                .h(leading_size)
                .rounded(leading_radius)
                .bg(bg)
                .text_size(leading_font)
                .text_color(icon_color)
                .flex()
                .items_center()
                .justify_center()
                .flex_shrink_0()
                .overflow_hidden();

            if let Some(leading) = self.leading {
                container = container.child(leading);
            }

            container
        };

        // ── Body section ────────────────────────────────────────────
        let body = {
            // Contract §8 Body gap 0.0625rem; Title weight 500 (medium).
            let mut col = div()
                .flex()
                .flex_col()
                .gap(px(rem_to_px(spec.body_gap_rem())))
                .flex_grow()
                .min_w_0();

            let title_el = div()
                .flex_grow()
                .min_w_0()
                .text_size(label_size)
                .font_weight(FontWeight::MEDIUM)
                .text_color(title_color)
                .overflow_x_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .child(spec.title.clone());

            // Header row — contract §2 Header: title + optional header-accessories
            // (corner). When a corner slot is present, the title and corner share
            // the header row; the corner cluster is shrink-proof and tertiary-colored
            // (contract §8 Header / Badges-and-Corner).
            if let Some(corner) = self.corner {
                let corner_color = resolve_color(theme, "color.text.tertiary");
                col = col.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(rem_to_px(0.375)))
                        .child(title_el)
                        .child(
                            // Header-accessories / corner cluster (contract §8):
                            // shrink-proof, inline-flex, gap inline-xs, tertiary color.
                            div()
                                .flex()
                                .flex_shrink_0()
                                .items_center()
                                .gap(resolve_px(theme, "space.inline.xs"))
                                .text_color(corner_color)
                                .child(corner),
                        ),
                );
            } else {
                col = col.child(title_el);
            }

            if let Some(ref subtitle) = spec.subtitle {
                col = col.child(
                    div()
                        .text_size(px(rem_to_px(spec.small_font_size_rem())))
                        .text_color(subtitle_color)
                        .overflow_x_hidden()
                        .text_ellipsis()
                        .whitespace_nowrap()
                        .child(subtitle.clone()),
                );
            }

            if let Some(footer) = self.footer {
                // Contract §8 Footer gap/margin-top 0.125rem.
                col = col.child(div().mt(px(rem_to_px(0.125))).child(footer));
            }

            col
        };

        // ── Meta section (contract §8 Meta: 0.75rem) ─────────────────
        let meta_el = spec.meta.as_ref().map(|meta| {
            div()
                .text_size(px(rem_to_px(spec.small_font_size_rem())))
                .text_color(meta_color)
                .flex_shrink_0()
                .child(meta.clone())
        });
        let _ = body_size;

        // ── Sash badge (top-left ribbon) ──────────────────────────
        // Contract §8 sash: top 0.34375rem, left -2.25rem, width 6rem,
        // rotate(-45deg). GPUI 0.2.2 `div` has no rotation transform, so the
        // ribbon is placed in the top-left corner without the diagonal rotate.
        // NOTE: rotation is a GPUI platform gap.
        let sash_el = spec.sash.as_ref().map(|sash_text| {
            let sash_bg = spec
                .sash_color
                .as_ref()
                .and_then(|c| parse_hex_color(c))
                .unwrap_or_else(|| resolve_color(theme, spec.sash_bg_token()));

            div()
                .absolute()
                .top(px(rem_to_px(0.34375)))
                .left(px(0.0))
                .px(px(rem_to_px(0.375)))
                .py(px(rem_to_px(0.125)))
                .bg(sash_bg)
                .text_color(resolve_color(theme, spec.on_accent_color_token()))
                .text_size(px(rem_to_px(0.5625)))
                .font_weight(FontWeight::BOLD)
                .line_height(px(rem_to_px(0.75)))
                .child(sash_text.to_uppercase())
        });

        let has_sash = sash_el.is_some();

        // ── Root container ──────────────────────────────────────────
        // Accessibility semantics (contract section 6):
        // When interactive/selectable: role="button", tabindex="0", aria-label from spec or title
        // When selectable: aria-pressed={selected}
        // When disabled: aria-disabled="true"
        // When href present: rendered as anchor (link semantics)
        // Non-interactive: no role (generic container)
        // Highlighted (contract §8 Root highlighted): accent-tinted border + an
        // accent-over-fill composite (gradient approximated as a flat blend).
        let (root_fill, root_border) = if spec.is_highlighted {
            (
                color_mix(accent, fill, 0.10),
                Hsla { a: accent.a * 0.34, ..accent },
            )
        } else {
            (fill, border)
        };

        let root = div()
            .id(SharedString::from(format!(
                "poodle-list-card-{}",
                spec.title
            )))
            .w_full()
            .px(resolve_px(theme, "space.inline.md"))
            .py(px(rem_to_px(0.625)))
            .flex()
            .gap(resolve_px(theme, "space.inline.md"))
            .rounded(radius)
            .bg(root_fill)
            .border_1()
            .border_color(root_border);

        // Stacked layout: column with top leading area; default: centered row.
        let root = if is_stacked {
            root.flex_col().items_start()
        } else {
            root.flex_row().items_center()
        };

        // Not-live: dashed border (contract 0.1875rem dashed). GPUI border width
        // is the 1px hairline; greyscale filter has no GPUI API. NOTE: greyscale
        // and the thicker dashed stroke are platform gaps.
        let mut root = if is_not_live {
            root.border_dashed()
        } else {
            root
        };

        // Selection indicator (contract §3): only when selectable AND
        // selectionIndicator="checkbox". Inline checkbox box; accent-filled
        // when selected.
        if spec.is_selectable && spec.selection_indicator == SelectionIndicator::Checkbox {
            let surface = resolve_color(theme, "color.background.surface");
            let box_bg = if spec.is_selected { accent } else { surface };
            let box_border = if spec.is_selected {
                accent
            } else {
                border_subtle
            };
            root = root.child(
                div()
                    .flex_shrink_0()
                    .w(icon_md)
                    .h(icon_md)
                    .rounded(radius_control)
                    .border_1()
                    .border_color(box_border)
                    .bg(box_bg),
            );
        }

        root = root.child(leading_el).child(body);

        if has_sash {
            root = root.relative().overflow_hidden();
        }

        if let Some(meta) = meta_el {
            root = root.child(meta);
        }

        if let Some(trailing) = self.trailing {
            root = root.child(trailing);
        }

        // Reorder drag handle (last child before the sash overlay).
        if spec.show_reorder_handle {
            let handle_color = resolve_color(theme, "color.text.secondary");
            // Two columns of three dots; dot size/gap from the inline-xs token.
            let dot_px = px(rem_to_px(0.1875));
            let gap_px = resolve_px(theme, "space.inline.xs") * 0.5;
            let dot = move |color: Hsla| div().w(dot_px).h(dot_px).rounded(dot_px * 0.5).bg(color);
            let col = move || {
                div()
                    .flex()
                    .flex_col()
                    .gap(gap_px)
                    .child(dot(handle_color))
                    .child(dot(handle_color))
                    .child(dot(handle_color))
            };
            root = root.child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .gap(gap_px)
                    .opacity(0.6)
                    .cursor(CursorStyle::OpenHand)
                    .child(col())
                    .child(col()),
            );
        }

        if let Some(sash) = sash_el {
            root = root.child(sash);
        }

        root = root.focus(move |s| {
            s.border_color(focus_ring_color)
                .shadow(crate::theme_ext::focus_ring_shadow(focus_ring_color))
        });

        // ── Interactive hover state ─────────────────────────────────
        if is_interactive {
            root = root
                .cursor_pointer()
                .hover(|style| style.bg(hover_fill).border_color(hover_border));
        }

        // ── Not-live state: reduced opacity (contract §8: 0.72) ──────
        if is_not_live {
            root = root.opacity(spec.not_live_opacity());
        }

        // ── Disabled state ──────────────────────────────────────────
        if is_disabled {
            root = root
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        root.into_any_element()
    }
}
