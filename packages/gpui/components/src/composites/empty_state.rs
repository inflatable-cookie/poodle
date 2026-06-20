//! EmptyState — real GPUI component backed by EmptyStateSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, EmptyStateSpec, EmptyStateVariant, IconSize, IconSpec, RemediationAction,
};

use crate::presentation::rem_to_px;
use crate::primitives::{Button, Icon};
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI empty state component backed by `EmptyStateSpec`.
///
/// Renders a centered empty state with title, optional message, and action buttons.
pub struct EmptyState {
    spec: EmptyStateSpec,
    theme: GpuiThemeProvider,
    illustration: Option<AnyElement>,
    on_action: Option<Box<dyn Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for EmptyState {
    type Target = EmptyStateSpec;
    fn deref(&self) -> &EmptyStateSpec {
        &self.spec
    }
}

impl EmptyState {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: EmptyStateSpec::new(title),
            theme: theme.clone(),
            illustration: None,
            on_action: None,
        }
    }

    pub fn from_spec(spec: EmptyStateSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            illustration: None,
            on_action: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }
    pub fn message(mut self, v: impl Into<String>) -> Self {
        self.spec.message = Some(v.into());
        self
    }
    pub fn variant(mut self, v: EmptyStateVariant) -> Self {
        self.spec.variant = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn actions(mut self, v: Vec<RemediationAction>) -> Self {
        self.spec.actions = v;
        self
    }
    pub fn compact(mut self, v: bool) -> Self {
        self.spec.compact = v;
        self
    }

    pub fn with_illustration(mut self, illustration: impl IntoElement) -> Self {
        self.illustration = Some(illustration.into_any_element());
        self
    }

    pub fn on_action(
        mut self,
        handler: impl Fn(&str, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_action = Some(Box::new(handler));
        self
    }
}

impl IntoElement for EmptyState {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let inline_gap = resolve_px(theme, "space.inline.sm");
        let body_size = resolve_px(theme, "typography.body.size");
        let gap = resolve_px(theme, spec.layout_gap_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");

        // ── Root: dashed border, variant-driven background tint, radius ──
        let border_default = resolve_color(theme, "color.border.default");
        // Contract: radius.surface - 0.125rem.
        let root_radius = resolve_radius(theme, "radius.surface") - px(rem_to_px(0.125));

        // Variant background tint — color-mix(in srgb, BASE X%, transparent).
        // Mixing toward transparent is alpha scaling: a' = base.a * ratio.
        let (tint_base, tint_ratio) = match spec.variant {
            // neutral: surface @ 76%
            EmptyStateVariant::Neutral => (resolve_color(theme, "color.background.surface"), 0.76),
            // search: accent-base @ 7%
            EmptyStateVariant::Search => (resolve_color(theme, "color.accent.base"), 0.07),
            // firstRun: status-success @ 7%
            EmptyStateVariant::FirstRun => (resolve_color(theme, "color.status.success"), 0.07),
        };
        let root_bg = Hsla {
            a: tint_base.a * tint_ratio,
            ..tint_base
        };

        // Svelte: title = 1.125rem default, 0.9375rem compact.
        // Svelte: padding = panel_y*1.5 vertical (default) / space.stack.lg (compact), panel_x horizontal.
        let title_size = px(rem_to_px(if spec.compact { 0.9375 } else { 1.125 }));
        let vertical_padding = if spec.compact {
            resolve_px(theme, "space.stack.lg")
        } else {
            // panel_y (default) * 1.5
            resolve_px(theme, "space.panel.y") * 1.5
        };
        let horiz_padding = resolve_px(theme, "space.panel.x");

        let mut container = div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .py(vertical_padding)
            .px(horiz_padding)
            .gap(gap)
            .border_1()
            .border_dashed()
            .border_color(border_default)
            .rounded(root_radius)
            .bg(root_bg);

        // ── Visual: custom illustration, else default variant icon circle ──
        // Circle: 2.25rem (1.75rem compact), panel @ 90% background, decorative.
        let circle_size = px(rem_to_px(if spec.compact { 1.75 } else { 2.25 }));
        let panel = resolve_color(theme, "color.background.panel");
        let circle_bg = Hsla {
            a: panel.a * 0.90,
            ..panel
        };
        let visual_inner: AnyElement = if let Some(illustration) = self.illustration {
            illustration
        } else {
            let icon_name = match spec.variant {
                EmptyStateVariant::Neutral => "inbox",
                EmptyStateVariant::Search => "search",
                EmptyStateVariant::FirstRun => "plus",
            };
            Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Md), theme)
                .with_color(text_secondary)
                .into_any_element()
        };
        container = container.child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(circle_size)
                .h(circle_size)
                .rounded(circle_size) // full pill — contract 999rem
                .bg(circle_bg)
                .child(visual_inner),
        );

        // Title
        container = container.child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .text_center()
                .child(spec.title.clone()),
        );

        // Message — copy max-width 24rem (contract §7).
        if let Some(ref message) = spec.message {
            container = container.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .text_center()
                    .max_w(px(rem_to_px(24.0)))
                    .child(message.clone()),
            );
        }

        // ── Actions: composed Button per RemediationAction, wired to on_action ──
        if !spec.actions.is_empty() {
            let mut actions_row = div().flex().flex_wrap().gap(inline_gap).items_center();
            let on_action = self.on_action.map(std::rc::Rc::new);

            for action in &spec.actions {
                let action_id = action.id.clone();
                let mut btn = Button::from_spec(
                    ButtonSpec::new()
                        .with_variant(action.variant)
                        .with_label(action.label.clone())
                        .with_disabled(action.is_disabled),
                    theme,
                )
                .with_id(format!("empty-action-{}", action.id));

                if let Some(ref handler) = on_action {
                    let handler = handler.clone();
                    btn = btn.on_click(move |event, window, cx| {
                        handler(&action_id, event, window, cx);
                    });
                }

                actions_row = actions_row.child(btn);
            }

            container = container.child(actions_row);
        }

        container.into_any_element()
    }
}
