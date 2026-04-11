use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_components::{AlertDialogSpec, AlertDialogTone, ControlDensity, ControlSize, SemanticControlSizeRole};
use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem, panel_space_x_rem, panel_space_y_rem};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

pub struct AlertDialog {
    spec: AlertDialogSpec,
    _backdrop_fill: Hsla,
    dialog_fill: Hsla,
    dialog_radius: Pixels,
    button_radius: Pixels,
    title_color: Hsla,
    description_color: Hsla,
    confirm_fill: Hsla,
    confirm_text_color: Hsla,
    cancel_text_color: Hsla,
    content_gap: Pixels,
    actions_gap: Pixels,
    padding_x: Pixels,
    padding_y: Pixels,
    border_color: Hsla,
    body_size: Pixels,
    heading_size: Pixels,
    header_gap: Pixels,
    button_pad_x: Pixels,
    button_pad_y: Pixels,
    brand_raised: bool,
    on_confirm: Option<Box<dyn Fn(&mut Window, &mut App)>>,
    on_cancel: Option<Box<dyn Fn(&mut Window, &mut App)>>,
}

impl std::ops::Deref for AlertDialog {
    type Target = AlertDialogSpec;
    fn deref(&self) -> &AlertDialogSpec { &self.spec }
}

impl AlertDialog {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        let spec = AlertDialogSpec::default();
        Self::build(spec, theme)
    }

    pub fn from_spec(spec: AlertDialogSpec, theme: &GpuiThemeProvider) -> Self {
        Self::build(spec, theme)
    }

    fn build(spec: AlertDialogSpec, theme: &GpuiThemeProvider) -> Self {
        let effective_size = resolve_semantic_size(spec.size, spec.size_role);
        let body_font = px(rem_to_px(size_font_rem(effective_size)));
        let density_pad_x = px(rem_to_px(panel_space_x_rem(spec.density)));
        let density_pad_y = px(rem_to_px(panel_space_y_rem(spec.density)));
        Self {
            _backdrop_fill: hsla(0.0, 0.0, 0.0, 0.5),
            dialog_fill: resolve_color(theme, spec.dialog_fill_token()),
            dialog_radius: resolve_radius(theme, spec.dialog_radius_token()),
            button_radius: resolve_radius(theme, spec.button_radius_token()),
            title_color: resolve_color(theme, spec.title_color_token()),
            description_color: resolve_color(theme, spec.description_color_token()),
            confirm_fill: resolve_color(theme, spec.confirm_fill_token()),
            confirm_text_color: resolve_color(theme, spec.confirm_text_color_token()),
            cancel_text_color: resolve_color(theme, spec.cancel_text_color_token()),
            content_gap: resolve_px(theme, spec.content_gap_token()),
            actions_gap: resolve_px(theme, spec.actions_gap_token()),
            padding_x: density_pad_x,
            padding_y: density_pad_y,
            border_color: resolve_color(theme, spec.border_token()),
            body_size: body_font,
            heading_size: resolve_px(theme, "typography.heading.size"),
            header_gap: resolve_px(theme, "space.inline.sm"),
            button_pad_x: resolve_px(theme, "space.inline.md"),
            button_pad_y: resolve_px(theme, "space.inline.sm"),
            brand_raised: theme.brand_raised,
            on_confirm: None,
            on_cancel: None,
            spec,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn open(mut self, v: bool) -> Self { self.spec.open = Some(v); self }
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.spec.description = Some(v.into()); self }
    pub fn tone(mut self, v: AlertDialogTone) -> Self { self.spec.tone = v; self }
    pub fn confirm_label(mut self, v: impl Into<String>) -> Self { self.spec.confirm_label = v.into(); self }
    pub fn cancel_label(mut self, v: impl Into<String>) -> Self { self.spec.cancel_label = v.into(); self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn size(mut self, v: ControlSize) -> Self { self.spec.size = v; self }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self { self.spec.size_role = v; self }
    pub fn with_density(mut self, v: ControlDensity) -> Self { self.spec.density = v; self }

    pub fn on_confirm(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Box::new(handler));
        self
    }

    pub fn on_cancel(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Box::new(handler));
        self
    }
}

impl IntoElement for AlertDialog {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let is_open = self.spec.open.unwrap_or(false);

        if !is_open {
            return div().into_any_element();
        }

        // Header
        let mut header = div()
            .flex()
            .flex_col()
            .gap(self.header_gap)
            .child(
                div()
                    .text_size(self.heading_size)
                    .text_color(self.title_color)
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(self.spec.title.clone()),
            );
        if let Some(ref desc) = self.spec.description {
            header = header.child(
                div()
                    .text_size(self.body_size)
                    .text_color(self.description_color)
                    .child(desc.clone()),
            );
        }

        // Cancel button
        let cancel_btn = div()
            .id("alert-dialog-cancel")
            .cursor_pointer()
            .px(self.button_pad_x)
            .py(self.button_pad_y)
            .rounded(self.button_radius)
            .text_size(self.body_size)
            .font_weight(FontWeight::MEDIUM)
            .text_color(self.cancel_text_color)
            .hover(|s| s.bg(hsla(0.0, 0.0, 0.5, 0.1)))
            .child(self.spec.cancel_label.clone());

        // Confirm button
        let mut confirm_btn = div()
            .id("alert-dialog-confirm")
            .cursor_pointer()
            .px(self.button_pad_x)
            .py(self.button_pad_y)
            .rounded(self.button_radius)
            .text_size(self.body_size)
            .font_weight(FontWeight::MEDIUM);

        // Brand-raised treatment: gradient fill for confirm button
        if self.brand_raised {
            use crate::theme_ext::{brand_raised_primary_fill, brand_raised_primary_shadow};
            confirm_btn = confirm_btn
                .bg(brand_raised_primary_fill(self.confirm_fill))
                .shadow(brand_raised_primary_shadow());
        } else {
            confirm_btn = confirm_btn.bg(self.confirm_fill);
        }

        let confirm_btn = confirm_btn
            .text_color(self.confirm_text_color)
            .hover(|s| s.opacity(0.9))
            .child(self.spec.confirm_label.clone());

        let actions = div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(self.actions_gap)
            .child(cancel_btn)
            .child(confirm_btn);

        // Dialog card — constrained width, not full-width
        let mut dialog_card = div()
            .id("alert-dialog")
            .focusable()
            .flex()
            .flex_col()
            .w(px(420.0));

        // Brand-raised treatment: gradient fill for dialog surface
        if self.brand_raised {
            dialog_card = dialog_card.bg(crate::theme_ext::brand_raised_surface_fill(self.dialog_fill));
        } else {
            dialog_card = dialog_card.bg(self.dialog_fill);
        }

        let dialog_card = dialog_card
            .rounded(self.dialog_radius)
            .border_1()
            .border_color(self.border_color)
            .shadow(vec![
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                gpui::BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(8.0),
                    spread_radius: px(0.0),
                },
            ])
            .px(self.padding_x)
            .py(self.padding_y)
            .gap(self.content_gap)
            .occlude()
            .child(header)
            .child(actions);

        // Render as inline positioned element (not absolute overlay)
        // The specimen page handles its own layout
        div()
            .flex()
            .items_center()
            .justify_center()
            .child(dialog_card)
            .into_any_element()
    }
}
