//! MarkdownEditor — markdown editing with preview backed by MarkdownEditorSpec.

use gpui::*;
use flint_gpui::GpuiThemeProvider;
use flint_composites::MarkdownEditorSpec;
use flint_primitives::{IconSize, IconSpec};
use crate::primitives::Icon;
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};

pub struct MarkdownEditor {
    spec: MarkdownEditorSpec,
    theme: GpuiThemeProvider,
    on_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
    on_mode_change: Option<Box<dyn Fn(&str, &mut Window, &mut App) + 'static>>,
}

impl std::ops::Deref for MarkdownEditor {
    type Target = MarkdownEditorSpec;
    fn deref(&self) -> &MarkdownEditorSpec { &self.spec }
}

impl MarkdownEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self { spec: MarkdownEditorSpec::new(), theme: theme.clone(), on_change: None, on_mode_change: None }
    }
    pub fn from_spec(spec: MarkdownEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self { spec, theme: theme.clone(), on_change: None, on_mode_change: None }
    }

    /// Called when the markdown content changes.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler)); self
    }

    /// Called when the editing mode changes (edit/split/preview).
    pub fn on_mode_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_mode_change = Some(Box::new(handler)); self
    }
}

impl IntoElement for MarkdownEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let fill = resolve_color(&self.theme, self.spec.fill_token());
        let border = resolve_color(&self.theme, self.spec.border_token());
        let toolbar_fill = resolve_color(&self.theme, self.spec.toolbar_fill_token());
        let radius = resolve_radius(&self.theme, "semantic.radius.surface");
        let text_color = resolve_color(&self.theme, "semantic.color.text.primary");
        let muted = resolve_color(&self.theme, "semantic.color.text.secondary");
        let hover_bg = resolve_color(&self.theme, "semantic.color.bg.hover");
        let active_bg = resolve_color(&self.theme, "semantic.color.bg.active");

        let display = if self.spec.value.is_empty() { self.spec.placeholder.as_deref().unwrap_or("Type here...") } else { &self.spec.value };
        let color = if self.spec.value.is_empty() { muted } else { text_color };

        // Helper: toolbar icon button
        let toolbar_btn = |icon_name: &str, theme: &GpuiThemeProvider| -> Div {
            div()
                .flex().items_center().justify_center()
                .w(px(28.0)).h(px(24.0)).rounded(px(4.0))
                .cursor(CursorStyle::PointingHand)
                .hover(|s| s.bg(hover_bg))
                .child(
                    Icon::from_spec(
                        IconSpec::new(icon_name).with_size(IconSize::Sm),
                        theme,
                    ).with_color(muted)
                )
        };

        // Wrap on_mode_change in Rc for sharing across buttons
        let on_mode_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
            self.on_mode_change.map(|h| std::rc::Rc::from(h));

        let min_h = self.spec.min_height.as_deref()
            .and_then(|v| v.trim_end_matches("px").parse::<f32>().ok())
            .unwrap_or(200.0);

        let mut el = div()
            .bg(fill).border_1().border_color(border).rounded(radius)
            .flex().flex_col().min_h(px(min_h))
            .overflow_hidden();

        let mode = self.spec.mode.as_str();
        let is_edit = mode == "edit" || mode == "split";
        let is_preview = mode == "preview" || mode == "split";

        // Toolbar separator
        let separator = || -> Div {
            div().w(px(1.0)).h(px(16.0)).bg(border).mx(px(4.0))
        };

        // Helper: mode switcher button
        let mode_btn = |label: &'static str, is_active: bool, mode_val: &'static str,
                        handler: &Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>>| -> Stateful<Div> {
            let id = SharedString::from(format!("flint-md-mode-{}", mode_val));
            let mut btn = div()
                .id(id)
                .px(px(8.0)).py(px(2.0)).rounded(px(3.0))
                .text_size(px(12.0))
                .cursor(CursorStyle::PointingHand)
                .child(label.to_string());
            if is_active {
                btn = btn.bg(active_bg).text_color(text_color);
            } else {
                btn = btn.text_color(muted).hover(|s| s.bg(hover_bg));
            }
            if !is_active {
                if let Some(ref h) = handler {
                    let h = h.clone();
                    let mv = mode_val.to_string();
                    btn = btn.on_click(move |_event, window, cx| {
                        h(&mv, window, cx);
                    });
                }
            }
            btn
        };

        // Toolbar
        el = el.child(
            div().bg(toolbar_fill).px(px(8.0)).py(px(4.0))
                .flex().flex_row().items_center().gap(px(2.0))
                .border_b_1().border_color(border)
                // Text formatting icons
                .child(toolbar_btn("bold", &self.theme))
                .child(toolbar_btn("italic", &self.theme))
                .child(toolbar_btn("heading", &self.theme))
                .child(toolbar_btn("code", &self.theme))
                // Separator
                .child(separator())
                // Structure icons
                .child(toolbar_btn("link", &self.theme))
                .child(toolbar_btn("list", &self.theme))
                .child(toolbar_btn("quote", &self.theme))
                // Spacer pushes mode switcher to the right
                .child(div().flex_grow())
                // Mode switcher segment
                .child(
                    div().flex().flex_row().gap(px(2.0))
                        .px(px(2.0)).py(px(2.0))
                        .rounded(px(4.0))
                        .child(mode_btn("Edit", mode == "edit", "edit", &on_mode_rc))
                        .child(mode_btn("Split", mode == "split", "split", &on_mode_rc))
                        .child(mode_btn("Preview", mode == "preview", "preview", &on_mode_rc))
                )
        );

        // Content area: edit pane, preview pane, or both (split)
        let content_area = div().flex().flex_row().flex_grow().min_h(px(0.0));

        let content_area = if is_edit {
            // Editor pane
            let mut editor_pane = div()
                .id("flint-md-editor-pane")
                .focusable()
                .px(px(12.0)).py(px(8.0)).flex_grow().flex_basis(px(0.0))
                .text_size(px(14.0)).text_color(color)
                .overflow_y_scroll()
                .child(display.to_string());

            // Basic text editing via key events
            if !self.spec.is_disabled {
                let current_value = self.spec.value.clone();
                let on_change_rc: Option<std::rc::Rc<dyn Fn(&str, &mut Window, &mut App)>> =
                    self.on_change.map(|h| std::rc::Rc::from(h));
                if let Some(ref handler) = on_change_rc {
                    let handler = handler.clone();
                    editor_pane = editor_pane.on_key_down(move |event: &KeyDownEvent, window, cx| {
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
                        } else if key.len() == 1 && !event.keystroke.modifiers.platform && !event.keystroke.modifiers.control {
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

        let content_area = if is_edit && is_preview {
            // Vertical divider between panes in split mode
            content_area.child(
                div().w(px(1.0)).bg(border)
            )
        } else {
            content_area
        };

        let content_area = if is_preview {
            // Preview pane
            let preview_content = if self.spec.value.is_empty() {
                "Nothing to preview".to_string()
            } else {
                self.spec.value.clone()
            };
            let preview_text_color = if self.spec.value.is_empty() { muted } else { text_color };
            let preview_pane = div()
                .id("flint-md-preview-pane")
                .px(px(12.0)).py(px(8.0)).flex_grow().flex_basis(px(0.0))
                .text_size(px(14.0)).text_color(preview_text_color)
                .overflow_y_scroll()
                .child(preview_content);
            content_area.child(preview_pane)
        } else {
            content_area
        };

        el = el.child(content_area);

        if self.spec.is_disabled {
            el = el.opacity(resolve_opacity(&self.theme, "semantic.state.opacity.disabled"));
        }
        el.into_any_element()
    }
}
