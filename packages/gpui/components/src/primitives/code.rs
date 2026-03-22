//! Code — real GPUI component backed by CodeSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{CodeSpec, IconSize, IconSpec};
use crate::primitives::Icon;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI code display component backed by `CodeSpec`.
pub struct Code {
    spec: CodeSpec,
    theme: GpuiThemeProvider,
    is_inline: bool,
    max_height_px: Option<f32>,
}

impl std::ops::Deref for Code {
    type Target = CodeSpec;
    fn deref(&self) -> &CodeSpec { &self.spec }
}

impl Code {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CodeSpec::new(), theme: theme.clone(), is_inline: false, max_height_px: None }
    }

    pub fn from_spec(spec: CodeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            is_inline: false,
            max_height_px: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = v.into(); self }
    pub fn language(mut self, v: impl Into<String>) -> Self { self.spec.language = Some(v.into()); self }
    pub fn show_line_numbers(mut self, v: bool) -> Self { self.spec.show_line_numbers = v; self }
    pub fn copyable(mut self, v: bool) -> Self { self.spec.is_copyable = v; self }

    // ── Local builders ────────────────────────────────────────
    pub fn inline(mut self, v: bool) -> Self { self.is_inline = v; self }
    pub fn max_height(mut self, v: f32) -> Self { self.max_height_px = Some(v); self }

}

impl IntoElement for Code {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());

        // ── Inline mode: minimal span-like rendering ──────────
        if self.is_inline {
            return div()
                .px(px(4.0))
                .py(px(1.0))
                .rounded(px(3.0))
                .bg(fill.opacity(0.6))
                .text_size(px(13.0))
                .text_color(text_color)
                .child(spec.content.clone())
                .into_any_element();
        }

        // ── Block mode ────────────────────────────────────────
        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let border = resolve_color(theme, spec.border_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        // Contract: radius-surface, not radius-control
        let radius = resolve_radius(theme, "semantic.radius.surface");

        let mut el = div()
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border)
            .text_color(text_color)
            // Contract: font 0.8125rem (13px), code family
            .text_size(px(13.0))
            .overflow_hidden()
            .flex()
            .flex_col();

        // Contract: toolbar with language label and copy button
        let has_toolbar = spec.language.is_some() || spec.is_copyable;
        if has_toolbar {
            let mut toolbar = div()
                .flex()
                .items_center()
                .justify_between()
                .px(panel_x)
                .py(px(6.0)) // 0.375rem
                .border_b_1()
                .border_color(border);

            // Language label
            if let Some(ref lang) = spec.language {
                toolbar = toolbar.child(
                    div()
                        .text_size(px(11.0)) // 0.6875rem
                        .text_color(text_secondary)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(lang.to_uppercase()),
                );
            } else {
                toolbar = toolbar.child(div()); // spacer
            }

            // Copy button placeholder
            if spec.is_copyable {
                toolbar = toolbar.child(
                    div()
                        .cursor_pointer()
                        .w(px(24.0)).h(px(24.0))
                        .rounded(px(4.0))
                        .flex().items_center().justify_center()
                        .hover(|s| s.bg(fill.opacity(0.8)))
                        .child(
                            Icon::from_spec(
                                IconSpec::new("clipboard-copy").with_size(IconSize::Sm),
                                theme,
                            ).with_color(text_secondary)
                        ),
                );
            }

            el = el.child(toolbar);
        }

        // Code content area
        let mut code_area = div().id("pug-code-area").px(panel_x).py(panel_y);

        // Apply max_height constraint (default 320px per contract)
        let mh = self.max_height_px.unwrap_or(320.0);
        code_area = code_area.max_h(px(mh)).overflow_y_scroll();

        // Content with optional line numbers
        if spec.show_line_numbers {
            let lines: Vec<&str> = spec.content.lines().collect();
            let mut content_col = div().flex().flex_col();

            for (i, line) in lines.iter().enumerate() {
                let line_number = format!("{:>3} ", i + 1);
                let line_text = line.to_string();
                content_col = content_col.child(
                    div()
                        .flex()
                        .gap(inline_gap)
                        .child(
                            div()
                                .text_color(text_color.opacity(0.4))
                                .child(line_number),
                        )
                        .child(div().child(line_text)),
                );
            }

            code_area = code_area.child(content_col);
        } else {
            code_area = code_area.child(spec.content.clone());
        }

        el = el.child(code_area);

        el.into_any_element()
    }
}
