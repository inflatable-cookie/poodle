//! PickerShell — real GPUI component backed by PickerShellSpec.

use gpui::prelude::FluentBuilder;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{BrowseState, PickerShellSpec, PickerVariant, SpinnerSize, SpinnerSpec};
use poodle_specs::{SpinnerTone, SpinnerVariant};

use crate::presentation::rem_to_px;
use crate::primitives::Spinner;
use crate::theme_ext::{resolve_color, resolve_px, resolve_radius};

/// A real GPUI picker shell component backed by `PickerShellSpec`.
///
/// Renders the shared picker workflow anatomy: header, optional toolbar,
/// optional selection summary, ready/state body, status region, and footer.
pub struct PickerShell {
    spec: PickerShellSpec,
    theme: GpuiThemeProvider,
    toolbar_slot: Option<AnyElement>,
    selection_slot: Option<AnyElement>,
    body_slot: Option<AnyElement>,
    state_slot: Option<AnyElement>,
    footer_slot: Option<AnyElement>,
}

impl std::ops::Deref for PickerShell {
    type Target = PickerShellSpec;
    fn deref(&self) -> &PickerShellSpec {
        &self.spec
    }
}

impl PickerShell {
    pub fn new(title: impl Into<String>, theme: &GpuiThemeProvider) -> Self {
        Self::from_spec(PickerShellSpec::new(title), theme)
    }

    pub fn from_spec(spec: PickerShellSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            toolbar_slot: None,
            selection_slot: None,
            body_slot: None,
            state_slot: None,
            footer_slot: None,
        }
    }

    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.spec.title = v.into();
        self
    }

    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.spec.description = Some(v.into());
        self
    }

    pub fn variant(mut self, v: PickerVariant) -> Self {
        self.spec.variant = v;
        self
    }

    pub fn state(mut self, v: BrowseState) -> Self {
        self.spec.state = v;
        self
    }

    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.spec.query = v.into();
        self
    }

    pub fn result_count(mut self, v: usize) -> Self {
        self.spec.result_count = Some(v);
        self
    }

    pub fn selected_count(mut self, v: usize) -> Self {
        self.spec.selected_count = v;
        self
    }

    pub fn aria_label(mut self, v: impl Into<String>) -> Self {
        self.spec.aria_label = Some(v.into());
        self
    }

    pub fn with_toolbar(mut self, toolbar: impl IntoElement) -> Self {
        self.toolbar_slot = Some(toolbar.into_any_element());
        self
    }

    pub fn with_search(self, search: impl IntoElement) -> Self {
        self.with_toolbar(search)
    }

    pub fn with_selection(mut self, selection: impl IntoElement) -> Self {
        self.selection_slot = Some(selection.into_any_element());
        self
    }

    pub fn with_body(mut self, body: impl IntoElement) -> Self {
        self.body_slot = Some(body.into_any_element());
        self
    }

    pub fn with_results(self, results: impl IntoElement) -> Self {
        self.with_body(results)
    }

    pub fn with_state_content(mut self, state_content: impl IntoElement) -> Self {
        self.state_slot = Some(state_content.into_any_element());
        self
    }

    pub fn with_footer(mut self, footer: impl IntoElement) -> Self {
        self.footer_slot = Some(footer.into_any_element());
        self
    }

    pub fn with_actions(self, actions: impl IntoElement) -> Self {
        self.with_footer(actions)
    }
}

impl IntoElement for PickerShell {
    type Element = AnyElement;

    fn into_element(self) -> Self::Element {
        let theme = &self.theme;
        let spec = &self.spec;

        let panel_x = resolve_px(theme, "space.panel.x");
        let panel_y = resolve_px(theme, "space.panel.y");
        let gap_sm = resolve_px(theme, "space.inline.sm");
        let gap_md = resolve_px(theme, "space.inline.md");
        let stack_sm = resolve_px(theme, "space.stack.sm");
        let stack_md = resolve_px(theme, "space.stack.md");
        let body_size = resolve_px(theme, "typography.body.size");
        let label_size = resolve_px(theme, "typography.label.size");
        let title_size = px(rem_to_px(1.25));
        let surface_radius = resolve_radius(theme, "radius.surface");
        let menu_max_h = resolve_px(theme, "size.menu.maxHeight");
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let border = resolve_color(theme, "color.border.subtle");
        let panel_bg = resolve_color(theme, "color.background.panel");
        let surface_bg = resolve_color(theme, "color.background.surface");

        let mut shell = div()
            .grid()
            .gap(stack_md)
            .bg(panel_bg)
            .border_1()
            .border_color(border)
            .rounded(surface_radius)
            .px(panel_x)
            .py(panel_y)
            .when(spec.is_modal_like(), |el| {
                el.shadow(crate::theme_ext::elevation_dialog_shadow())
                .max_w(px(480.0))
            });

        let mut title_block = div().flex().flex_col().gap(stack_sm).child(
            div()
                .text_size(title_size)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(text_primary)
                .child(spec.title.clone()),
        );

        if let Some(description) = spec.description.as_ref() {
            title_block = title_block.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(description.clone()),
            );
        }

        let mut meta = div().flex().flex_wrap().items_center().gap(gap_sm).child(
            div()
                .text_size(label_size)
                .text_color(text_secondary)
                .child(spec.selected_count_text()),
        );

        if let Some(result_text) = spec.result_count_text() {
            meta = meta.child(
                div()
                    .text_size(label_size)
                    .text_color(text_secondary)
                    .child(result_text),
            );
        }

        shell = shell.child(
            div()
                .w_full()
                .flex()
                .flex_wrap()
                .justify_between()
                .gap(gap_md)
                .child(title_block)
                .child(meta),
        );

        if let Some(toolbar) = self.toolbar_slot {
            shell = shell.child(div().w_full().child(toolbar));
        }

        if let Some(selection) = self.selection_slot {
            shell = shell.child(div().w_full().child(selection));
        }

        if let Some(status_text) = spec.status_text.as_ref() {
            let status_id = spec
                .status_id
                .clone()
                .unwrap_or_else(|| "poodle-picker-shell-status".to_string());
            let status = div()
                .id(SharedString::from(status_id))
                .w(px(1.0))
                .h(px(1.0))
                .overflow_hidden()
                .child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(status_text.clone()),
                );
            shell = shell.child(status);
        }

        if spec.state == BrowseState::Ready {
            let mut body = div()
                .id("poodle-picker-shell-body")
                .w_full()
                .min_h(px(0.0))
                .overflow_y_scroll()
                .max_h(menu_max_h);
            if let Some(content) = self.body_slot {
                body = body.child(content);
            }
            shell = shell.child(body);
        } else if let Some(state_content) = self.state_slot {
            shell = shell.child(div().w_full().child(state_content));
        } else {
            let mut state = div()
                .w_full()
                .grid()
                .gap(stack_sm)
                .justify_start()
                .px(panel_x)
                .py(panel_y * 1.5)
                .border_1()
                .border_color(border)
                .rounded(surface_radius)
                .bg(surface_bg);

            if spec.state == BrowseState::Loading {
                state = state.child(
                    div().child(Spinner::from_spec(
                        SpinnerSpec::new()
                            .with_variant(SpinnerVariant::Grid)
                            .with_size(SpinnerSize::Md)
                            .with_tone(SpinnerTone::Accent),
                        theme,
                    )),
                );
            }

            state = state.child(
                div()
                    .text_size(body_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_primary)
                    .child(spec.effective_state_title().to_string()),
            );

            if let Some(message) = spec.effective_state_message() {
                state = state.child(
                    div()
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .child(message.to_string()),
                );
            }

            shell = shell.child(state);
        }

        if let Some(footer) = self.footer_slot {
            shell = shell.child(
                div()
                    .w_full()
                    .flex()
                    .items_center()
                    .gap(resolve_px(theme, spec.footer_gap_token()))
                    .child(footer),
            );
        }

        shell.into_any_element()
    }
}
