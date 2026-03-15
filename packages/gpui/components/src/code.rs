//! PugCode — real GPUI component backed by CodeSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_gpui_primitives::CodeSpec;

use crate::theme_ext::{resolve_color, resolve_radius};

/// A real GPUI code display component backed by `CodeSpec`.
pub struct PugCode {
    spec: CodeSpec,
    theme: GpuiThemeProvider,
}

impl PugCode {
    pub fn new(spec: CodeSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
        }
    }
}

impl IntoElement for PugCode {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let border = resolve_color(theme, spec.border_token());
        let radius = resolve_radius(theme, "semantic.radius.control");

        let mut el = div()
            .px(px(12.0))
            .py(px(10.0))
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(border)
            .text_color(text_color)
            .text_xs()
            .overflow_hidden();

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
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_color(text_color.opacity(0.4))
                                .child(line_number),
                        )
                        .child(div().child(line_text)),
                );
            }

            el = el.child(content_col);
        } else {
            el = el.child(spec.content.clone());
        }

        el.into_any_element()
    }
}
