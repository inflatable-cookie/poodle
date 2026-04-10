//! Collapsible — real GPUI component backed by CollapsibleSpec.
//!
//! Contract: `docs/contracts/foundation/collapsible.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_primitives::{CollapsibleSpec, ControlDensity, ControlSize, IconSize, IconSpec, SemanticControlSizeRole};

use super::icon::Icon;
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem};
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
    fn deref(&self) -> &CollapsibleSpec { &self.spec }
}

impl Collapsible {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: CollapsibleSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None, content: None }
    }

    pub fn from_spec(spec: CollapsibleSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), id_suffix: None, on_toggle: None, content: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn default_open(mut self, v: bool) -> Self { self.spec.default_open = v; self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

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
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let _density_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        let title_font = px(rem_to_px(size_font_rem(effective_size)));

        let heading_size = title_font;
        let label_size = resolve_px(theme, "typography.label.size");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border_color = resolve_color(theme, "color.border.subtle");
        let _panel_bg = resolve_color(theme, "color.background.panel");
        let radius = resolve_radius(theme, "radius.surface");
        let focus_ring = resolve_color(theme, "color.accent.focusRing");
        let panel_pad = density_pad_x;

        // Svelte: border = color-mix(border-subtle 42%, transparent)
        let root_border = Hsla { a: border_color.a * 0.42, ..border_color };
        // Svelte: bg = color-mix(surface 88%, text-primary)
        let surface_bg = resolve_color(theme, "color.background.surface");
        let root_bg = color_mix(surface_bg, text_primary, 0.88);

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("poodle-collapsible-{}", suffix)
        } else {
            "poodle-collapsible".to_string()
        };

        // ── Root (contract: grid, gap 0.5rem when open, 0 when closed) ──
        let gap = if is_open { px(8.0) } else { px(0.0) }; // 0.5rem = 8px
        let mut root = div()
            .flex().flex_col()
            .gap(gap)
            .min_w(px(0.0))
            // Contract: padding from panel spacing token
            .p(panel_pad)
            .border_1().border_color(root_border)
            .rounded(radius)
            .bg(root_bg);

        // ── Trigger (contract: grid 1fr auto, gap 0.75rem) ──
        let mut trigger = div()
            .id(SharedString::from(id_str))
            .focusable()
            .flex().items_center()
            .gap(px(12.0)) // 0.75rem
            .w_full()
            .focus(move |s| s.border_color(focus_ring).shadow(crate::theme_ext::focus_ring_shadow(focus_ring)));

        // Title block (1fr)
        let mut title_block = div().flex().flex_col().flex_1().min_w(px(0.0));
        if let Some(ref title_text) = spec.title {
            title_block = title_block.child(
                div()
                    .text_color(text_primary)
                    .text_size(heading_size)
                    .font_weight(FontWeight::BOLD)
                    .line_height(relative(1.2))
                    .child(title_text.clone())
            );
        }
        if let Some(ref desc) = spec.description {
            title_block = title_block.child(
                div()
                    .text_color(text_secondary)
                    .text_size(label_size)
                    .child(desc.clone())
            );
        }
        trigger = trigger.child(title_block);

        // Chevron indicator (auto)
        let chevron_icon = if is_open { "chevron-down" } else { "chevron-right" };
        trigger = trigger.child(
            Icon::from_spec(
                IconSpec::new(chevron_icon).with_size(IconSize::Sm),
                theme,
            )
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
        if is_open {
            if let Some(content) = self.content {
                root = root.child(content);
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
