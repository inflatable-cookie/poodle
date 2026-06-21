//! DetailShell — real GPUI component backed by DetailShellSpec.

use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{DetailShellSpec, DetailState, ScrollOwner};
use poodle_specs::{SpinnerSize, SpinnerSpec, SpinnerTone, SpinnerVariant};

use crate::primitives::Spinner;
use crate::theme_ext::{color_mix, resolve_color, resolve_px};

/// A real GPUI detail shell component backed by `DetailShellSpec`.
///
/// Renders a detail page container with a header area (title) and a content slot.
/// Shows loading/error/empty states when content is not ready.
pub struct DetailShell {
    spec: DetailShellSpec,
    theme: GpuiThemeProvider,
    header_slot: Option<AnyElement>,
    content_slot: Option<AnyElement>,
    state_slot: Option<AnyElement>,
}

impl std::ops::Deref for DetailShell {
    type Target = DetailShellSpec;
    fn deref(&self) -> &DetailShellSpec {
        &self.spec
    }
}

impl DetailShell {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: DetailShellSpec::new(),
            theme: theme.clone(),
            header_slot: None,
            content_slot: None,
            state_slot: None,
        }
    }

    pub fn from_spec(spec: DetailShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            header_slot: None,
            content_slot: None,
            state_slot: None,
        }
    }

    // ── Forwarded spec builders ───────────────────────────────
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = Some(v.into());
        self
    }
    pub fn scroll_owner(mut self, v: ScrollOwner) -> Self {
        self.spec.scroll_owner = v;
        self
    }
    pub fn state(mut self, v: DetailState) -> Self {
        self.spec.state = v;
        self
    }
    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }
    pub fn state_title(mut self, v: impl Into<String>) -> Self {
        self.spec.state_title = Some(v.into());
        self
    }
    pub fn state_message(mut self, v: impl Into<String>) -> Self {
        self.spec.state_message = Some(v.into());
        self
    }

    pub fn with_header(mut self, header: impl IntoElement) -> Self {
        self.header_slot = Some(header.into_any_element());
        self
    }

    pub fn with_content(mut self, content: impl IntoElement) -> Self {
        self.content_slot = Some(content.into_any_element());
        self
    }

    pub fn with_state_content(mut self, content: impl IntoElement) -> Self {
        self.state_slot = Some(content.into_any_element());
        self
    }
}

impl IntoElement for DetailShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        // ── Contract §9-10 token-resolved geometry ───────────────
        let inline_gap = resolve_px(theme, "space.inline.sm");
        // Root + region stacking gap = space.stack.lg.
        let stack_gap = resolve_px(theme, spec.stack_gap_token());
        let body_size = resolve_px(theme, "typography.body.size");
        let heading_size = resolve_px(theme, "typography.heading.size");

        // Header padding from panel tokens (no raw px).
        let panel_x = resolve_px(theme, "space.panel.x");
        let panel_y = resolve_px(theme, "space.panel.y");
        // State region: doubled panel-y, 1.5× panel-x (Svelte `calc(panel-y*2) calc(panel-x*1.5)`).
        let state_pad_y = resolve_px(theme, spec.state_pad_y_token()) * 2.0;
        let state_pad_x = resolve_px(theme, spec.state_pad_x_token()) * 1.5;
        // Loading spinner→copy gap = space.stack.md.
        let loading_gap = resolve_px(theme, "space.stack.md");

        let body_bg = resolve_color(theme, spec.body_fill_token());
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, spec.state_message_color_token());
        let state_title_color = resolve_color(theme, spec.state_title_color_token());
        let border = resolve_color(theme, "color.border.subtle");
        // State surface: background-panel mixed 96% toward background-elevated.
        let state_bg = color_mix(
            resolve_color(theme, spec.state_fill_token()),
            resolve_color(theme, spec.state_fill_mix_token()),
            0.96,
        );
        let state_radius = resolve_px(theme, spec.state_radius_token());

        let mut shell = div()
            .id("poodle-detail-shell")
            .w_full()
            .flex()
            .flex_col()
            .flex_grow()
            .gap(stack_gap)
            .bg(body_bg);
        // Scroll ownership (contract §3 `scrollMode`): shell scrolls vs body scrolls.
        shell = match spec.scroll_owner {
            ScrollOwner::Shell => shell.overflow_y_scroll(),
            ScrollOwner::Content => shell.overflow_hidden(),
        };

        // Header section (rendered when a title or header slot is present).
        if spec.title.is_some() || self.header_slot.is_some() {
            let mut header = div()
                .w_full()
                .px(panel_x)
                .py(panel_y)
                .border_b_1()
                .border_color(border)
                .flex()
                .items_center()
                .gap(inline_gap);

            if let Some(ref title) = spec.title {
                header = header.child(
                    div()
                        .text_size(heading_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(text_primary)
                        .child(title.clone()),
                );
            }

            if let Some(header_content) = self.header_slot {
                header = header.child(header_content);
            }

            shell = shell.child(header);
        }

        // Body / state region — state-dependent.
        let body = if spec.state == DetailState::Ready {
            let mut content_area = div()
                .id("poodle-detail-shell-body")
                .w_full()
                .flex_grow()
                .px(panel_x)
                .py(panel_y);
            // When the body owns scrolling, it scrolls; otherwise the shell does.
            if spec.scroll_owner == ScrollOwner::Content {
                content_area = content_area.overflow_y_scroll();
            }
            if let Some(content) = self.content_slot {
                content_area = content_area.child(content);
            }
            content_area.into_any_element()
        } else {
            // State region: subtle surface, doubled padding, radius-surface.
            let mut region = div()
                .w_full()
                .flex_grow()
                .flex()
                .flex_col()
                .gap(stack_gap)
                .px(state_pad_x)
                .py(state_pad_y)
                .bg(state_bg)
                .rounded(state_radius);

            if let Some(state_content) = self.state_slot {
                // Host-provided custom state content.
                region = region.child(state_content);
            } else {
                // Loading prepends the shared grid spinner before the copy.
                if spec.state == DetailState::Loading {
                    region = region.child(
                        div()
                            .flex()
                            .gap(loading_gap)
                            .items_center()
                            .child(Spinner::from_spec(
                                SpinnerSpec::new()
                                    .with_variant(SpinnerVariant::Grid)
                                    .with_size(SpinnerSize::Md)
                                    .with_tone(SpinnerTone::Accent),
                                theme,
                            )),
                    );
                }
                // Default <strong> state title + optional <p> message.
                region = region.child(
                    div()
                        .text_size(body_size)
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(state_title_color)
                        .child(spec.effective_state_title().to_string()),
                );
                if let Some(ref message) = spec.state_message {
                    region = region.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_secondary)
                            .child(message.clone()),
                    );
                }
            }

            region.into_any_element()
        };

        shell = shell.child(body);

        shell.into_any_element()
    }
}
