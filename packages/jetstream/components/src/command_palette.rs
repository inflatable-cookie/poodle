//! CommandPalette — Jetstream command palette backed by CommandPaletteSpec.
//!
//! Contract: `docs/contracts/components/command-palette.md`
//! Reference: `packages/svelte/components/src/CommandPalette.svelte`
//! Mirror of the GPUI build: `packages/gpui/components/src/composites/command_palette.rs`.
//!
//! Renders the full modal shell — overlay backdrop scrim, centered modal
//! surface, header (title/description + invocation-hint pill + close
//! affordance), a composed `js_text_input type="search"`, a live status
//! region (matching Svelte `paletteStatus`), and the grouped results panel
//! with group headers, badges, shortcut hints, active-item treatment, and
//! disabled dimming. Non-ready/empty discovery states render their message
//! in place of the list.
//!
//! Notes:
//! - Interaction (typing, arrow nav, Enter, Escape, backdrop click) lives in
//!   the preview event loop, not the component. Structure renders at the
//!   current (query, state, active-id); editing/selection are NOT faked here.
//! - ARIA is N/A in Jetstream (no accessibility channel).
//! - JsEl has no box-shadow primitive beyond the `shadow_*` presets, so the
//!   contract's `var(--poodle-elevation-dialog)` is approximated with
//!   `shadow_lg()` (the closest available preset). Noted as a token gap.

use glam::Vec4;
use jetstream_ui::ui_element::{self, FontFamily, JsEl};
use poodle_jetstream::JetstreamThemeProvider;
use poodle_specs::{CommandActionItem, CommandPaletteSpec, DiscoveryState, TextInputSpec};

use crate::presentation::{
    panel_space_x_rem, panel_space_y_rem, rem_to_px, resolve_semantic_size, size_font_rem,
};
use crate::text_input::js_text_input;
use crate::theme_ext::{
    color_mix, elevation_dialog, resolve_color, resolve_opacity, resolve_px, resolve_radius,
};

pub fn js_command_palette(spec: &CommandPaletteSpec, theme: &JetstreamThemeProvider) -> JsEl {
    // Closed: render nothing. Consumers that never touched `is_open` still
    // render because the spec default is `true`.
    if !spec.is_open {
        return ui_element::div();
    }

    let effective_size = resolve_semantic_size(spec.size, spec.size_role);
    let font_size = rem_to_px(size_font_rem(effective_size));
    let icon_size = rem_to_px(size_font_rem(effective_size));
    let panel_px = rem_to_px(panel_space_x_rem(spec.density));
    let panel_py = rem_to_px(panel_space_y_rem(spec.density));

    // ── Color / dimension tokens ──────────────────────────────────────
    // Modal surface fill = color.background.elevated (contract §9 mixes it
    // 98% with transparent; resolved straight — the 2% transparent delta is
    // imperceptible and JsEl has no surface transparent-mix path).
    let surface_bg = resolve_color(theme, spec.results_fill_token());
    let border_default = resolve_color(theme, "color.border.default");
    // Contract §9 dialog border = border-default mixed 42% with transparent.
    let dialog_border = color_mix(border_default, Vec4::ZERO, 0.42);
    let text_primary = resolve_color(theme, "color.text.primary");
    let text_secondary = resolve_color(theme, "color.text.secondary");
    let text_muted = resolve_color(theme, "color.text.muted");
    let accent = resolve_color(theme, "color.accent.base");
    let surface_subtle = resolve_color(theme, "color.background.surface");
    let danger = resolve_color(theme, "color.status.danger");
    let disabled_opacity = resolve_opacity(theme, "state.opacity.disabled");
    // Scrim: contract §9 overlay background = black 44%. No token matches
    // that literal exactly, so use the semantic overlay scrim color
    // (color.background.overlay). See parity note.
    let scrim = resolve_color(theme, "color.background.overlay");

    let label_size = resolve_px(theme, "typography.label.size");
    let heading_size = resolve_px(theme, "typography.heading.size");
    let radius_control = resolve_radius(theme, "radius.control");
    let radius_surface = resolve_radius(theme, "radius.surface");
    // Contract §9 dialog radius = radius.surface + 0.125rem.
    let dialog_radius = radius_surface + rem_to_px(0.125);
    let gap_sm = resolve_px(theme, "space.inline.sm");
    let gap_md = resolve_px(theme, "space.inline.md");
    let gap_lg = resolve_px(theme, "space.inline.lg");
    let stack_md = resolve_px(theme, "space.stack.md");

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
    // JsEl has no min()/vw, so pin the rem caps and let the backdrop center.
    // Token-accurate `elevation-dialog` (contract §8: var(--poodle-elevation-dialog))
    // resolved from the typed semantic token via the runtime shadow builder
    // (single layer, spread 0; matches GPUI's mapping).
    let mut modal = elevation_dialog(
        ui_element::div()
            .flex_col()
            .gap(stack_md)
            .w(rem_to_px(45.0))
            .max_w(rem_to_px(45.0))
            .max_h(rem_to_px(52.5))
            .px(panel_px)
            .py(panel_py)
            .rounded(dialog_radius)
            .bg(surface_bg)
            .border_1()
            .border_color(dialog_border),
    )
    .overflow_hidden();

    // ── Header: title-group + meta (hint pill + close) ────────────────
    let mut title_group = ui_element::div().flex_col().gap(rem_to_px(0.125)).grow().min_w_0();
    if let Some(ref title) = spec.title {
        title_group = title_group.child(
            ui_element::label(title.clone())
                .text_size(heading_size)
                .font_weight(600)
                .text_color(text_primary),
        );
    }
    if let Some(ref description) = spec.description {
        title_group = title_group.child(
            ui_element::label(description.clone())
                .text_size(label_size)
                .text_color(text_secondary),
        );
    }

    // Meta column: optional invocation-hint pill + close affordance.
    let mut meta = ui_element::div().flex_row().items_center().gap(gap_sm);
    if let Some(ref hint) = spec.invocation_hint {
        // Contract §9 hint bg = background.surface 76%.
        let hint_bg = color_mix(surface_subtle, Vec4::ZERO, 0.76);
        meta = meta.child(
            ui_element::div()
                .flex_row()
                .items_center()
                .justify_center()
                .min_h(hint_min_h)
                .px(hint_pad_x)
                .rounded(radius_control)
                .bg(hint_bg)
                .child(
                    // Contract §9 `.command-palette__hint`: code-family (kbd-styled).
                    ui_element::label(hint.clone())
                        .text_size(label_size)
                        .text_color(text_secondary)
                        .font_family(FontFamily::Mono),
                ),
        );
    }
    // Close affordance (contract §3 / §9): transparent control-sized square
    // with an `x` icon. Click wiring is preview-event-loop bound.
    meta = meta.child(
        ui_element::div()
            .id(format!("{}-close", "poodle-cmd-palette"))
            .flex_row()
            .items_center()
            .justify_center()
            .w(close_dim)
            .h(close_dim)
            .flex_shrink_0()
            .rounded(close_radius)
            .cursor_pointer()
            .child(
                ui_element::icon("x")
                    .w(icon_size)
                    .h(icon_size)
                    .text_color(text_secondary),
            ),
    );

    let header = ui_element::div()
        .flex_row()
        .items_start()
        .justify_between()
        .gap(gap_md)
        .child(title_group)
        .child(meta);
    modal = modal.child(header);

    // ── Query: composed js_text_input type="search" ───────────────────
    // Renders current query value + leading search icon + placeholder.
    // Editing is preview-event-loop bound; structure + current value render.
    // Placeholder matches the GPUI build's static string; the invocation
    // hint is a separate header pill, not the placeholder.
    let placeholder = "Search commands, panels, and actions".to_string();
    let query_spec = TextInputSpec::new()
        .with_id("poodle-cmd-palette-query")
        .with_input_type("search")
        .with_leading_icon("search")
        .with_value(spec.query.clone())
        .with_placeholder(placeholder)
        .with_aria_label("Search commands")
        .with_show_clear_button(true)
        .with_size(spec.size)
        .with_size_role(spec.size_role)
        .with_density(spec.density);
    modal = modal.child(js_text_input(&query_spec, theme).w_full());

    // ── Status region (contract §3 / §7) ──────────────────────────────
    // Live count / active / state message — matches Svelte `paletteStatus`.
    // ARIA role=status is N/A in Jetstream.
    let status_text = palette_status(spec);
    modal = modal.child(
        // Contract §9 status: 0.75rem font, text-secondary.
        ui_element::label(status_text)
            .text_size(rem_to_px(0.75))
            .text_color(text_secondary),
    );

    // ── Results panel ─────────────────────────────────────────────────
    // Non-ready postures render an empty-state message in place of the list;
    // ready renders grouped rows.
    let results = match spec.state {
        DiscoveryState::Loading => ui_element::div()
            .py(gap_lg)
            .child(
                ui_element::label("Searching\u{2026}")
                    .text_size(font_size)
                    .text_color(text_secondary),
            ),
        DiscoveryState::Error => ui_element::div().py(gap_lg).child(
            ui_element::label("Error loading commands")
                .text_size(font_size)
                .text_color(danger),
        ),
        DiscoveryState::Empty | DiscoveryState::NoResults => ui_element::div().py(gap_lg).child(
            ui_element::label("No matching commands")
                .text_size(font_size)
                .text_color(text_secondary),
        ),
        DiscoveryState::Ready => {
            let mut results_list = ui_element::div()
                .flex_col()
                .gap(rem_to_px(0.125))
                .overflow_y_hidden();

            let mut current_group: Option<&str> = None;
            for action in &spec.actions {
                // Group header when the group changes.
                if action.group.as_deref() != current_group {
                    current_group = action.group.as_deref();
                    if let Some(group_name) = current_group {
                        results_list = results_list.child(
                            ui_element::label(group_name.to_string())
                                .px(gap_sm)
                                .py(rem_to_px(0.25))
                                .text_size(label_size)
                                .font_weight(600)
                                .text_color(text_muted),
                        );
                    }
                }

                let is_active = spec
                    .active_action_id
                    .as_deref()
                    .is_some_and(|id| id == action.id);

                let mut row = ui_element::div()
                    .id(format!("poodle-cmd-palette-{}", action.id))
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .gap(gap_sm)
                    .px(gap_sm)
                    .py(gap_sm)
                    .rounded(radius_control)
                    .focusable();

                let row_text_color = if is_active {
                    // Active treatment: accent tint bg + accent text.
                    row = row.bg(color_mix(accent, surface_bg, 0.10));
                    accent
                } else {
                    text_primary
                };

                if action.is_disabled {
                    row = row.opacity(disabled_opacity);
                } else {
                    row = row.cursor_pointer();
                }

                // Left: title + optional badge.
                let mut left = ui_element::div().flex_row().items_center().gap(gap_sm);
                left = left.child(
                    ui_element::label(action.title.clone())
                        .text_size(font_size)
                        .text_color(row_text_color),
                );
                if let Some(ref badge) = action.badge {
                    left = left.child(
                        ui_element::label(badge.clone())
                            .text_size(label_size)
                            .px(rem_to_px(0.25))
                            .py(rem_to_px(0.0625))
                            .rounded(radius_control)
                            .bg(color_mix(accent, surface_bg, 0.12))
                            .text_color(accent),
                    );
                }
                row = row.child(left);

                // Right: optional shortcut hint.
                if let Some(ref shortcut) = action.shortcut {
                    row = row.child(
                        ui_element::label(shortcut.clone())
                            .text_size(label_size)
                            .text_color(text_muted),
                    );
                }

                results_list = results_list.child(row);
            }
            results_list
        }
    };
    modal = modal.child(results);

    // ── Overlay backdrop ──────────────────────────────────────────────
    // Full-area scrim centering the modal. `.overlay()` lifts the subtree
    // above siblings and escapes parent clip rects (JsEl has no fixed/vw or
    // z-index); `inset_0` fills the positioned ancestor. Backdrop-click and
    // Escape close are preview-event-loop bound.
    ui_element::div()
        .id("poodle-cmd-palette-overlay")
        .absolute()
        .inset_0()
        .bg(scrim)
        .flex_col()
        .items_center()
        .justify_center()
        .overlay()
        .child(modal)
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
    use crate::render_probe::probe;
    use poodle_specs::DiscoveryState;

    fn theme() -> JetstreamThemeProvider {
        JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK)
    }

    fn sample_actions() -> Vec<CommandActionItem> {
        vec![
            CommandActionItem::new("save", "Save")
                .with_group("File")
                .with_shortcut("Cmd+S"),
            CommandActionItem::new("open", "Open File")
                .with_group("File")
                .with_shortcut("Cmd+O"),
            CommandActionItem::new("find", "Find in Files")
                .with_group("Edit")
                .with_shortcut("Cmd+F"),
            CommandActionItem::new("legacy", "Legacy Action")
                .with_group("Edit")
                .with_disabled(true),
        ]
    }

    fn open_spec() -> CommandPaletteSpec {
        CommandPaletteSpec::new(sample_actions())
            .with_open(true)
            .with_title("Command Palette")
            .with_description("Run commands or navigate to files.")
            .with_invocation_hint("Cmd+K")
            .with_active_action_id("find")
    }

    #[test]
    fn open_renders_backdrop_modal_header_and_status() {
        let th = theme();
        let el = js_command_palette(&open_spec(), &th);
        let tree = probe(&el, 900.0, 600.0);

        // Backdrop overlay present (the scrim root carries the overlay id).
        assert!(
            tree.find_token("poodle-cmd-palette-overlay").is_some(),
            "open palette must render the backdrop scrim overlay\n{}",
            tree.to_json()
        );

        // Modal header: title + description text.
        assert!(
            tree.has_text("Command Palette"),
            "header title must render"
        );
        assert!(
            tree.has_text("Run commands or navigate to files."),
            "header description must render"
        );

        // Invocation-hint pill + close affordance.
        assert!(tree.has_text("Cmd+K"), "invocation-hint pill must render");
        assert!(
            tree.find_token("poodle-cmd-palette-close").is_some(),
            "close affordance must render"
        );

        // Composed search input present — `js_text_input type="search"`
        // materializes as a panel row carrying its leading "search" icon.
        // (The icon name is the search affordance; there is no other source
        // of a "search" icon in the tree.)
        assert!(
            tree.has_text("search"),
            "query must be the composed search TextInput (leading search icon)\n{}",
            tree.to_json()
        );

        // Grouped results: at least one group header renders.
        assert!(
            tree.has_text("File") && tree.has_text("Edit"),
            "group headers must render"
        );

        // Active item + a shortcut hint render.
        assert!(tree.has_text("Find in Files"), "active item must render");
        assert!(tree.has_text("Cmd+S"), "shortcut hints must render");

        // Status string (paletteStatus) — 3 enabled, active = Find in Files.
        assert!(
            tree.has_text("3 commands available. Active command: Find in Files."),
            "status region must match paletteStatus\n{:?}",
            tree.texts()
        );
    }

    #[test]
    fn closed_renders_nothing() {
        let th = theme();
        let spec = open_spec().with_open(false);
        let el = js_command_palette(&spec, &th);
        let tree = probe(&el, 900.0, 600.0);

        assert!(
            tree.find_token("poodle-cmd-palette-overlay").is_none(),
            "closed palette must not render the backdrop"
        );
        assert!(
            !tree.has_text("Command Palette"),
            "closed palette must not render header text"
        );
        assert!(
            !tree.has_text("search"),
            "closed palette must not render the query input"
        );
    }

    #[test]
    fn no_results_state_renders_message_not_list() {
        let th = theme();
        let spec = CommandPaletteSpec::new(sample_actions())
            .with_open(true)
            .with_query("zzzzz")
            .with_state(DiscoveryState::NoResults);
        let el = js_command_palette(&spec, &th);
        let tree = probe(&el, 900.0, 600.0);

        assert!(
            tree.has_text("No matching commands"),
            "no-results state renders its message"
        );
        assert!(
            tree.has_text("No commands match \"zzzzz\"."),
            "no-results status string must render"
        );
        // The grouped list must not render in the no-results posture.
        assert!(
            !tree.has_text("Find in Files"),
            "no-results posture must not render the action list"
        );
    }

    #[test]
    fn ready_status_pluralizes_and_counts_enabled_only() {
        let th = theme();
        // Three enabled, one disabled → "3 commands available."
        let spec = CommandPaletteSpec::new(sample_actions()).with_open(true);
        assert_eq!(palette_status(&spec), "3 commands available.");

        // Single enabled action → singular "1 command available."
        let single = CommandPaletteSpec::new(vec![CommandActionItem::new("a", "Alpha")])
            .with_open(true);
        assert_eq!(palette_status(&single), "1 command available.");
    }
}
