//! Code — real GPUI component backed by CodeSpec.
//!
//! Contract: `docs/contracts/components/code.md`. Inline fragment + block
//! container with optional toolbar (language label + copy button), line
//! numbers, line highlighting, and max-height scroll. All geometry/colour
//! resolves from tokens; clipboard + 2s copied-state feedback is a
//! preview-loop interaction (render-only here).

use crate::primitives::Icon;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CodeInlineVariant, CodeSpec, CodeTypography, ControlDensity, ControlSize, IconSpec,
    SemanticControlSizeRole,
};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::theme_ext::{color_mix, color_mix_black, resolve_color, resolve_radius};

/// A real GPUI code display component backed by `CodeSpec`.
pub struct Code {
    spec: CodeSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Code {
    type Target = CodeSpec;
    fn deref(&self) -> &CodeSpec {
        &self.spec
    }
}

impl Code {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CodeSpec::new(),
            theme: theme.clone(),
        }
    }

    pub fn from_spec(spec: CodeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self {
        self.spec.content = v.into();
        self
    }
    pub fn language(mut self, v: impl Into<String>) -> Self {
        self.spec.language = Some(v.into());
        self
    }
    pub fn show_line_numbers(mut self, v: bool) -> Self {
        self.spec.show_line_numbers = v;
        self
    }
    pub fn copyable(mut self, v: bool) -> Self {
        self.spec.is_copyable = v;
        self
    }
    pub fn highlight_lines(mut self, v: Vec<usize>) -> Self {
        self.spec.highlight_lines = v;
        self
    }
    pub fn max_height(mut self, v: f64) -> Self {
        self.spec.max_height = Some(v);
        self
    }
    pub fn inline(mut self, v: bool) -> Self {
        self.spec.is_inline = v;
        self
    }
    pub fn inline_variant(mut self, v: CodeInlineVariant) -> Self {
        self.spec.inline_variant = v;
        self
    }
    pub fn typography(mut self, v: CodeTypography) -> Self {
        self.spec.typography = v;
        self
    }
    pub fn size(mut self, v: ControlSize) -> Self {
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

impl IntoElement for Code {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let text_color = resolve_color(theme, spec.text_color_token());
        let text_secondary = resolve_color(theme, spec.text_secondary_token());
        let panel = resolve_color(theme, spec.panel_token());
        let elevated = resolve_color(theme, spec.elevated_token());
        let canvas = resolve_color(theme, spec.canvas_token());
        let accent = resolve_color(theme, spec.accent_token());

        // ── Inline mode: span-like fragment ───────────────────────
        if spec.is_inline {
            // Font: per-size em scale × code adjustmentRatio (contract §8).
            // `typography="inline"` uses 1em × ratio instead of the body scale.
            // The ratio is a typed f32 token (no string-resolver channel for
            // plain float tokens — read the typed constant directly).
            let ratio = poodle_tokens::typed::semantic::TYPOGRAPHY_CODE_ADJUSTMENT_RATIO;
            let base_em = match spec.typography {
                CodeTypography::Inline => 1.0,
                CodeTypography::Body => size_font_rem(effective_size),
            };
            let inline_font = px(rem_to_px(base_em * ratio));

            let mut el = div()
                .text_size(inline_font)
                .text_color(text_color)
                .child(spec.content.clone());

            // `Default` carries inline chrome; `Plain` drops it (contract §8).
            if spec.inline_variant == CodeInlineVariant::Default {
                // bg = color-mix(panel 72%, elevated)
                let inline_bg = color_mix(panel, elevated, 0.72);
                el = el
                    .px(px(rem_to_px(0.375)))
                    .py(px(rem_to_px(0.125)))
                    .rounded(px(rem_to_px(0.25)))
                    .bg(inline_bg);
            }

            return el.into_any_element();
        }

        // ── Block mode ────────────────────────────────────────────
        // Pre padding (density-adjusted) — contract values match the panel
        // space scale exactly (default 0.75rem×1rem, compact 0.5×0.75, etc).
        let pre_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let pre_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        // Source font: contract 0.8125rem; size variants scale it.
        let source_font = px(rem_to_px(size_font_rem(effective_size)));
        let source_line_height = px(rem_to_px(size_font_rem(effective_size) * 1.4));

        // Block root: border-subtle (plain token via spec), surface radius,
        // NO background — the visible surface lives on the pre area.
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, spec.surface_radius_token());

        let mut el = div()
            .rounded(radius)
            .border_1()
            .border_color(border)
            .text_color(text_color)
            .overflow_hidden()
            .flex()
            .flex_col();

        // ── Toolbar (language label + copy button) ────────────────
        let has_toolbar = spec.language.is_some() || spec.is_copyable;
        if has_toolbar {
            // bg = color-mix(elevated 60%, panel); border-bottom = border-subtle.
            let toolbar_bg = color_mix(elevated, panel, 0.60);
            let mut toolbar = div()
                .flex()
                .items_center()
                .justify_between()
                // Contract toolbar padding: 0.375rem 0.625rem.
                .px(px(rem_to_px(0.625)))
                .py(px(rem_to_px(0.375)))
                .bg(toolbar_bg)
                .border_b_1()
                .border_color(border);

            // Language label: 0.6875rem, label-weight (500 = MEDIUM), uppercase.
            // Programmatic uppercase (GPUI has no text-transform). Per-element
            // font-family is omitted: the theme exposes no font-family resolver
            // channel (token gap — see notes).
            if let Some(ref lang) = spec.language {
                toolbar = toolbar.child(
                    div()
                        .text_size(px(rem_to_px(0.6875)))
                        .text_color(text_secondary)
                        .font_weight(FontWeight::MEDIUM)
                        .child(lang.to_uppercase()),
                );
            } else {
                toolbar = toolbar.child(div()); // spacer keeps actions right-aligned
            }

            // Copy button — bespoke 1.5rem square (contract §8 `.code__copy`),
            // transparent, text-secondary icon, hover→text-primary. Composes
            // the real Icon primitive for the 0.875rem SVG. Clipboard write +
            // 2s check-icon swap is a preview-loop interaction (render-only).
            if spec.is_copyable {
                toolbar = toolbar.child(
                    div()
                        .id("poodle-code-copy")
                        .cursor_pointer()
                        .w(px(rem_to_px(1.5)))
                        .h(px(rem_to_px(1.5)))
                        .rounded(px(rem_to_px(0.25)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(text_secondary)
                        .hover(move |s| s.text_color(text_color))
                        .child(
                            Icon::from_spec(IconSpec::new("copy"), theme)
                                // Contract icon size: 0.875rem square.
                                .with_px_size(rem_to_px(0.875))
                                .with_color(text_secondary),
                        ),
                );
            }

            el = el.child(toolbar);
        }

        // ── Code surface (pre + source) ───────────────────────────
        // bg = color-mix(canvas 92%, black) — pre carries the visible surface.
        let pre_bg = color_mix_black(canvas, 0.92);
        // Line-highlight bg = color-mix(accent-base 12%, transparent).
        let highlight_bg = accent.opacity(accent.a * 0.12);

        let mut scroll = div()
            .id("poodle-code-scroll")
            .bg(pre_bg)
            .px(pre_pad_x)
            .py(pre_pad_y)
            .text_size(source_font)
            .line_height(source_line_height)
            .overflow_x_scroll();

        // max-height → vertical scroll container (contract §7).
        if let Some(mh) = spec.max_height {
            scroll = scroll.max_h(px(mh as f32)).overflow_y_scroll();
        }

        let needs_per_line =
            spec.show_line_numbers || !spec.highlight_lines.is_empty();

        if needs_per_line {
            let lines: Vec<&str> = spec.content.lines().collect();
            let mut source_col = div().flex().flex_col();

            for (i, line) in lines.iter().enumerate() {
                let line_no = i + 1;
                let is_highlighted = spec.highlight_lines.contains(&line_no);
                let line_text = line.to_string();

                // Contract: highlighted line bleeds ±1rem so the accent band
                // reaches the pre's content edges. Negative horizontal margin
                // plus matching padding mirrors the Svelte `margin/padding 0 ±1rem`.
                let mut row = div().flex().items_baseline();
                if is_highlighted {
                    row = row
                        .bg(highlight_bg)
                        .mx(px(rem_to_px(-1.0)))
                        .px(px(rem_to_px(1.0)));
                }

                if spec.show_line_numbers {
                    // Gutter: 2.5rem wide, right-aligned, tabular, padding-right
                    // 1rem, text-secondary.
                    row = row.child(
                        div()
                            .w(px(rem_to_px(2.5)))
                            .pr(px(rem_to_px(1.0)))
                            .text_color(text_secondary)
                            .text_right()
                            .child(line_no.to_string()),
                    );
                }

                row = row.child(div().child(line_text));
                source_col = source_col.child(row);
            }

            scroll = scroll.child(source_col);
        } else {
            scroll = scroll.child(spec.content.clone());
        }

        el = el.child(scroll);

        el.into_any_element()
    }
}
