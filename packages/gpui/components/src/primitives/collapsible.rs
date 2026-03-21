//! Collapsible — real GPUI component backed by CollapsibleSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::CollapsibleSpec;

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI collapsible component backed by `CollapsibleSpec`.
pub struct Collapsible {
    spec: CollapsibleSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    /// The content to show when expanded.
    content: Option<AnyElement>,
}

impl std::ops::Deref for Collapsible {
    type Target = CollapsibleSpec;
    fn deref(&self) -> &CollapsibleSpec { &self.spec }
}

impl Collapsible {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CollapsibleSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None, content: None }
    }

    pub fn from_spec(spec: CollapsibleSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            content: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }


    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

impl IntoElement for Collapsible {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let border = resolve_color(theme, "semantic.color.border.default");
        let is_open = spec.current_open();

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("pug-collapsible-{}", suffix)
        } else {
            "pug-collapsible".to_string()
        };

        // Header/trigger
        let mut header = div()
            .id(SharedString::from(id_str))
            .flex()
            .items_center()
            .gap(px(6.0))
            .py(px(6.0));

        if spec.activation_allowed() {
            header = header
                .cursor_pointer()
                .hover(|s| s.bg(hover_bg));
        } else {
            header = header.opacity(disabled_opacity);
        }

        // Expand indicator
        header = header.child(
            div()
                .text_xs()
                .text_color(text_secondary)
                .child(if is_open { "▾" } else { "▸" }),
        );

        // Title
        if let Some(ref title) = spec.title {
            header = header.child(div().text_sm().child(title.clone()));
        }

        // Click handler
        if let Some(handler) = self.on_toggle {
            if spec.activation_allowed() {
                let next_open = !is_open;
                header = header.on_click(move |_event, window, cx| {
                    handler(&next_open, window, cx);
                });
            }
        }

        let mut col = div()
            .flex()
            .flex_col()
            .border_b_1()
            .border_color(border)
            .child(header);

        // Content (when open)
        if is_open {
            if let Some(content) = self.content {
                col = col.child(div().py(px(6.0)).pl(px(16.0)).child(content));
            }
        }

        col.into_any_element()
    }
}
