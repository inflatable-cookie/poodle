//! Drag-and-drop semantic kernel — Rust mirror of core `drag-drop.ts`.
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//! Shared vectors: `vectors/machines.json` (`dragDrop`), executed by both this
//! crate (`tests/conformance.rs`) and the TypeScript core.
//!
//! The kernel owns lifecycle, session identity, semantic intent, cancellation,
//! nested-target arbitration, and exactly-once terminal effects. No pointer,
//! keyboard, GPUI, geometry, timer, transport, file, or host vocabulary
//! appears here; adapters translate their platform into these events and
//! execute the effect intents.
//!
//! Exactly-once is a property of the phase, not of a flag: `EmitDragStart` can
//! only be emitted on `Armed -> Dragging`, `RequestDrop` only on
//! `Dragging -> Dropping`, and the terminal quartet only on the single
//! transition into `Ended` or `Cancelled`. A repeat of any of those events
//! arrives in a phase that no longer accepts it and is inert.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragOperation {
    Move,
    Copy,
    Link,
}

/// Free-form so consumers can define their own placements; `before`, `inside`,
/// and `after` are the vocabulary every target shares.
pub type DropPosition = String;

pub const DROP_POSITION_BEFORE: &str = "before";
pub const DROP_POSITION_INSIDE: &str = "inside";
pub const DROP_POSITION_AFTER: &str = "after";

/// The whole portable payload. `kind` selects a consumer-defined subject
/// family and `id` resolves the live subject through consumer state. Neither
/// is display text, a path, a record, or authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DragSubject {
    pub kind: String,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropIntent {
    pub target_id: String,
    pub position: DropPosition,
    pub operation: DragOperation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DropEligibility {
    Accepted { intent: DropIntent },
    Rejected { reason: Option<String> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragSessionPhase {
    Idle,
    Preparing,
    Armed,
    Dragging,
    Dropping,
    Ended,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragCancelReason {
    PreparationDeclined,
    PreparationFailed,
    Superseded,
    Escape,
    Explicit,
    SourceLost,
    TargetLost,
    TransportLost,
    WindowLost,
}

/// `Ended` carries an authoritative drop result — committed, rejected, or
/// failed. `Cancelled` carries the reason the session aborted without one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragTerminalOutcome {
    Committed { intent: DropIntent },
    Rejected { reason: Option<String> },
    Failed { reason: Option<String> },
    Cancelled { reason: DragCancelReason },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragAnnouncementKind {
    Pickup,
    IntentChanged,
    IntentCleared,
    Dropped,
    Rejected,
    Failed,
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DragSession {
    pub session_id: String,
    pub source_id: String,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub allowed_operations: Vec<DragOperation>,
    pub intent: Option<DropIntent>,
}

/// Present from `Preparing` through the terminal phases; cleared by `Reset`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DragSessionContext {
    pub session: Option<DragSession>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragSessionEvent {
    Prepare {
        session_id: String,
        source_id: String,
        subject: DragSubject,
        operation: DragOperation,
        allowed_operations: Vec<DragOperation>,
    },
    Prepared {
        session_id: String,
    },
    PrepareDeclined {
        session_id: String,
    },
    PrepareFailed {
        session_id: String,
    },
    Activate {
        session_id: String,
    },
    TargetIntent {
        session_id: String,
        intent: DropIntent,
    },
    TargetCleared {
        session_id: String,
    },
    OperationChanged {
        session_id: String,
        operation: DragOperation,
    },
    DropRequested {
        session_id: String,
    },
    DropCommitted {
        session_id: String,
        intent: DropIntent,
    },
    DropRejected {
        session_id: String,
        reason: Option<String>,
    },
    DropFailed {
        session_id: String,
        reason: Option<String>,
    },
    Escape {
        session_id: String,
    },
    Cancel {
        session_id: String,
    },
    SourceLost {
        session_id: String,
    },
    TargetLost {
        session_id: String,
        target_id: String,
    },
    TransportLost {
        session_id: String,
    },
    WindowLost {
        session_id: String,
    },
    Reset {
        session_id: String,
    },
}

impl DragSessionEvent {
    /// Every event names the session it was created for, so a completion that
    /// arrives after supersession can be rejected as stale.
    pub fn session_id(&self) -> &str {
        match self {
            DragSessionEvent::Prepare { session_id, .. }
            | DragSessionEvent::Prepared { session_id }
            | DragSessionEvent::PrepareDeclined { session_id }
            | DragSessionEvent::PrepareFailed { session_id }
            | DragSessionEvent::Activate { session_id }
            | DragSessionEvent::TargetIntent { session_id, .. }
            | DragSessionEvent::TargetCleared { session_id }
            | DragSessionEvent::OperationChanged { session_id, .. }
            | DragSessionEvent::DropRequested { session_id }
            | DragSessionEvent::DropCommitted { session_id, .. }
            | DragSessionEvent::DropRejected { session_id, .. }
            | DragSessionEvent::DropFailed { session_id, .. }
            | DragSessionEvent::Escape { session_id }
            | DragSessionEvent::Cancel { session_id }
            | DragSessionEvent::SourceLost { session_id }
            | DragSessionEvent::TargetLost { session_id, .. }
            | DragSessionEvent::TransportLost { session_id }
            | DragSessionEvent::WindowLost { session_id }
            | DragSessionEvent::Reset { session_id } => session_id,
        }
    }
}

/// Effects are intents, not payloads. `Announce` carries only the announcement
/// kind because the adapter already holds the session (target, position,
/// operation) and, for a terminal announcement, the `EmitDropResult` that
/// immediately precedes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragSessionEffect {
    PrepareSession {
        session_id: String,
        source_id: String,
        subject: DragSubject,
    },
    EmitDragStart {
        session_id: String,
        source_id: String,
        subject: DragSubject,
        operation: DragOperation,
    },
    RequestDrop {
        session_id: String,
        intent: DropIntent,
    },
    EmitDropResult {
        session_id: String,
        outcome: DragTerminalOutcome,
    },
    Announce {
        kind: DragAnnouncementKind,
    },
    ReturnFocus {
        session_id: String,
        subject: DragSubject,
    },
    CleanupSession {
        session_id: String,
    },
}

/// One already-measured nested-target candidate. Geometry is adapter-owned:
/// the adapter decides `contains_point` and `depth`, the kernel decides which
/// candidate wins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropTargetCandidate {
    pub target_id: String,
    /// Registration depth; deeper wins.
    pub depth: i32,
    /// Stable registration order; lower wins the final tie-break.
    pub order: i32,
    /// Explicit priority, applied only among equal-depth candidates.
    pub priority: i32,
    pub contains_point: bool,
    pub eligibility: DropEligibility,
}

type Transition = (DragSessionPhase, DragSessionContext, Vec<DragSessionEffect>);

fn is_terminal_phase(phase: DragSessionPhase) -> bool {
    matches!(phase, DragSessionPhase::Ended | DragSessionPhase::Cancelled)
}

fn is_active_phase(phase: DragSessionPhase) -> bool {
    matches!(
        phase,
        DragSessionPhase::Preparing
            | DragSessionPhase::Armed
            | DragSessionPhase::Dragging
            | DragSessionPhase::Dropping
    )
}

/// The single transition into a terminal phase: result, announcement,
/// focus-return (only when a pickup actually happened), then cleanup.
fn terminal(
    phase: DragSessionPhase,
    session: DragSession,
    outcome: DragTerminalOutcome,
    kind: DragAnnouncementKind,
    next: DragSessionPhase,
) -> Transition {
    let mut effects = vec![
        DragSessionEffect::EmitDropResult {
            session_id: session.session_id.clone(),
            outcome,
        },
        DragSessionEffect::Announce { kind },
    ];

    if matches!(phase, DragSessionPhase::Dragging | DragSessionPhase::Dropping) {
        effects.push(DragSessionEffect::ReturnFocus {
            session_id: session.session_id.clone(),
            subject: session.subject.clone(),
        });
    }

    effects.push(DragSessionEffect::CleanupSession {
        session_id: session.session_id.clone(),
    });

    (
        next,
        DragSessionContext {
            session: Some(session),
        },
        effects,
    )
}

fn cancel(phase: DragSessionPhase, session: DragSession, reason: DragCancelReason) -> Transition {
    terminal(
        phase,
        session,
        DragTerminalOutcome::Cancelled { reason },
        DragAnnouncementKind::Cancelled,
        DragSessionPhase::Cancelled,
    )
}

fn prepare(
    phase: DragSessionPhase,
    context: DragSessionContext,
    session_id: String,
    source_id: String,
    subject: DragSubject,
    operation: DragOperation,
    allowed_operations: Vec<DragOperation>,
) -> Transition {
    if !allowed_operations.contains(&operation) {
        return (phase, context, vec![]);
    }

    // An active gesture owns its session; a terminal one must be reset first.
    if matches!(phase, DragSessionPhase::Dragging | DragSessionPhase::Dropping)
        || is_terminal_phase(phase)
    {
        return (phase, context, vec![]);
    }

    let begin = DragSessionEffect::PrepareSession {
        session_id: session_id.clone(),
        source_id: source_id.clone(),
        subject: subject.clone(),
    };
    let session = DragSession {
        session_id,
        source_id,
        subject,
        operation,
        allowed_operations,
        intent: None,
    };

    if phase == DragSessionPhase::Idle {
        return (
            DragSessionPhase::Preparing,
            DragSessionContext {
                session: Some(session),
            },
            vec![begin],
        );
    }

    let superseded = match &context.session {
        Some(current) if current.session_id != session.session_id => current.session_id.clone(),
        _ => return (phase, context, vec![]),
    };

    (
        DragSessionPhase::Preparing,
        DragSessionContext {
            session: Some(session),
        },
        vec![
            DragSessionEffect::EmitDropResult {
                session_id: superseded.clone(),
                outcome: DragTerminalOutcome::Cancelled {
                    reason: DragCancelReason::Superseded,
                },
            },
            DragSessionEffect::Announce {
                kind: DragAnnouncementKind::Cancelled,
            },
            DragSessionEffect::CleanupSession {
                session_id: superseded,
            },
            begin,
        ],
    )
}

pub fn drag_session_transition(
    phase: DragSessionPhase,
    context: DragSessionContext,
    event: DragSessionEvent,
) -> Transition {
    if let DragSessionEvent::Prepare {
        session_id,
        source_id,
        subject,
        operation,
        allowed_operations,
    } = event
    {
        return prepare(
            phase,
            context,
            session_id,
            source_id,
            subject,
            operation,
            allowed_operations,
        );
    }

    // A stale event naming a session that is not the current one is inert.
    let session = match &context.session {
        Some(current) if current.session_id == event.session_id() => current.clone(),
        _ => return (phase, context, vec![]),
    };

    match event {
        DragSessionEvent::Prepared { .. } if phase == DragSessionPhase::Preparing => {
            (DragSessionPhase::Armed, context, vec![])
        }
        DragSessionEvent::PrepareDeclined { .. } if phase == DragSessionPhase::Preparing => {
            cancel(phase, session, DragCancelReason::PreparationDeclined)
        }
        DragSessionEvent::PrepareFailed { .. } if phase == DragSessionPhase::Preparing => {
            cancel(phase, session, DragCancelReason::PreparationFailed)
        }
        DragSessionEvent::Activate { .. } if phase == DragSessionPhase::Armed => {
            let effects = vec![
                DragSessionEffect::EmitDragStart {
                    session_id: session.session_id.clone(),
                    source_id: session.source_id.clone(),
                    subject: session.subject.clone(),
                    operation: session.operation,
                },
                DragSessionEffect::Announce {
                    kind: DragAnnouncementKind::Pickup,
                },
            ];

            (DragSessionPhase::Dragging, context, effects)
        }
        DragSessionEvent::TargetIntent { intent, .. }
            if phase == DragSessionPhase::Dragging
                && session.allowed_operations.contains(&intent.operation)
                && session.intent.as_ref() != Some(&intent) =>
        {
            let next = DragSession {
                operation: intent.operation,
                intent: Some(intent),
                ..session
            };

            (
                DragSessionPhase::Dragging,
                DragSessionContext {
                    session: Some(next),
                },
                vec![DragSessionEffect::Announce {
                    kind: DragAnnouncementKind::IntentChanged,
                }],
            )
        }
        DragSessionEvent::TargetCleared { .. }
            if phase == DragSessionPhase::Dragging && session.intent.is_some() =>
        {
            let next = DragSession {
                intent: None,
                ..session
            };

            (
                DragSessionPhase::Dragging,
                DragSessionContext {
                    session: Some(next),
                },
                vec![DragSessionEffect::Announce {
                    kind: DragAnnouncementKind::IntentCleared,
                }],
            )
        }
        DragSessionEvent::OperationChanged { operation, .. }
            if phase == DragSessionPhase::Dragging
                && session.allowed_operations.contains(&operation)
                && session.operation != operation =>
        {
            let intent = session.intent.clone().map(|current| DropIntent {
                operation,
                ..current
            });
            let announced = intent.is_some();
            let next = DragSession {
                operation,
                intent,
                ..session
            };

            (
                DragSessionPhase::Dragging,
                DragSessionContext {
                    session: Some(next),
                },
                if announced {
                    vec![DragSessionEffect::Announce {
                        kind: DragAnnouncementKind::IntentChanged,
                    }]
                } else {
                    vec![]
                },
            )
        }
        DragSessionEvent::DropRequested { .. } if phase == DragSessionPhase::Dragging => {
            match session.intent.clone() {
                Some(intent) => (
                    DragSessionPhase::Dropping,
                    context,
                    vec![DragSessionEffect::RequestDrop {
                        session_id: session.session_id,
                        intent,
                    }],
                ),
                None => (phase, context, vec![]),
            }
        }
        DragSessionEvent::DropCommitted { intent, .. } if phase == DragSessionPhase::Dropping => {
            let committed = DragSession {
                operation: intent.operation,
                intent: Some(intent.clone()),
                ..session
            };

            terminal(
                phase,
                committed,
                DragTerminalOutcome::Committed { intent },
                DragAnnouncementKind::Dropped,
                DragSessionPhase::Ended,
            )
        }
        DragSessionEvent::DropRejected { reason, .. } if phase == DragSessionPhase::Dropping => {
            terminal(
                phase,
                session,
                DragTerminalOutcome::Rejected { reason },
                DragAnnouncementKind::Rejected,
                DragSessionPhase::Ended,
            )
        }
        DragSessionEvent::DropFailed { reason, .. } if phase == DragSessionPhase::Dropping => {
            terminal(
                phase,
                session,
                DragTerminalOutcome::Failed { reason },
                DragAnnouncementKind::Failed,
                DragSessionPhase::Ended,
            )
        }
        DragSessionEvent::Escape { .. } if is_active_phase(phase) => {
            cancel(phase, session, DragCancelReason::Escape)
        }
        DragSessionEvent::Cancel { .. } if is_active_phase(phase) => {
            cancel(phase, session, DragCancelReason::Explicit)
        }
        DragSessionEvent::SourceLost { .. } if is_active_phase(phase) => {
            cancel(phase, session, DragCancelReason::SourceLost)
        }
        DragSessionEvent::TargetLost { target_id, .. }
            if is_active_phase(phase)
                && session
                    .intent
                    .as_ref()
                    .is_some_and(|intent| intent.target_id == target_id) =>
        {
            cancel(phase, session, DragCancelReason::TargetLost)
        }
        DragSessionEvent::TransportLost { .. } if is_active_phase(phase) => {
            cancel(phase, session, DragCancelReason::TransportLost)
        }
        DragSessionEvent::WindowLost { .. } if is_active_phase(phase) => {
            cancel(phase, session, DragCancelReason::WindowLost)
        }
        DragSessionEvent::Reset { .. } if is_terminal_phase(phase) => (
            DragSessionPhase::Idle,
            DragSessionContext { session: None },
            vec![],
        ),
        _ => (phase, context, vec![]),
    }
}

fn outranks(candidate: &DropTargetCandidate, best: &DropTargetCandidate) -> bool {
    if candidate.depth != best.depth {
        return candidate.depth > best.depth;
    }

    if candidate.priority != best.priority {
        return candidate.priority > best.priority;
    }

    candidate.order < best.order
}

/// Deterministic nested-target arbitration over already-measured candidates:
/// discard non-containing and ineligible candidates, prefer the deepest, then
/// explicit priority among equal depth, then stable registration order.
/// Returns at most one intent — never several simultaneous drops.
pub fn resolve_drop_target(candidates: &[DropTargetCandidate]) -> Option<DropIntent> {
    let mut best: Option<&DropTargetCandidate> = None;
    let mut best_intent: Option<&DropIntent> = None;

    for candidate in candidates {
        let intent = match (&candidate.eligibility, candidate.contains_point) {
            (DropEligibility::Accepted { intent }, true) => intent,
            _ => continue,
        };

        let wins = match best {
            None => true,
            Some(current) => outranks(candidate, current),
        };

        if wins {
            best = Some(candidate);
            best_intent = Some(intent);
        }
    }

    best_intent.cloned()
}

#[cfg(test)]
mod tests {
    //! The claims the shared vectors cannot state.
    //!
    //! Lifecycle, ordering, and inertia live in the cross-language corpus
    //! (`vectors/machines.json`, `dragDrop`). What is left here is Rust-side:
    //! the stale-event gate reads one session id per event, and the resolver's
    //! result must not depend on the order the adapter happened to collect its
    //! candidates in.

    use super::*;

    fn subject() -> DragSubject {
        DragSubject {
            kind: "track".to_string(),
            id: "t1".to_string(),
        }
    }

    fn intent(target_id: &str) -> DropIntent {
        DropIntent {
            target_id: target_id.to_string(),
            position: DROP_POSITION_INSIDE.to_string(),
            operation: DragOperation::Move,
        }
    }

    fn candidate(target_id: &str, depth: i32, order: i32, priority: i32) -> DropTargetCandidate {
        DropTargetCandidate {
            target_id: target_id.to_string(),
            depth,
            order,
            priority,
            contains_point: true,
            eligibility: DropEligibility::Accepted {
                intent: intent(target_id),
            },
        }
    }

    /// Every variant must report the session it was created for; a variant that
    /// returned someone else's id would slip past the stale-event gate.
    #[test]
    fn every_event_reports_its_own_session() {
        let id = "s1".to_string();
        let events = vec![
            DragSessionEvent::Prepare {
                session_id: id.clone(),
                source_id: "src-a".to_string(),
                subject: subject(),
                operation: DragOperation::Move,
                allowed_operations: vec![DragOperation::Move],
            },
            DragSessionEvent::Prepared {
                session_id: id.clone(),
            },
            DragSessionEvent::PrepareDeclined {
                session_id: id.clone(),
            },
            DragSessionEvent::PrepareFailed {
                session_id: id.clone(),
            },
            DragSessionEvent::Activate {
                session_id: id.clone(),
            },
            DragSessionEvent::TargetIntent {
                session_id: id.clone(),
                intent: intent("list"),
            },
            DragSessionEvent::TargetCleared {
                session_id: id.clone(),
            },
            DragSessionEvent::OperationChanged {
                session_id: id.clone(),
                operation: DragOperation::Copy,
            },
            DragSessionEvent::DropRequested {
                session_id: id.clone(),
            },
            DragSessionEvent::DropCommitted {
                session_id: id.clone(),
                intent: intent("list"),
            },
            DragSessionEvent::DropRejected {
                session_id: id.clone(),
                reason: None,
            },
            DragSessionEvent::DropFailed {
                session_id: id.clone(),
                reason: None,
            },
            DragSessionEvent::Escape {
                session_id: id.clone(),
            },
            DragSessionEvent::Cancel {
                session_id: id.clone(),
            },
            DragSessionEvent::SourceLost {
                session_id: id.clone(),
            },
            DragSessionEvent::TargetLost {
                session_id: id.clone(),
                target_id: "list".to_string(),
            },
            DragSessionEvent::TransportLost {
                session_id: id.clone(),
            },
            DragSessionEvent::WindowLost {
                session_id: id.clone(),
            },
            DragSessionEvent::Reset {
                session_id: id.clone(),
            },
        ];

        assert_eq!(events.len(), 19);

        for event in &events {
            assert_eq!(event.session_id(), "s1", "{event:?}");
        }
    }

    #[test]
    fn an_inert_event_returns_the_caller_context_unchanged() {
        let context = DragSessionContext {
            session: Some(DragSession {
                session_id: "s1".to_string(),
                source_id: "src-a".to_string(),
                subject: subject(),
                operation: DragOperation::Move,
                allowed_operations: vec![DragOperation::Move],
                intent: None,
            }),
        };

        let (phase, next, effects) = drag_session_transition(
            DragSessionPhase::Dragging,
            context.clone(),
            DragSessionEvent::DropRequested {
                session_id: "s1".to_string(),
            },
        );

        assert_eq!(phase, DragSessionPhase::Dragging);
        assert_eq!(next, context);
        assert!(effects.is_empty());
    }

    #[test]
    fn arbitration_does_not_depend_on_candidate_order() {
        let candidates = vec![
            candidate("root", 0, 0, 0),
            candidate("row", 2, 2, 0),
            candidate("group", 1, 1, 0),
        ];
        let mut reversed = candidates.clone();
        reversed.reverse();

        assert_eq!(resolve_drop_target(&candidates), Some(intent("row")));
        assert_eq!(resolve_drop_target(&reversed), Some(intent("row")));
    }

    #[test]
    fn equal_depth_and_priority_fall_back_to_registration_order() {
        let candidates = vec![candidate("second", 1, 1, 2), candidate("first", 1, 0, 2)];

        assert_eq!(resolve_drop_target(&candidates), Some(intent("first")));
    }

    #[test]
    fn priority_applies_only_within_one_depth() {
        let candidates = vec![candidate("shallow", 0, 0, 99), candidate("deep", 1, 1, -99)];

        assert_eq!(resolve_drop_target(&candidates), Some(intent("deep")));
    }
}
