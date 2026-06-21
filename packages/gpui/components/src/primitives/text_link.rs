use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::TextLinkSpec;

use crate::theme_ext::{focus_ring_shadow, resolve_color, resolve_opacity};

pub struct TextLink {
    spec: TextLinkSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TextLink {
    pub fn new(label: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(TextLinkSpec::new(label), theme)
    }

    pub fn from_spec(spec: TextLinkSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TextLink {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let color = resolve_color(&self.theme, self.spec.color_token());
        let disabled_opacity = resolve_opacity(&self.theme, "state.opacity.disabled");
        let focus_ring = resolve_color(&self.theme, "color.accent.focusRing");

        // Svelte underline treatment (TextLink.svelte:76-93): rest decoration color
        // is a subtle current-color mix (`color-mix(currentColor 55%, transparent)`
        // → text color at 0.55 alpha); hover and focus-visible strengthen it to the
        // full current color. GPUI has no `currentColor`, so the resolved tone color
        // stands in (matches the spec's `color_token()` — same approximation as the
        // tone color itself; `tone="inherit"` → text-primary is the no-currentColor
        // runtime delta noted in the parity doc).
        let underline_rest = Hsla {
            a: color.a * 0.55,
            ..color
        };
        let underline_strong = color;

        let mut el = div()
            .id(SharedString::from(format!(
                "poodle-text-link-{}",
                self.spec.label
            )))
            .cursor_pointer()
            .text_color(color)
            .underline()
            .text_decoration_color(underline_rest)
            .hover(move |style| style.text_decoration_color(underline_strong))
            .focus(move |style| {
                style
                    .text_decoration_color(underline_strong)
                    .shadow(focus_ring_shadow(focus_ring))
            })
            .when(self.spec.disabled, |el| {
                el.opacity(disabled_opacity).cursor_default()
            })
            .child(self.spec.label.clone());

        if !self.spec.disabled {
            if let Some(handler) = self.on_click {
                el = el.on_click(move |event, window, app| handler(event, window, app));
            }
        }

        el.into_any_element()
    }
}
