//! Pill — real GPUI component backed by PillSpec.

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{IconSize, IconSpec, PillSpec};

use super::icon::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// A real GPUI pill component backed by `PillSpec`.
pub struct Pill {
    spec: PillSpec,
    theme: GpuiThemeProvider,
    on_remove: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for Pill {
    type Target = PillSpec;
    fn deref(&self) -> &PillSpec { &self.spec }
}

impl Pill {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: PillSpec::new(), theme: theme.clone(), on_remove: None }
    }

    pub fn from_spec(spec: PillSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_remove: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = v.into(); self }
    pub fn removable(mut self, v: bool) -> Self { self.spec.is_removable = v; self }
    pub fn selected(mut self, v: bool) -> Self { self.spec.is_selected = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }


    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Pill {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let fill = resolve_color(theme, spec.fill_token());
        let text_color = resolve_color(theme, spec.text_color_token());
        let radius = resolve_radius(theme, "semantic.radius.pill");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let inline_gap = resolve_px(theme, "semantic.space.inline.xs");

        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());

        // Contract: font 0.75rem / 600
        let mut el = div()
            .id(SharedString::from("pug-pill"))
            .focusable()
            .px(px(8.0))
            .py(px(2.0))
            .rounded(radius)
            .bg(fill)
            .border_1()
            .border_color(fill) // subtle border same as fill
            .text_size(px(12.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(text_color)
            .flex()
            .items_center()
            .gap(inline_gap)
            .child(spec.label.clone());

        el = el.focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            el = el
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        if spec.is_removable {
            let icon_muted = resolve_color(theme, "semantic.color.icon.muted");
            let remove_id = SharedString::from("pug-pill-remove");
            let mut remove_btn = div()
                .id(remove_id)
                .cursor_pointer()
                .flex()
                .items_center()
                .child(
                    Icon::from_spec(
                        IconSpec::new("x").with_size(IconSize::Sm),
                        theme,
                    )
                    .with_color(icon_muted),
                );

            if let Some(handler) = self.on_remove {
                remove_btn =
                    remove_btn.on_click(move |event, window, cx| handler(event, window, cx));
            }

            el = el.child(remove_btn);
        }

        el.into_any_element()
    }
}
