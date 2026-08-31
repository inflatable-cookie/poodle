//! Shared drag-and-drop construction for `poodle-node` trees.
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//!
//! Components declare *what* they are — a reorderable row, a nested tree row,
//! a list that accepts one subject kind — and this module turns that into the
//! renderer-neutral registrations the backend's drag controller consumes. Two
//! components that both reorder a flat list must not write the band rule
//! twice; a rule written twice is the drift the substrate exists to remove.
//!
//! Nothing here is GPUI-, Jetstream-, or DOM-shaped. Both native backends
//! build from these same registrations, which is what keeps deferred Jetstream
//! construction renderer-neutral.

use std::sync::Arc;

use poodle_node::{
    DragOperation, DragSubject, DropEdge, DropEligibility, DropIntent, DropPosition, Node,
    NodeDragSource, NodeDropPositionInput, NodeDropTarget, NodeKeyboardDropDirection,
    NodeKeyboardPositionInput, DROP_POSITION_AFTER, DROP_POSITION_BEFORE, DROP_POSITION_INSIDE,
};

/// The subject-kind prefix every list-reorder component uses for its own rows.
///
/// Never a kind on its own. Reorder moves a row *within* the surface that owns
/// it, so the kind carries the instance scope: one shared kind would let a
/// row from one Tabs resolve a target in another Tabs, in Tree, or in
/// ModelCatalogueEditor whenever they share a controller, and overlapping
/// values would then mutate the wrong component. Scoped ids alone do not stop
/// that — eligibility does.
pub const REORDER_SUBJECT_KIND: &str = "poodle.reorder-item";

/// The subject kind one reorder surface accepts, and nothing else does.
pub fn reorder_kind(scope: &str) -> String {
    format!("{REORDER_SUBJECT_KIND}:{scope}")
}

/// Turn a semantic position into the closed three-value edge a reorder
/// component's public callback speaks. An unknown consumer-defined position
/// has no edge.
pub fn edge_from_position(position: &str) -> Option<DropEdge> {
    match position {
        DROP_POSITION_BEFORE => Some(DropEdge::Before),
        DROP_POSITION_INSIDE => Some(DropEdge::Inside),
        DROP_POSITION_AFTER => Some(DropEdge::After),
        _ => None,
    }
}

/// The inverse of [`edge_from_position`].
pub fn position_from_edge(edge: DropEdge) -> DropPosition {
    match edge {
        DropEdge::Before => DROP_POSITION_BEFORE,
        DropEdge::Inside => DROP_POSITION_INSIDE,
        DropEdge::After => DROP_POSITION_AFTER,
    }
    .to_string()
}

/// Split a target's height into before / inside / after bands.
///
/// A target that cannot take an inside drop splits in half rather than
/// collapsing its middle to a default: a leaf row whose whole body answered
/// `inside` would silently swallow every reorder aimed at its edges.
pub fn position_for_fraction(fraction: f32, accepts_inside: bool) -> DropPosition {
    if accepts_inside {
        if fraction < 0.25 {
            DROP_POSITION_BEFORE
        } else if fraction > 0.75 {
            DROP_POSITION_AFTER
        } else {
            DROP_POSITION_INSIDE
        }
    } else if fraction < 0.5 {
        DROP_POSITION_BEFORE
    } else {
        DROP_POSITION_AFTER
    }
    .to_string()
}

/// The subject a reorder row carries: this surface's kind and the component's
/// own row value.
pub fn reorder_subject(scope: &str, value: &str) -> DragSubject {
    DragSubject {
        kind: reorder_kind(scope),
        id: value.to_string(),
    }
}

/// A move-only reorder source for one row.
///
/// `scope` is the component instance id, so two mounted lists in one
/// controller never mint the same source id.
pub fn reorder_source(scope: &str, value: &str, label: &str) -> NodeDragSource {
    reorder_source_in_family(scope, &reorder_kind(scope), value, label)
}

/// A reorder source in an explicit semantic family.
///
/// The registration id stays scoped to the surface instance while the subject
/// kind is chosen by an owning composite. Those are different things: the kind
/// says who may consider this row, the id says which registration it is, and
/// two strips in one controller may legitimately hold the same row values.
pub fn reorder_source_in_family(
    scope: &str,
    kind: &str,
    value: &str,
    label: &str,
) -> NodeDragSource {
    NodeDragSource::new(
        format!("{scope}:source:{value}"),
        DragSubject {
            kind: kind.to_string(),
            id: value.to_string(),
        },
        label,
    )
}

/// A flat reorder target: before / after by half, no nesting.
///
/// Accepts this surface's scoped kind only, and refuses a row dropped onto
/// itself — the two rules every reorder surface needs and none of them should
/// restate.
pub fn reorder_target(scope: &str, value: &str, label: &str) -> NodeDropTarget {
    let mut target = NodeDropTarget::new(
        format!("{scope}:target:{value}"),
        reorder_kind(scope),
        label,
    );
    target.resolve_position = Some(vertical_band_resolver(false));
    target.resolve_keyboard_position = Some(linear_keyboard_resolver());
    target.can_drop = Some(rejects_self(value));
    target
}

/// A reorder target in an explicit semantic family.
///
/// `owned` is this surface's own row values. A shared family means another
/// surface's subject can reach this target, so it refuses one it does not own
/// *during eligibility* — arbitration then discards it and an eligible
/// ancestor composite target wins. Claiming the drop and rejecting it at
/// commit would swallow it instead.
pub fn reorder_target_in_family(
    scope: &str,
    kind: &str,
    value: &str,
    label: &str,
    owned: Vec<String>,
) -> NodeDropTarget {
    let mut target = NodeDropTarget::new(format!("{scope}:target:{value}"), kind, label);
    target.resolve_position = Some(vertical_band_resolver(false));
    target.resolve_keyboard_position = Some(linear_keyboard_resolver());
    target.can_drop = Some(rejects_foreign_or_self(value, owned));
    target
}

/// Refuse a row dropped onto itself, and any subject this surface does not own.
pub fn rejects_foreign_or_self(
    value: &str,
    owned: Vec<String>,
) -> Arc<dyn Fn(&DropIntent, &DragSubject) -> DropEligibility + Send + Sync> {
    let value = value.to_string();
    Arc::new(move |intent: &DropIntent, subject: &DragSubject| {
        if !owned.iter().any(|known| *known == subject.id) {
            return DropEligibility::Rejected {
                reason: Some("That row belongs to another surface".to_string()),
            };
        }
        if subject.id == value {
            return DropEligibility::Rejected {
                reason: Some("A row cannot be dropped onto itself".to_string()),
            };
        }
        DropEligibility::Accepted {
            intent: intent.clone(),
        }
    })
}

/// A nested placement target: before / inside / after by thirds when the row
/// can hold children, halves when it cannot. Same scope and self-drop rules as
/// [`reorder_target`].
pub fn nested_target(scope: &str, value: &str, label: &str, accepts_inside: bool) -> NodeDropTarget {
    let mut target = NodeDropTarget::new(
        format!("{scope}:target:{value}"),
        reorder_kind(scope),
        label,
    );
    target.resolve_position = Some(vertical_band_resolver(accepts_inside));
    target.resolve_keyboard_position = Some(linear_keyboard_resolver());
    target.can_drop = Some(rejects_self(value));
    target
}

/// The band rule along the horizontal axis — a tab bar reorders left to
/// right, so splitting its rows would answer `before` for the whole strip.
pub fn horizontal_band_resolver(
    accepts_inside: bool,
) -> Arc<dyn Fn(&NodeDropPositionInput) -> Option<DropPosition> + Send + Sync> {
    Arc::new(move |input: &NodeDropPositionInput| {
        Some(position_for_fraction(input.fraction_x, accepts_inside))
    })
}

/// The vertical band rule as a reusable resolver.
pub fn vertical_band_resolver(
    accepts_inside: bool,
) -> Arc<dyn Fn(&NodeDropPositionInput) -> Option<DropPosition> + Send + Sync> {
    Arc::new(move |input: &NodeDropPositionInput| {
        Some(position_for_fraction(input.fraction_y, accepts_inside))
    })
}

/// Traversal-to-position for a linear list: `previous` lands before the
/// target, `next` after it, and first/last stay explicit rather than being
/// inferred from a synthetic midpoint.
pub fn linear_keyboard_resolver(
) -> Arc<dyn Fn(&NodeKeyboardPositionInput) -> Option<DropPosition> + Send + Sync> {
    Arc::new(|input: &NodeKeyboardPositionInput| {
        Some(
            match input.direction {
                NodeKeyboardDropDirection::Previous | NodeKeyboardDropDirection::First => {
                    DROP_POSITION_BEFORE
                }
                NodeKeyboardDropDirection::Next | NodeKeyboardDropDirection::Last => {
                    DROP_POSITION_AFTER
                }
            }
            .to_string(),
        )
    })
}

/// Refuse a drop that would land a row on itself.
///
/// Every reorder surface needs this and none of them should express it as a
/// silently-ignored callback: a self-drop must be *rejected*, so the target
/// posture and the announcement both say so.
pub fn rejects_self(
    value: &str,
) -> Arc<dyn Fn(&DropIntent, &DragSubject) -> DropEligibility + Send + Sync> {
    let value = value.to_string();
    Arc::new(move |intent: &DropIntent, subject: &DragSubject| {
        if subject.id == value {
            DropEligibility::Rejected {
                reason: Some("A row cannot be dropped onto itself".to_string()),
            }
        } else {
            DropEligibility::Accepted {
                intent: intent.clone(),
            }
        }
    })
}

/// Attach a source registration to a node, unless the row is inert.
///
/// A disabled row is not registered at all rather than registered-and-ignored:
/// an unregistered source cannot be picked up by pointer *or* keyboard, and
/// cannot appear in an announcement.
pub fn attach_source(node: &mut Node, enabled: bool, source: NodeDragSource) {
    if enabled && !source.disabled {
        node.interaction.drag_source = Some(source);
    }
}

/// Attach a target registration to a node, unless the row is inert.
pub fn attach_target(node: &mut Node, enabled: bool, target: NodeDropTarget) {
    if enabled && !target.disabled {
        node.interaction.drop_target = Some(target);
    }
}

/// The operation set a reorder surface allows: move only. Copy and link are
/// consumer policy a reorder contract does not have.
pub fn move_only() -> Vec<DragOperation> {
    vec![DragOperation::Move]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_nested_target_keeps_its_edge_bands_and_a_leaf_never_answers_inside() {
        assert_eq!(position_for_fraction(0.1, true), DROP_POSITION_BEFORE);
        assert_eq!(position_for_fraction(0.5, true), DROP_POSITION_INSIDE);
        assert_eq!(position_for_fraction(0.9, true), DROP_POSITION_AFTER);

        assert_eq!(position_for_fraction(0.25, false), DROP_POSITION_BEFORE);
        assert_eq!(position_for_fraction(0.5, false), DROP_POSITION_AFTER);
        assert_eq!(position_for_fraction(0.99, false), DROP_POSITION_AFTER);
    }

    /// The closed component edge and the open semantic position must round
    /// trip, and a consumer-defined position must not be forced into one of
    /// the three.
    #[test]
    fn edges_round_trip_and_an_unknown_position_has_none() {
        for edge in [DropEdge::Before, DropEdge::Inside, DropEdge::After] {
            assert_eq!(edge_from_position(&position_from_edge(edge)), Some(edge));
        }
        assert_eq!(edge_from_position("into-bus"), None);
    }

    #[test]
    fn a_reorder_row_rejects_itself_with_a_reason_rather_than_going_quiet() {
        let target = reorder_target("list", "kick", "Kick");
        let eligibility = target.can_drop.clone().expect("self-rejection is installed");
        let intent = DropIntent {
            target_id: target.target_id.clone(),
            position: DROP_POSITION_AFTER.to_string(),
            operation: DragOperation::Move,
        };

        let refused = eligibility(&intent, &reorder_subject("list", "kick"));
        assert!(matches!(
            refused,
            DropEligibility::Rejected { reason: Some(_) }
        ));
        assert_eq!(
            eligibility(&intent, &reorder_subject("list", "snare")),
            DropEligibility::Accepted {
                intent: intent.clone()
            }
        );
    }

    /// A nested surface needs the same two rules; Tree must not be the one
    /// component that silently accepts a row dropped on itself.
    #[test]
    fn a_nested_target_carries_the_same_scope_and_self_drop_rules() {
        let target = nested_target("tree", "kick", "Kick", true);
        assert_eq!(target.accepted_kinds, vec![reorder_kind("tree")]);
        assert!(target.can_drop.is_some());
    }

    /// Two mounted instances must not mint one id, or the controller's
    /// duplicate-id rule would fire on an ordinary two-list page.
    #[test]
    fn source_and_target_ids_are_scoped_per_instance() {
        assert_ne!(
            reorder_source("list-a", "row", "Row").source_id,
            reorder_source("list-b", "row", "Row").source_id
        );
        assert_ne!(
            reorder_target("list-a", "row", "Row").target_id,
            reorder_target("list-b", "row", "Row").target_id
        );
    }

    /// Scoped ids are not enough: two reorder surfaces sharing one controller
    /// must be ineligible for each other, or a row from one list resolves a
    /// target in the other and mutates the wrong component.
    #[test]
    fn one_reorder_surface_is_ineligible_for_another_surfaces_subject() {
        let mine = reorder_target("list-a", "row", "Row");
        assert!(mine.accepts(&reorder_subject("list-a", "other")));
        assert!(!mine.accepts(&reorder_subject("list-b", "other")));
        assert!(!mine.accepts(&reorder_subject("tree", "other")));
    }

    /// `previous` and `next` are distinct resolver inputs; a linear list maps
    /// them onto before and after rather than a synthetic centre point.
    #[test]
    fn keyboard_traversal_maps_direction_onto_placement() {
        let resolve = linear_keyboard_resolver();
        let input = |direction| NodeKeyboardPositionInput {
            direction,
            subject: reorder_subject("list", "kick"),
            operation: DragOperation::Move,
        };

        assert_eq!(
            resolve(&input(NodeKeyboardDropDirection::Previous)).as_deref(),
            Some(DROP_POSITION_BEFORE)
        );
        assert_eq!(
            resolve(&input(NodeKeyboardDropDirection::Next)).as_deref(),
            Some(DROP_POSITION_AFTER)
        );
        assert_eq!(
            resolve(&input(NodeKeyboardDropDirection::First)).as_deref(),
            Some(DROP_POSITION_BEFORE)
        );
        assert_eq!(
            resolve(&input(NodeKeyboardDropDirection::Last)).as_deref(),
            Some(DROP_POSITION_AFTER)
        );
    }

    /// An inert row registers nothing: a registered-but-ignored source would
    /// still be announced and still be keyboard-reachable.
    #[test]
    fn an_inert_row_registers_neither_source_nor_target() {
        let mut node = Node::container();
        attach_source(&mut node, false, reorder_source("list", "row", "Row"));
        attach_target(&mut node, false, reorder_target("list", "row", "Row"));
        assert!(node.interaction.drag_source.is_none());
        assert!(node.interaction.drop_target.is_none());

        let mut disabled_source = reorder_source("list", "row", "Row");
        disabled_source.disabled = true;
        attach_source(&mut node, true, disabled_source);
        assert!(node.interaction.drag_source.is_none());
    }
}
