use crate::presentation::rem_to_px;
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardLayout, CardSpec, CardVariant};

pub struct Card {
    spec: CardSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    media: Option<AnyElement>,
    header: Option<AnyElement>,
    body: Option<AnyElement>,
    footer: Option<AnyElement>,
}

impl std::ops::Deref for Card {
    type Target = CardSpec;
    fn deref(&self) -> &CardSpec {
        &self.spec
    }
}

impl Card {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CardSpec::new(),
            theme: theme.clone(),
            id_prefix: "poodle-card".to_string(),
            media: None,
            header: None,
            body: None,
            footer: None,
        }
    }

    pub fn from_spec(spec: CardSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "poodle-card".to_string(),
            media: None,
            header: None,
            body: None,
            footer: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn variant(mut self, v: CardVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn layout(mut self, v: CardLayout) -> Self {
        self.spec.layout = v;
        self
    }
    pub fn with_density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
    pub fn interactive(mut self) -> Self {
        self.spec.is_interactive = true;
        self
    }
    pub fn selected(mut self) -> Self {
        self.spec.is_selected = true;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn with_media(mut self, media: impl IntoElement) -> Self {
        self.media = Some(media.into_any_element());
        self.spec.has_media = true;
        self
    }

    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn with_body(mut self, body: impl IntoElement) -> Self {
        self.body = Some(body.into_any_element());
        self
    }

    pub fn with_footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }
}

impl IntoElement for Card {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let spec = &self.spec;
        let theme = &self.theme;

        // ── Resolve tokens at render time ─────────────────────
        let radius = resolve_radius(theme, spec.radius_token());
        let selected_border_color = spec
            .selected_border_token()
            .map(|t| resolve_color(theme, t));
        let _disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let border_width = resolve_px(theme, spec.border_width_token());

        // Density/layout-aware padding, gap, and footer spacing (contract §8
        // density table). The spec drives the values; default-density maps to
        // tokens while compact/comfortable use the contract-exact rem fallbacks
        // (no exact token for 0.625rem / 0.875rem / 1rem).
        let gap = px(rem_to_px(spec.gap_rem()));
        let effective_px = px(rem_to_px(spec.padding_x_rem()));
        let effective_py = px(rem_to_px(spec.padding_y_rem()));
        let footer_pt = px(rem_to_px(spec.footer_padding_top_rem()));

        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_default = resolve_color(theme, "color.border.default");
        let panel = resolve_color(theme, "color.background.panel");
        let elevated = resolve_color(theme, "color.background.elevated");
        let canvas = resolve_color(theme, "color.background.canvas");
        let text_inverse = resolve_color(theme, "color.text.inverse");

        // Light vs dark — runtime-owned detection (contract Known Delta): a
        // high-lightness canvas background ⇒ light theme. Selects the elevated
        // shadow recipe.
        let is_light = canvas.l > 0.5;

        // Match Svelte Card.svelte:
        // Default/Outlined: --poodle-recipe-card-fill = color-mix(panel 10%, elevated)
        // Elevated: --poodle-treatment-surface-elevated-fill = color-mix(elevated 98%, panel)
        let fill = match spec.variant {
            CardVariant::Elevated => color_mix(elevated, panel, 0.98),
            _ => color_mix(panel, elevated, 0.10),
        };

        // Border: Default subtle at 18%, Outlined at 76% border-default,
        // Elevated uses treatment-surface-elevated-border
        let border_color = match spec.variant {
            CardVariant::Default => Some(Hsla {
                a: border_subtle.a * 0.18,
                ..border_subtle
            }),
            CardVariant::Outlined => Some(Hsla {
                a: border_default.a * 0.76,
                ..border_default
            }),
            // Svelte: color-mix(treatment-border-or-border-default 82%, border-default)
            // In default theme (no treatment override) = plain border-default
            CardVariant::Elevated => Some(border_default),
        };

        // Hover fill: treatment-surface-hover-fill
        let hover_fill = if spec.is_interactive {
            Some(color_mix(elevated, panel, 0.94))
        } else {
            None
        };

        // Footer divider: 52% border-subtle mixed with transparent
        let footer_divider = Hsla {
            a: border_subtle.a * 0.52,
            ..border_subtle
        };

        let is_horizontal = matches!(spec.layout, CardLayout::Horizontal);
        let card_id = SharedString::from(self.id_prefix.clone());

        // Accessibility semantics (contract section 6):
        // Root element is <article> in Svelte — GPUI uses div with article-like semantics
        // aria-label applied when provided via spec.aria_label
        // Interactive cards: known delta — role="button", tabindex="0", and keyboard
        // activation are documented but NOT yet implemented (matches Svelte parity)
        let mut el = div()
            .id(card_id)
            .bg(fill)
            .rounded(radius)
            .px(effective_px)
            .py(effective_py)
            .gap(gap)
            .overflow_hidden()
            // Focus ring
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

        // Layout direction
        if is_horizontal {
            el = el.flex().flex_row();
        } else {
            el = el.flex().flex_col();
        }

        // Border + shadow. Border width resolves from the border-width token;
        // shadow offsets/blurs resolve from the contract-exact rem values.
        let bw = rem_to_px(0.0625); // contract shadow geometry uses 0.0625rem ring/inset
        if let Some(sel_border) = selected_border_color {
            // Selected: accent border + accent ring + inset accent shadow
            // (contract §8 Selected state).
            el = el
                .border(border_width)
                .border_color(sel_border)
                .shadow(vec![
                    gpui::BoxShadow {
                        color: sel_border,
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(bw),
                    },
                    gpui::BoxShadow {
                        color: Hsla {
                            a: sel_border.a * 0.12,
                            ..sel_border
                        },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(bw),
                    },
                ]);
        } else if matches!(spec.variant, CardVariant::Elevated) {
            // Elevated: border + multi-layer drop shadow, light/dark-aware
            // (contract §8 elevated box-shadow tables).
            if let Some(border) = border_color {
                el = el.border(border_width).border_color(border);
            }
            let shadow = if is_light {
                // rgba(49,66,85,..) ≈ hsl with l~0.26; approximate via rgba()→hsla.
                let slate = |a: f32| -> Hsla { Rgba { r: 49.0 / 255.0, g: 66.0 / 255.0, b: 85.0 / 255.0, a }.into() };
                vec![
                    gpui::BoxShadow {
                        color: slate(0.1),
                        offset: point(px(0.0), px(rem_to_px(0.875))),
                        blur_radius: px(rem_to_px(1.75)),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: slate(0.06),
                        offset: point(px(0.0), px(rem_to_px(0.25))),
                        blur_radius: px(rem_to_px(0.625)),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 1.0, 0.72),
                        offset: point(px(0.0), px(bw)),
                        blur_radius: px(0.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: Hsla { a: border_default.a * 0.10, ..border_default },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(bw),
                    },
                ]
            } else {
                vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.38),
                        offset: point(px(0.0), px(rem_to_px(1.125))),
                        blur_radius: px(rem_to_px(2.5)),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.24),
                        offset: point(px(0.0), px(rem_to_px(0.375))),
                        blur_radius: px(rem_to_px(0.875)),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: Hsla { a: text_inverse.a * 0.10, ..text_inverse },
                        offset: point(px(0.0), px(bw)),
                        blur_radius: px(0.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: Hsla { a: border_default.a * 0.12, ..border_default },
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(bw),
                    },
                ]
            };
            el = el.shadow(shadow);
        } else {
            // Default/Outlined: border + treatment inset shadow
            if let Some(border) = border_color {
                el = el.border(border_width).border_color(border);
            }
            // Svelte treatment-surface-shadow: inset 0.0625rem border-subtle at 18%
            el = el.shadow(vec![gpui::BoxShadow {
                color: Hsla {
                    a: border_subtle.a * 0.18,
                    ..border_subtle
                },
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(bw),
            }]);
        }

        // Interactive hover
        if let Some(hover_fill) = hover_fill {
            el = el.cursor_pointer().hover(|s| s.bg(hover_fill));
        }

        // Media slot — overflow-clipped region with inset radius
        // (contract §8 Media: radius = card-radius - 0.1875rem).
        if let Some(media) = self.media {
            let media_radius = (radius - px(rem_to_px(spec.media_radius_inset_rem()))).max(px(0.0));
            el = el.child(
                div()
                    .flex_shrink_0()
                    .overflow_hidden()
                    .rounded(media_radius)
                    .child(media),
            );
        }

        // Header slot
        if let Some(header) = self.header {
            el = el.child(div().flex_shrink_0().child(header));
        }

        // Body slot
        if let Some(body) = self.body {
            el = el.child(div().flex_grow().child(body));
        }

        // Footer slot — with top divider and density-aware top padding (contract §8)
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(footer_divider)
                    .pt(footer_pt)
                    .child(footer),
            );
        }

        el.into_any_element()
    }
}
