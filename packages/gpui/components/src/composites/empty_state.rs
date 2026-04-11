//! EmptyState — real GPUI component backed by EmptyStateSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{EmptyStateSpec, EmptyStateVariant, RemediationAction};

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
    fn deref(&self) -> &EmptyStateSpec { &self.spec }
}

impl EmptyState {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self { spec: EmptyStateSpec::new(title), theme: theme.clone(), illustration: None, on_action: None }
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
    pub fn title(mut self, v: impl Into<String>) -> Self { self.spec.title = v.into(); self }
    pub fn message(mut self, v: impl Into<String>) -> Self { self.spec.message = Some(v.into()); self }
    pub fn variant(mut self, v: EmptyStateVariant) -> Self { self.spec.variant = v; self }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self { self.spec.aria_label = Some(v.into()); self }
    pub fn actions(mut self, v: Vec<RemediationAction>) -> Self { self.spec.actions = v; self }
    pub fn compact(mut self, v: bool) -> Self { self.spec.compact = v; self }


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
        let heading_size = resolve_px(theme, "typography.heading.size");
        let control_radius = resolve_radius(theme, "radius.control");
        let gap = resolve_px(theme, spec.layout_gap_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");

        // Compact mode halves the vertical padding and reduces the
        // title size, suitable for embedding in tight containers.
        let vertical_padding = if spec.compact { px(24.0) } else { px(48.0) };
        let title_size = if spec.compact { body_size } else { heading_size };

        let mut container = div()
            .w_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .py(vertical_padding)
            .px(px(24.0))
            .gap(gap);

        // Illustration slot
        if let Some(illustration) = self.illustration {
            container = container.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(illustration),
            );
        }

        // Title
        container = container.child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .text_center()
                .child(spec.title.clone()),
        );

        // Message
        if let Some(ref message) = spec.message {
            container = container.child(
                div()
                    .text_size(body_size)
                    .text_color(text_secondary)
                    .text_center()
                    .max_w(px(400.0))
                    .child(message.clone()),
            );
        }

        // Actions
        if !spec.actions.is_empty() {
            let mut actions_row = div().flex().gap(inline_gap).items_center();

            for action in &spec.actions {
                let is_primary = action.variant == poodle_specs::ButtonVariant::Primary;
                let label = action.label.clone();
                let action_id = SharedString::from(format!("empty-action-{}", action.id));

                let btn = div()
                    .id(action_id)
                    .cursor_pointer()
                    .px(px(16.0))
                    .py(px(8.0))
                    .rounded(control_radius)
                    .text_size(body_size)
                    .font_weight(FontWeight::MEDIUM)
                    .when(is_primary, |el| {
                        el.bg(accent).text_color(gpui::white())
                    })
                    .when(!is_primary, |el| {
                        el.border_1()
                            .border_color(accent)
                            .text_color(accent)
                    })
                    .when(action.is_disabled, |el| el.opacity(0.5))
                    .child(label);

                actions_row = actions_row.child(btn);
            }

            container = container.child(actions_row);
        }

        container.into_any_element()
    }
}
