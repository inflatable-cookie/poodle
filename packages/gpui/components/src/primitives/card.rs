use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{CardLayout, CardSpec, CardVariant};

pub struct Card {
    spec: CardSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
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
        let gap = resolve_px(theme, spec.gap_token());
        let padding_x = resolve_px(theme, spec.padding_x_token());
        let padding_y = resolve_px(theme, spec.padding_y_token());
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");

        // Contract: compact layout uses smaller padding (0.5rem x 0.625rem)
        let (effective_px, effective_py) = match spec.layout {
            CardLayout::Compact => (gap_md, gap_sm),
            _ => (padding_x, padding_y),
        };

        let border_subtle = resolve_color(theme, "color.border.subtle");
        let border_default = resolve_color(theme, "color.border.default");
        let panel = resolve_color(theme, "color.background.panel");
        let elevated = resolve_color(theme, "color.background.elevated");

        // Match Svelte Card.svelte:
        // Default/Outlined: --poodle-recipe-card-fill = color-mix(panel 98%, elevated)
        // Elevated: --poodle-treatment-surface-elevated-fill = color-mix(elevated 98%, panel)
        let fill = match spec.variant {
            CardVariant::Elevated => color_mix(elevated, panel, 0.98),
            _ => color_mix(panel, elevated, 0.98),
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

        // Border + shadow
        if let Some(sel_border) = selected_border_color {
            // Selected: accent border + shadow ring
            el = el
                .border_1()
                .border_color(sel_border)
                .shadow(vec![gpui::BoxShadow {
                    color: sel_border,
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(1.0),
                }]);
        } else if matches!(spec.variant, CardVariant::Elevated) {
            // Elevated: border + drop shadow (matches Svelte elevated card)
            if let Some(border) = border_color {
                el = el.border_1().border_color(border);
            }
            el = el.shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.38),
                    offset: point(px(0.0), px(18.0)),
                    blur_radius: px(40.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.24),
                    offset: point(px(0.0), px(6.0)),
                    blur_radius: px(14.0),
                    spread_radius: px(0.0),
                },
            ]);
        } else {
            // Default/Outlined: border + treatment inset shadow
            if let Some(border) = border_color {
                el = el.border_1().border_color(border);
            }
            // Svelte treatment-surface-shadow: inset 1px border-subtle at 18%
            el = el.shadow(vec![gpui::BoxShadow {
                color: Hsla {
                    a: border_subtle.a * 0.18,
                    ..border_subtle
                },
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(1.0),
            }]);
        }

        // Interactive hover
        if let Some(hover_fill) = hover_fill {
            el = el.cursor_pointer().hover(|s| s.bg(hover_fill));
        }

        // Header slot
        if let Some(header) = self.header {
            el = el.child(div().flex_shrink_0().child(header));
        }

        // Body slot
        if let Some(body) = self.body {
            el = el.child(div().flex_grow().child(body));
        }

        // Footer slot — with top divider per contract
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(footer_divider)
                    .pt(gap) // Use gap as top padding for visual separation
                    .child(footer),
            );
        }

        el.into_any_element()
    }
}
