//! MarkdownEditor — markdown editing with preview backed by MarkdownEditorSpec.

use crate::presentation::{rem_to_px, resolve_semantic_size, size_font_rem};
use crate::primitives::{Icon, IconButton};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::MarkdownEditorSpec;
use poodle_specs::{
    ControlDensity, ControlSize, IconButtonSpec, IconSize, IconSpec, SemanticControlSizeRole,
};

pub struct MarkdownEditor {
    spec: MarkdownEditorSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_mode_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for MarkdownEditor {
    type Target = MarkdownEditorSpec;
    fn deref(&self) -> &MarkdownEditorSpec {
        &self.spec
    }
}

impl MarkdownEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: MarkdownEditorSpec::new(),
            theme: theme.clone(),
            on_change: None,
            on_mode_change: None,
        }
    }
    pub fn from_spec(spec: MarkdownEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            on_change: None,
            on_mode_change: None,
        }
    }

    /// Called when the markdown content changes.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Called when the editing mode changes (edit/split/preview).
    pub fn on_mode_change(
        mut self,
        handler: impl Fn(&str, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_mode_change = Some(Box::new(handler));
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

impl IntoElement for MarkdownEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let effective_size = resolve_semantic_size(self.spec.size, self.spec.size_role);

        // ── Size / density geometry (contract §8 tables, token-resolved rem) ──
        let tool_size = rem_to_px(self.spec.tool_size_rem());
        let toolbar_y = rem_to_px(self.spec.toolbar_y_rem());
        let toolbar_x = rem_to_px(self.spec.toolbar_x_rem());
        let tool_gap = rem_to_px(self.spec.tool_gap_rem());
        let pane_pad = rem_to_px(self.spec.pane_pad_rem());
        let mode_y = rem_to_px(self.spec.mode_y_rem());
        let toolbar_gap = rem_to_px(0.5); // contract toolbar `gap: 0.5rem`
        // Textarea font-size 0.8125rem (contract); preview body 0.875rem.
        let textarea_size = px(rem_to_px(0.8125));
        let preview_size = px(rem_to_px(0.875));
        let _ = size_font_rem(effective_size); // size scale exercised via tool_size

        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let toolbar_border = resolve_color(&self.theme, self.spec.toolbar_border_token());
        let split_divider = resolve_color(&self.theme, self.spec.split_divider_color_token());
        let toolbar_fill = resolve_color(&self.theme, self.spec.toolbar_fill_token());
        let radius = resolve_radius(&self.theme, "radius.surface");
        let radius_control = resolve_radius(&self.theme, "radius.control");
        let text_color = resolve_color(&self.theme, self.spec.textarea_color_token());
        let tool_color = resolve_color(&self.theme, self.spec.tool_color_token());
        let tool_hover_color = resolve_color(&self.theme, self.spec.tool_hover_color_token());
        let placeholder_color = resolve_color(&self.theme, self.spec.placeholder_color_token());
        let preview_empty_color =
            resolve_color(&self.theme, self.spec.preview_empty_color_token());
        // Tool hover fill: accent-base @ 12% (contract `color-mix(accent 12%, transparent)`).
        let accent = resolve_color(&self.theme, self.spec.tool_hover_fill_token());
        let tool_hover_bg = Hsla { a: accent.a * 0.12, ..accent };

        let display = if self.spec.value.is_empty() {
            self.spec.placeholder.as_deref().unwrap_or("Write markdown...")
        } else {
            &self.spec.value
        };
        let color = if self.spec.value.is_empty() {
            placeholder_color
        } else {
            text_color
        };

        let tools_disabled = self.spec.tools_disabled();
        let tool_disabled_opacity = 0.4_f32; // contract tool `:disabled` opacity

        // Helper: toolbar icon button (contract anatomy: tool-button per action)
        let toolbar_btn = |icon_name: &str, theme: &GpuiThemeProvider| -> Div {
            let mut b = div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(tool_size))
                .h(px(tool_size))
                .rounded(radius_control)
                .child(
                    Icon::from_spec(IconSpec::new(icon_name).with_size(IconSize::Sm), theme)
                        .with_color(tool_color),
                );
            if tools_disabled {
                b = b.opacity(tool_disabled_opacity).cursor(CursorStyle::Arrow);
            } else {
                let hc = tool_hover_color;
                b = b
                    .cursor(CursorStyle::PointingHand)
                    .hover(move |s| s.bg(tool_hover_bg).text_color(hc));
            }
            b
        };

        // Wrap on_mode_change in Rc for sharing across buttons
        let on_mode_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_mode_change.map(|h| std::rc::Rc::from(h));

        // Contract minHeight default "12rem"; parse rem/px via the spec helper.
        let min_h = rem_to_px(self.spec.min_height_rem());

        let mut el = div()
            .bg(fill)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .flex()
            .flex_col()
            .min_h(px(min_h))
            .overflow_hidden();

        let mode = self.spec.mode.as_str();
        let is_edit = self.spec.shows_editor();
        let is_preview = self.spec.shows_preview();

        let mode_btn = |icon: &'static str,
                        tooltip: &'static str,
                        is_active: bool,
                        mode_val: &'static str,
                        handler: &Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>>|
         -> AnyElement {
            let mut btn = IconButton::from_spec(
                IconButtonSpec::new()
                    .with_icon(icon)
                    .with_aria_label(tooltip)
                    .with_tooltip(tooltip)
                    .with_variant(if is_active {
                        poodle_specs::ButtonVariant::Secondary
                    } else {
                        poodle_specs::ButtonVariant::Ghost
                    })
                    .with_size(self.spec.size)
                    .with_size_role(self.spec.size_role)
                    .with_density(self.spec.density),
                &self.theme,
            )
            .with_id(format!("md-mode-{mode_val}"));

            if !is_active {
                if let Some(ref h) = handler {
                    let h = h.clone();
                    let mv = mode_val.to_string();
                    btn = btn.on_click(move |_event, window, cx| {
                        h(&mv, window, cx);
                    });
                }
            }

            btn.into_any_element()
        };

        // Toolbar: tools (left) + mode switcher (right), space-between.
        // Tool order matches contract §2 anatomy: bold, italic, heading, link,
        // code, quote, list. No separator — the contract anatomy lists none.
        el = el.child(
            div()
                .bg(toolbar_fill)
                .px(px(toolbar_x))
                .py(px(toolbar_y))
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap(px(toolbar_gap))
                .flex_wrap()
                .border_b_1()
                .border_color(toolbar_border)
                // Tools container
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(tool_gap))
                        .child(toolbar_btn("bold", &self.theme))
                        .child(toolbar_btn("italic", &self.theme))
                        .child(toolbar_btn("heading", &self.theme))
                        .child(toolbar_btn("link", &self.theme))
                        .child(toolbar_btn("code", &self.theme))
                        .child(toolbar_btn("quote", &self.theme))
                        .child(toolbar_btn("list", &self.theme)),
                )
                // Mode switcher segment
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(tool_gap))
                        .px(px(rem_to_px(self.spec.mode_x_rem())))
                        .py(px(mode_y))
                        .child(mode_btn(
                            "pencil",
                            "Edit",
                            mode == "edit",
                            "edit",
                            &on_mode_rc,
                        ))
                        .child(mode_btn(
                            "columns-2",
                            "Split",
                            mode == "split",
                            "split",
                            &on_mode_rc,
                        ))
                        .child(mode_btn(
                            "eye",
                            "Preview",
                            mode == "preview",
                            "preview",
                            &on_mode_rc,
                        )),
                ),
        );

        // Content area: edit pane, preview pane, or both (split)
        let content_area = div().flex().flex_row().flex_grow().min_h(px(0.0));

        let content_area = if is_edit {
            // Editor pane. In split mode the textarea gets a right border
            // (contract `border-right: 0.0625rem solid border-subtle`).
            let mut editor_pane = div()
                .id("poodle-md-editor-pane")
                .focusable()
                .px(px(pane_pad))
                .py(px(pane_pad))
                .flex_grow()
                .flex_basis(px(0.0))
                .text_size(textarea_size)
                .text_color(color)
                .overflow_y_scroll()
                .child(display.to_string());

            if is_preview {
                // Contract split textarea `border-right: 0.0625rem` (= 1px).
                editor_pane = editor_pane.border_r_1().border_color(split_divider);
            }

            // Basic text editing via key events
            if !self.spec.is_disabled {
                let current_value = self.spec.value.clone();
                let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
                    self.on_change.map(|h| std::rc::Rc::from(h));
                if let Some(ref handler) = on_change_rc {
                    let handler = handler.clone();
                    editor_pane =
                        editor_pane.on_key_down(move |event: &KeyDownEvent, window, cx| {
                            let key = event.keystroke.key.as_str();
                            if key == "enter" {
                                let new_val = format!("{}\n", current_value);
                                handler(&new_val, window, cx);
                            } else if key == "backspace" {
                                let mut chars: Vec<char> = current_value.chars().collect();
                                if !chars.is_empty() {
                                    chars.pop();
                                    let new_val: String = chars.into_iter().collect();
                                    handler(&new_val, window, cx);
                                }
                            } else if key.len() == 1
                                && !event.keystroke.modifiers.platform
                                && !event.keystroke.modifiers.control
                            {
                                let new_val = format!("{}{}", current_value, key);
                                handler(&new_val, window, cx);
                            }
                        });
                }
            }

            content_area.child(editor_pane)
        } else {
            content_area
        };

        // Split separation is the textarea's right border (set above), not a
        // standalone divider div — matches contract anatomy.

        let content_area = if is_preview {
            // Preview pane. The contract preview renders parsed HTML; rendering a
            // markdown→HTML tree is Tier-3 freedom and lives in the preview loop,
            // so this shows the source text (or the empty placeholder).
            let preview_content = if self.spec.value.is_empty() {
                "Nothing to preview".to_string()
            } else {
                self.spec.value.clone()
            };
            let preview_text_color = if self.spec.value.is_empty() {
                preview_empty_color
            } else {
                text_color
            };
            let preview_pane = div()
                .id("poodle-md-preview-pane")
                .px(px(pane_pad))
                .py(px(pane_pad))
                .flex_grow()
                .flex_basis(px(0.0))
                .text_size(preview_size)
                .text_color(preview_text_color)
                .overflow_y_scroll()
                .child(preview_content);
            content_area.child(preview_pane)
        } else {
            content_area
        };

        el = el.child(content_area);

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
