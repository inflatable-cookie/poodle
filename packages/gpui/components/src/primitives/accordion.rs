//! Accordion — real GPUI component backed by AccordionSpec.

use std::rc::Rc;
use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{AccordionItemSpec, AccordionSelectionValue, AccordionSpec};

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI accordion component backed by `AccordionSpec`.
pub struct Accordion {
    spec: AccordionSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_toggle: Option<Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Accordion {
    type Target = AccordionSpec;
    fn deref(&self) -> &AccordionSpec { &self.spec }
}

impl Accordion {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: AccordionSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_toggle: None }
    }

    pub fn from_spec(spec: AccordionSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "pug-accordion".to_string(),
            on_toggle: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn items(mut self, v: Vec<AccordionItemSpec>) -> Self { self.spec.items = v; self }
    pub fn value(mut self, v: AccordionSelectionValue) -> Self { self.spec.value = Some(v); self }
    pub fn default_value(mut self, v: AccordionSelectionValue) -> Self { self.spec.default_value = Some(v); self }
    pub fn allow_multiple(mut self, v: bool) -> Self { self.spec.allow_multiple = v; self }
    pub fn collapsible(mut self, v: bool) -> Self { self.spec.is_collapsible = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for Accordion {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let border = resolve_color(theme, self.spec.border_color_token());
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");

        let expanded = self.spec.expanded_values();

        let mut col = div().flex().flex_col();

        for item in &self.spec.items {
            let is_open = expanded.contains(&item.value.as_str());
            let is_disabled = item.is_disabled;
            let item_id = SharedString::from(format!("{}-{}", self.id_prefix, item.value));

            // Header
            let mut header = div()
                .id(item_id)
                .flex()
                .items_center()
                .justify_between()
                .py(px(8.0))
                .border_b_1()
                .border_color(border);

            if !is_disabled {
                header = header
                    .cursor_pointer()
                    .hover(|s| s.bg(hover_bg));
            } else {
                header = header.opacity(disabled_opacity);
            }

            header = header
                .child(div().text_sm().child(item.label.clone()))
                .child(
                    div()
                        .text_xs()
                        .text_color(text_secondary)
                        .child(if is_open { "▾" } else { "▸" }),
                );

            // Click handler
            if !is_disabled {
                if let Some(ref handler) = self.on_toggle {
                    let handler = handler.clone();
                    let value = item.value.clone();
                    header = header.on_click(move |_event, window, cx| {
                        handler(&value, window, cx);
                    });
                }
            }

            col = col.child(header);

            // Content (when expanded)
            if is_open {
                if let Some(ref desc) = item.description {
                    col = col.child(
                        div()
                            .py(px(8.0))
                            .pl(px(8.0))
                            .border_b_1()
                            .border_color(border)
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(text_secondary)
                                    .child(desc.clone()),
                            ),
                    );
                }
            }
        }

        col.into_any_element()
    }
}
