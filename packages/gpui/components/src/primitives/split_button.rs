//! SplitButton — real GPUI component backed by SplitButtonSpec.
//!
//! Contract: `docs/contracts/foundation/split-button.md`

use gpui::*;
use pug_gpui::GpuiThemeProvider;
use pug_primitives::{
    ButtonTone, ButtonVariant, ControlSize, IconSize, IconSpec, SplitButtonSpec, SplitMenuItem,
};

use super::icon::Icon;
use crate::theme_ext::{color_mix, color_mix_black, resolve_color, resolve_opacity, resolve_px, resolve_radius};

pub struct SplitButton {
    spec: SplitButtonSpec,
    theme: GpuiThemeProvider,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_dropdown: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for SplitButton {
    type Target = SplitButtonSpec;
    fn deref(&self) -> &SplitButtonSpec { &self.spec }
}

impl SplitButton {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: SplitButtonSpec::new(),
            theme: theme.clone(),
            on_click: None,
            on_dropdown: None,
        }
    }

    pub fn from_spec(spec: SplitButtonSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_click: None, on_dropdown: None }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn variant(mut self, v: ButtonVariant) -> Self { self.spec.variant = v; self }
    pub fn tone(mut self, v: ButtonTone) -> Self { self.spec.tone = v; self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn label(mut self, v: impl Into<String>) -> Self { self.spec.label = Some(v.into()); self }
    pub fn items(mut self, v: Vec<SplitMenuItem>) -> Self { self.spec.items = v; self }
    pub fn disabled(mut self, v: bool) -> Self { self.spec.is_disabled = v; self }
    pub fn loading(mut self, v: bool) -> Self { self.spec.is_loading = v; self }
    pub fn open(mut self, v: bool) -> Self { self.spec.is_open = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn menu_aria_label(mut self, v: impl Into<String>) -> Self { self.spec.menu_aria_label = v.into(); self }

    // ── GPUI-specific builders ────────────────────────────────
    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn on_dropdown(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_dropdown = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SplitButton {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Colors ────────────────────────────────────────────────
        let base_fill = resolve_color(theme, spec.fill_token());
        let base_border = resolve_color(theme, spec.border_token());
        let text_color = resolve_color(theme, spec.text_token());
        let separator_color = resolve_color(theme, spec.separator_token());
        let elevated = resolve_color(theme, "semantic.color.background.elevated");
        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let radius = resolve_radius(theme, spec.radius_token());
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");

        // Size
        let base_height = resolve_px(theme, spec.control_height_token());
        let size_offset: f32 = match spec.size {
            ControlSize::Sm => -6.0, ControlSize::Md => 0.0, ControlSize::Lg => 6.0,
        };
        let height = base_height + px(size_offset);
        let pad_x = resolve_px(theme, "semantic.space.control.x");
        let font_size: f32 = match spec.size {
            ControlSize::Sm => 12.0, ControlSize::Md => 13.0, ControlSize::Lg => 14.0,
        };

        // Variant fills
        let (fill, border_color) = match spec.variant {
            ButtonVariant::Primary => (base_fill, color_mix_black(base_fill, 0.84)),
            ButtonVariant::Ghost => {
                let surface = resolve_color(theme, "semantic.color.background.surface");
                (Hsla { a: surface.a * 0.42, ..surface }, gpui::transparent_black())
            }
            _ => (base_fill, base_border),
        };

        let is_unavailable = spec.is_unavailable();
        let hover_fill = color_mix(fill, elevated, 0.84);
        let hover_border = color_mix(border_color, text_primary, 0.78);
        let active_fill = color_mix(fill, elevated, 0.72);

        // ── Root ──────────────────────────────────────────────────
        let mut root = div()
            .id("pug-split-btn")
            .flex()
            .items_end()
            .rounded(radius);

        // ── Primary half ──────────────────────────────────────────
        let label_text = spec.label.clone().unwrap_or_default();

        let mut primary = div()
            .id("pug-split-primary")
            .h(height)
            .min_w(px(64.0))
            .px(pad_x)
            .bg(fill)
            .border_1().border_color(border_color)
            .rounded_l(radius)
            .text_color(text_color)
            .text_size(px(font_size))
            .font_weight(FontWeight::MEDIUM)
            .line_height(relative(1.0))
            .flex().items_center().justify_center();

        if !is_unavailable {
            primary = primary
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill).border_color(hover_border))
                .active(move |s| s.bg(active_fill))
                .focus(move |s| s.border_color(focus_ring));
        }

        if !label_text.is_empty() {
            primary = primary.child(div().whitespace_nowrap().min_w(px(0.0)).child(label_text));
        }

        // ── Divider (60% height, centered) ────────────────────────
        let divider = div().w(px(1.0)).h(height * 0.6).my_auto().bg(separator_color);

        // ── Toggle half ───────────────────────────────────────────
        let mut toggle = div()
            .id("pug-split-toggle")
            .h(height)
            .w(px(32.0))
            .bg(fill)
            .border_1().border_color(border_color)
            .rounded_r(radius)
            .text_color(text_color)
            .flex().items_center().justify_center();

        if !is_unavailable {
            toggle = toggle
                .cursor_pointer()
                .hover(move |s| s.bg(hover_fill))
                .focus(move |s| s.border_color(focus_ring));
        }

        toggle = toggle.child(
            Icon::from_spec(IconSpec::new("chevron-down").with_size(IconSize::Sm), theme)
                .with_color(text_color),
        );

        // ── Disabled/loading ──────────────────────────────────────
        if is_unavailable {
            let opacity = resolve_opacity(theme, spec.disabled_opacity_token());
            root = root.opacity(opacity).cursor(CursorStyle::OperationNotAllowed);
        }

        root = root.child(primary).child(divider).child(toggle);

        // ── Menu overlay (when open) ──────────────────────────────
        if spec.is_open && !spec.items.is_empty() {
            let menu_fill = resolve_color(theme, spec.overlay_fill_token());
            let menu_border = resolve_color(theme, "semantic.color.border.default");
            let menu_radius = resolve_radius(theme, "semantic.radius.surface");
            let item_text = resolve_color(theme, "semantic.color.text.primary");

            let mut menu = div()
                .bg(menu_fill).border_1().border_color(menu_border)
                .rounded(menu_radius).shadow_md()
                .min_w(px(192.0)) // 12rem
                .py(px(4.0)).mt(px(4.0))
                .flex().flex_col();

            for item in &spec.items {
                match item {
                    SplitMenuItem::Action { label, is_disabled, .. } => {
                        let mut item_el = div()
                            .px(px(8.0)).py(px(6.0))
                            .text_sm().text_color(item_text)
                            .rounded(px(4.0));
                        if !is_disabled {
                            let accent = resolve_color(theme, "semantic.color.accent.base");
                            let hover_bg = Hsla { a: 0.08, ..accent };
                            item_el = item_el.cursor_pointer().hover(move |s| s.bg(hover_bg));
                        } else {
                            item_el = item_el.opacity(resolve_opacity(theme, spec.disabled_opacity_token()));
                        }
                        menu = menu.child(item_el.child(label.clone()));
                    }
                    SplitMenuItem::Separator => {
                        menu = menu.child(div().h(px(1.0)).mx(px(4.0)).my(px(4.0)).bg(separator_color));
                    }
                }
            }
            root = root.child(menu);
        }

        // ── Click handlers ────────────────────────────────────────
        if let Some(handler) = self.on_click {
            if !is_unavailable {
                root = root.on_click(move |event, window, cx| handler(event, window, cx));
            }
        }

        root.into_any_element()
    }
}
