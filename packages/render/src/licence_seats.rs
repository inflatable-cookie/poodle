//! LicenceSeats — a list of authority-reported activation seats with inline
//! machine naming and release requests.
//!
//! Contract: `docs/contracts/components/licence-seats.md`
//!
//! Rows derive once through `poodle_headless::licence::licence_seat_rows`.
//! Raw `machine_id` values are callback data and list keys only: every
//! visible and accessible identity is the supplied label or `Unnamed
//! machine`. Rename persistence, the release command, and the resulting seat
//! refresh are host-owned — Poodle never optimistically rewrites authority
//! state. Confirm-open and edit-open are host state (the Rust targets own
//! overlay/edit open state, like every other composed control).

use std::sync::Arc;

use poodle_headless::licence::{
    licence_machine_label, licence_seat_rows, LICENCE_RELEASE_CONFIRM_TITLE, LICENCE_THIS_MACHINE,
};
use poodle_node::{
    CrossAxisAlignment, CursorHint, LayoutDirection, MainAxisAlignment, Node, NodeRole,
};
use poodle_specs::{
    ButtonTone, ButtonVariant, ConfirmActionSpec, EditableLabelActivation, EditableLabelSpec,
    EditableLabelVariant, IconButtonSpec, LicenceSeatsSpec, StatusTone,
};

use crate::confirm_action::{confirm_action_with_slots, ConfirmActionHandlers};
use crate::context::RenderContext;
use crate::editable_label::{editable_label_with_handlers, EditableLabelHandlers};
use crate::icon_button::icon_button;
use crate::presentation::rem_to_px;

/// Host callbacks. Every payload carries the exact machine id; the rename
/// label is trimmed (`None` for a blank name).
#[derive(Default)]
pub struct LicenceSeatsHandlers {
    /// A machine-name edit committed. `label` is the trimmed value or `None`.
    pub on_rename: Option<Arc<dyn Fn(&str, Option<&str>) + Send + Sync>>,
    /// A machine-name edit was started (host flips the row into editing).
    pub on_rename_edit: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// A release was confirmed/requested.
    pub on_release: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// The release trigger was pressed (host opens that row's confirm).
    pub on_release_trigger: Option<Arc<dyn Fn(&str) + Send + Sync>>,
    /// The confirm dialog was dismissed without releasing.
    pub on_release_cancel: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

pub fn licence_seats(
    spec: &LicenceSeatsSpec,
    ctx: &RenderContext<'_>,
    handlers: LicenceSeatsHandlers,
) -> Node {
    // Empty authority renders nothing at all — a synthetic seat count would
    // be Poodle inventing an account the authority did not give it.
    if spec.seats.is_empty() {
        return Node::container();
    }

    let text_primary = ctx.theme().resolve_color("color.text.primary");
    let text_secondary = ctx.theme().resolve_color("color.text.secondary");
    let text_tertiary = ctx.theme().resolve_color("color.text.tertiary");
    let monitor_size = rem_to_px(1.25);

    let rows =
        licence_seat_rows(&spec.seats, spec.pending_machine_id.as_deref(), &spec.release_label);

    // ── List ──
    let mut list = Node::container();
    {
        let s = &mut list.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.xs");
    }
    for row in &rows {
        list = list.child(seat_row(
            spec,
            row,
            ctx,
            &handlers,
            text_secondary,
            text_tertiary,
            monitor_size,
        ));
    }

    // ── Section ──
    let mut root = Node::container();
    {
        let s = &mut root.style;
        s.descriptor.layout.direction = LayoutDirection::Column;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.stack.sm");
    }
    let mut title = Node::text(&spec.title);
    title.style.text_size = Some(rem_to_px(1.0));
    title.style.text_weight = Some(600);
    title.style.descriptor.text_color = Some(text_primary);
    let mut root = root.child(title).child(list);

    root.a11y.role = Some(NodeRole::Region);
    root.a11y.label = Some(spec.title.clone());
    root
}

#[allow(clippy::too_many_arguments)]
fn seat_row(
    spec: &LicenceSeatsSpec,
    row: &poodle_headless::licence::LicenceSeatRow,
    ctx: &RenderContext<'_>,
    handlers: &LicenceSeatsHandlers,
    text_secondary: poodle_node::ColorValue,
    text_tertiary: poodle_node::ColorValue,
    monitor_size: f32,
) -> Node {
    // ── Decorative computer icon (aria-hidden: the label owns identity) ──
    let mut machine_icon = Node::icon("monitor", monitor_size);
    machine_icon.style.descriptor.text_color = Some(text_secondary);

    // ── Inline machine naming ──
    let editing_this_row = spec.editing_machine_id.as_deref() == Some(row.machine_id.as_str());
    let edit_start = handlers.on_rename_edit.clone().map(move |handler| {
        let machine_id = row.machine_id.clone();
        Arc::new(move || handler(&machine_id)) as Arc<dyn Fn() + Send + Sync>
    });
    let commit = handlers.on_rename.clone().map(move |handler| {
        let machine_id = row.machine_id.clone();
        Arc::new(move |value: &str| {
            let label = licence_machine_label(value);
            handler(&machine_id, label.as_deref());
        }) as Arc<dyn Fn(&str) + Send + Sync>
    });
    let editable = editable_label_with_handlers(
        &EditableLabelSpec::new()
            .with_value(if row.named {
                row.display_label.clone()
            } else {
                String::new()
            })
            .with_editing(editing_this_row)
            .with_activation_mode(EditableLabelActivation::EnterOrSpace)
            .with_variant(EditableLabelVariant::Flush)
            .with_empty_text(poodle_headless::licence::LICENCE_UNNAMED_MACHINE)
            .with_placeholder(poodle_headless::licence::LICENCE_UNNAMED_MACHINE)
            .with_show_edit_icon(true)
            .with_aria_label(if row.named {
                format!("Rename {}", row.display_label)
            } else {
                "Rename unnamed machine".to_string()
            })
            .with_size(ctx.base_size(spec.size))
            .with_density(ctx.resolve_density(spec.density)),
        ctx,
        EditableLabelHandlers {
            on_edit_start: edit_start,
            on_commit: commit,
            ..EditableLabelHandlers::default()
        },
    );

    // ── Current-machine marker ──
    let mut identity = Node::container();
    {
        let s = &mut identity.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.sm");
    }
    let mut identity = identity.child(machine_icon).child(editable);
    if row.this_machine {
        let mut marker = Node::text(LICENCE_THIS_MACHINE);
        marker.style.text_size = Some(ctx.theme().resolve_space("typography.label.size"));
        marker.style.descriptor.text_color = Some(text_tertiary);
        identity = identity.child(marker);
    }

    // ── Release action ──
    let action = if row.releasable {
        let trigger = release_icon_button(spec, row, ctx);
        if spec.confirm_release {
            let confirm_spec = ConfirmActionSpec::new(
                LICENCE_RELEASE_CONFIRM_TITLE,
                row.confirm_body.clone(),
                spec.release_label.clone(),
                "Cancel",
            )
            .with_tone(StatusTone::Warning)
            .with_open(spec.open_confirm_machine_id.as_deref() == Some(row.machine_id.as_str()))
            .with_size(ctx.base_size(spec.size))
            .with_density(ctx.resolve_density(spec.density));
            confirm_action_with_slots(
                &confirm_spec,
                ctx,
                Some(trigger),
                None,
                ConfirmActionHandlers {
                    on_trigger: handlers
                        .on_release_trigger
                        .as_ref()
                        .map(|handler| {
                            let machine_id = row.machine_id.clone();
                            let handler = Arc::clone(handler);
                            Arc::new(move || handler(&machine_id)) as Arc<dyn Fn() + Send + Sync>
                        }),
                    on_confirm: handlers
                        .on_release
                        .as_ref()
                        .map(|handler| {
                            let machine_id = row.machine_id.clone();
                            let handler = Arc::clone(handler);
                            Arc::new(move || handler(&machine_id)) as Arc<dyn Fn() + Send + Sync>
                        }),
                    on_cancel: handlers
                        .on_release_cancel
                        .as_ref()
                        .map(|handler| {
                            let machine_id = row.machine_id.clone();
                            let handler = Arc::clone(handler);
                            Arc::new(move || handler(&machine_id)) as Arc<dyn Fn() + Send + Sync>
                        }),
                },
            )
        } else {
            // No confirmation: the ghost danger IconButton releases directly.
            if let Some(on_release) = &handlers.on_release {
                let mut button = trigger;
                let handler = Arc::clone(on_release);
                let machine_id = row.machine_id.clone();
                button.interaction.on_activate = Some(Arc::new(move || handler(&machine_id)));
                button
            } else {
                trigger
            }
        }
    } else {
        Node::container()
    };

    // ── Row ──
    let mut row_node = Node::container();
    {
        let s = &mut row_node.style;
        s.descriptor.layout.direction = LayoutDirection::Row;
        s.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        s.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        s.descriptor.layout.spacing.gap = ctx.theme().resolve_space("space.inline.md");
    }
    row_node.a11y.role = Some(NodeRole::ListItem);
    row_node.child(identity).child(action)
}

/// The ghost danger trash IconButton that triggers release. Its accessible
/// name carries the action and machine identity; the random machine id stays
/// callback-only.
fn release_icon_button(
    spec: &LicenceSeatsSpec,
    row: &poodle_headless::licence::LicenceSeatRow,
    ctx: &RenderContext<'_>,
) -> Node {
    let mut button = icon_button(
        &IconButtonSpec::new()
            .with_icon("trash-2")
            .with_variant(ButtonVariant::Ghost)
            .with_tone(ButtonTone::Danger)
            .with_aria_label(row.release_name.clone())
            .with_disabled(row.pending)
            .with_loading(row.pending)
            .with_size(ctx.base_size(spec.size))
            .with_density(ctx.resolve_density(spec.density)),
        ctx,
        None,
    );
    button.style.descriptor.cursor = CursorHint::Pointer;
    button
}

#[cfg(test)]
mod tests {
    use super::*;
    use poodle_headless::licence::LicenceSeat;
    use poodle_specs::LicenceSeatsSpec;

    fn theme() -> poodle_jetstream::JetstreamThemeProvider {
        poodle_jetstream::JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE)
    }

    fn seat(machine_id: &str, label: Option<&str>, this_machine: bool) -> LicenceSeat {
        LicenceSeat {
            machine_id: machine_id.to_string(),
            label: label.map(str::to_string),
            this_machine,
        }
    }

    #[test]
    fn empty_seats_render_nothing() {
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(
            &LicenceSeatsSpec::new(),
            &ctx,
            LicenceSeatsHandlers::default(),
        );
        assert!(node.children.is_empty());
        assert!(node.texts().is_empty());
    }

    #[test]
    fn labelled_and_unnamed_rows_render_honestly() {
        let spec = LicenceSeatsSpec::new().with_seats(vec![
            seat("id-a", Some("Studio rig"), true),
            seat("id-b", None, false),
        ]);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(&spec, &ctx, LicenceSeatsHandlers::default());
        let texts = node
            .texts()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        assert!(texts.iter().any(|t| t == "Studio rig"));
        assert!(texts.iter().any(|t| t == "Unnamed machine"));
        assert!(texts.iter().any(|t| t == LICENCE_THIS_MACHINE));
        assert!(
            !texts.iter().any(|t| t.contains("id-a") || t.contains("id-b")),
            "raw machine ids never reach rendered or accessible text"
        );
    }

    #[test]
    fn this_machine_cannot_be_released_others_can() {
        let spec = LicenceSeatsSpec::new().with_seats(vec![
            seat("id-a", Some("Studio rig"), true),
            seat("id-b", None, false),
        ]);
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(&spec, &ctx, LicenceSeatsHandlers::default());
        // The only release affordance names the other seat.
        assert!(node
            .find(&|n| n.a11y.label.as_deref() == Some("Release unnamed machine"))
            .is_some());
        assert!(node
            .find(&|n| n.a11y.label.as_deref() == Some("Release Studio rig"))
            .is_none());
    }

    #[test]
    fn rename_emits_trimmed_label_and_null_for_blank() {
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let on_rename: Arc<dyn Fn(&str, Option<&str>) + Send + Sync> =
            Arc::new(move |machine_id: &str, label: Option<&str>| {
                sink.lock()
                    .unwrap()
                    .push((machine_id.to_string(), label.map(str::to_string)))
            });

        // Editing a supplied label commits the trimmed value.
        let spec = LicenceSeatsSpec::new()
            .with_seats(vec![seat("id-b", Some("  work-mac  "), false)])
            .with_editing_machine(Some("id-b".to_string()));
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(
            &spec,
            &ctx,
            LicenceSeatsHandlers {
                on_rename: Some(Arc::clone(&on_rename)),
                ..LicenceSeatsHandlers::default()
            },
        );
        let input = node
            .find(&|n| n.interaction.on_submit.is_some())
            .expect("the inline rename input");
        (input.interaction.on_submit.as_ref().unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [("id-b".to_string(), Some("work-mac".to_string()))],
            "a committed name is trimmed before emission"
        );

        // An unnamed row committed as-is normalizes blank to null.
        let spec = LicenceSeatsSpec::new()
            .with_seats(vec![seat("id-b", None, false)])
            .with_editing_machine(Some("id-b".to_string()));
        let node = licence_seats(
            &spec,
            &ctx,
            LicenceSeatsHandlers {
                on_rename: Some(Arc::clone(&on_rename)),
                ..LicenceSeatsHandlers::default()
            },
        );
        let input = node
            .find(&|n| n.interaction.on_submit.is_some())
            .expect("the inline rename input");
        (input.interaction.on_submit.as_ref().unwrap())();
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            [
                ("id-b".to_string(), Some("work-mac".to_string())),
                ("id-b".to_string(), None),
            ],
            "a blank committed name normalizes to null"
        );
    }

    #[test]
    fn release_emits_the_exact_machine_id_after_confirm() {
        let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = Arc::clone(&seen);
        let spec = LicenceSeatsSpec::new()
            .with_seats(vec![seat("id-b", None, false)])
            .with_open_confirm(Some("id-b".to_string()));
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(
            &spec,
            &ctx,
            LicenceSeatsHandlers {
                on_release: Some(Arc::new(move |machine_id: &str| {
                    sink.lock().unwrap().push(machine_id.to_string())
                })),
                ..LicenceSeatsHandlers::default()
            },
        );
        // Open: the ConfirmAction's confirm button (labelled "Release").
        let confirm = node
            .find(&|n| {
                matches!(&n.kind, poodle_node::NodeKind::Button { label } if label == "Release")
            })
            .expect("the confirm button");
        (confirm.interaction.on_activate.as_ref().unwrap())();
        assert_eq!(seen.lock().unwrap().as_slice(), ["id-b"]);
    }

    #[test]
    fn pending_affects_only_the_matching_row() {
        let spec = LicenceSeatsSpec::new()
            .with_seats(vec![seat("id-b", None, false), seat("id-c", None, false)])
            .with_pending_machine_id(Some("id-b".to_string()));
        let rows = licence_seat_rows(
            &spec.seats,
            spec.pending_machine_id.as_deref(),
            &spec.release_label,
        );
        assert!(rows[0].pending);
        assert!(!rows[1].pending);
        // The pending row's release affordance is loading/disabled; the
        // other row still carries its action.
        let theme = theme();
        let ctx = RenderContext::new(&theme);
        let node = licence_seats(&spec, &ctx, LicenceSeatsHandlers::default());
        assert!(node
            .find(&|n| n.a11y.label.as_deref() == Some("Release unnamed machine"))
            .is_some());
    }
}
