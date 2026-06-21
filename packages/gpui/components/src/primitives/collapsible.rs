//! Collapsible — real GPUI component backed by CollapsibleSpec.
//!
//! Contract: `docs/contracts/components/collapsible.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    CollapsibleSpec, ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole,
};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct Collapsible {
    spec: CollapsibleSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    content: Option<AnyElement>,
}

impl std::ops::Deref for Collapsible {
    type Target = CollapsibleSpec;
    fn deref(&self) -> &CollapsibleSpec {
        &self.spec
    }
}

impl Collapsible {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: CollapsibleSpec::new(),
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            content: None,
        }
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
    pub fn open(mut self, v: bool) -> Self {
        self.spec.open = Some(v);
        self
    }
    pub fn default_open(mut self, v: bool) -> Self {
        self.spec.default_open = v;
        self
    }
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
        self
    }
    pub fn highlighted(mut self, v: bool) -> Self {
        self.spec.highlighted = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
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

    pub fn with_id(mut self, suffix: impl Into<String>) -> Self {
        self.id_suffix = Some(suffix.into());
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
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
        let is_open = spec.current_open();
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        // Svelte: collapsible density X = compact=0.5, default=1.0, comfortable=1.0rem
        // (not the standard panel_space_x_rem which gives compact=0.75, comfortable=1.25)
        let density_pad_x = px(rem_to_px(match spec.density {
            ControlDensity::Compact => 0.5,
            ControlDensity::Default => 1.0,
            ControlDensity::Comfortable => 1.0,
        }));
        // Svelte: collapsible Y is hardcoded 0.625rem (not density-based)
        let density_pad_y = px(rem_to_px(0.625));
        // Svelte: title is heading-scale (xs=0.8125, sm=0.875, md=1.0, lg=1.0625, xl=1.125rem)
        let title_font = px(rem_to_px(match effective_size {
            ControlSize::Xs => 0.8125,
            ControlSize::Sm => 0.875,
            ControlSize::Md => 1.0,
            ControlSize::Lg => 1.0625,
            ControlSize::Xl => 1.125,
        }));

        let heading_size = title_font;
        // Description scales per-size (contract §8 size table: xs=0.6875 … xl=0.9375rem)
        let description_font = px(rem_to_px(crate::presentation::size_font_rem(effective_size)));
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border_color = resolve_color(theme, "color.border.subtle");
        let panel_bg = resolve_color(theme, "color.background.panel");
        let elevated_bg = resolve_color(theme, "color.background.elevated");
        let radius = resolve_radius(theme, "radius.surface");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let accent_base = resolve_color(theme, spec.highlight_accent_token());

        // Contract §8: border = color-mix(border-subtle 36%, transparent)
        let root_border = Hsla {
            a: border_color.a * spec.border_subtle_alpha(),
            ..border_color
        };
        // Contract §8: bg = color-mix(background-elevated 40%, background-panel)
        let root_bg = color_mix(elevated_bg, panel_bg, 0.40);
        // Contract §8 highlighted: border accent-base 55%, halo accent-base 12%
        let highlight_border = Hsla {
            a: accent_base.a * spec.highlight_border_alpha(),
            ..accent_base
        };
        let highlight_halo = Hsla {
            a: accent_base.a * spec.highlight_halo_alpha(),
            ..accent_base
        };

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-collapsible-{}", suffix)
        } else {
            "poodle-collapsible".to_string()
        };

        // ── Root (contract: grid, gap space.stack.md when open, 0 when closed) ──
        // Svelte: open gap = space.stack.md (12px), not inline.sm (8px)
        let gap = if is_open { resolve_px(theme, "space.stack.md") } else { px(0.0) };
        let mut root = div()
            .flex()
            .flex_col()
            .gap(gap)
            .min_w(px(0.0))
            // Svelte: padding X is density-based, Y is hardcoded 0.625rem
            .px(density_pad_x)
            .py(density_pad_y)
            .border_1()
            .border_color(if spec.highlighted {
                highlight_border
            } else {
                root_border
            })
            .rounded(radius)
            .bg(root_bg);

        // Contract §8 highlighted: 0 0 0 0.125rem accent-base 12% halo
        if spec.highlighted {
            root = root.shadow(vec![gpui::BoxShadow {
                color: highlight_halo,
                offset: point(px(0.0), px(0.0)),
                blur_radius: px(0.0),
                spread_radius: px(rem_to_px(0.125)),
            }]);
        }

        // ── Trigger (contract: grid 1fr auto, gap 0.75rem) ──
        let mut trigger = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex()
            .items_center()
            .gap(resolve_px(theme, "space.inline.md"))
            .w_full()
            .focus(move |s| {
                s.border_color(focus_ring)
                    .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
            });

        // Title block (1fr)
        let mut title_block = div().flex().flex_col().flex_1().min_w(px(0.0));
        if let Some(ref title_text) = spec.title {
            title_block = title_block.child(
                div()
                    .text_color(text_primary)
                    .text_size(heading_size)
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .child(title_text.clone()),
            );
        }
        if let Some(ref desc) = spec.description {
            title_block = title_block.child(
                div()
                    .text_color(text_secondary)
                    .text_size(description_font)
                    .line_height(relative(1.45))
                    .child(desc.clone()),
            );
        }
        trigger = trigger.child(title_block);

        // Chevron indicator (auto)
        let chevron_icon = if is_open {
            "chevron-down"
        } else {
            "chevron-right"
        };
        trigger = trigger.child(
            Icon::from_spec(IconSpec::new(chevron_icon).with_size(IconSize::Sm), theme)
                .with_color(text_secondary),
        );

        if spec.activation_allowed() {
            trigger = trigger.cursor_pointer();
        } else {
            trigger = trigger.cursor(CursorStyle::OperationNotAllowed);
        }

        // Click + keyboard handlers
        if let Some(handler) = self.on_toggle {
            if spec.activation_allowed() {
                let next_open = !is_open;
                let handler = std::rc::Rc::new(handler);
                let key_handler = handler.clone();
                trigger = trigger
                    .on_click(move |_event, window, cx| {
                        handler(&next_open, window, cx);
                    })
                    .on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&next_open, window, cx);
                        }
                    });
            }
        }

        root = root.child(trigger);

        // ── Content region (only when open) ──
        // Contract §8 Content: min-width 0, padding-top 0.125rem
        if is_open {
            if let Some(content) = self.content {
                root = root.child(
                    div()
                        .min_w(px(0.0))
                        .pt(px(rem_to_px(0.125)))
                        .child(content),
                );
            }
        }

        // ── Disabled ──
        if spec.is_disabled {
            let opacity = resolve_opacity(theme, "state.opacity.disabled");
            root = root.opacity(opacity);
        }

        root.into_any_element()
    }
}
