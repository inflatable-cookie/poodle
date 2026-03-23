//! ActionDiscoveryPanel — real GPUI component backed by ActionDiscoveryPanelSpec.

use gpui::*;
use flint_adapter::ThemeProvider;
use flint_gpui::GpuiThemeProvider;
use flint_composites::{ActionDiscoveryPanelSpec, ActionDiscoverySection, DiscoveryState};

use crate::theme_ext::{resolve_color, resolve_opacity};

/// A real GPUI action discovery panel backed by `ActionDiscoveryPanelSpec`.
///
/// Renders categorized action sections, each with a heading and a list of
/// action items showing title, shortcut, and optional badge.
pub struct ActionDiscoveryPanel {
    spec: ActionDiscoveryPanelSpec,
    theme: GpuiThemeProvider,
    id_prefix: String,
    on_select: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for ActionDiscoveryPanel {
    type Target = ActionDiscoveryPanelSpec;
    fn deref(&self) -> &ActionDiscoveryPanelSpec { &self.spec }
}

impl ActionDiscoveryPanel {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: ActionDiscoveryPanelSpec::default(), theme: theme.clone(), id_prefix: String::new(), on_select: None }
    }

    pub fn from_spec(spec: ActionDiscoveryPanelSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            id_prefix: "flint-action-discovery".to_string(),
            on_select: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn sections(mut self, v: Vec<ActionDiscoverySection>) -> Self { self.spec.sections = v; self }
    pub fn state(mut self, v: DiscoveryState) -> Self { self.spec.state = v; self }
    pub fn empty_message(mut self, v: impl Into<String>) -> Self { self.spec.empty_message = Some(v.into()); self }


    pub fn with_id(mut self, prefix: impl Into<String>) -> Self {
        self.id_prefix = prefix.into();
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl IntoElement for ActionDiscoveryPanel {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let text_primary = resolve_color(theme, "semantic.color.text.primary");
        let text_secondary = resolve_color(theme, "semantic.color.text.secondary");
        let text_muted = resolve_color(theme, "semantic.color.text.muted");
        let border = resolve_color(theme, "semantic.color.border.default");
        let disabled_opacity = resolve_opacity(theme, "semantic.state.opacity.disabled");
        let hover_bg = resolve_color(theme, "semantic.color.background.elevated");
        let focus_ring = resolve_color(theme, "semantic.color.accent.focusRing");
        let gap = theme.resolve_space(spec.gap_token());

        let mut panel = div()
            .flex()
            .flex_col()
            .gap(px(gap))
            .size_full();

        match spec.state {
            DiscoveryState::Loading => {
                panel = panel.child(
                    div()
                        .p(px(16.0))
                        .text_size(px(14.0))
                        .text_color(text_secondary)
                        .child("Loading actions\u{2026}"),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Error => {
                panel = panel.child(
                    div()
                        .p(px(16.0))
                        .text_size(px(14.0))
                        .text_color(resolve_color(theme, "semantic.color.status.danger"))
                        .child("Failed to load actions"),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Empty | DiscoveryState::NoResults => {
                let msg = spec
                    .empty_message
                    .as_deref()
                    .unwrap_or("No actions found");
                panel = panel.child(
                    div()
                        .p(px(16.0))
                        .text_size(px(14.0))
                        .text_color(text_secondary)
                        .child(msg.to_string()),
                );
                return panel.into_any_element();
            }
            DiscoveryState::Ready => {}
        }

        // Render each section
        for section in &spec.sections {
            let mut section_el = div().flex().flex_col().gap(px(4.0));

            // Section heading
            section_el = section_el.child(
                div()
                    .px(px(8.0))
                    .py(px(4.0))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_muted)
                    .child(section.title.clone()),
            );

            // Description
            if let Some(ref desc) = section.description {
                section_el = section_el.child(
                    div()
                        .px(px(8.0))
                        .text_size(px(12.0))
                        .text_color(text_secondary)
                        .child(desc.clone()),
                );
            }

            // Action items
            for action in &section.actions {
                let action_id =
                    SharedString::from(format!("{}-{}", self.id_prefix, action.id));

                let mut row = div()
                    .id(action_id)
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(8.0))
                    .py(px(6.0))
                    .rounded(px(4.0))
                    .text_size(px(14.0))
                    .text_color(text_primary);

                row = row.focus(move |s| s.border_color(focus_ring));

                if action.is_disabled {
                    row = row
                        .opacity(disabled_opacity)
                        .cursor(CursorStyle::OperationNotAllowed);
                } else {
                    row = row
                        .cursor_pointer()
                        .hover(|s| s.bg(hover_bg));
                }

                // Left: title + badge
                let mut left = div().flex().items_center().gap(px(6.0));
                left = left.child(action.title.clone());

                if let Some(ref badge) = action.badge {
                    left = left.child(
                        div()
                            .text_size(px(12.0))
                            .px(px(4.0))
                            .py(px(1.0))
                            .rounded(px(3.0))
                            .bg(resolve_color(theme, "semantic.color.accent.base").opacity(0.12))
                            .text_color(resolve_color(theme, "semantic.color.accent.base"))
                            .child(badge.clone()),
                    );
                }

                row = row.child(left);

                // Right: shortcut
                if let Some(ref shortcut) = action.shortcut {
                    row = row.child(
                        div()
                            .text_size(px(12.0))
                            .text_color(text_muted)
                            .child(shortcut.clone()),
                    );
                }

                section_el = section_el.child(row);
            }

            // Section separator
            section_el = section_el.child(
                div()
                    .h(px(1.0))
                    .w_full()
                    .bg(border),
            );

            panel = panel.child(section_el);
        }

        panel.into_any_element()
    }
}
