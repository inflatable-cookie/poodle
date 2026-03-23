//! ColorPicker — real GPUI component backed by ColorPickerSpec.
//!
//! Contract: 2.25rem square trigger with color swatch,
//! elevated surface overlay with swatch grid.
//! Focus ring on trigger. No hover.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_primitives::ColorPickerSpec;

use crate::theme_ext::{resolve_color, resolve_opacity, resolve_px, resolve_radius};

/// Parse a hex color string (e.g. "#ff0000") into an Hsla color.
fn parse_hex_color(hex: &str) -> Hsla {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f32 / 255.0;
    let rgba = gpui::Rgba { r, g, b, a: 1.0 };
    rgba.into()
}

/// A real GPUI color picker component with swatch grid backed by `ColorPickerSpec`.
pub struct ColorPicker {
    spec: ColorPickerSpec,
    theme: GpuiThemeProvider,
    id_suffix: Option<String>,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
    on_change: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ColorPicker {
    type Target = ColorPickerSpec;
    fn deref(&self) -> &ColorPickerSpec { &self.spec }
}

impl ColorPicker {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ColorPickerSpec::new(), theme: theme.clone(), id_suffix: None, on_toggle: None, on_change: None }
    }

    pub fn from_spec(spec: ColorPickerSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_suffix: None,
            on_toggle: None,
            on_change: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn value(mut self, v: impl Into<String>) -> Self { self.spec.value = Some(v.into()); self }
    pub fn default_value(mut self, v: impl Into<String>) -> Self { self.spec.default_value = Some(v.into()); self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn show_alpha(mut self, v: bool) -> Self { self.spec.show_alpha = v; self }
    pub fn swatches(mut self, v: Vec<String>) -> Self { self.spec.swatches = v; self }

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

    pub fn on_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl IntoElement for ColorPicker {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let stack_gap = resolve_px(theme, "semantic.space.stack.sm");
        let trigger_radius = resolve_radius(theme, spec.trigger_radius_token());
        let surface_radius = resolve_radius(theme, spec.surface_radius_token());

        let border = resolve_color(theme, spec.border_token());
        let elevated_bg = resolve_color(theme, spec.overlay_fill_token());
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let focus_ring = resolve_color(theme, spec.focus_ring_color_token());
        let disabled_opacity = resolve_opacity(theme, spec.disabled_opacity_token());

        let current = spec
            .current_value()
            .unwrap_or("#000000")
            .to_string();
        let current_color = parse_hex_color(&current);

        // Contract: trigger 2.25rem × 2.25rem square
        let trigger_size = px(36.0); // 2.25rem

        let id_str = if let Some(ref suffix) = self.id_suffix {
            format!("flint-color-picker-{}", suffix)
        } else {
            "flint-color-picker".to_string()
        };

        let mut trigger = div()
            .id(SharedString::from(id_str))
            .focusable()
            .w(trigger_size)
            .h(trigger_size)
            .rounded(trigger_radius)
            .bg(current_color)
            .border_1()
            .border_color(border)
            .cursor_pointer()
            // Contract: focus ring on trigger
            .focus(move |s| s.border_color(focus_ring));

        if spec.is_disabled {
            trigger = trigger
                .opacity(disabled_opacity)
                .cursor(CursorStyle::OperationNotAllowed);
        }

        let is_open = spec.is_open;
        let is_disabled = spec.is_disabled;

        if let Some(handler) = self.on_toggle {
            if !is_disabled {
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
                        } else if event.keystroke.key == "escape" && is_open {
                            key_handler(&false, window, cx);
                        }
                    });
            }
        }

        let mut wrapper = div()
            .relative()
            .flex()
            .flex_col()
            .gap(stack_gap)
            .child(trigger);

        if spec.is_open && !spec.is_disabled {
            // Contract: swatch size 1.25rem, border-radius 0.1875rem
            let swatch_size = px(20.0); // 1.25rem
            let swatch_radius = px(3.0); // 0.1875rem

            // Contract: surface width 24rem, padding 0.75rem
            let mut overlay = div()
                .w(px(384.0)) // 24rem
                .rounded(surface_radius)
                .bg(elevated_bg)
                .border_1()
                .border_color(border)
                .shadow(vec![
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.10),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(16.0),
                        spread_radius: px(0.0),
                    },
                    gpui::BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.06),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(4.0),
                        spread_radius: px(0.0),
                    },
                ])
                .p(px(12.0)) // 0.75rem
                .flex()
                .flex_col()
                .gap(px(8.0));

            // Swatch grid — only rendered when caller supplies swatches
            if !spec.swatches.is_empty() {
                let mut grid = div()
                    .flex()
                    .flex_wrap()
                    .gap(stack_gap);

                for (idx, color_hex) in spec.swatches.iter().enumerate() {
                    let swatch_color = parse_hex_color(color_hex);
                    let swatch_id = SharedString::from(format!(
                        "{}-swatch-{}",
                        if let Some(ref suffix) = self.id_suffix {
                            format!("flint-color-picker-{}", suffix)
                        } else {
                            "flint-color-picker".to_string()
                        },
                        idx,
                    ));
                    let mut swatch = div()
                        .id(swatch_id)
                        .w(swatch_size)
                        .h(swatch_size)
                        .rounded(swatch_radius)
                        .border_1()
                        .border_color(border)
                        .bg(swatch_color)
                        .cursor_pointer();

                    if let Some(ref on_change) = self.on_change {
                        let handler = on_change.clone();
                        let hex_value = color_hex.clone();
                        swatch = swatch.on_click(move |_event, window, cx| {
                            handler(&hex_value, window, cx);
                        });
                    }

                    grid = grid.child(swatch);
                }
                overlay = overlay.child(grid);
            }

            // Hex input display — Contract: code font, editable
            // Only render when show_input is true (default)
            if spec.show_input {
                let mode_label = match spec.default_mode {
                    flint_primitives::ColorInputMode::Hex => "HEX",
                    flint_primitives::ColorInputMode::Rgb => "RGB",
                    flint_primitives::ColorInputMode::Hsl => "HSL",
                };

                // Mode label
                overlay = overlay.child(
                    div()
                        .text_size(px(10.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_secondary)
                        .child(mode_label.to_string())
                );

                let hex_id = SharedString::from(format!(
                    "{}-hex",
                    if let Some(ref suffix) = self.id_suffix {
                        format!("flint-color-picker-{}", suffix)
                    } else {
                        "flint-color-picker".to_string()
                    },
                ));

                let mut hex_input = div()
                    .id(hex_id)
                    .focusable()
                    .w_full()
                    .h(px(28.0))
                    .px(px(8.0))
                    .rounded(px(4.0))
                    .bg(resolve_color(theme, "semantic.color.background.surface"))
                    .border_1()
                    .border_color(border)
                    .flex()
                    .items_center()
                    .text_size(px(12.0))
                    .text_color(text_primary)
                    .focus(move |s| s.border_color(focus_ring))
                    .child(current.clone());

                // Keyboard editing: type hex chars, backspace to delete
                if let Some(ref on_change) = self.on_change {
                    let change_handler = on_change.clone();
                    let current_hex = current.clone();
                    hex_input = hex_input.on_key_down(move |event: &KeyDownEvent, window, cx| {
                        let key = event.keystroke.key.as_str();
                        if key == "backspace" {
                            let mut chars: Vec<char> = current_hex.chars().collect();
                            if chars.len() > 1 { // keep at least "#"
                                chars.pop();
                                let new_val: String = chars.into_iter().collect();
                                change_handler(&new_val, window, cx);
                            }
                        } else if key.len() == 1
                            && !event.keystroke.modifiers.platform
                            && !event.keystroke.modifiers.control
                        {
                            let ch = key.chars().next().unwrap();
                            if ch.is_ascii_hexdigit() && current_hex.len() < 7 {
                                let new_val = format!("{}{}", current_hex, ch);
                                change_handler(&new_val, window, cx);
                            }
                        }
                    });
                }

                overlay = overlay.child(hex_input);
            }

            wrapper = wrapper.child(overlay);
        }

        wrapper.into_any_element()
    }
}
