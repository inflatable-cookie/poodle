//! CommandPalette — a query box over a list of actions.
//!
//! Contract: `docs/contracts/components/command-palette.md`
//! Ported from: `packages/jetstream/components/src/command_palette.rs`.
//!
//! Renders the full modal shell — overlay backdrop scrim, centered modal
//! surface, header (title/description + invocation-hint pill + close
//! affordance), a composed `text_input type="search"`, a live status region
//! (matching Svelte `paletteStatus`), and the grouped results panel with
//! group headers, badges, shortcut hints, active-item treatment, and
//! disabled dimming. Non-ready/empty discovery states render their message
//! in place of the list.
//!
//! Typing, arrow nav, Enter, Escape and backdrop-click close are host-owned;
//! structure renders at the current (query, state, active-id). `on_select`
//! fires with the chosen action's id; disabled actions never fire.

use std::sync::Arc;

use poodle_node::{
    CrossAxisAlignment, CursorHint, FontFamily, LayoutDirection, LayoutOverflow, LayoutSizing,
    MainAxisAlignment, Node, NodeKey, NodeRole,
};
use poodle_specs::{
    CommandActionItem, CommandPaletteSpec, DialogSpec, DiscoveryState, TextInputSpec,
};

use crate::color::{mix_srgb, with_alpha};
use crate::context::RenderContext;
use crate::dialog::dialog;
use crate::presentation::{panel_space_x_rem, panel_space_y_rem, rem_to_px, size_font_rem};
use crate::text_input::{text_input_with_handlers, TextInputHandlers};

#[derive(Default)]
pub struct CommandPaletteHandlers {
    pub select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub query_change: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    pub active_change: Option<Arc<dyn Fn(Option<&str>) + Send + Sync>>,
    pub close: Option<Arc<dyn Fn() + Send + Sync>>,
    pub instance_id: Option<String>,
}

fn scoped_id(instance_id: Option<&str>, part: &str, fallback: impl FnOnce() -> String) -> String {
    instance_id
        .map(|scope| format!("command-palette:{scope}:{part}"))
        .unwrap_or_else(fallback)
}

fn next_active_action(
    actions: &[CommandActionItem],
    active_id: Option<&str>,
    key: NodeKey,
) -> Option<String> {
    let enabled = actions
        .iter()
        .filter(|action| !action.is_disabled)
        .collect::<Vec<_>>();
    if enabled.is_empty() {
        return None;
    }
    let current = enabled
        .iter()
        .position(|action| Some(action.id.as_str()) == active_id);
    let next = match key {
        NodeKey::ArrowDown => current.map_or(0, |index| (index + 1) % enabled.len()),
        NodeKey::ArrowUp => current.map_or(0, |index| (index + enabled.len() - 1) % enabled.len()),
        NodeKey::Home => 0,
        NodeKey::End => enabled.len() - 1,
        _ => return None,
    };
    Some(enabled[next].id.clone())
}

pub fn command_palette(
    spec: &CommandPaletteSpec,
    ctx: &RenderContext<'_>,
    on_select: Option<Arc<dyn Fn(&str) + Send + Sync>>,
) -> Node {
    command_palette_with_handlers(
        spec,
        ctx,
        CommandPaletteHandlers {
            select: on_select,
            ..Default::default()
        },
    )
}

pub fn command_palette_with_handlers(
    spec: &CommandPaletteSpec,
    ctx: &RenderContext<'_>,
    handlers: CommandPaletteHandlers,
) -> Node {
    // Closed: render nothing. Consumers that never touched `is_open` still
    // render because the spec default is `true`.
    if !spec.is_open {
        return Node::container();
    }

    let CommandPaletteHandlers {
        select,
        query_change,
        active_change,
        close: close_handler,
        instance_id,
    } = handlers;
    let instance_id = instance_id.as_deref();
    let effective_size = ctx.resolve_size(spec.size, spec.size_role);
    let base_size = ctx.base_size(spec.size);
    let density = ctx.resolve_density(spec.density);
    let theme = ctx.theme();
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_size = theme.resolve_space("size.icon.md");
    let panel_px = rem_to_px(panel_space_x_rem(density));
    let panel_py = rem_to_px(panel_space_y_rem(density));

    // ── Color / dimension tokens ──────────────────────────────────────
    // Modal surface fill = color.background.elevated (contract §9 mixes it
    // 98% with transparent; resolved straight — the 2% delta is imperceptible
    // and there is no surface transparent-mix path).
    let surface_bg = theme.resolve_color(spec.results_fill_token());
    let border_default = theme.resolve_color("color.border.default");
    // Contract §9 dialog border = border-default mixed 42% with transparent.
    let dialog_border = with_alpha(border_default, border_default.3 * 0.42);
    let text_primary = theme.resolve_color("color.text.primary");
    let text_secondary = theme.resolve_color("color.text.secondary");
    let text_muted = theme.resolve_color("color.text.muted");
    let accent = theme.resolve_color("color.accent.base");
    let surface_subtle = theme.resolve_color("color.background.surface");
    let danger = theme.resolve_color("color.status.danger");
    let disabled_opacity = theme.resolve_opacity("state.opacity.disabled");
    let label_size = theme.resolve_space("typography.label.size");
    let heading_size = theme.resolve_space("typography.heading.size");
    let radius_control = theme.resolve_radius("radius.control");
    let radius_surface = theme.resolve_radius("radius.surface");
    // Contract §9 dialog radius = radius.surface + 0.125rem.
    let dialog_radius = radius_surface + rem_to_px(0.125);
    let gap_sm = theme.resolve_space("space.inline.sm");
    let gap_md = theme.resolve_space("space.inline.md");
    let gap_lg = theme.resolve_space("space.inline.lg");
    let stack_md = theme.resolve_space("space.stack.md");

    // Contract §9 geometry literals (rem from the contract, not magic px):
    // close button is 1.75rem square at radius.control − 0.0625rem; hint pill
    // is min-height 1.5rem, x-pad 0.5rem.
    let close_dim = rem_to_px(1.75);
    let close_radius = (radius_control - rem_to_px(0.0625)).max(0.0);
    let hint_min_h = rem_to_px(1.5);
    let hint_pad_x = rem_to_px(0.5);

    // ── Modal surface ─────────────────────────────────────────────────
    // Centered dialog: vertical stack of header / query / status / results,
    // gap = space.stack.md (contract §9). Contract width is
    // min(45rem, calc(100vw − 2rem)) and max-height min(78vh, 52.5rem);
    // no min()/vw here, so pin the rem caps and let the backdrop center.
    let mut modal = Node::container();
    {
        let s = &mut modal.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = stack_md;
        s.descriptor.layout.width = LayoutSizing::Fixed(rem_to_px(45.0));
        s.max_width = Some(rem_to_px(45.0));
        s.max_height = Some(rem_to_px(52.5));
        let pad = &mut s.descriptor.layout.spacing.padding;
        pad.left = panel_px;
        pad.right = panel_px;
        pad.top = panel_py;
        pad.bottom = panel_py;
        s.descriptor.corner_radii.top_left = dialog_radius;
        s.descriptor.corner_radii.top_right = dialog_radius;
        s.descriptor.corner_radii.bottom_right = dialog_radius;
        s.descriptor.corner_radii.bottom_left = dialog_radius;
        s.descriptor.background = Some(surface_bg);
        s.descriptor.border.width = 1.0;
        s.descriptor.border.color = dialog_border;
        // Token-accurate elevation.dialog (modal tier).
        s.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_DIALOG);
        s.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
    }

    // ── Header: title-group + meta (hint pill + close) ────────────────
    let mut title_group = Node::container();
    {
        let s = &mut title_group.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = rem_to_px(0.125);
        s.descriptor.layout.width = LayoutSizing::Grow;
        s.min_width = Some(0.0);
    }
    if let Some(ref title) = spec.title {
        let mut t = Node::text(title);
        t.runtime_id = Some(scoped_id(instance_id, "title", || {
            "poodle-cmd-palette-title".to_string()
        }));
        t.style.text_size = Some(heading_size);
        t.style.text_weight = Some(600);
        t.style.descriptor.text_color = Some(text_primary);
        title_group = title_group.child(t);
    }
    if let Some(ref description) = spec.description {
        let mut d = Node::text(description);
        d.runtime_id = Some(scoped_id(instance_id, "description", || {
            "poodle-cmd-palette-description".to_string()
        }));
        d.style.text_size = Some(label_size);
        d.style.descriptor.text_color = Some(text_secondary);
        title_group = title_group.child(d);
    }

    // Meta column: optional invocation-hint pill + close affordance.
    let mut meta = Node::container();
    {
        let s = &mut meta.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = gap_sm;
    }
    if let Some(ref hint) = spec.invocation_hint {
        // Contract §9 hint bg = background.surface 76%.
        let hint_bg = with_alpha(surface_subtle, surface_subtle.3 * 0.76);
        let mut pill = Node::container();
        pill.runtime_id = Some(scoped_id(instance_id, "hint", || {
            "poodle-cmd-palette-hint".to_string()
        }));
        {
            let s = &mut pill.style;
            s.descriptor.layout.direction = LayoutDirection::Row;
            s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
            s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
            s.min_height = Some(hint_min_h);
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.left = hint_pad_x;
            pad.right = hint_pad_x;
            s.descriptor.corner_radii.top_left = radius_control;
            s.descriptor.corner_radii.top_right = radius_control;
            s.descriptor.corner_radii.bottom_right = radius_control;
            s.descriptor.corner_radii.bottom_left = radius_control;
            s.descriptor.background = Some(hint_bg);
        }
        // Contract §9 `.command-palette__hint`: code-family (kbd-styled).
        let mut kbd = Node::text(hint);
        kbd.style.text_size = Some(label_size);
        kbd.style.descriptor.text_color = Some(text_secondary);
        kbd.style.font_family = Some(FontFamily::Mono);
        meta = meta.child(pill.child(kbd));
    }
    // Close affordance (contract §3 / §9): transparent control-sized square
    // with an `x` icon. Click wiring is host-owned. Svelte renders a real
    // `<button aria-label="Close command palette">`; the role and name both
    // have to be stated or it is announced as anonymous structure.
    let mut close = Node::container();
    close.a11y.role = Some(NodeRole::Button);
    close.a11y.label = Some("Close command palette".to_string());
    let close_id = scoped_id(instance_id, "close", || {
        "poodle-cmd-palette-close".to_string()
    });
    close.id = Some(close_id.clone());
    close.runtime_id = Some(close_id);
    close.interaction.focusable = true;
    close.interaction.on_activate = close_handler.clone();
    {
        let s = &mut close.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        s.descriptor.layout.width = LayoutSizing::Fixed(close_dim);
        s.descriptor.layout.height = LayoutSizing::Fixed(close_dim);
        s.flex_shrink_zero = true;
        s.descriptor.corner_radii.top_left = close_radius;
        s.descriptor.corner_radii.top_right = close_radius;
        s.descriptor.corner_radii.bottom_right = close_radius;
        s.descriptor.corner_radii.bottom_left = close_radius;
        s.descriptor.cursor = CursorHint::Pointer;
    }
    let mut x = Node::icon("x", icon_size);
    x.style.descriptor.text_color = Some(text_secondary);
    meta = meta.child(close.child(x));

    let mut header = Node::container();
    header.runtime_id = Some(scoped_id(instance_id, "header", || {
        "poodle-cmd-palette-header".to_string()
    }));
    {
        let s = &mut header.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.spacing.gap = gap_md;
    }
    modal = modal.child(header.child(title_group).child(meta));

    // ── Query: composed text_input type="search" ──────────────────────
    // Renders current query value + leading search icon + placeholder.
    // Editing is host-owned; structure + current value render.
    let query_spec_id = scoped_id(instance_id, "query", || {
        "poodle-cmd-palette-query".to_string()
    });
    let status_id = scoped_id(instance_id, "status", || {
        "poodle-cmd-palette-status".to_string()
    });
    let query_len = spec.query.chars().count();
    let query_spec = TextInputSpec::new()
        .with_id(query_spec_id)
        .with_input_type("search")
        .with_leading_icon("search")
        .with_value(spec.query.clone())
        .with_selection(query_len, query_len)
        .with_is_focused(true)
        .with_placeholder("Search commands, panels, and actions".to_string())
        .with_aria_label("Search commands")
        .with_description_id(status_id.clone())
        .with_show_clear_button(true)
        .with_size(base_size)
        .with_size_role(spec.size_role)
        .with_density(density);
    let active_action = spec.active_action_id.as_deref().and_then(|id| {
        spec.actions
            .iter()
            .find(|action| action.id == id && !action.is_disabled)
    });
    let submit = select.clone().and_then(|select| {
        active_action.map(|action| {
            let id = action.id.clone();
            Arc::new(move || select(&id)) as Arc<dyn Fn() + Send + Sync>
        })
    });
    let mut query = text_input_with_handlers(
        &query_spec,
        ctx,
        TextInputHandlers {
            on_change: query_change,
            on_submit: submit,
            on_cancel: close_handler.clone(),
            ..TextInputHandlers::default()
        },
    );
    query
        .roles
        .insert("dependency".to_owned(), "text-input".to_owned());
    query.roles.insert("part".to_owned(), "query".to_owned());
    if let Some(active_change) = active_change.clone() {
        let actions = spec.actions.clone();
        let current = spec.active_action_id.clone();
        query.interaction.on_key = Some(Arc::new(move |key, _modifiers| {
            if let Some(next) = next_active_action(&actions, current.as_deref(), key) {
                active_change(Some(&next));
            }
            None
        }));
    }
    query.style.fill_width = true;
    modal = modal.child(query);

    // ── Status region (contract §3 / §7) ──────────────────────────────
    // Live count / active / state message — matches Svelte `paletteStatus`.
    // Contract: the result-count line is a `status`, so a search that
    // narrows to nothing is announced rather than silently emptying.
    let mut status = Node::text(palette_status(spec));
    status.id = Some(status_id.clone());
    status.runtime_id = Some(status_id);
    status.a11y.role = Some(NodeRole::Status);
    status.roles.insert("part".to_owned(), "status".to_owned());
    status.style.text_size = Some(rem_to_px(0.75));
    status.style.descriptor.text_color = Some(text_secondary);
    modal = modal.child(status);

    // ── Results panel ─────────────────────────────────────────────────
    // Non-ready postures render an empty-state message in place of the list;
    // ready renders grouped rows.
    let state_message = |text: &str, color: poodle_node::ColorValue| {
        let mut wrap = Node::container();
        {
            let s = &mut wrap.style;
            // Explicit Row (see switch.rs).
            s.descriptor.layout.direction = LayoutDirection::Row;
            let pad = &mut s.descriptor.layout.spacing.padding;
            pad.top = gap_lg;
            pad.bottom = gap_lg;
        }
        let mut msg = Node::text(text);
        msg.style.text_size = Some(font_size);
        msg.style.descriptor.text_color = Some(color);
        wrap.child(msg)
    };
    let results = match spec.state {
        DiscoveryState::Loading => state_message("Searching\u{2026}", text_secondary),
        DiscoveryState::Error => state_message("Error loading commands", danger),
        DiscoveryState::Empty | DiscoveryState::NoResults => {
            state_message("No matching commands", text_secondary)
        }
        DiscoveryState::Ready => {
            let mut results_list = Node::container();
            results_list.a11y.role = Some(NodeRole::ListBox);
            results_list.a11y.label = Some("Command results".to_string());
            {
                let s = &mut results_list.style;
                s.descriptor.layout.direction = LayoutDirection::Column;
                s.descriptor.layout.spacing.gap = rem_to_px(0.125);
                s.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
            }

            let mut current_group: Option<&str> = None;
            for action in &spec.actions {
                // Group header when the group changes.
                if action.group.as_deref() != current_group {
                    current_group = action.group.as_deref();
                    if let Some(group_name) = current_group {
                        let mut g = Node::text(group_name);
                        {
                            let s = &mut g.style;
                            let pad = &mut s.descriptor.layout.spacing.padding;
                            pad.left = gap_sm;
                            pad.right = gap_sm;
                            pad.top = rem_to_px(0.25);
                            pad.bottom = rem_to_px(0.25);
                            s.text_size = Some(label_size);
                            s.text_weight = Some(600);
                            s.descriptor.text_color = Some(text_muted);
                        }
                        results_list = results_list.child(g);
                    }
                }

                let is_active = spec
                    .active_action_id
                    .as_deref()
                    .is_some_and(|id| id == action.id);

                let mut row = Node::container();
                let row_id = scoped_id(instance_id, &format!("action:{}", action.id), || {
                    format!("poodle-cmd-palette-{}", action.id)
                });
                row.id = Some(row_id.clone());
                row.runtime_id = Some(row_id);
                row.a11y.role = Some(NodeRole::ListBoxOption);
                row.a11y.label = Some(action.title.clone());
                row.a11y.selected = Some(is_active);
                {
                    let s = &mut row.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
                    s.descriptor.layout.spacing.gap = gap_sm;
                    let pad = &mut s.descriptor.layout.spacing.padding;
                    pad.left = gap_sm;
                    pad.right = gap_sm;
                    pad.top = gap_sm;
                    pad.bottom = gap_sm;
                    s.descriptor.corner_radii.top_left = radius_control;
                    s.descriptor.corner_radii.top_right = radius_control;
                    s.descriptor.corner_radii.bottom_right = radius_control;
                    s.descriptor.corner_radii.bottom_left = radius_control;
                }
                let row_text_color = if is_active {
                    // Active treatment: accent tint bg + accent text.
                    row.style.descriptor.background = Some(mix_srgb(accent, surface_bg, 0.10));
                    accent
                } else {
                    text_primary
                };

                if action.is_disabled {
                    row.style.descriptor.opacity = disabled_opacity;
                    row.style.descriptor.cursor = CursorHint::NotAllowed;
                    row.interaction.disabled = true;
                    row.a11y.tab_index = Some(-1);
                } else {
                    row.interaction.focusable = true;
                    row.a11y.tab_index = Some(if is_active { 0 } else { -1 });
                    row.style.descriptor.cursor = CursorHint::Pointer;
                    if select.is_some() || active_change.is_some() {
                        let select = select.clone();
                        let active_change = active_change.clone();
                        let id = action.id.clone();
                        row.interaction.on_activate = Some(Arc::new(move || {
                            if let Some(active_change) = &active_change {
                                active_change(Some(&id));
                            }
                            if let Some(select) = &select {
                                select(&id);
                            }
                        }));
                    }
                }

                // Left: title + optional badge.
                let mut left = Node::container();
                {
                    let s = &mut left.style;
                    s.descriptor.layout.direction = LayoutDirection::Row;
                    s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
                    s.descriptor.layout.spacing.gap = gap_sm;
                }
                let mut title = Node::text(&action.title);
                title.style.text_size = Some(font_size);
                title.style.descriptor.text_color = Some(row_text_color);
                let mut left = left.child(title);
                if let Some(ref badge) = action.badge {
                    let mut b = Node::text(badge);
                    {
                        let s = &mut b.style;
                        s.text_size = Some(label_size);
                        let pad = &mut s.descriptor.layout.spacing.padding;
                        pad.left = rem_to_px(0.25);
                        pad.right = rem_to_px(0.25);
                        pad.top = rem_to_px(0.0625);
                        pad.bottom = rem_to_px(0.0625);
                        s.descriptor.corner_radii.top_left = radius_control;
                        s.descriptor.corner_radii.top_right = radius_control;
                        s.descriptor.corner_radii.bottom_right = radius_control;
                        s.descriptor.corner_radii.bottom_left = radius_control;
                        s.descriptor.background = Some(mix_srgb(accent, surface_bg, 0.12));
                        s.descriptor.text_color = Some(accent);
                    }
                    left = left.child(b);
                }
                let mut row = row.child(left);

                // Right: optional shortcut hint.
                if let Some(ref shortcut) = action.shortcut {
                    let mut hint = Node::text(shortcut);
                    hint.style.text_size = Some(label_size);
                    hint.style.descriptor.text_color = Some(text_muted);
                    row = row.child(hint);
                }

                results_list = results_list.child(row);
            }
            results_list
        }
    };
    let mut results = results;
    let results_id = scoped_id(instance_id, "results", || {
        "poodle-cmd-palette-results".to_string()
    });
    results.id = Some(results_id.clone());
    results.runtime_id = Some(results_id);
    results
        .roles
        .insert("part".to_owned(), "results".to_owned());
    modal = modal.child(results);

    // Delegate backdrop containment and Escape/outside dismissal to the
    // production Dialog renderer, then apply CommandPalette's exact surface
    // geometry and anatomy to that Dialog-owned panel.
    let mut dialog_spec = DialogSpec::new()
        .with_open(true)
        .with_bare(true)
        .with_aria_label(spec.title.as_deref().unwrap_or("Command palette"))
        .with_size(effective_size)
        .with_size_role(spec.size_role)
        .with_density(density);
    dialog_spec.dismiss_on_escape = true;
    dialog_spec.dismiss_on_backdrop = true;
    let mut root = dialog(&dialog_spec, ctx, Vec::new(), None, close_handler);
    let overlay_id = scoped_id(instance_id, "overlay", || {
        "poodle-cmd-palette-overlay".to_string()
    });
    root.id = Some(overlay_id.clone());
    root.runtime_id = Some(overlay_id);
    root.roles
        .insert("dependency".to_owned(), "dialog".to_owned());
    root.roles
        .insert("component".to_owned(), "command-palette".to_owned());
    if spec.description.is_some() {
        root.a11y.described_by = Some(scoped_id(instance_id, "description", || {
            "poodle-cmd-palette-description".to_string()
        }));
    }
    let panel = root
        .children
        .first_mut()
        .expect("Dialog renderer always provides a surface panel");
    let dialog_id = scoped_id(instance_id, "dialog", || {
        "poodle-cmd-palette-dialog".to_string()
    });
    panel.id = Some(dialog_id.clone());
    panel.runtime_id = Some(dialog_id);
    panel.style = modal.style;
    panel.children = modal.children;
    panel
        .roles
        .insert("dependency".to_owned(), "dialog".to_owned());
    panel.roles.insert("part".to_owned(), "dialog".to_owned());
    panel.interaction.dismiss_layer = Some(scoped_id(instance_id, "layer", || {
        "poodle-cmd-palette-layer".to_string()
    }));
    root
}

/// Live status string for the palette — mirrors Svelte `paletteStatus` and the
/// GPUI build exactly.
fn palette_status(spec: &CommandPaletteSpec) -> String {
    match spec.state {
        DiscoveryState::Loading => "Loading commands.".to_string(),
        DiscoveryState::Error => "Command palette unavailable.".to_string(),
        DiscoveryState::Empty => "No commands are available in this workspace.".to_string(),
        DiscoveryState::NoResults => format!("No commands match \"{}\".", spec.query),
        DiscoveryState::Ready => {
            let enabled: Vec<&CommandActionItem> =
                spec.actions.iter().filter(|a| !a.is_disabled).collect();
            let count = enabled.len();
            let plural = if count == 1 { "" } else { "s" };
            let active_suffix = spec
                .active_action_id
                .as_deref()
                .and_then(|id| spec.actions.iter().find(|a| a.id == id))
                .map(|a| format!(" Active command: {}.", a.title))
                .unwrap_or_default();
            format!("{count} command{plural} available.{active_suffix}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    #[test]
    fn query_select_and_close_handlers_reach_their_nodes() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let query_seen = Arc::clone(&seen);
        let select_seen = Arc::clone(&seen);
        let close_seen = Arc::clone(&seen);
        let spec = CommandPaletteSpec::new(vec![CommandActionItem::new("save", "Save")]);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = command_palette_with_handlers(
            &spec,
            &ctx,
            CommandPaletteHandlers {
                query_change: Some(Arc::new(move |value| {
                    query_seen.lock().unwrap().push(format!("query:{value}"));
                })),
                select: Some(Arc::new(move |value| {
                    select_seen.lock().unwrap().push(format!("select:{value}"));
                })),
                close: Some(Arc::new(move || {
                    close_seen.lock().unwrap().push("close".to_string());
                })),
                ..CommandPaletteHandlers::default()
            },
        );

        let query = node
            .find(&|node| matches!(&node.kind, poodle_node::NodeKind::Input { .. }))
            .expect("query input");
        let save = node
            .find(&|node| node.id.as_deref() == Some("poodle-cmd-palette-save"))
            .expect("save action");
        let close = node
            .find(&|node| node.id.as_deref() == Some("poodle-cmd-palette-close"))
            .expect("close action");
        (query.interaction.on_text_change.as_ref().unwrap())("sav");
        (save.interaction.on_activate.as_ref().unwrap())();
        (close.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            ["query:sav", "select:save", "close"]
        );
    }
}
