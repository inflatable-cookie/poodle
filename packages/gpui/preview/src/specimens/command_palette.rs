use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, CommandPalette, Eyebrow};
use crate::specimens::specimen_axes::{density_key, size_key};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{ButtonSpec, CommandActionItem, CommandPaletteSpec, DiscoveryState};
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec};
use std::sync::Arc;

fn open_click(state: &AppState, key: impl Into<String>) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value: true,
        });
    })
}

fn set_text(state: &AppState, key: impl Into<String>) -> Arc<dyn Fn(&str) + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move |value| {
        events.lock().unwrap().push(NodeSpecimenEvent::SetText {
            key: key.clone(),
            value: value.to_string(),
        });
    })
}

fn close_click(state: &AppState, key: impl Into<String>) -> Arc<dyn Fn() + Send + Sync> {
    let events = state.node_events.clone();
    let key = key.into();
    Arc::new(move || {
        events.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value: false,
        });
    })
}

/// The action set both the Examples pane and the axis representatives use.
fn axis_actions() -> Vec<CommandActionItem> {
    vec![
        CommandActionItem::new("save", "Save")
            .with_group("File")
            .with_shortcut("\u{2318}S"),
        CommandActionItem::new("open-file", "Open File")
            .with_group("File")
            .with_shortcut("\u{2318}O"),
        CommandActionItem::new("close-tab", "Close Tab")
            .with_group("File")
            .with_shortcut("\u{2318}W"),
        CommandActionItem::new("find-in-files", "Find in Files")
            .with_group("Edit")
            .with_shortcut("\u{21E7}\u{2318}F"),
        CommandActionItem::new("find-and-replace", "Find and Replace")
            .with_group("Edit")
            .with_shortcut("\u{2318}H"),
        CommandActionItem::new("toggle-terminal", "Toggle Terminal")
            .with_group("View")
            .with_shortcut("\u{2318}`"),
        CommandActionItem::new("toggle-sidebar", "Toggle Sidebar")
            .with_group("View")
            .with_shortcut("\u{2318}B"),
    ]
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");

    let actions = axis_actions();

    let query = state
        .specimens
        .text
        .get("cmd-palette-query")
        .cloned()
        .unwrap_or_default();
    let is_open = state.specimens.is_on("cmd-palette-open");
    let compact_open = state.specimens.is_on("cmd-palette-compact-open");

    // ── Triggers section ──────────────────────────────────────────
    let triggers = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Command Palette"),
                    theme,
                ))
                .child(
                    div()
                        .text_sm()
                        .text_color(color_to_hsla(text_secondary))
                        .child(
                            "Click below to open the palette. Close with Escape, click outside, or the X button.",
                        ),
                )
                .child(
                    Button::from_spec(ButtonSpec::new().with_label("Open Command Palette"), theme)
                        .with_id("cmd-palette-open")
                        .on_click(open_click(state, "cmd-palette-open")),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(Eyebrow::from_spec(
                    EyebrowSpec::new().with_content("Semantic presentation"),
                    theme,
                ))
                .child(
                    Button::from_spec(ButtonSpec::new().with_label("Open compact palette"), theme)
                        .with_id("cmd-palette-compact-open")
                        .on_click(open_click(state, "cmd-palette-compact-open")),
                ),
        );

    // The specimen column is `relative` so the palette's `absolute
    // inset_0` backdrop fills this region (GPUI has no `fixed`/`vw`).
    let mut root = div()
        .relative()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(triggers);

    // ── Open: main grouped palette ────────────────────────────────
    if is_open {
        let mut spec = CommandPaletteSpec::new(actions.clone())
            .with_title("Command palette")
            .with_invocation_hint("\u{2318}K");
        if !query.is_empty() {
            spec = spec.with_query(&query);
        }
        spec = spec.with_open(true);

        root = root.child(
            CommandPalette::from_spec(spec, theme)
                .with_id("cmd-palette")
                .on_select(set_text(state, "cmd-palette-query"))
                .on_query_change(set_text(state, "cmd-palette-query"))
                .on_close(close_click(state, "cmd-palette-open")),
        );
    }

    // ── Open: compact palette (size sm + compact density) ─────────
    if compact_open {
        root = root.child(
            CommandPalette::from_spec(
                CommandPaletteSpec::new(vec![
                    CommandActionItem::new("save", "Save")
                        .with_group("File")
                        .with_shortcut("\u{2318}S"),
                    CommandActionItem::new("open", "Open File")
                        .with_group("File")
                        .with_shortcut("\u{2318}O"),
                ])
                .with_open(true)
                .with_title("Quick actions")
                .with_size(ControlSize::Sm)
                .with_density(ControlDensity::Compact)
                .with_invocation_hint("Cmd+K"),
                theme,
            )
            .with_id("cmd-palette-compact")
            .on_close(close_click(state, "cmd-palette-compact-open")),
        );
    }

    // ── Open states (contract §6: ready/loading/error/empty/no-results) ──
    // Each palette mounts inside its own `relative` group container so the
    // `absolute inset_0` backdrop is confined to that group region.
    let state_groups = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Open states"),
            theme,
        ))
        .child(open_state_group(
            "Open / ready",
            actions.clone(),
            "",
            DiscoveryState::Ready,
            "cmd-state-ready",
            theme,
        ))
        .child(open_state_group(
            "Open / loading",
            actions.clone(),
            "",
            DiscoveryState::Loading,
            "cmd-state-loading",
            theme,
        ))
        .child(open_state_group(
            "Open / no-results",
            actions.clone(),
            "zxqv",
            DiscoveryState::NoResults,
            "cmd-state-noresults",
            theme,
        ))
        .child(open_state_group(
            "Open / empty",
            Vec::new(),
            "",
            DiscoveryState::Empty,
            "cmd-state-empty",
            theme,
        ))
        .child(open_state_group(
            "Open / error",
            actions.clone(),
            "",
            DiscoveryState::Error,
            "cmd-state-error",
            theme,
        ));
    // The always-open demo palettes stack over the same region the live
    // palette's overlay uses, and they paint later — which buries the live
    // palette and makes its input unreachable. A real palette hides the
    // page behind it anyway, so while the interactive one is open the demo
    // groups stand down.
    if !is_open {
        root = root.child(state_groups);
    }

    // ── Sizes (xs–xl): one open palette per intrinsic size ────────
    let mut sizes_row = div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Sizes"),
            theme,
        ));
    for (label, size) in [
        ("XS", ControlSize::Xs),
        ("SM", ControlSize::Sm),
        ("MD", ControlSize::Md),
        ("LG", ControlSize::Lg),
        ("XL", ControlSize::Xl),
    ] {
        sizes_row = sizes_row.child(
            div()
                .relative()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
                .child(
                    CommandPalette::from_spec(
                        CommandPaletteSpec::new(actions.clone())
                            .with_open(true)
                            .with_title(format!("{label} command palette"))
                            .with_invocation_hint("\u{2318}K")
                            .with_size(size),
                        theme,
                    )
                    .with_id(format!("cmd-size-{label}")),
                ),
        );
    }
    if !is_open {
        root = root.child(sizes_row);
    }

    // ── Densities: one open palette per density ───────────────────
    let mut densities_row = div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content("Densities"),
            theme,
        ));
    for (label, density) in [
        ("Compact", ControlDensity::Compact),
        ("Default", ControlDensity::Default),
        ("Comfortable", ControlDensity::Comfortable),
    ] {
        densities_row = densities_row.child(
            div()
                .relative()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(color_to_hsla(text_secondary))
                        .child(label),
                )
                .child(
                    CommandPalette::from_spec(
                        CommandPaletteSpec::new(actions.clone())
                            .with_open(true)
                            .with_title(format!("{label} command palette"))
                            .with_invocation_hint("\u{2318}K")
                            .with_density(density),
                        theme,
                    )
                    .with_id(format!("cmd-density-{label}")),
                ),
        );
    }
    if !is_open {
        root = root.child(densities_row);
    }
    let examples = root.into_any_element();

    specimen_layout(
        state,
        cx,
        "command-palette",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme: &GpuiThemeProvider| {
                let open_key = format!("cmd-palette-axis-size-{}", size_key(size));
                let mut row = div().relative().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new().with_label(format!("Open {} palette", size_key(size))),
                        theme,
                    )
                    .with_id(format!("cmd-palette-axis-size-{}-trigger", size_key(size)))
                    .on_click(open_click(state, open_key.clone())),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        CommandPalette::from_spec(
                            CommandPaletteSpec::new(axis_actions())
                                .with_title("Command palette")
                                .with_invocation_hint("\u{2318}K")
                                .with_open(true)
                                .with_size(size),
                            theme,
                        )
                        .with_id(format!("cmd-palette-axis-size-{}", size_key(size)))
                        .on_close(close_click(state, open_key.clone())),
                    );
                }
                row.into_any_element()
            })
            .with_densities(|density, theme: &GpuiThemeProvider| {
                let open_key = format!("cmd-palette-axis-density-{}", density_key(density));
                let mut row = div().relative().flex().flex_col().gap(px(8.0)).child(
                    Button::from_spec(
                        ButtonSpec::new()
                            .with_label(format!("Open {} palette", density_key(density))),
                        theme,
                    )
                    .with_id(format!(
                        "cmd-palette-axis-density-{}-trigger",
                        density_key(density)
                    ))
                    .on_click(open_click(state, open_key.clone())),
                );
                if state.specimens.is_on(&open_key) {
                    row = row.child(
                        CommandPalette::from_spec(
                            CommandPaletteSpec::new(axis_actions())
                                .with_title("Command palette")
                                .with_invocation_hint("\u{2318}K")
                                .with_open(true)
                                .with_density(density),
                            theme,
                        )
                        .with_id(format!("cmd-palette-axis-density-{}", density_key(density)))
                        .on_close(close_click(state, open_key.clone())),
                    );
                }
                row.into_any_element()
            }),
    )
}

/// Build one labeled group containing an always-open palette demonstrating a
/// single `DiscoveryState`. Wrapped in its own `relative` container so the
/// palette's `absolute inset_0` backdrop stays inside this group's bounds.
fn open_state_group(
    label: &str,
    actions: Vec<CommandActionItem>,
    query: &str,
    state: DiscoveryState,
    id: &str,
    theme: &poodle_gpui::GpuiThemeProvider,
) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");
    let mut spec = CommandPaletteSpec::new(actions)
        .with_open(true)
        .with_title("Command palette")
        .with_invocation_hint("\u{2318}K")
        .with_state(state);
    if !query.is_empty() {
        spec = spec.with_query(query);
    }
    div()
        .relative()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(
            div()
                .text_xs()
                .text_color(color_to_hsla(text_secondary))
                .child(label.to_string()),
        )
        .child(CommandPalette::from_spec(spec, theme).with_id(id.to_string()))
}
