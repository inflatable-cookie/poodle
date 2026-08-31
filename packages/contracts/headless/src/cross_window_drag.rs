//! Cross-window drag host bridge — Rust mirror of core `cross-window-drag.ts`.
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//!
//! The bridge is split by ownership, and the split is the whole point. A
//! *source* preparation belongs to one draggable subject: it is armed,
//! started, and cancelled with that subject's gesture. Incoming *projection*,
//! commit, and accessible target picking belong to one document or native
//! window: they outlive any one subject and there may be no local source at
//! all. Combining them into one controller-wide object would tie two different
//! lifetimes to one handle, and a host would have to invent a null half.
//!
//! Only [`CrossWindowDragReceipt`] crosses the wire — a protocol version and
//! an opaque token. Everything else here is a *host-local projection*: the
//! host resolves a receipt into semantic values inside the window that is
//! going to render them. Poodle never serializes a subject, a label, geometry,
//! or a session, and never stores the authoritative transaction.
//!
//! # Why callbacks and not futures
//!
//! TypeScript's bridge returns promises. This crate is renderer-neutral and
//! runs inside GPUI's single-threaded frame loop, which has no executor to
//! await on: an async trait here would force every host to bring a runtime for
//! the sake of one lease allocation. The completion callback is the same
//! lifecycle with the same exactly-once and stale-session rules — the shapes
//! are identical, only the delivery differs, which is exactly the latitude the
//! spec grants Rust.

use crate::drag_drop::{
    DragCancelReason, DragOperation, DragSubject, DragTerminalOutcome, DropIntent, DropPosition,
};

/// The authoritative answer to one revalidated drop request.
///
/// Mirrors core's `DragDropCommitResult`. A local target returns it from its
/// commit handler and a cross-window host bridge returns it from
/// [`CrossWindowDragTargetBridge::commit`]; both map onto the same kernel
/// terminal event, so a host refusal and a local refusal are the same
/// observation to everything downstream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragDropCommitResult {
    Committed,
    Rejected { reason: Option<String> },
    Failed { reason: Option<String> },
}

/// What the host can actually observe, resolved once rather than negotiated
/// per gesture.
///
/// A source does not advertise a cross-window affordance the host cannot
/// carry: `touch` is true only when the host can follow a touch contact
/// *outside* the source window. Internal same-window touch is unaffected — it
/// has its own capability report on the runtime's drag controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CrossWindowDragCapabilities {
    pub pointer: bool,
    pub touch: bool,
    pub keyboard_target_picker: bool,
}

/// The entire portable payload of a cross-window drag.
///
/// `token` is opaque: Poodle compares it, carries it, and hands it back. It is
/// not parsed, not a path, not a record, and not authority on its own — the
/// host resolves it against its own live transaction, which is what makes an
/// expired or forged token safe to receive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossWindowDragReceipt {
    pub protocol_version: u32,
    pub token: String,
}

/// The protocol version this build writes and the only one it accepts.
pub const CROSS_WINDOW_DRAG_PROTOCOL_VERSION: u32 = 1;

/// The default bounded envelope MIME type.
pub const CROSS_WINDOW_DRAG_MIME_TYPE: &str = "application/x-poodle-cross-window-drag+json";

/// The longest token this build will write or read.
///
/// External data is untrusted, so the bound is checked before the value is
/// handed to anything that could act on it.
pub const CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH: usize = 512;

impl CrossWindowDragReceipt {
    /// Whether this is a receipt this build can carry.
    ///
    /// Deliberately strict: an unknown or future protocol version is rejected
    /// rather than best-effort accepted, because a receipt Poodle cannot fully
    /// understand is one it cannot honestly claim to have matched.
    pub fn is_valid(&self) -> bool {
        self.protocol_version == CROSS_WINDOW_DRAG_PROTOCOL_VERSION
            && !self.token.is_empty()
            && self.token.len() <= CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH
    }
}

/// How the host is carrying this gesture between windows.
///
/// `DataTransfer` is the browser's own drag, with the receipt written into a
/// bounded envelope. `WindowCapture` is a host that follows the OS pointer
/// itself. `KeyboardPicker` is the accessible route, which never has a pointer
/// at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossWindowDragTransport {
    DataTransfer,
    WindowCapture,
    KeyboardPicker,
}

/// What Poodle knows when it asks the host to allocate a transaction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossWindowDragPrepareRequest {
    pub session_id: String,
    pub source_id: String,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub allowed_operations: Vec<DragOperation>,
}

/// Which device drove the gesture the host is projecting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossWindowDragInputKind {
    Pointer,
    Touch,
    Keyboard,
}

/// The host's answer to "what is over this window right now".
///
/// Every field is resolved by the host *in the receiving window* — none of it
/// travels beside the receipt. `target_id` names at most one registered Poodle
/// target, so a projection can never produce two simultaneous drops, and
/// Poodle still re-runs that target's own kind, disabled, and eligibility
/// gates before anything commits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossWindowDragProjection {
    pub receipt: CrossWindowDragReceipt,
    pub source_id: String,
    pub source_label: String,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub input_kind: CrossWindowDragInputKind,
    pub target_id: Option<String>,
    pub position: Option<DropPosition>,
}

/// What a window-owned bridge publishes to its subscriber.
///
/// There is no "dropped" event: a drop is a local observation, and the host
/// answers it through [`CrossWindowDragTargetBridge::commit`]. Making the host
/// announce the drop as well would give one gesture two commit paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrossWindowDragTargetEvent {
    Projection {
        projection: CrossWindowDragProjection,
    },
    Left {
        receipt: CrossWindowDragReceipt,
    },
    Cancelled {
        receipt: CrossWindowDragReceipt,
        reason: DragCancelReason,
    },
}

/// The one revalidated drop Poodle asks the host to make durable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossWindowDragCommitRequest {
    pub receipt: CrossWindowDragReceipt,
    pub subject: DragSubject,
    pub intent: DropIntent,
}

/// Per draggable source.
///
/// `prepare` runs on the accepted pre-drag gesture, *before* activation, so a
/// host that must allocate a lease has somewhere to do it that is not inside
/// the runtime's synchronous drag start. A source with this bridge cannot
/// advertise or start a cross-window gesture until its own receipt is armed; a
/// decline or failure cancels only that attempt.
///
/// `start` installs the one authoritative terminal subscription and returns
/// its cleanup. A native drag end or pointer release never manufactures a
/// committed result — only the terminal callback does.
pub trait CrossWindowDragSourceBridge {
    fn capabilities(&self) -> CrossWindowDragCapabilities;

    /// Allocate the host transaction. `complete` receives `None` for a decline
    /// and is called at most once; a completion for a superseded session is
    /// rejected by the kernel on session id.
    fn prepare(
        &self,
        request: CrossWindowDragPrepareRequest,
        complete: Box<dyn FnOnce(Option<CrossWindowDragReceipt>)>,
    );

    /// The gesture is live under `transport`. Returns the subscription's own
    /// cleanup, run exactly once when the session leaves the host's hands.
    fn start(
        &self,
        receipt: CrossWindowDragReceipt,
        transport: CrossWindowDragTransport,
        on_terminal: Box<dyn Fn(DragTerminalOutcome)>,
    ) -> Box<dyn FnOnce()>;

    /// Idempotent at the boundary, and called only while the receipt is still
    /// live.
    fn cancel(&self, receipt: CrossWindowDragReceipt, reason: DragCancelReason);
}

/// Per document or native window.
///
/// `subscribe` is live host projection; `commit` is the authoritative durable
/// step, run only after Poodle revalidates the exact live target the
/// projection named. `pick_target` is required exactly when
/// `keyboard_target_picker` is true and reaches the same revalidation, commit,
/// announcement, and terminal path as the pointer route — a second
/// keyboard-only callback would be a second transaction.
pub trait CrossWindowDragTargetBridge {
    fn capabilities(&self) -> CrossWindowDragCapabilities;

    fn subscribe(&self, listener: Box<dyn Fn(CrossWindowDragTargetEvent)>) -> Box<dyn FnOnce()>;

    fn commit(
        &self,
        request: CrossWindowDragCommitRequest,
        complete: Box<dyn FnOnce(DragDropCommitResult)>,
    );

    /// The accessible cross-window route. `None` when the host advertises no
    /// keyboard picker; a `Some` implementation must be present exactly when
    /// [`CrossWindowDragCapabilities::keyboard_target_picker`] is true.
    fn pick_target(&self, _complete: Box<dyn FnOnce(Option<CrossWindowDragProjection>)>) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn receipt(version: u32, token: &str) -> CrossWindowDragReceipt {
        CrossWindowDragReceipt {
            protocol_version: version,
            token: token.to_string(),
        }
    }

    /// The receipt is the whole wire, so its bounds are the whole wire's
    /// bounds: a future version, an empty token, and an oversized token are
    /// all refused rather than best-effort accepted.
    #[test]
    fn a_receipt_is_valid_only_at_this_protocol_version_and_within_token_bounds() {
        assert!(receipt(CROSS_WINDOW_DRAG_PROTOCOL_VERSION, "opaque").is_valid());
        assert!(!receipt(CROSS_WINDOW_DRAG_PROTOCOL_VERSION + 1, "opaque").is_valid());
        assert!(!receipt(CROSS_WINDOW_DRAG_PROTOCOL_VERSION, "").is_valid());
        assert!(!receipt(
            CROSS_WINDOW_DRAG_PROTOCOL_VERSION,
            &"t".repeat(CROSS_WINDOW_DRAG_MAX_TOKEN_LENGTH + 1)
        )
        .is_valid());
    }

    /// Two receipts naming one transaction compare by their whole value, so a
    /// token reused under a different protocol version is a different receipt.
    #[test]
    fn receipts_compare_by_version_and_token_together() {
        assert_eq!(receipt(1, "a"), receipt(1, "a"));
        assert_ne!(receipt(1, "a"), receipt(1, "b"));
        assert_ne!(receipt(1, "a"), receipt(2, "a"));
    }
}
