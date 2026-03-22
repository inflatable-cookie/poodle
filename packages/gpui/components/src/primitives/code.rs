//! Code — real GPUI component backed by CodeSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::CodeSpec;

use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI code display component backed by `CodeSpec`.
pub struct Code {
    spec: CodeSpec,
    theme: GpuiThemeProvider,
}

impl std::ops::Deref for Code {
    type Target = CodeSpec;
    fn deref(&self) -> &CodeSpec { &self.spec }
}

impl Code {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CodeSpec::new(), theme: theme.clone() }
    }

    pub fn from_spec(spec: CodeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn content(mut self, v: impl Into<String>) -> Self { self.spec.content = v.into(); self }
    pub fn language(mut self, v: impl Into<String>) -> Self { self.spec.language = Some(v.into()); self }
    pub fn show_line_numbers(mut self, v: bool) -> Self { self.spec.show_line_numbers = v; self }
    pub fn copyable(mut self, v: bool) -> Self { self.spec.is_copyable = v; self }

}

impl IntoElement for Code {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let panel_x = resolve_px(theme, "semantic.space.panel.x");
        let panel_y = resolve_px(theme, "semantic.space.panel.y");
        let inline_gap = resolve_px(theme, "semantic.space.inline.sm");

        let fill = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());
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
                        .text_size(px(11.0))
                        .text_color(text_secondary)
                        .cursor_pointer()
                        .child("Copy"),
                );
            }

            el = el.child(toolbar);
        }

        // Code content area
        let mut code_area = div().px(panel_x).py(panel_y);

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
