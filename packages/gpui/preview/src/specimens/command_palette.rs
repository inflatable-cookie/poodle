use crate::app_state::AppState;
use crate::specimens::overlay_state;
use crate::style_bridge::color_to_hsla;
use crate::PreviewRoot;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui_components::{Button, CommandPalette, Eyebrow};
use poodle_specs::{ButtonSpec, CommandActionItem, CommandPaletteSpec, DiscoveryState};
use poodle_specs::{ControlDensity, ControlSize, EyebrowSpec};

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let text_secondary = theme.resolve_color("color.text.secondary");
    let root_handle = cx.weak_entity();

    let actions = vec![
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
    ];

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
                        .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                            overlay_state::set_toggle(this, "cmd-palette-open", true, cx);
                        })),
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
                        .on_click(cx.listener(|this, _event: &ClickEvent, _window, cx| {
                            overlay_state::set_toggle(this, "cmd-palette-compact-open", true, cx);
                        })),
                ),
        );

    // The specimen column is `relative` so the palette's `absolute
    // inset_0` backdrop fills this region (GPUI has no `fixed`/`vw`).
    let mut root = div().relative().flex().flex_col().gap(px(24.0)).child(triggers);

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
                .on_select(cx.listener(|this, val: &str, _w, cx| {
                    this.state
                        .specimens
                        .text
                        .insert("cmd-palette-query".to_string(), val.to_string());
                    cx.notify();
                }))
                .on_query_change(cx.listener(|this, val: &str, _w, cx| {
                    this.state
                        .specimens
                        .text
                        .insert("cmd-palette-query".to_string(), val.to_string());
                    cx.notify();
                }))
                .on_open_change({
                    let root = root_handle.clone();
                    move |open, _window, cx| {
                        overlay_state::set_toggle_via_entity(&root, "cmd-palette-open", open, cx);
                    }
                }),
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
            .on_open_change({
                let root = root_handle.clone();
                move |open, _window, cx| {
                    overlay_state::set_toggle_via_entity(
                        &root,
                        "cmd-palette-compact-open",
                        open,
                        cx,
                    );
                }
            }),
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

    root
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
