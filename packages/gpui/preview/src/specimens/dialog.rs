use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, Dialog, Eyebrow, IntoCompatNode, Pill};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{CrossAxisAlignment, LayoutDirection, MainAxisAlignment, Node};
use poodle_specs::{
    ButtonSpec, ButtonTone, ButtonVariant, DialogKind, DialogSpec, DialogWidth, EyebrowSpec,
    PillAppearance, PillSpec, PillTone,
};
use std::sync::Arc;

fn set_toggle_click(
    state: &AppState,
    key: impl Into<String>,
    value: bool,
) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value,
        });
    })
}

fn set_toggle_open_change(
    state: &AppState,
    key: impl Into<String>,
) -> Arc<dyn Fn(bool) + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value,
        });
    })
}

fn text(theme: &impl ThemeProvider, value: impl Into<String>, size: f32, token: &str) -> Node {
    let mut node = Node::text(value);
    node.style.text_size = Some(size);
    node.style.descriptor.text_color = Some(theme.resolve_color(token));
    node
}

fn column(children: impl IntoIterator<Item = Node>, gap: f32) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Column;
    node.style.descriptor.layout.spacing.gap = gap;
    node.children = children.into_iter().collect();
    node
}

fn row(children: impl IntoIterator<Item = Node>, gap: f32, justify_end: bool) -> Node {
    let mut node = Node::container();
    node.style.descriptor.layout.direction = LayoutDirection::Row;
    node.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
    if justify_end {
        node.style.descriptor.layout.alignment.main = MainAxisAlignment::End;
    }
    node.style.descriptor.layout.spacing.gap = gap;
    node.children = children.into_iter().collect();
    node
}

fn action_row(children: impl IntoIterator<Item = Node>) -> Node {
    row(children, 8.0, true)
}

fn button(
    theme: &GpuiThemeProvider,
    spec: ButtonSpec,
    id: impl Into<String>,
    on_click: Arc<dyn Fn() + Send + Sync>,
) -> Node {
    Button::from_spec(spec, theme)
        .with_id(id)
        .on_click(on_click)
        .into_compat_node()
}

fn close_button(theme: &GpuiThemeProvider, state: &AppState, key: &str) -> Node {
    button(
        theme,
        ButtonSpec::new()
            .with_variant(ButtonVariant::Ghost)
            .with_label("Close"),
        format!("{key}-close"),
        set_toggle_click(state, key, false),
    )
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = "color.text.secondary";
    let accent = "color.accent.base";

    let open_button = |id: &'static str, key: &'static str, label: &'static str| {
        Button::from_spec(
            ButtonSpec::new()
                .with_variant(ButtonVariant::Secondary)
                .with_label(label),
            theme,
        )
        .with_id(id)
        .on_click(set_toggle_click(state, key, true))
        .into_any_element()
    };

    let group = |title: &'static str, body: AnyElement| {
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(Eyebrow::from_spec(
                EyebrowSpec::new().with_content(title),
                theme,
            ))
            .child(body)
    };

    let trigger_row = |buttons: Vec<AnyElement>| {
        buttons.into_iter().fold(
            div().flex().flex_wrap().items_center().gap(px(8.0)),
            |row, button| row.child(button),
        )
    };

    let width_buttons = [
        ("sm", "dialog-width-sm-trigger", "dialog-width-sm-open"),
        ("md", "dialog-width-md-trigger", "dialog-width-md-open"),
        ("lg", "dialog-width-lg-trigger", "dialog-width-lg-open"),
        ("xl", "dialog-width-xl-trigger", "dialog-width-xl-open"),
        (
            "full",
            "dialog-width-full-trigger",
            "dialog-width-full-open",
        ),
    ]
    .into_iter()
    .fold(
        div().flex().flex_wrap().items_center().gap(px(8.0)),
        |row, (label, id, key)| row.child(open_button(id, key, label)),
    );

    let mut root = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(group(
            "Basic and alert dialogs",
            trigger_row(vec![
                open_button(
                    "dialog-shortcuts-trigger",
                    "dialog-shortcuts-open",
                    "View details",
                ),
                open_button("dialog-alert-trigger", "dialog-alert-open", "Delete item"),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Forms and nested controls",
            trigger_row(vec![open_button(
                "dialog-form-trigger",
                "dialog-form-open",
                "Create project",
            )])
            .into_any_element(),
        ))
        .child(group(
            "Custom header and footer",
            trigger_row(vec![
                open_button(
                    "dialog-changelog-trigger",
                    "dialog-changelog-open",
                    "View changelog",
                ),
                open_button(
                    "dialog-terms-trigger",
                    "dialog-terms-open",
                    "Terms & conditions",
                ),
            ])
            .into_any_element(),
        ))
        .child(group(
            "Bare content",
            trigger_row(vec![open_button(
                "dialog-bare-trigger",
                "dialog-bare-open",
                "Preview image",
            )])
            .into_any_element(),
        ))
        .child(group(
            "Scrolling and width presets",
            div()
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(8.0))
                .child(open_button(
                    "dialog-scroll-trigger",
                    "dialog-scroll-open",
                    "View log",
                ))
                .child(width_buttons)
                .into_any_element(),
        ))
        .child(group(
            "Dismissal rules",
            trigger_row(vec![open_button(
                "dialog-persistent-trigger",
                "dialog-persistent-open",
                "Open persistent",
            )])
            .into_any_element(),
        ));

    let mut root = div().flex().flex_col().gap(px(24.0)).child(root);

    if state.specimens.is_on("dialog-shortcuts-open") {
        let shortcuts = [
            ("⌘ K", "Command palette"),
            ("⌘ S", "Save"),
            ("⌘ /", "Toggle comment"),
            ("⌘ ⇧ P", "Quick actions"),
            ("Esc", "Close dialog"),
        ]
        .into_iter()
        .map(|(keys, label)| {
            row(
                [
                    text(theme, keys, 12.0, "color.text.primary"),
                    text(theme, label, 13.0, text_secondary),
                ],
                12.0,
                false,
            )
        });
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Keyboard shortcuts")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-shortcuts-open"))
            .with_content(column(shortcuts, 8.0)),
        );
    }

    if state.specimens.is_on("dialog-form-open") {
        let fields = [
            ("Project name", "My project"),
            ("Template", "Choose a template"),
            ("Description", "What is this project for?"),
        ]
        .into_iter()
        .map(|(label, placeholder)| {
            column(
                [
                    text(theme, label, 13.0, "color.text.primary"),
                    text(theme, placeholder, 13.0, text_secondary),
                ],
                4.0,
            )
        });
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("New project")
                    .with_description("Set up a new project workspace.")
                    .with_width(DialogWidth::Lg)
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-form-open"))
            .with_content(column(fields, 12.0))
            .with_actions(action_row([
                button(
                    theme,
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_label("Cancel"),
                    "dialog-form-cancel",
                    set_toggle_click(state, "dialog-form-open", false),
                ),
                button(
                    theme,
                    ButtonSpec::new().with_label("Create project"),
                    "dialog-form-create",
                    set_toggle_click(state, "dialog-form-open", false),
                ),
            ])),
        );
    }

    if state.specimens.is_on("dialog-changelog-open") {
        let header = row(
            [
                text(theme, "What's new", 16.0, "color.text.primary"),
                Pill::from_spec(
                    PillSpec::new()
                        .with_label("v2.4.0")
                        .with_tone(PillTone::Info)
                        .with_appearance(PillAppearance::Badge),
                    theme,
                )
                .into_compat_node(),
            ],
            10.0,
            false,
        );
        let body = column(
            [
                column(
                    [
                        text(theme, "Dialog flexibility improvements", 14.0, "color.text.primary"),
                        text(theme, "Dialogs now support custom headers, footers, width presets, and bare mode.", 13.0, text_secondary),
                    ],
                    4.0,
                ),
                column(
                    [
                        text(theme, "Size propagation fixes", 14.0, "color.text.primary"),
                        text(theme, "All parent components now correctly forward size and density to embedded children.", 13.0, text_secondary),
                    ],
                    4.0,
                ),
            ],
            14.0,
        );
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_show_close_button(true)
                    .with_aria_label("What's new"),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-changelog-open"))
            .with_header(header)
            .with_content(body),
        );
    }

    if state.specimens.is_on("dialog-terms-open") {
        let footer = row(
            [
                text(theme, "Read full terms", 13.0, accent),
                action_row([
                    button(
                        theme,
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Ghost)
                            .with_label("Decline"),
                        "dialog-terms-decline",
                        set_toggle_click(state, "dialog-terms-open", false),
                    ),
                    button(
                        theme,
                        ButtonSpec::new().with_label("Accept"),
                        "dialog-terms-accept",
                        set_toggle_click(state, "dialog-terms-open", false),
                    ),
                ]),
            ],
            16.0,
            false,
        );
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Terms of service")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-terms-open"))
            .with_content(text(
                theme,
                "By using this service, you agree to our terms and conditions.",
                13.0,
                text_secondary,
            ))
            .with_footer(footer),
        );
    }

    if state.specimens.is_on("dialog-bare-open") {
        let mut preview = Node::container();
        preview.style.descriptor.layout.direction = LayoutDirection::Column;
        preview.style.min_height = Some(320.0);
        preview.style.descriptor.background = Some(theme.resolve_color("color.background.canvas"));
        preview = preview.child(row(
            [text(theme, "2400 × 1600", 14.0, text_secondary)],
            0.0,
            false,
        ));
        let details = row(
            [
                column(
                    [
                        text(theme, "landscape-hero.png", 13.0, "color.text.primary"),
                        text(theme, "2.4 MB · Uploaded today", 12.0, text_secondary),
                    ],
                    2.0,
                ),
                action_row([
                    close_button(theme, state, "dialog-bare-open"),
                    button(
                        theme,
                        ButtonSpec::new().with_label("Download"),
                        "dialog-bare-download",
                        set_toggle_click(state, "dialog-bare-open", false),
                    ),
                ]),
            ],
            16.0,
            false,
        );
        preview = preview.child(details);
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_bare(true)
                    .with_width(DialogWidth::Lg)
                    .with_aria_label("Image preview"),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-bare-open"))
            .with_content(preview),
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
        let entries = (0..20u32).map(|i| {
            let hour = (9 + i / 3).min(23);
            let minute = (i * 17) % 60;
            row(
                [
                    text(
                        theme,
                        format!("{hour:02}:{minute:02}"),
                        12.0,
                        text_secondary,
                    ),
                    text(
                        theme,
                        messages[(i as usize) % messages.len()],
                        13.0,
                        "color.text.primary",
                    ),
                ],
                12.0,
                false,
            )
        });
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_title("Activity log")
                    .with_description("Recent activity across all projects.")
                    .with_show_close_button(true),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-scroll-open"))
            .with_content(column(entries, 4.0))
            .with_actions(action_row([
                button(
                    theme,
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Ghost)
                        .with_label("Close"),
                    "dialog-scroll-close",
                    set_toggle_click(state, "dialog-scroll-open", false),
                ),
                button(
                    theme,
                    ButtonSpec::new().with_label("Export log"),
                    "dialog-scroll-export",
                    set_toggle_click(state, "dialog-scroll-open", false),
                ),
            ])),
        );
    }

    for (label, width, key) in [
        ("sm", DialogWidth::Sm, "dialog-width-sm-open"),
        ("md", DialogWidth::Md, "dialog-width-md-open"),
        ("lg", DialogWidth::Lg, "dialog-width-lg-open"),
        ("xl", DialogWidth::Xl, "dialog-width-xl-open"),
        ("full", DialogWidth::Full, "dialog-width-full-open"),
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
                .on_open_change(set_toggle_open_change(state, key))
                .with_content(text(
                    theme,
                    format!("This dialog uses width=\"{label}\"."),
                    13.0,
                    text_secondary,
                ))
                .with_actions(close_button(theme, state, key)),
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
            .on_open_change(set_toggle_open_change(state, "dialog-persistent-open"))
            .with_content(text(
                theme,
                "This dialog cannot be dismissed by clicking the backdrop or pressing Escape.",
                13.0,
                text_secondary,
            ))
            .with_actions(button(
                theme,
                ButtonSpec::new().with_label("Done"),
                "dialog-persistent-done",
                set_toggle_click(state, "dialog-persistent-open", false),
            )),
        );
    }

    if state.specimens.is_on("dialog-alert-open") {
        root = root.child(
            Dialog::from_spec(
                DialogSpec::new()
                    .with_role(DialogKind::AlertDialog)
                    .with_title("Delete item?")
                    .with_description(
                        "This will permanently remove the item and all associated data.",
                    ),
                theme,
            )
            .on_open_change(set_toggle_open_change(state, "dialog-alert-open"))
            .with_content(text(
                theme,
                "This action cannot be undone.",
                13.0,
                text_secondary,
            ))
            .with_actions(action_row([
                button(
                    theme,
                    ButtonSpec::new()
                        .with_variant(ButtonVariant::Secondary)
                        .with_label("Cancel"),
                    "dialog-alert-cancel",
                    set_toggle_click(state, "dialog-alert-open", false),
                ),
                button(
                    theme,
                    ButtonSpec::new()
                        .with_tone(ButtonTone::Danger)
                        .with_label("Delete"),
                    "dialog-alert-delete",
                    set_toggle_click(state, "dialog-alert-open", false),
                ),
            ])),
        );
    }
    let examples = root.into_any_element();

    specimen_layout(
        state,
        cx,
        "dialog",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let open_key = format!("dialog-axis-size-{}", size_key(size));
                let mut row = div().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label(format!("Open {} dialog", size_key(size))),
                        theme,
                    )
                    .with_id(format!("dialog-axis-size-{}-trigger", size_key(size)))
                    .on_click(set_toggle_click(state, open_key.clone(), true)),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        Dialog::from_spec(
                            DialogSpec::new()
                                .with_title(format!("size: {}", size_key(size)))
                                .with_show_close_button(true)
                                .with_size(size),
                            theme,
                        )
                        .on_open_change(set_toggle_open_change(state, open_key.clone()))
                        .with_content(text(
                            theme,
                            "Command palette, save, and toggle comment live here.",
                            13.0,
                            "color.text.secondary",
                        )),
                    );
                }
                row.into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let open_key = format!("dialog-axis-density-{}", density_key(density));
                let mut row = div().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_variant(ButtonVariant::Secondary)
                            .with_label(format!("Open {} dialog", density_key(density))),
                        theme,
                    )
                    .with_id(format!(
                        "dialog-axis-density-{}-trigger",
                        density_key(density)
                    ))
                    .on_click(set_toggle_click(state, open_key.clone(), true)),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        Dialog::from_spec(
                            DialogSpec::new()
                                .with_title(format!("density: {}", density_key(density)))
                                .with_show_close_button(true)
                                .with_density(density),
                            theme,
                        )
                        .on_open_change(set_toggle_open_change(state, open_key.clone()))
                        .with_content(text(
                            theme,
                            "Command palette, save, and toggle comment live here.",
                            13.0,
                            "color.text.secondary",
                        )),
                    );
                }
                row.into_any_element()
            }),
    )
}
