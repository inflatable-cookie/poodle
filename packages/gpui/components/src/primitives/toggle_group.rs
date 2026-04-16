//! ToggleGroup — grouped toggle selection control.
//!
//! Contract: `docs/contracts/components/toggle-group.md`

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ControlSize, ToggleGroupOption, ToggleGroupSelectionMode, ToggleGroupSpec};

use crate::presentation::{
    rem_to_px, resolve_semantic_size, size_font_rem, size_height_offset_rem,
    size_padding_x_offset_rem,
};
use crate::theme_ext::{color_mix, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct ToggleGroup {
    spec: ToggleGroupSpec,
    theme: GpuiThemeProvider,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ToggleGroup {
    type Target = ToggleGroupSpec;
    fn deref(&self) -> &ToggleGroupSpec {
        &self.spec
    }
}

impl ToggleGroup {
    pub fn new(options: Vec<ToggleGroupOption>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: ToggleGroupSpec::new(options),
            theme: theme.clone(),
            on_change: None,
        }
    }

    pub fn from_spec(spec: ToggleGroupSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: Vec<String>) -> Self {
        self.spec.value = Some(v);
        self
    }
    pub fn default_value(mut self, v: Vec<String>) -> Self {
        self.spec.default_value = Some(v);
        self
    }
    pub fn selection_mode(mut self, v: ToggleGroupSelectionMode) -> Self {
        self.spec.selection_mode = v;
        self
    }
    pub fn disabled(mut self, v: bool) -> Self {
        self.spec.is_disabled = v;
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
    pub fn size_role(mut self, v: poodle_specs::SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn density(mut self, v: poodle_specs::ControlDensity) -> Self {
        self.spec.density = v;
        self
    }

    // ── GPUI-specific ─────────────────────────────────────────
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for ToggleGroup {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Resolve effective size from size + size_role ────────
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);

        let gap = resolve_px(theme, spec.item_gap_token());
        let radius = resolve_radius(theme, "radius.control");
        let accent = resolve_color(theme, "color.accent.base");
        let surface = resolve_color(theme, "color.background.surface");
        let elevated = resolve_color(theme, "color.background.elevated");
        let border_default = resolve_color(theme, "color.border.default");
        let border_subtle = resolve_color(theme, "color.border.subtle");
        let text_primary = resolve_color(theme, "color.text.primary");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let label_size = px(rem_to_px(size_font_rem(effective_size)));

        // Size-aware item height and padding
        let base_height = resolve_px(theme, "size.control.height");
        let item_height = base_height + px(rem_to_px(size_height_offset_rem(effective_size)));
        let base_pad_x = resolve_px(theme, "space.control.x");
        let item_pad_x = base_pad_x + px(rem_to_px(size_padding_x_offset_rem(effective_size)));

        // Svelte unselected: treatment-interactive-fill = color-mix(surface 93%, text-primary)
        let item_fill = color_mix(surface, text_primary, 0.93);
        // Svelte unselected border: treatment-interactive-border = border-subtle 82%
        let item_border = Hsla { a: border_subtle.a * 0.82, ..border_subtle };
        // Svelte selected: gradient(accent 22%, transparent) over item_fill
        let selected_fill = color_mix(accent, item_fill, 0.22);
        let selected_border = color_mix(accent, border_default, 0.42);

        let mut el = div().flex().flex_row().flex_wrap().gap(gap);

        let option_values: Vec<String> = spec.options.iter().map(|o| o.value.clone()).collect();

        for (idx, option) in spec.options.iter().enumerate() {
            let is_selected = spec.is_selected(&option.value);
            let item_disabled = option.is_disabled || spec.is_disabled;

            let (fill, border_color, text_color) = if is_selected {
                (selected_fill, selected_border, text_primary)
            } else {
                (item_fill, item_border, text_primary)
            };

            let hover_fill = color_mix(fill, elevated, 0.84);

            let mut item = div()
                .id(SharedString::from(format!("tg-{}", option.value)))
                .focusable()
                .h(item_height)
                .px(item_pad_x)
                .rounded(radius);

            // Brand-raised treatment: gradient fills for selected items
            if theme.brand_raised && !item_disabled && is_selected {
                use crate::theme_ext::{brand_raised_primary_fill, brand_raised_primary_shadow};
                item = item
                    .bg(brand_raised_primary_fill(fill))
                    .shadow(brand_raised_primary_shadow());
            } else if theme.brand_raised && !item_disabled && !is_selected {
                use crate::theme_ext::{
                    brand_raised_interactive_fill, brand_raised_interactive_shadow,
                };
                item = item
                    .bg(brand_raised_interactive_fill(fill))
                    .shadow(brand_raised_interactive_shadow());
            } else {
                item = item.bg(fill);
            }

            item = item
                .border_1()
                .border_color(border_color)
                .text_color(text_color)
                .text_size(label_size)
                .font_weight(FontWeight::SEMIBOLD)
                .flex()
                .items_center()
                .justify_center()
                .focus(move |s| {
                    s.border_color(focus_ring)
                        .shadow(crate::theme_ext::focus_ring_shadow(focus_ring))
                });

            if item_disabled {
                let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
                item = item
                    .opacity(opacity)
                    .cursor(CursorStyle::OperationNotAllowed);
            } else {
                item = item.cursor_pointer().hover(move |s| s.bg(hover_fill));

                if let Some(ref handler) = self.on_change {
                    let click_handler = handler.clone();
                    let val = option.value.clone();
                    item = item.on_click(move |_event, window, cx| {
                        click_handler(&val, window, cx);
                    });

                    // Space/Enter to toggle + arrow keys to navigate
                    let key_handler = handler.clone();
                    let arrow_handler = handler.clone();
                    let val2 = option.value.clone();
                    let ovs = option_values.clone();
                    let current_idx = idx;
                    item = item.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        if event.keystroke.key == "space" || event.keystroke.key == "enter" {
                            key_handler(&val2, window, cx);
                        } else if event.keystroke.key == "right" || event.keystroke.key == "down" {
                            let next = (current_idx + 1) % ovs.len();
                            arrow_handler(&ovs[next], window, cx);
                        } else if event.keystroke.key == "left" || event.keystroke.key == "up" {
                            let prev = if current_idx == 0 {
                                ovs.len() - 1
                            } else {
                                current_idx - 1
                            };
                            arrow_handler(&ovs[prev], window, cx);
                        }
                    });
                }
            }

            item = item.child(option.label.clone());
            el = el.child(item);
        }

        if spec.is_disabled {
            let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            el = el.opacity(opacity);
        }

        el.into_any_element()
    }
}
