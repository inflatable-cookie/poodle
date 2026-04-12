use crate::app_state::AppState;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, Dialog, Eyebrow, Pill};
use poodle_specs::{
    ButtonSpec, ButtonVariant, DialogSpec, DialogWidth, EyebrowSpec, PillAppearance, PillSpec,
    PillTone,
};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let border_default = theme.resolve_color("color.border.default");
    let panel_bg = theme.resolve_color("color.background.panel");
    let accent = theme.resolve_color("color.accent.base");
    let border_subtle = theme.resolve_color("color.border.subtle");
    let root_handle = cx.weak_entity();

    let button_row = |label: &'static str, button: AnyElement| {
        div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(12.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(label),
                theme,
            ))
            .child(button)
    };

    let open_button = |id: &'static str,
                       key: &'static str,
                       label: &'static str,
                       cx: &mut Context<PreviewRoot>| {
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label(label),
            theme,
        )
        .with_id(id)
        .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
            this.state.specimens.toggles.insert(key.to_string(), true);
            cx.notify();
        }))
        .into_any_element()
    };

    let mut content = div().flex().flex_col().gap(px(12.0)).child(button_row(
        "Informational",
        open_button(
            "dialog-shortcuts-trigger",
            "dialog-shortcuts-open",
            "View details",
            cx,
        ),
    ));

    content = content.child(button_row(
        "Form",
        open_button(
            "dialog-form-trigger",
            "dialog-form-open",
            "Create project",
            cx,
        ),
    ));

    content = content.child(button_row(
        "Custom header",
        open_button(
            "dialog-changelog-trigger",
            "dialog-changelog-open",
            "View changelog",
            cx,
        ),
    ));

    content = content.child(button_row(
        "Custom footer",
        open_button(
            "dialog-terms-trigger",
            "dialog-terms-open",
            "Terms & conditions",
            cx,
        ),
    ));

    content = content.child(button_row(
        "Bare mode",
        open_button(
            "dialog-bare-trigger",
            "dialog-bare-open",
            "Preview image",
            cx,
        ),
    ));

    content = content.child(button_row(
        "Scrollable",
        open_button(
            "dialog-scroll-trigger",
            "dialog-scroll-open",
            "View log",
            cx,
        ),
    ));

    let width_buttons = div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(8.0))
        .child(open_button(
            "dialog-width-sm-trigger",
            "dialog-width-sm-open",
            "sm",
            cx,
        ))
        .child(open_button(
            "dialog-width-md-trigger",
            "dialog-width-md-open",
            "md",
            cx,
        ))
        .child(open_button(
            "dialog-width-lg-trigger",
            "dialog-width-lg-open",
            "lg",
            cx,
        ))
        .child(open_button(
            "dialog-width-xl-trigger",
            "dialog-width-xl-open",
            "xl",
            cx,
        ))
        .into_any_element();
    content = content.child(button_row("Width presets", width_buttons));

    content = content.child(button_row(
        "Non-dismissible",
        open_button(
            "dialog-persistent-trigger",
            "dialog-persistent-open",
            "Open persistent",
            cx,
        ),
    ));

    let mut root = div().flex().flex_col().gap(px(24.0)).child(content);

    if state.specimens.is_on("dialog-shortcuts-open") {
        let shortcuts = |keys: &'static str, label: &'static str| {
            let mut kbd_bg = color_to_hsla(panel_bg);
            kbd_bg.a *= 0.8;
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .min_w(px(72.0))
                        .px(px(8.0))
                        .py(px(4.0))
                        .border_1()
                        .border_color(color_to_hsla(border_default))
                        .rounded(px(4.0))
                        .bg(kbd_bg)
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(keys),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
        };

        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Keyboard shortcuts")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-shortcuts-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(shortcuts("\u{2318} K", "Command palette"))
                    .child(shortcuts("\u{2318} S", "Save"))
                    .child(shortcuts("\u{2318} /", "Toggle comment"))
                    .child(shortcuts("\u{2318} \u{21E7} P", "Quick actions"))
                    .child(shortcuts("Esc", "Close dialog")),
            ),
        );
    }

    if state.specimens.is_on("dialog-form-open") {
        let field = |label: &'static str, placeholder: &'static str| {
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child(label),
                )
                .child(
                    div()
                        .h(px(32.0))
                        .px(px(10.0))
                        .flex()
                        .items_center()
                        .border_1()
                        .border_color(color_to_hsla(border_default))
                        .rounded(px(4.0))
                        .bg(color_to_hsla(panel_bg))
                        .text_size(px(13.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(placeholder),
                )
        };

        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("New project")
                    .with_description("Set up a new project workspace.")
                    .with_width(DialogWidth::Lg)
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-form-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .child(
                        div()
                            .flex()
                            .gap(px(16.0))
                            .child(div().flex_1().child(field("Project name", "My project")))
                            .child(div().flex_1().child(field("Template", "Choose a template"))),
                    )
                    .child(field("Description", "What is this project for?"))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .border_1()
                                    .border_color(color_to_hsla(border_default))
                                    .rounded(px(3.0)),
                            )
                            .child(div().text_size(px(13.0)).child("Make this project private")),
                    ),
            )
            .with_actions(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Cancel"),
                            theme,
                        )
                        .with_id("dialog-form-cancel")
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("dialog-form-open".to_string(), false);
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Button::from_spec(ButtonSpec::new().with_label("Create project"), theme)
                            .with_id("dialog-form-create")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("dialog-form-open".to_string(), false);
                                cx.notify();
                            })),
                    ),
            ),
        );
    }

    if state.specimens.is_on("dialog-changelog-open") {
        let entry = |title: &'static str, body: &'static str| {
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(body),
                )
        };

        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_show_close_button(true)
                    .with_aria_label("What's new"),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-changelog-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_header(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("What's new"),
                    )
                    .child(Pill::from_spec(
                        PillSpec::new()
                            .with_label("v2.4.0")
                            .with_tone(PillTone::Info)
                            .with_appearance(PillAppearance::Badge),
                        theme,
                    )),
            )
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    .child(entry(
                        "Dialog flexibility improvements",
                        "Dialogs now support custom headers, footers, width presets, and bare mode.",
                    ))
                    .child(entry(
                        "Size propagation fixes",
                        "All parent components now correctly forward size and density to embedded children.",
                    )),
            ),
        );
    }

    if state.specimens.is_on("dialog-terms-open") {
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Terms of service")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-terms-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(
                div()
                    .text_size(px(13.0))
                    .text_color(color_to_hsla(text_secondary))
                    .child("By using this service, you agree to our terms and conditions."),
            )
            .with_footer(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(color_to_hsla(accent))
                            .child("Read full terms"),
                    )
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(
                                Button::from_spec(
                                    ButtonSpec::new()
                                        .with_variant(ButtonVariant::Ghost)
                                        .with_label("Decline"),
                                    theme,
                                )
                                .with_id("dialog-terms-decline")
                                .on_click(cx.listener(
                                    |this, _e: &ClickEvent, _w, cx| {
                                        this.state
                                            .specimens
                                            .toggles
                                            .insert("dialog-terms-open".to_string(), false);
                                        cx.notify();
                                    },
                                )),
                            )
                            .child(
                                Button::from_spec(ButtonSpec::new().with_label("Accept"), theme)
                                    .with_id("dialog-terms-accept")
                                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                        this.state
                                            .specimens
                                            .toggles
                                            .insert("dialog-terms-open".to_string(), false);
                                        cx.notify();
                                    })),
                            ),
                    ),
            ),
        );
    }

    if state.specimens.is_on("dialog-bare-open") {
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_bare(true)
                    .with_width(DialogWidth::Lg)
                    .with_aria_label("Image preview"),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-bare-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .w_full()
                            .min_h(px(320.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(hsla(0.0, 0.0, 0.08, 1.0))
                            .child(
                                div()
                                    .text_size(px(14.0))
                                    .text_color(color_to_hsla(text_secondary))
                                    .child("2400 × 1600"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(16.0))
                            .px(px(16.0))
                            .py(px(12.0))
                            .border_t_1()
                            .border_color(color_to_hsla(border_subtle))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(2.0))
                                    .child(
                                        div()
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("landscape-hero.png"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .text_color(color_to_hsla(text_secondary))
                                            .child("2.4 MB · Uploaded today"),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new()
                                                .with_variant(ButtonVariant::Ghost)
                                                .with_label("Close"),
                                            theme,
                                        )
                                        .with_id("dialog-bare-close")
                                        .on_click(
                                            cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                                this.state
                                                    .specimens
                                                    .toggles
                                                    .insert("dialog-bare-open".to_string(), false);
                                                cx.notify();
                                            }),
                                        ),
                                    )
                                    .child(
                                        Button::from_spec(
                                            ButtonSpec::new().with_label("Download"),
                                            theme,
                                        )
                                        .with_id("dialog-bare-download")
                                        .on_click(
                                            cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                                this.state
                                                    .specimens
                                                    .toggles
                                                    .insert("dialog-bare-open".to_string(), false);
                                                cx.notify();
                                            }),
                                        ),
                                    ),
                            ),
                    ),
            ),
        );
    }

    if state.specimens.is_on("dialog-scroll-open") {
        let messages = [
            "User signed in",
            "Project created",
            "File uploaded",
            "Settings updated",
            "Comment added",
            "Build completed",
            "Deploy started",
            "Review requested",
        ];
        let mut log_list = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .max_h(px(288.0))
            .overflow_hidden();
        for i in 0..20u32 {
            let hour = (9 + i / 3).min(23);
            let minute = (i * 17) % 60;
            log_list = log_list.child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .py(px(6.0))
                    .border_b_1()
                    .border_color(color_to_hsla(border_subtle))
                    .child(
                        div()
                            .min_w(px(48.0))
                            .text_size(px(12.0))
                            .text_color(color_to_hsla(text_secondary))
                            .child(format!("{:02}:{:02}", hour, minute)),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .child(messages[(i as usize) % messages.len()].to_string()),
                    ),
            );
        }

        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Activity log")
                    .with_description("Recent activity across all projects.")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-scroll-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(log_list)
            .with_actions(
                div()
                    .flex()
                    .gap(px(8.0))
                    .child(
                        Button::from_spec(
                            ButtonSpec::new()
                                .with_variant(ButtonVariant::Ghost)
                                .with_label("Close"),
                            theme,
                        )
                        .with_id("dialog-scroll-close")
                        .on_click(cx.listener(
                            |this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("dialog-scroll-open".to_string(), false);
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Button::from_spec(ButtonSpec::new().with_label("Export log"), theme)
                            .with_id("dialog-scroll-export")
                            .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                                this.state
                                    .specimens
                                    .toggles
                                    .insert("dialog-scroll-open".to_string(), false);
                                cx.notify();
                            })),
                    ),
            ),
        );
    }

    for (label, width, key) in [
        ("sm", DialogWidth::Sm, "dialog-width-sm-open"),
        ("md", DialogWidth::Md, "dialog-width-md-open"),
        ("lg", DialogWidth::Lg, "dialog-width-lg-open"),
        ("xl", DialogWidth::Xl, "dialog-width-xl-open"),
    ] {
        if state.specimens.is_on(key) {
            root = root.child(
                Dialog::from_spec(
                    DialogSpec::new()
                        .with_title(format!("Width: {label}"))
                        .with_width(width)
                        .with_show_close_button(true),
                    theme,
                )
                .on_open_change({
                    let root = root_handle.clone();
                    move |_open, _window, cx| {
                        root.update(cx, |this, cx| {
                            this.state.specimens.toggles.insert(key.to_string(), false);
                            cx.notify();
                        })
                        .ok();
                    }
                })
                .with_content(
                    div()
                        .text_size(px(13.0))
                        .text_color(color_to_hsla(text_secondary))
                        .child(format!("This dialog uses width=\"{label}\".")),
                )
                .with_actions(
                    Button::from_spec(ButtonSpec::new().with_label("Close"), theme)
                        .with_id(format!("dialog-width-{label}-close"))
                        .on_click(cx.listener(move |this, _e: &ClickEvent, _w, cx| {
                            this.state.specimens.toggles.insert(key.to_string(), false);
                            cx.notify();
                        })),
                ),
            );
        }
    }

    if state.specimens.is_on("dialog-persistent-open") {
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Processing")
                    .with_dismiss_on_backdrop(false)
                    .with_dismiss_on_escape(false),
                theme,
            )
            .on_open_change({
                let root = root_handle.clone();
                move |_open, _window, cx| {
                    root.update(cx, |this, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-persistent-open".to_string(), false);
                        cx.notify();
                    })
                    .ok();
                }
            })
            .with_content(
                div()
                    .text_size(px(13.0))
                    .text_color(color_to_hsla(text_secondary))
                    .child(
                        "This dialog cannot be dismissed by clicking the backdrop or pressing Escape.",
                    ),
            )
            .with_actions(
                Button::from_spec(ButtonSpec::new().with_label("Done"), theme)
                    .with_id("dialog-persistent-done")
                    .on_click(cx.listener(|this, _e: &ClickEvent, _w, cx| {
                        this.state
                            .specimens
                            .toggles
                            .insert("dialog-persistent-open".to_string(), false);
                        cx.notify();
                    })),
            ),
        );
    }

    root
}
