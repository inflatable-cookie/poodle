//! Renderer-neutral drag-and-drop registration vocabulary.
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//!
//! A node declares that it *is* a drag source or a drop target; it never
//! describes a gesture. The semantic lifecycle belongs to the shared Rust
//! kernel (`poodle_headless::drag_drop`) and the mechanisms that cannot cross
//! a renderer boundary — capture, hit testing, measured geometry, native event
//! translation, preview painting, focus, announcements — belong to the
//! backend's drag controller.
//!
//! The types the kernel already owns are re-exported rather than mirrored:
//! one `DragSubject`, one `DropIntent`, one `DropPosition`, one terminal
//! outcome. A second copy here would be a second lifecycle in slow motion.
//!
//! Geometry stops at this boundary the same way it stops at [`crate::DropEdge`]
//! and `Interaction::on_scrub`: a position resolver receives *fractions* of
//! the target's own bounds, never a window point, rectangle, event, entity, or
//! backend handle.

use std::sync::Arc;

pub use poodle_headless::cross_window_drag::{
    CrossWindowAbort, CrossWindowCleanup, CrossWindowCommitComplete, CrossWindowDragCapabilities,
    CrossWindowDragCommitRequest, CrossWindowDragInputKind, CrossWindowDragPrepareRequest,
    CrossWindowDragProjection, CrossWindowDragReceipt, CrossWindowDragSourceBridge,
    CrossWindowDragTargetBridge, CrossWindowDragTargetEvent, CrossWindowDragTransport,
    CrossWindowPrepareComplete, CrossWindowTerminal, DragDropCommitResult,
    CROSS_WINDOW_DRAG_MIME_TYPE, CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
};
pub use poodle_headless::drag_drop::{
    DragAnnouncementKind, DragCancelReason, DragOperation, DragSession, DragSessionPhase,
    DragSubject, DragTerminalOutcome, DropEligibility, DropIntent, DropPosition,
    DROP_POSITION_AFTER, DROP_POSITION_BEFORE, DROP_POSITION_INSIDE,
};
pub use poodle_headless::external_file_drag::{
    can_export_anything, is_presentable_file_name, validate_file_export,
    validate_inbound_files, DragExportBridge, DragExportCapabilities, DragExportForm,
    DragExportPrepareComplete, DragExportPrepareRequest, DragExportRefusal, DragExportSnapshot,
    DragExportState, DragExportTerminal, DragExportTerminalCallback, DragExportValidation,
    InboundFileBatch, InboundFileCapabilities, InboundFileConstraints, InboundFileEvent,
    InboundFileHostBridge, InboundFileOutcome, InboundFileReceipt, InboundFileRefusal,
    InboundFileTransport, InboundFileValidation, PreparedFileExport, INBOUND_FILE_PROTOCOL_VERSION,
    INBOUND_FILE_SUBJECT_KIND,
};

/// Which input device drove the gesture that produced an observation.
///
/// A runtime reports what it actually received. Synthesized mouse input is
/// [`Self::Mouse`] — it never becomes evidence for pen or touch.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeDragInputKind {
    Mouse,
    Pen,
    Touch,
    Keyboard,
}

/// The immutable input-capability report a runtime's drag controller
/// publishes.
///
/// This is a statement about the runtime, resolved once and never negotiated
/// per gesture: a consumer decides whether to offer a drag affordance and what
/// accessible instructions to claim by reading it. A field is `true` only when
/// the platform delivers that class of input to the controller directly.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeDragCapabilities {
    /// Mouse buttons and mouse movement.
    pub mouse: bool,
    /// A pen/stylus with its own pointer identity, distinguishable from mouse.
    pub pen: bool,
    /// Touch contacts, with hold-versus-scroll arbitration.
    pub touch: bool,
    /// Keyboard pickup, traversal, drop, and cancellation.
    pub keyboard: bool,
    /// Movement keeps reaching the active session after the pointer leaves the
    /// source's bounds, for the lifetime of the gesture inside this window.
    pub in_window_capture: bool,
    /// The platform reports a device-originated pointer cancellation (an
    /// interrupted contact), distinct from release, Escape, or host cancel.
    pub device_cancel: bool,
}

/// Where the pointer sits inside a drop target's own bounds.
///
/// Fractions, not coordinates: `0.0` is the target's leading/top edge and
/// `1.0` its trailing/bottom edge, both clamped. The backend already owns the
/// measured rectangle, so handing over a fraction keeps layout out of the
/// component exactly as [`crate::DropEdge`] and `on_scrub` do.
#[derive(Clone, Debug)]
pub struct NodeDropPositionInput {
    pub fraction_x: f32,
    pub fraction_y: f32,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub input_kind: NodeDragInputKind,
}

/// A keyboard traversal step over the ordered target registry.
///
/// `Previous` and `Next` are distinct inputs rather than a synthetic point:
/// a linear list normally maps them onto `before` and `after`, and `First` /
/// `Last` stay explicit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKeyboardDropDirection {
    Previous,
    Next,
    First,
    Last,
}

/// The keyboard counterpart of [`NodeDropPositionInput`] — no rectangle,
/// because a keyboard intent is not a point.
#[derive(Clone, Debug)]
pub struct NodeKeyboardPositionInput {
    pub direction: NodeKeyboardDropDirection,
    pub subject: DragSubject,
    pub operation: DragOperation,
}

/// The authoritative result a consumer reports for a requested drop.
///
/// Hover acceptance never authorizes durable mutation; this is the answer to
/// the one revalidated commit request, and the controller maps it onto the
/// kernel's terminal event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeDropCommit {
    Committed,
    Rejected { reason: Option<String> },
    Failed { reason: Option<String> },
}

/// The revalidated drop the controller asks a target to commit.
///
/// The web controller hands `onDrop` the intent alone because a DOM consumer
/// reads the live subject from the controller snapshot it is already
/// subscribed to. A Rust closure has no snapshot store, so the same two facts
/// travel together here: identical semantics, one less way to get the subject
/// wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeDropCommitEvent {
    pub subject: DragSubject,
    pub intent: DropIntent,
    /// The external files being committed, when the subject is an inbound
    /// batch. Receipts and display metadata only — never a path or a handle.
    pub inbound_files: Option<InboundFileBatch>,
}

/// The current session intent, projected onto the target that owns it.
///
/// This is presentation state — the drop indicator a row draws — not a second
/// lifecycle callback. Exactly one registered target holds the current intent
/// at a time, and it is told when it stops holding it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeDropIntentEvent {
    pub subject: DragSubject,
    pub position: DropPosition,
    pub operation: DragOperation,
}

pub type NodeDragStartHandler = Arc<dyn Fn(&DragSession) + Send + Sync>;
pub type NodeDragEndHandler = Arc<dyn Fn(&DragTerminalOutcome) + Send + Sync>;
pub type NodeDropPositionResolver =
    Arc<dyn Fn(&NodeDropPositionInput) -> Option<DropPosition> + Send + Sync>;
pub type NodeKeyboardPositionResolver =
    Arc<dyn Fn(&NodeKeyboardPositionInput) -> Option<DropPosition> + Send + Sync>;
pub type NodeDropEligibilityResolver =
    Arc<dyn Fn(&DropIntent, &DragSubject) -> DropEligibility + Send + Sync>;
pub type NodeDropCommitHandler = Arc<dyn Fn(&NodeDropCommitEvent) -> NodeDropCommit + Send + Sync>;
pub type NodeDropIntentHandler = Arc<dyn Fn(&NodeDropIntentEvent) + Send + Sync>;
pub type NodeDropIntentClearedHandler = Arc<dyn Fn() + Send + Sync>;

/// Marks this node as a semantic drag source.
///
/// The registration is declarative and re-read on every frame: a host that
/// rebuilds with `disabled: true`, a different subject, or no registration at
/// all has changed the source, and the controller reacts through the kernel's
/// ordinary paths rather than a component-owned lifecycle.
#[derive(Clone)]
pub struct NodeDragSource {
    /// Stable within one controller. A duplicate live id is an error, not
    /// last-writer-wins.
    pub source_id: String,
    /// The whole portable payload: a consumer-defined kind and an id that
    /// resolves the live subject through consumer state.
    pub subject: DragSubject,
    pub allowed_operations: Vec<DragOperation>,
    /// The operation the session starts with; must be in
    /// [`Self::allowed_operations`] or the kernel refuses to prepare.
    pub operation: DragOperation,
    pub disabled: bool,
    /// Required accessible name, used by the controller's announcements.
    pub label: String,
    /// Optional accessible instructions for the keyboard route.
    pub instructions: Option<String>,
    /// Opt-in keyboard pickup, and the origin for ordered logical traversal.
    /// A source that omits it leaves Space and Enter to the host component.
    pub keyboard_order: Option<i32>,
    /// This source narrates its own sessions, so the controller's live region
    /// says nothing about them.
    ///
    /// A composite with a contract-mandated live region of its own — the model
    /// catalogue announces "Moved X to position 3 of 4" — otherwise has one
    /// move read out twice, in two different sentences, from two regions. The
    /// renderer-neutral half of core's `ownsAnnouncements`.
    pub owns_announcements: bool,
    /// Host preparation for a drag that may leave this window.
    ///
    /// Optional and per source, because a lease belongs to the subject being
    /// dragged — which is the half of the split the *source* owns. The window
    /// half (projection, commit, accessible picking) is installed on the
    /// runtime's controller instead, because it outlives any one subject and
    /// arrives with no local source at all.
    pub cross_window_source_bridge: Option<Arc<dyn CrossWindowDragSourceBridge>>,
    /// Host preparation for a drag that may leave for the operating system.
    ///
    /// Optional and per source, because what gets exported belongs to the
    /// subject being dragged. Mutually exclusive with
    /// [`Self::cross_window_source_bridge`]: one gesture can only leave one
    /// way, and a source declaring both would need a silent precedence rule.
    pub file_export_bridge: Option<Arc<dyn DragExportBridge>>,
    pub on_drag_start: Option<NodeDragStartHandler>,
    pub on_drag_end: Option<NodeDragEndHandler>,
}

impl NodeDragSource {
    /// A move-only source with the required identity and accessible name.
    pub fn new(source_id: impl Into<String>, subject: DragSubject, label: impl Into<String>) -> Self {
        Self {
            source_id: source_id.into(),
            subject,
            allowed_operations: vec![DragOperation::Move],
            operation: DragOperation::Move,
            disabled: false,
            label: label.into(),
            instructions: None,
            keyboard_order: None,
            owns_announcements: false,
            cross_window_source_bridge: None,
            file_export_bridge: None,
            on_drag_start: None,
            on_drag_end: None,
        }
    }
}

/// Marks this node as a semantic drop target.
///
/// `resolve_position` turns adapter-owned geometry into a semantic position;
/// `can_drop` answers eligibility for the resulting intent and must not
/// mutate, because it runs during hover *and* again at commit.
#[derive(Clone)]
pub struct NodeDropTarget {
    /// Stable within one controller. A duplicate live id is an error.
    pub target_id: String,
    /// Subject kinds this target will consider. Empty accepts every kind.
    pub accepted_kinds: Vec<String>,
    pub disabled: bool,
    /// Applied only among equal-depth candidates; deeper always wins first.
    pub priority: i32,
    /// Required accessible name, used by the controller's announcements.
    pub label: String,
    /// Pointer geometry to semantic position. `None` returns no intent for
    /// this target at this point. Absent resolver means the target never
    /// resolves a pointer intent.
    pub resolve_position: Option<NodeDropPositionResolver>,
    /// Position in the ordered keyboard registry. A target that omits it is
    /// not reachable by keyboard traversal.
    pub keyboard_order: Option<i32>,
    /// Traversal direction to semantic position, for the keyboard route.
    pub resolve_keyboard_position: Option<NodeKeyboardPositionResolver>,
    /// What this target takes when the subject is an external file batch.
    ///
    /// Checked before [`Self::can_drop`], on every hover and again at drop:
    /// external metadata is untrusted input, and a target should answer "do I
    /// want this" rather than "is this even real".
    pub inbound_files: Option<InboundFileConstraints>,
    /// Eligibility for a resolved intent. Absent means accepted.
    pub can_drop: Option<NodeDropEligibilityResolver>,
    /// The revalidated commit. Absent means the target cannot commit and the
    /// drop is rejected.
    pub on_drop: Option<NodeDropCommitHandler>,
    /// This target became, or updated, the session's single current intent.
    pub on_intent: Option<NodeDropIntentHandler>,
    /// This target stopped holding the current intent — the pointer moved to
    /// another target, left every target, or the session ended.
    pub on_intent_cleared: Option<NodeDropIntentClearedHandler>,
}

impl NodeDropTarget {
    /// A target with the required identity and accessible name, accepting one
    /// subject kind and nothing else.
    pub fn new(
        target_id: impl Into<String>,
        accepted_kind: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            target_id: target_id.into(),
            accepted_kinds: vec![accepted_kind.into()],
            disabled: false,
            priority: 0,
            label: label.into(),
            resolve_position: None,
            keyboard_order: None,
            resolve_keyboard_position: None,
            inbound_files: None,
            can_drop: None,
            on_drop: None,
            on_intent: None,
            on_intent_cleared: None,
        }
    }

    /// Whether this target will consider the subject at all. An empty
    /// `accepted_kinds` accepts every kind; `disabled` accepts none.
    pub fn accepts(&self, subject: &DragSubject) -> bool {
        !self.disabled
            && (self.accepted_kinds.is_empty()
                || self.accepted_kinds.iter().any(|kind| *kind == subject.kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subject(kind: &str) -> DragSubject {
        DragSubject {
            kind: kind.to_string(),
            id: "one".to_string(),
        }
    }

    /// The kind filter is the target's own gate, not the controller's: a
    /// disabled target accepts nothing even when its kind list matches, and an
    /// empty list is "any kind" rather than "no kind".
    #[test]
    fn a_target_gates_on_kind_and_disabled_posture() {
        let mut target = NodeDropTarget::new("row", "track", "Row");
        assert!(target.accepts(&subject("track")));
        assert!(!target.accepts(&subject("clip")));

        target.accepted_kinds.clear();
        assert!(target.accepts(&subject("clip")));

        target.disabled = true;
        assert!(!target.accepts(&subject("track")));
    }

    /// The default source is move-only and keyboard-silent: Space and Enter
    /// stay with the host component until a `keyboard_order` opts in.
    #[test]
    fn a_new_source_is_move_only_and_opts_out_of_keyboard_pickup() {
        let source = NodeDragSource::new("row-1", subject("track"), "Kick");

        assert_eq!(source.allowed_operations, vec![DragOperation::Move]);
        assert_eq!(source.operation, DragOperation::Move);
        assert!(source.keyboard_order.is_none());
        assert!(!source.disabled);
    }
}
