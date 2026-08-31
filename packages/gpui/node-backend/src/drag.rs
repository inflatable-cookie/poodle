//! The public GPUI drag-and-drop controller (g16.025).
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//!
//! One [`DragDropController`] owns one semantic drag session. It translates
//! stock crates.io GPUI 0.2.2 input into the shared Rust kernel
//! (`poodle_headless::drag_drop`) and executes the kernel's effect intents:
//! nothing here re-decides lifecycle, arbitration, or exactly-once cleanup.
//!
//! ## Why the session lives on the controller and not on the backend
//!
//! The previous payload path kept one thread-local session for the whole
//! backend. Two mounted surfaces in one window therefore shared it, and the
//! second pickup silently ended the first. A controller is an ordinary value a
//! host constructs, so two providers own two sessions and neither can reach
//! the other's sources, targets, bounds, or intent.
//!
//! ## What stock GPUI actually gives us
//!
//! - `on_drag` fires once past GPUI's own drag threshold — the pointer start.
//! - `on_drag_move::<T>` runs in the **capture** phase with no hitbox test, so
//!   one listener on the provider host receives every move for the lifetime of
//!   the gesture, inside or outside the source. That is the observable result
//!   pointer capture exists to produce, which is why
//!   [`NodeDragCapabilities::in_window_capture`] is true.
//! - `on_mouse_up` / `on_mouse_up_out` close release on both sides of the host.
//! - Key events reach the host through the focus dispatch path.
//!
//! GPUI 0.2.2 exposes no touch contact, no pen identity, and no
//! device-originated pointer-cancel event. Those capabilities are published as
//! `false` and no amount of synthesized mouse input may flip them: a mouse
//! fixture is mouse evidence.
//!
//! ## Ordering
//!
//! Hit testing reads bounds this controller recorded itself, in the paint
//! pass, keyed by the registration's own `target_id`. Arbitration therefore
//! never depends on the order GPUI happened to dispatch per-element listeners
//! in, and one move produces exactly one resolved intent.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use gpui::{
    canvas, div, px, AnyView, App, AppContext, Bounds, Div, InteractiveElement, IntoElement,
    KeyDownEvent, MouseButton, MouseUpEvent, ParentElement, Pixels, Point, Render,
    StatefulInteractiveElement, Styled, Window,
};
use poodle_headless::drag_drop::{
    drag_session_transition, resolve_drop_target, DragAnnouncementKind,
    DragOperation, DragSession, DragSessionContext, DragSessionEffect, DragSessionEvent,
    DragSessionPhase, DragSubject, DragTerminalOutcome, DropEligibility, DropIntent, DropPosition,
    DropTargetCandidate,
};
use poodle_node::{
    Node, NodeDragCapabilities, NodeDragInputKind, NodeDragSource, NodeDropCommit,
    NodeDropCommitEvent, NodeDropIntentEvent, NodeDropPositionInput, NodeDropTarget,
    NodeKeyboardDropDirection, NodeKeyboardPositionInput,
};

/// What crates.io GPUI 0.2.2 certifies for this transport.
///
/// This is the whole capability claim and it is a constant, not a runtime
/// probe: a value that could be recomputed could also be talked into being
/// true. `pen`, `touch`, and `device_cancel` are named active-runtime debt in
/// architecture 011 — the crate does not deliver those events at all.
pub const GPUI_DRAG_CAPABILITIES: NodeDragCapabilities = NodeDragCapabilities {
    mouse: true,
    pen: false,
    touch: false,
    keyboard: true,
    in_window_capture: true,
    device_cancel: false,
};

/// How many composed announcements a controller keeps.
///
/// Announcements are presentation text: the live one is on the snapshot, and
/// the tail exists for diagnosis and tests. GPUI ships no accessibility API,
/// so there is no assistive technology to flood and nothing to throttle — but
/// a controller lives as long as its host, so the log still has to be bounded.
pub const ANNOUNCEMENT_LOG_LIMIT: usize = 64;

/// The value stock GPUI carries for the duration of a pointer drag.
///
/// `controller` scopes it: `on_drag_move` is dispatched by payload *type*, so
/// every provider in the window hears every move and each one must be able to
/// tell whether the gesture is its own.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativeDragPayload {
    pub controller: u64,
    pub source_id: String,
}

/// How the target under the gesture is answering.
///
/// `Rejected` is not the same as no target, and a custom surface has to be
/// able to tell them apart: one draws a refusal with its reason, the other
/// draws nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragDropTargetPosture {
    Accepted,
    Rejected,
}

/// An immutable presentation read of the controller's current session.
///
/// Snapshots expose semantics plus adapter-owned pointer state. They never
/// expose registries, elements, entities, bounds, or handlers.
#[derive(Clone, Debug, PartialEq)]
pub struct DragDropSnapshot {
    pub phase: DragSessionPhase,
    pub session_id: Option<String>,
    pub source_id: Option<String>,
    pub subject: Option<DragSubject>,
    pub operation: Option<DragOperation>,
    /// The accepted intent's target while one exists, otherwise the target
    /// currently refusing the gesture. Read it with [`Self::target_posture`].
    pub target_id: Option<String>,
    /// `Some` only when [`Self::target_posture`] is
    /// [`DragDropTargetPosture::Accepted`]: a refusal has no placement.
    pub position: Option<DropPosition>,
    pub target_posture: Option<DragDropTargetPosture>,
    /// The refusing target's reason, suitable for presentation. Cleared the
    /// moment an intent is accepted.
    pub rejected_reason: Option<String>,
    pub input_kind: Option<NodeDragInputKind>,
    pub pointer: Option<(f32, f32)>,
    pub announcement: Option<String>,
}

/// The subset a custom preview renderer receives.
#[derive(Clone, Debug, PartialEq)]
pub struct DragPreviewSnapshot {
    pub source_id: String,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub label: String,
}

/// A read-only description of one announcement, for hosts that want their own
/// wording. It is not a second lifecycle callback.
#[derive(Clone, Debug, PartialEq)]
pub struct DragAnnouncementEvent {
    pub kind: DragAnnouncementKind,
    pub source_label: String,
    pub target_label: Option<String>,
    pub position: Option<DropPosition>,
    pub operation: Option<DragOperation>,
    pub reason: Option<String>,
}

type PreviewRenderer = Rc<dyn Fn(&DragPreviewSnapshot, &mut Window, &mut App) -> AnyView>;
type AnnouncementRenderer = Rc<dyn Fn(&DragAnnouncementEvent) -> Option<String>>;

struct SourceRecord {
    registration: NodeDragSource,
    element_id: String,
    generation: u64,
    order: i32,
}

struct TargetRecord {
    registration: NodeDropTarget,
    generation: u64,
    order: i32,
    depth: i32,
    /// Last painted rectangle. `None` until this target has painted once —
    /// an unmeasured target cannot contain a point and never wins.
    bounds: Option<Bounds<Pixels>>,
}

/// What the controller last told a target it was holding, so exactly one
/// target is ever "current" and the previous one is always told it stopped.
type NotifiedIntent = (String, DropPosition, DragOperation);

struct ControllerState {
    id: u64,
    phase: DragSessionPhase,
    context: DragSessionContext,
    sources: HashMap<String, SourceRecord>,
    targets: HashMap<String, TargetRecord>,
    /// Nesting depth per target id, rebuilt from the node tree each frame.
    depths: HashMap<String, i32>,
    generation: u64,
    order_counter: i32,
    input_kind: Option<NodeDragInputKind>,
    pointer: Option<(f32, f32)>,
    notified: Option<NotifiedIntent>,
    /// The clear callback of the target currently holding the intent, held
    /// here rather than looked up on demand. The registry is rebuilt every
    /// frame, so a target that was removed while it held the intent would
    /// otherwise never be told it stopped — the one case the public
    /// registration contract promises it will be.
    notified_clear: Option<poodle_node::NodeDropIntentClearedHandler>,
    announcement: Option<String>,
    /// A bounded tail of composed announcements, for diagnosis and tests.
    /// Bounded because a controller lives as long as its host: an unbounded
    /// log would grow for every hover of every drag for the life of the app.
    announcements: VecDeque<String>,
    /// The target refusing the gesture right now, and why. Set only while no
    /// intent is accepted — an accepted intent is the answer, and a stale
    /// refusal beside it would let a surface paint both at once.
    rejected: Option<(String, Option<String>)>,
    /// Duplicate live ids and other registration conflicts, for diagnosis.
    conflicts: Vec<String>,
    /// The source registration the live session started from, plus its element
    /// id. Held for the session's lifetime because the registry is rebuilt
    /// every frame: a host that removes the dragged row still has to receive
    /// its own terminal callback and get focus back.
    active_source: Option<(NodeDragSource, String)>,
    last_outcome: Option<DragTerminalOutcome>,
    next_session: u64,
    keyboard_index: Option<usize>,
    /// The active source's `keyboard_order` — the traversal origin. The first
    /// step lands on the nearest target past it in the chosen direction, not
    /// on the end of the registry.
    keyboard_origin: Option<i32>,
    /// Activation keys handled on the way down, whose matching key-up must
    /// suppress GPUI's Enter/Space click synthesis. Otherwise a keyboard
    /// pickup also activates the focused row it picked up.
    ///
    /// A set keyed by the released key, not one flag: any *other* key-up
    /// arriving first — a modifier, a neighbouring shortcut, an overlapping
    /// Enter while Space is held — would consume a single flag and let the
    /// real release through.
    suppress_activation: std::collections::BTreeSet<String>,
    /// A terminal ran without a window in reach; the next windowed handler
    /// clears GPUI's own drag state.
    pending_stop_active_drag: bool,
    preview: Option<PreviewRenderer>,
    describe_announcement: Option<AnnouncementRenderer>,
}

/// One provider's drag session, registries, and native input translation.
///
/// Cheap to clone: every clone is the same controller, the way a handle is.
#[derive(Clone)]
pub struct DragDropController {
    state: Rc<RefCell<ControllerState>>,
}

impl Default for DragDropController {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    /// Controllers currently building their subtree. Innermost wins, so a
    /// provider nested inside another provider claims its own registrations.
    static PROVIDER_STACK: RefCell<Vec<DragDropController>> = const { RefCell::new(Vec::new()) };
    /// The frame's top-level provider, current for the whole frame.
    static FRAME_CONTROLLER: RefCell<Option<DragDropController>> = const { RefCell::new(None) };
    /// The window host whose root is currently being built.
    ///
    /// A stack, not a value, only so a malformed nest cannot strand the
    /// entry. It is scoped strictly to one `drag_drop_window_host` call and
    /// holds no census of its own: the census lives on the host, which is what
    /// keeps one window's frame from ever reaching another window's
    /// controllers.
    static WINDOW_HOST_STACK: RefCell<Vec<DragDropWindowHost>> = const { RefCell::new(Vec::new()) };
    static NEXT_CONTROLLER_ID: std::cell::Cell<u64> = const { std::cell::Cell::new(1) };
}

/// The controller a registration made right now belongs to.
///
/// The provider stack is the precise answer while a provider's own closure is
/// running, and nesting resolves innermost-first. But GPUI renders a
/// `RenderOnce` child or a list row during *layout*, after every provider
/// closure has returned, so a host that composes its page out of component
/// elements rather than one inline conversion would otherwise register
/// nothing. The frame's top-level provider therefore stays current until its
/// sweep runs.
///
/// The boundary that follows: a provider that needs its registrations isolated
/// from a sibling must convert its node trees **inside its own closure**.
/// Anything a nested provider renders lazily belongs to the frame's top-level
/// provider, because by then nothing distinguishes it from the rest of the
/// page.
pub(crate) fn current_controller() -> Option<DragDropController> {
    PROVIDER_STACK
        .with(|stack| stack.borrow().last().cloned())
        .or_else(|| FRAME_CONTROLLER.with(|current| current.borrow().clone()))
}

impl DragDropController {
    pub fn new() -> Self {
        let id = NEXT_CONTROLLER_ID.with(|next| {
            let id = next.get();
            next.set(id + 1);
            id
        });
        Self {
            state: Rc::new(RefCell::new(ControllerState {
                id,
                phase: DragSessionPhase::Idle,
                context: DragSessionContext::default(),
                sources: HashMap::new(),
                targets: HashMap::new(),
                depths: HashMap::new(),
                generation: 0,
                order_counter: 0,
                input_kind: None,
                pointer: None,
                notified: None,
                notified_clear: None,
                announcement: None,
                announcements: VecDeque::new(),
                rejected: None,
                conflicts: Vec::new(),
                active_source: None,
                last_outcome: None,
                next_session: 0,
                keyboard_index: None,
                keyboard_origin: None,
                suppress_activation: std::collections::BTreeSet::new(),
                pending_stop_active_drag: false,
                preview: None,
                describe_announcement: None,
            })),
        }
    }

    /// What this runtime actually supports. Immutable by construction.
    pub fn capabilities(&self) -> NodeDragCapabilities {
        GPUI_DRAG_CAPABILITIES
    }

    /// Stable identity, carried on [`NativeDragPayload`] so a provider can
    /// recognise its own gesture.
    pub fn id(&self) -> u64 {
        self.state.borrow().id
    }

    /// Render a custom drag preview instead of the empty default.
    pub fn set_preview(
        &self,
        preview: impl Fn(&DragPreviewSnapshot, &mut Window, &mut App) -> AnyView + 'static,
    ) {
        self.state.borrow_mut().preview = Some(Rc::new(preview));
    }

    /// Replace the default announcement wording. Returning `None` keeps the
    /// default for that observation.
    pub fn set_announcement_description(
        &self,
        describe: impl Fn(&DragAnnouncementEvent) -> Option<String> + 'static,
    ) {
        self.state.borrow_mut().describe_announcement = Some(Rc::new(describe));
    }

    pub fn snapshot(&self) -> DragDropSnapshot {
        let state = self.state.borrow();
        let session = state.context.session.as_ref();
        let intent = session.and_then(|s| s.intent.as_ref());
        let dragging = state.phase == DragSessionPhase::Dragging;
        // A refusal is only current while the gesture is live and nothing has
        // been accepted, matching `createDragDropController`'s
        // `targetPosture` rule exactly.
        let rejected = state.rejected.as_ref().filter(|_| dragging && intent.is_none());
        let target_posture = match (dragging, intent.is_some(), rejected.is_some()) {
            (true, true, _) => Some(DragDropTargetPosture::Accepted),
            (true, false, true) => Some(DragDropTargetPosture::Rejected),
            _ => None,
        };

        DragDropSnapshot {
            phase: state.phase,
            session_id: session.map(|s| s.session_id.clone()),
            source_id: session.map(|s| s.source_id.clone()),
            subject: session.map(|s| s.subject.clone()),
            operation: session.map(|s| s.operation),
            target_id: intent
                .map(|intent| intent.target_id.clone())
                .or_else(|| rejected.map(|(target_id, _)| target_id.clone())),
            position: intent.map(|intent| intent.position.clone()),
            target_posture,
            rejected_reason: rejected.and_then(|(_, reason)| reason.clone()),
            input_kind: state.input_kind,
            pointer: state.pointer,
            announcement: state.announcement.clone(),
        }
    }

    /// The most recent announcements this controller composed, oldest first.
    ///
    /// A bounded tail, not a transcript: see [`ANNOUNCEMENT_LOG_LIMIT`].
    pub fn announcements(&self) -> Vec<String> {
        self.state.borrow().announcements.iter().cloned().collect()
    }

    /// Registration conflicts — duplicate live source or target ids. The
    /// contract calls these errors rather than last-writer-wins; a render pass
    /// is the wrong place to panic, so they are recorded and the later
    /// registration is refused.
    pub fn conflicts(&self) -> Vec<String> {
        self.state.borrow().conflicts.clone()
    }

    /// Live registered source ids, in registration order.
    pub fn source_ids(&self) -> Vec<String> {
        let state = self.state.borrow();
        let mut ids: Vec<_> = state
            .sources
            .values()
            .map(|record| (record.order, record.registration.source_id.clone()))
            .collect();
        ids.sort();
        ids.into_iter().map(|(_, id)| id).collect()
    }

    /// Live registered target ids, in registration order.
    pub fn target_ids(&self) -> Vec<String> {
        let state = self.state.borrow();
        let mut ids: Vec<_> = state
            .targets
            .values()
            .map(|record| (record.order, record.registration.target_id.clone()))
            .collect();
        ids.sort();
        ids.into_iter().map(|(_, id)| id).collect()
    }

    /// Cancel an active session explicitly (a host "cancel move" command).
    pub fn cancel(&self, cx: &mut App) {
        if let Some(session_id) = self.active_session_id() {
            self.dispatch(DragSessionEvent::Cancel { session_id }, cx);
        }
    }

    // ── Frame boundary ─────────────────────────────────────────────────────

    /// Start a build pass: registrations made from here until [`Self::frame_end`]
    /// are this frame's live set.
    fn frame_begin(&self) {
        let mut state = self.state.borrow_mut();
        state.generation += 1;
        state.order_counter = 0;
        state.depths.clear();
        state.conflicts.clear();
    }

    /// Close a build pass: anything not re-registered is gone.
    ///
    /// A vanished source cancels the session and a vanished current target
    /// takes the kernel's `TargetLost` path — that is how host rebuild and
    /// removal are handled, rather than a second removal lifecycle. A
    /// surviving session then re-resolves against the new registrations, so an
    /// eligibility change with no pointer motion still moves the intent.
    fn frame_end(&self, window: &mut Window, cx: &mut App) {
        FRAME_CONTROLLER.with(|current| {
            let mut current = current.borrow_mut();
            if current
                .as_ref()
                .is_some_and(|claimed| Rc::ptr_eq(&claimed.state, &self.state))
            {
                *current = None;
            }
        });
        let (stale_source, stale_target) = {
            let mut state = self.state.borrow_mut();
            let generation = state.generation;
            state
                .sources
                .retain(|_, record| record.generation == generation);
            state
                .targets
                .retain(|_, record| record.generation == generation);

            let session = state.context.session.clone();
            // Removed, newly disabled, or now carrying a different subject.
            // The registration contract calls all three a changed source, and
            // the third matters most: a rebuild that reuses one `source_id`
            // for a new row would otherwise leave the old subject dragging and
            // let it commit against the new tree.
            let stale_source = session
                .as_ref()
                .filter(|session| {
                    state.sources.get(&session.source_id).is_none_or(|record| {
                        record.registration.disabled
                            || record.registration.subject != session.subject
                    })
                })
                .map(|session| session.session_id.clone());
            let stale_target = session
                .as_ref()
                .and_then(|session| {
                    session
                        .intent
                        .as_ref()
                        .map(|intent| (session.session_id.clone(), intent.target_id.clone()))
                })
                .filter(|(_, target_id)| !state.targets.contains_key(target_id));
            (stale_source, stale_target)
        };

        if !self.is_active() {
            self.sync_intent_notifications();
            self.drain_pending_stop(window, cx);
            return;
        }
        if let Some(session_id) = stale_source {
            self.dispatch(DragSessionEvent::SourceLost { session_id }, cx);
        } else if let Some((session_id, target_id)) = stale_target {
            self.dispatch(
                DragSessionEvent::TargetLost {
                    session_id,
                    target_id,
                },
                cx,
            );
        } else {
            // Same pointer, new registrations: an eligibility flip or a moved
            // row must change the intent without waiting for another move.
            let pointer = self.state.borrow().pointer;
            if let Some((x, y)) = pointer {
                self.resolve_pointer_intent(x, y, cx);
            }
        }
        self.sync_intent_notifications();
        self.drain_pending_stop(window, cx);
    }

    // ── Registration ───────────────────────────────────────────────────────

    fn register_source(&self, registration: &NodeDragSource, element_id: &str) -> bool {
        let mut state = self.state.borrow_mut();
        let generation = state.generation;
        if state
            .sources
            .get(&registration.source_id)
            .is_some_and(|record| record.generation == generation)
        {
            let id = registration.source_id.clone();
            state
                .conflicts
                .push(format!("duplicate live drag source id `{id}`"));
            return false;
        }
        let order = state.order_counter;
        state.order_counter += 1;
        state.sources.insert(
            registration.source_id.clone(),
            SourceRecord {
                registration: registration.clone(),
                element_id: element_id.to_owned(),
                generation,
                order,
            },
        );
        true
    }

    fn register_target(&self, registration: &NodeDropTarget) -> bool {
        let mut state = self.state.borrow_mut();
        let generation = state.generation;
        if state
            .targets
            .get(&registration.target_id)
            .is_some_and(|record| record.generation == generation)
        {
            let id = registration.target_id.clone();
            state
                .conflicts
                .push(format!("duplicate live drop target id `{id}`"));
            return false;
        }
        let order = state.order_counter;
        state.order_counter += 1;
        let depth = state
            .depths
            .get(&registration.target_id)
            .copied()
            .unwrap_or(0);
        // Geometry survives a rebuild: a target re-registered between paints
        // must not become unmeasured and lose the gesture mid-drag.
        let bounds = state
            .targets
            .get(&registration.target_id)
            .and_then(|record| record.bounds);
        state.targets.insert(
            registration.target_id.clone(),
            TargetRecord {
                registration: registration.clone(),
                generation,
                order,
                depth,
                bounds,
            },
        );
        true
    }

    /// Drop every registration this controller holds.
    ///
    /// Used only when the provider that owned them is gone: the tree that
    /// declared them will never build again, so keeping them would leave a
    /// registry describing elements that are not on screen.
    fn forget_registrations(&self) {
        let mut state = self.state.borrow_mut();
        state.sources.clear();
        state.targets.clear();
        state.depths.clear();
        state.conflicts.clear();
    }

    fn record_target_bounds(&self, target_id: &str, bounds: Bounds<Pixels>) {
        if let Some(record) = self.state.borrow_mut().targets.get_mut(target_id) {
            record.bounds = Some(bounds);
        }
    }

    /// Record nesting depth for this subtree, keeping the first answer of the
    /// frame.
    ///
    /// The backend converts each child through the public `to_gpui` entry, so
    /// this walk re-enters once per node with the depth reset to zero. The
    /// outermost walk is the one that saw the whole tree, and it runs first —
    /// so the first value recorded is the true depth and every later
    /// re-entry must leave it alone. Overwriting flattened every nested
    /// target to depth zero and handed arbitration to explicit priority.
    fn record_depths(&self, node: &Node, depth: i32) {
        let next = match &node.interaction.drop_target {
            Some(target) => {
                self.state
                    .borrow_mut()
                    .depths
                    .entry(target.target_id.clone())
                    .or_insert(depth);
                depth + 1
            }
            None => depth,
        };
        for child in &node.children {
            self.record_depths(child, next);
        }
    }

    // ── Native input translation ───────────────────────────────────────────

    fn is_active(&self) -> bool {
        matches!(
            self.state.borrow().phase,
            DragSessionPhase::Preparing
                | DragSessionPhase::Armed
                | DragSessionPhase::Dragging
                | DragSessionPhase::Dropping
        )
    }

    fn active_session_id(&self) -> Option<String> {
        if !self.is_active() {
            return None;
        }
        self.state
            .borrow()
            .context
            .session
            .as_ref()
            .map(|session| session.session_id.clone())
    }

    /// Open a session for a live enabled source.
    ///
    /// Preparation is synchronous on this transport — there is no host lease
    /// to await — but the semantic `Prepare → Prepared → Activate` steps still
    /// run, so a later cross-window bridge slots into the same lifecycle
    /// rather than growing a second one.
    fn begin_session(&self, source_id: &str, kind: NodeDragInputKind, cx: &mut App) -> bool {
        let prepared = {
            let mut state = self.state.borrow_mut();
            let Some(record) = state.sources.get(source_id) else {
                return false;
            };
            if record.registration.disabled {
                return false;
            }
            let registration = record.registration.clone();
            let element_id = record.element_id.clone();
            state.next_session += 1;
            let session_id = format!("gpui-drag-{}-{}", state.id, state.next_session);
            state.input_kind = Some(kind);
            state.keyboard_index = None;
            state.keyboard_origin = registration.keyboard_order;
            state.last_outcome = None;
            state.active_source = Some((registration.clone(), element_id));
            (session_id, registration)
        };
        let (session_id, registration) = prepared;

        self.dispatch(
            DragSessionEvent::Prepare {
                session_id: session_id.clone(),
                source_id: registration.source_id.clone(),
                subject: registration.subject.clone(),
                operation: registration.operation,
                allowed_operations: registration.allowed_operations.clone(),
            },
            cx,
        );
        if self.state.borrow().phase != DragSessionPhase::Preparing {
            return false;
        }
        self.dispatch(
            DragSessionEvent::Prepared {
                session_id: session_id.clone(),
            },
            cx,
        );
        self.dispatch(DragSessionEvent::Activate { session_id }, cx);
        self.state.borrow().phase == DragSessionPhase::Dragging
    }

    /// A pointer move for this controller's session: hit test, arbitrate, and
    /// report at most one intent.
    fn pointer_move(&self, position: Point<Pixels>, window: &mut Window, cx: &mut App) {
        self.drain_pending_stop(window, cx);
        if self.state.borrow().phase != DragSessionPhase::Dragging {
            return;
        }
        let (x, y): (f32, f32) = (position.x.into(), position.y.into());
        self.state.borrow_mut().pointer = Some((x, y));
        self.resolve_pointer_intent(x, y, cx);
        self.sync_intent_notifications();
        cx.refresh_windows();
    }

    fn resolve_pointer_intent(&self, x: f32, y: f32, cx: &mut App) {
        let (session, candidates) = {
            let state = self.state.borrow();
            if state.phase != DragSessionPhase::Dragging {
                return;
            }
            let Some(session) = state.context.session.clone() else {
                return;
            };
            let kind = state.input_kind.unwrap_or(NodeDragInputKind::Mouse);
            let candidates: Vec<DropTargetCandidate> = state
                .targets
                .values()
                .filter_map(|record| pointer_candidate(record, &session, x, y, kind))
                .collect();
            (session, candidates)
        };

        let resolved = resolve_drop_target(&candidates);
        // A refusal is presentation, not lifecycle: it never becomes an
        // intent, and it only stands while nothing was accepted.
        let refused = resolved
            .is_none()
            .then(|| resolve_rejected_target(&candidates))
            .flatten();
        self.state.borrow_mut().rejected = refused;
        self.apply_intent(&session, resolved, cx);
    }

    fn apply_intent(&self, session: &DragSession, intent: Option<DropIntent>, cx: &mut App) {
        match intent {
            Some(intent) if session.intent.as_ref() != Some(&intent) => self.dispatch(
                DragSessionEvent::TargetIntent {
                    session_id: session.session_id.clone(),
                    intent,
                },
                cx,
            ),
            None if session.intent.is_some() => self.dispatch(
                DragSessionEvent::TargetCleared {
                    session_id: session.session_id.clone(),
                },
                cx,
            ),
            _ => {}
        }
    }

    /// Release at a point. The point decides, not the last hover.
    ///
    /// A gesture can reach mouse-up without an intervening move — release
    /// outside the window, or a move coalesced away — and committing whatever
    /// the previous move happened to leave would drop on a target the pointer
    /// is no longer over. So the release position is hit-tested first, exactly
    /// like a move, and only then does a live intent commit.
    fn pointer_release(&self, position: Point<Pixels>, window: &mut Window, cx: &mut App) {
        self.drain_pending_stop(window, cx);
        let Some(session_id) = self.active_session_id() else {
            return;
        };
        if self.state.borrow().input_kind != Some(NodeDragInputKind::Mouse) {
            return;
        }
        let (x, y): (f32, f32) = (position.x.into(), position.y.into());
        self.state.borrow_mut().pointer = Some((x, y));
        self.resolve_pointer_intent(x, y, cx);

        let has_intent = self
            .state
            .borrow()
            .context
            .session
            .as_ref()
            .is_some_and(|session| session.intent.is_some());
        if has_intent {
            self.dispatch(DragSessionEvent::DropRequested { session_id }, cx);
        } else {
            self.dispatch(
                DragSessionEvent::Cancel {
                    session_id: session_id.clone(),
                },
                cx,
            );
        }
        self.sync_intent_notifications();
        self.drain_pending_stop(window, cx);
        cx.refresh_windows();
    }

    /// Keyboard: pickup on a focused opted-in source, traversal over the
    /// ordered keyboard registry, drop, and cancel. Every route creates the
    /// same semantic session as the pointer and reaches the same commit,
    /// announcement, focus-return, and terminal cleanup.
    fn key(&self, key: &str, window: &mut Window, cx: &mut App) -> bool {
        self.drain_pending_stop(window, cx);
        if !self.is_active() {
            return matches!(key, "space" | "enter") && self.keyboard_pickup(cx);
        }
        let Some(session_id) = self.active_session_id() else {
            return false;
        };
        // One sensor owns an open gesture. Escape is the exception on purpose:
        // it is the accessible cancel for *any* session, and a mouse drag a
        // user cannot abandon from the keyboard is a trap. Everything else —
        // traversal, drop — would otherwise let a keystroke move or commit a
        // drag the mouse is still holding.
        if self.state.borrow().input_kind != Some(NodeDragInputKind::Keyboard) {
            if key != "escape" {
                return false;
            }
            self.dispatch(DragSessionEvent::Escape { session_id }, cx);
            self.sync_intent_notifications();
            self.drain_pending_stop(window, cx);
            cx.refresh_windows();
            return true;
        }
        let handled = match key {
            "escape" => {
                self.dispatch(DragSessionEvent::Escape { session_id }, cx);
                true
            }
            "enter" | "space" => {
                let has_intent = self
                    .state
                    .borrow()
                    .context
                    .session
                    .as_ref()
                    .is_some_and(|session| session.intent.is_some());
                // The pickup key is also the drop key, so with no target
                // chosen it puts the row back down. Reporting the key handled
                // and leaving the drag open would strand a keyboard user in a
                // gesture their own pickup key could not close.
                if has_intent {
                    self.dispatch(DragSessionEvent::DropRequested { session_id }, cx);
                } else {
                    self.dispatch(DragSessionEvent::Cancel { session_id }, cx);
                }
                true
            }
            "up" | "left" => self.keyboard_step(NodeKeyboardDropDirection::Previous, cx),
            "down" | "right" => self.keyboard_step(NodeKeyboardDropDirection::Next, cx),
            "home" => self.keyboard_step(NodeKeyboardDropDirection::First, cx),
            "end" => self.keyboard_step(NodeKeyboardDropDirection::Last, cx),
            _ => false,
        };
        self.sync_intent_notifications();
        self.drain_pending_stop(window, cx);
        cx.refresh_windows();
        handled
    }

    fn keyboard_pickup(&self, cx: &mut App) -> bool {
        let focused = {
            let state = self.state.borrow();
            let mut candidates: Vec<_> = state
                .sources
                .values()
                .filter(|record| {
                    record.registration.keyboard_order.is_some()
                        && !record.registration.disabled
                        && crate::focus_state_for(&record.element_id) == Some(true)
                })
                .map(|record| (record.order, record.registration.source_id.clone()))
                .collect();
            candidates.sort();
            candidates.into_iter().next().map(|(_, id)| id)
        };
        let Some(source_id) = focused else {
            return false;
        };
        let started = self.begin_session(&source_id, NodeDragInputKind::Keyboard, cx);
        self.sync_intent_notifications();
        cx.refresh_windows();
        started
    }

    fn keyboard_step(&self, direction: NodeKeyboardDropDirection, cx: &mut App) -> bool {
        let (session, target_id, position) = {
            let mut state = self.state.borrow_mut();
            let Some(session) = state.context.session.clone() else {
                return false;
            };
            let mut ordered: Vec<(i32, i32, String)> = state
                .targets
                .values()
                .filter_map(|record| {
                    record
                        .registration
                        .keyboard_order
                        .filter(|_| record.registration.accepts(&session.subject))
                        .map(|order| (order, record.order, record.registration.target_id.clone()))
                })
                .collect();
            ordered.sort();
            if ordered.is_empty() {
                return false;
            }
            let last = ordered.len() - 1;
            // The first step is measured from the source's own
            // `keyboard_order` — the declared traversal origin — and it can
            // fail. A source past every target has nothing after it, so Next
            // selects nothing rather than wrapping backwards to the end; a
            // source before every target has nothing before it. Traversal also
            // stops at the ends instead of restating the current index. This
            // mirrors `createDragDropController`'s `firstTargetAfterSource` /
            // `firstTargetBeforeSource`, which return and leave the intent
            // alone.
            let origin = state.keyboard_origin;
            let first_after = || match origin {
                Some(origin) => ordered.iter().position(|(order, ..)| *order > origin),
                None => Some(0),
            };
            let last_before = || match origin {
                Some(origin) => ordered.iter().rposition(|(order, ..)| *order < origin),
                None => Some(last),
            };
            let index = match direction {
                NodeKeyboardDropDirection::First => Some(0),
                NodeKeyboardDropDirection::Last => Some(last),
                NodeKeyboardDropDirection::Next => match state.keyboard_index {
                    Some(current) if current >= last => None,
                    Some(current) => Some(current + 1),
                    None => first_after(),
                },
                NodeKeyboardDropDirection::Previous => match state.keyboard_index {
                    Some(0) => None,
                    Some(current) => Some(current - 1),
                    None => last_before(),
                },
            };
            // Handled — the key belongs to the open gesture — but there is
            // nowhere to go, so the current intent stands.
            let Some(index) = index else {
                return true;
            };
            state.keyboard_index = Some(index);
            let target_id = ordered[index].2.clone();
            let position = state
                .targets
                .get(&target_id)
                .and_then(|record| record.registration.resolve_keyboard_position.clone())
                .and_then(|resolve| {
                    resolve(&NodeKeyboardPositionInput {
                        direction,
                        subject: session.subject.clone(),
                        operation: session.operation,
                    })
                });
            (session, target_id, position)
        };

        let Some(position) = position else {
            self.state.borrow_mut().rejected = None;
            self.apply_intent(&session, None, cx);
            return true;
        };
        let intent = DropIntent {
            target_id: target_id.clone(),
            position,
            operation: session.operation,
        };
        let eligible = {
            let state = self.state.borrow();
            state
                .targets
                .get(&target_id)
                .map(|record| eligibility_for(&record.registration, &intent, &session.subject))
        };
        match eligible {
            Some(DropEligibility::Accepted { intent }) => {
                self.state.borrow_mut().rejected = None;
                self.apply_intent(&session, Some(intent), cx);
            }
            Some(DropEligibility::Rejected { reason }) => {
                self.state.borrow_mut().rejected = Some((target_id, reason));
                self.apply_intent(&session, None, cx);
            }
            None => {
                self.state.borrow_mut().rejected = None;
                self.apply_intent(&session, None, cx);
            }
        }
        true
    }

    /// GPUI's own drag state outlives a terminal that had no window in reach
    /// (a rebuild sweep runs on `App` alone). The next windowed handler clears
    /// it, so a cancelled gesture cannot keep painting a preview.
    fn drain_pending_stop(&self, window: &mut Window, cx: &mut App) {
        let pending = {
            let mut state = self.state.borrow_mut();
            std::mem::take(&mut state.pending_stop_active_drag)
        };
        if pending {
            cx.stop_active_drag(window);
        }
    }

    // ── Kernel ─────────────────────────────────────────────────────────────

    /// Run one event and everything it causes.
    ///
    /// Effects are executed **outside** the state borrow and anything they
    /// produce is queued rather than dispatched re-entrantly: a `RequestDrop`
    /// calls consumer code that may rebuild the host, and a nested
    /// `borrow_mut` there would panic.
    fn dispatch(&self, event: DragSessionEvent, cx: &mut App) {
        let mut queue = VecDeque::from([event]);
        while let Some(event) = queue.pop_front() {
            let effects = {
                let mut state = self.state.borrow_mut();
                let (phase, context, effects) =
                    drag_session_transition(state.phase, state.context.clone(), event);
                state.phase = phase;
                state.context = context;
                effects
            };
            for effect in effects {
                self.run_effect(effect, &mut queue, cx);
            }
        }
    }

    fn run_effect(
        &self,
        effect: DragSessionEffect,
        queue: &mut VecDeque<DragSessionEvent>,
        cx: &mut App,
    ) {
        match effect {
            // No host lease on this transport; the semantic step exists so the
            // cross-window bridge has somewhere to land.
            DragSessionEffect::PrepareSession { .. } => {}
            DragSessionEffect::EmitDragStart { .. } => {
                let (handler, session) = {
                    let state = self.state.borrow();
                    (
                        state
                            .active_source
                            .as_ref()
                            .and_then(|(source, _)| source.on_drag_start.clone()),
                        state.context.session.clone(),
                    )
                };
                if let (Some(handler), Some(session)) = (handler, session) {
                    handler(&session);
                }
            }
            DragSessionEffect::RequestDrop { session_id, intent } => {
                queue.push_back(self.commit(&session_id, intent));
            }
            DragSessionEffect::EmitDropResult { outcome, .. } => {
                let handler = {
                    let mut state = self.state.borrow_mut();
                    state.last_outcome = Some(outcome.clone());
                    state
                        .active_source
                        .as_ref()
                        .and_then(|(source, _)| source.on_drag_end.clone())
                };
                if let Some(handler) = handler {
                    handler(&outcome);
                }
            }
            DragSessionEffect::Announce { kind } => self.announce(kind),
            DragSessionEffect::ReturnFocus { .. } => {
                let element = {
                    let state = self.state.borrow();
                    state
                        .active_source
                        .as_ref()
                        .map(|(_, element_id)| element_id.clone())
                        .filter(|id| !id.is_empty())
                        // Focus only returns to a source that is still mounted;
                        // a removed row has nothing to focus.
                        .filter(|id| crate::focus_handle_for(id).is_some())
                };
                if let Some(element) = element {
                    crate::request_focus(&element);
                }
            }
            DragSessionEffect::CleanupSession { session_id } => {
                self.clear_intent_notification();
                {
                    let mut state = self.state.borrow_mut();
                    state.input_kind = None;
                    state.pointer = None;
                    state.keyboard_index = None;
                    state.keyboard_origin = None;
                    state.active_source = None;
                    state.rejected = None;
                    state.pending_stop_active_drag = true;
                }
                queue.push_back(DragSessionEvent::Reset { session_id });
                cx.refresh_windows();
            }
        }
    }

    /// Revalidate, then ask the target to commit. Eligibility is checked again
    /// here because hover acceptance never authorizes durable mutation.
    fn commit(&self, session_id: &str, intent: DropIntent) -> DragSessionEvent {
        let (registration, subject) = {
            let state = self.state.borrow();
            let subject = state
                .context
                .session
                .as_ref()
                .map(|session| session.subject.clone());
            (
                state
                    .targets
                    .get(&intent.target_id)
                    .map(|record| record.registration.clone()),
                subject,
            )
        };
        let session_id = session_id.to_owned();
        let (Some(registration), Some(subject)) = (registration, subject) else {
            return DragSessionEvent::DropRejected {
                session_id,
                reason: Some("The drop target is no longer registered".to_string()),
            };
        };

        let revalidated = match eligibility_for(&registration, &intent, &subject) {
            DropEligibility::Accepted { intent } => intent,
            DropEligibility::Rejected { reason } => {
                return DragSessionEvent::DropRejected { session_id, reason }
            }
        };
        let Some(on_drop) = registration.on_drop.clone() else {
            return DragSessionEvent::DropRejected {
                session_id,
                reason: Some("The drop target has no commit handler".to_string()),
            };
        };

        match on_drop(&NodeDropCommitEvent {
            subject,
            intent: revalidated.clone(),
        }) {
            NodeDropCommit::Committed => DragSessionEvent::DropCommitted {
                session_id,
                intent: revalidated,
            },
            NodeDropCommit::Rejected { reason } => {
                DragSessionEvent::DropRejected { session_id, reason }
            }
            NodeDropCommit::Failed { reason } => DragSessionEvent::DropFailed { session_id, reason },
        }
    }

    // ── Target posture ─────────────────────────────────────────────────────

    /// Tell at most one target it holds the current intent, and always tell
    /// the previous one it stopped.
    fn sync_intent_notifications(&self) {
        let (clear, notify) = {
            let mut state = self.state.borrow_mut();
            let current = state
                .context
                .session
                .as_ref()
                .filter(|_| state.phase == DragSessionPhase::Dragging)
                .and_then(|session| {
                    session.intent.as_ref().map(|intent| {
                        (
                            (
                                intent.target_id.clone(),
                                intent.position.clone(),
                                intent.operation,
                            ),
                            session.subject.clone(),
                        )
                    })
                });
            let next = current.as_ref().map(|(key, _)| key.clone());
            if state.notified == next {
                return;
            }
            let previous = state.notified.take();
            let clear = match (&previous, &next) {
                (Some((previous_id, ..)), Some((next_id, ..))) if previous_id == next_id => {
                    // Same target, new position: it is still holding the
                    // intent, so it is not told it stopped.
                    state.notified_clear.clone()
                }
                (Some(_), _) => state.notified_clear.take(),
                _ => None,
            };
            let clear = match (&previous, &next) {
                (Some((previous_id, ..)), Some((next_id, ..))) if previous_id == next_id => None,
                _ => clear,
            };
            let notify = current.and_then(|((target_id, position, operation), subject)| {
                let record = state.targets.get(&target_id)?;
                let cleared = record.registration.on_intent_cleared.clone();
                let intent = record.registration.on_intent.clone();
                Some((cleared, intent, position, operation, subject))
            });
            let notify = notify.map(|(cleared, intent, position, operation, subject)| {
                state.notified_clear = cleared;
                (
                    intent,
                    NodeDropIntentEvent {
                        subject,
                        position,
                        operation,
                    },
                )
            });
            if notify.is_none() {
                state.notified_clear = None;
            }
            state.notified = next;
            (clear, notify)
        };
        if let Some(clear) = clear {
            clear();
        }
        if let Some((Some(handler), event)) = notify {
            handler(&event);
        }
    }

    /// Terminal cleanup: whatever holds the intent is told it stopped.
    ///
    /// The held callback is used rather than a registry lookup, because the
    /// frame sweep that noticed the removal has already pruned the target this
    /// is about.
    fn clear_intent_notification(&self) {
        let clear = {
            let mut state = self.state.borrow_mut();
            state.notified = None;
            state.notified_clear.take()
        };
        if let Some(clear) = clear {
            clear();
        }
    }

    // ── Announcements ──────────────────────────────────────────────────────

    fn announce(&self, kind: DragAnnouncementKind) {
        let (event, describe) = {
            let state = self.state.borrow();
            let Some(session) = state.context.session.as_ref() else {
                return;
            };
            let source_label = state
                .active_source
                .as_ref()
                .map(|(source, _)| source.label.clone())
                .unwrap_or_else(|| session.subject.id.clone());
            let target = session.intent.as_ref().and_then(|intent| {
                state
                    .targets
                    .get(&intent.target_id)
                    .map(|record| record.registration.label.clone())
            });
            let reason = match state.last_outcome.as_ref() {
                Some(DragTerminalOutcome::Rejected { reason })
                | Some(DragTerminalOutcome::Failed { reason }) => reason.clone(),
                _ => None,
            };
            (
                DragAnnouncementEvent {
                    kind,
                    source_label,
                    target_label: target,
                    position: session.intent.as_ref().map(|intent| intent.position.clone()),
                    operation: Some(session.operation),
                    reason,
                },
                state.describe_announcement.clone(),
            )
        };

        let text = describe
            .and_then(|describe| describe(&event))
            .unwrap_or_else(|| default_announcement(&event));
        let mut state = self.state.borrow_mut();
        state.announcements.push_back(text.clone());
        while state.announcements.len() > ANNOUNCEMENT_LOG_LIMIT {
            state.announcements.pop_front();
        }
        state.announcement = Some(text);
    }

    // ── Host element ───────────────────────────────────────────────────────

    /// Attach this controller's window-level listeners to a host element.
    ///
    /// The move listener is capture-phase and hitbox-free, so it keeps
    /// receiving the gesture after the pointer leaves the source — the
    /// observable result pointer capture produces, without a per-pointer
    /// capture handle GPUI does not expose. Release is bound on both sides of
    /// the host so an outside release still closes the session, and it carries
    /// its own position: the release point decides, not the last hover.
    ///
    /// Keys are taken in the **capture** phase, outermost first, so a drag key
    /// is claimed before the focused row it is aimed at sees it. GPUI
    /// synthesizes a click from Enter/Space on *key-up* for any focused
    /// element with a click listener, so the matching key-up prevents that
    /// default — otherwise a keyboard pickup also activates the row it picked
    /// up.
    fn attach_host<E>(&self, el: E) -> E
    where
        E: InteractiveElement + 'static,
    {
        let moves = self.clone();
        let up = self.clone();
        let up_out = self.clone();
        let keys = self.clone();
        let key_ups = self.clone();
        let id = self.id();
        el.on_drag_move::<NativeDragPayload>(move |event, window, cx| {
            // `on_drag_move` dispatches by payload TYPE: every provider in the
            // window hears every drag. Only the controller whose own source
            // started this gesture may drive its session from these moves.
            let mine = event
                .dragged_item()
                .downcast_ref::<NativeDragPayload>()
                .is_some_and(|payload| payload.controller == id);
            if !mine {
                return;
            }
            moves.pointer_move(event.event.position, window, cx);
        })
        .on_mouse_up(MouseButton::Left, move |event: &MouseUpEvent, window, cx| {
            up.pointer_release(event.position, window, cx);
        })
        .on_mouse_up_out(MouseButton::Left, move |event: &MouseUpEvent, window, cx| {
            up_out.pointer_release(event.position, window, cx);
        })
        .capture_key_down(move |event: &KeyDownEvent, window, cx| {
            let key = event.keystroke.key.as_str();
            let handled = keys.key(key, window, cx);
            if handled && matches!(key, "space" | "enter") {
                keys.state
                    .borrow_mut()
                    .suppress_activation
                    .insert(key.to_owned());
            }
        })
        .capture_key_up(move |event: &gpui::KeyUpEvent, window, _cx| {
            let key = event.keystroke.key.as_str();
            let suppressed = key_ups.state.borrow_mut().suppress_activation.remove(key);
            if suppressed {
                window.prevent_default();
            }
        })
    }
}

/// One drop target's candidacy at a pointer position.
///
/// The kernel decides which candidate wins; this only measures. A target with
/// no painted bounds cannot contain the point, and a target with no position
/// resolver produces no intent at all.
fn pointer_candidate(
    record: &TargetRecord,
    session: &DragSession,
    x: f32,
    y: f32,
    input_kind: NodeDragInputKind,
) -> Option<DropTargetCandidate> {
    let bounds = record.bounds?;
    let left: f32 = bounds.origin.x.into();
    let top: f32 = bounds.origin.y.into();
    let width: f32 = bounds.size.width.into();
    let height: f32 = bounds.size.height.into();
    let contains_point =
        width > 0.0 && height > 0.0 && x >= left && x < left + width && y >= top && y < top + height;

    let resolve = record.registration.resolve_position.clone()?;
    let position = resolve(&NodeDropPositionInput {
        fraction_x: ((x - left) / width.max(1.0)).clamp(0.0, 1.0),
        fraction_y: ((y - top) / height.max(1.0)).clamp(0.0, 1.0),
        subject: session.subject.clone(),
        operation: session.operation,
        input_kind,
    })?;

    let intent = DropIntent {
        target_id: record.registration.target_id.clone(),
        position,
        operation: session.operation,
    };
    Some(DropTargetCandidate {
        target_id: record.registration.target_id.clone(),
        depth: record.depth,
        order: record.order,
        priority: record.registration.priority,
        contains_point,
        eligibility: eligibility_for(&record.registration, &intent, &session.subject),
    })
}

/// The refused target the pointer is over, by the same arbitration the
/// accepted set uses.
///
/// `resolve_drop_target` discards rejected candidates — correctly, because a
/// refusal must never become an intent. But a custom surface still has to be
/// able to style the target refusing it, so the refused set is handed to that
/// same resolver as if it were acceptable, and only the winner's identity and
/// reason are kept. Keeping a second copy of the deepest / priority / order
/// rule here is exactly the drift this substrate exists to remove, and the two
/// copies would disagree the first time one of them changed.
///
/// This is stricter than the web controller, which takes the first refusal in
/// registry iteration order. Deterministic beats incidental: with nested
/// targets, "whichever we happened to visit first" is not a rule a consumer
/// can rely on.
fn resolve_rejected_target(
    candidates: &[DropTargetCandidate],
) -> Option<(String, Option<String>)> {
    let refused: Vec<DropTargetCandidate> = candidates
        .iter()
        .filter(|candidate| matches!(candidate.eligibility, DropEligibility::Rejected { .. }))
        .map(|candidate| DropTargetCandidate {
            eligibility: DropEligibility::Accepted {
                intent: DropIntent {
                    target_id: candidate.target_id.clone(),
                    position: String::new(),
                    operation: DragOperation::Move,
                },
            },
            target_id: candidate.target_id.clone(),
            ..*candidate
        })
        .collect();

    let winner = resolve_drop_target(&refused)?.target_id;
    let reason = candidates
        .iter()
        .find(|candidate| candidate.target_id == winner)
        .and_then(|candidate| match &candidate.eligibility {
            DropEligibility::Rejected { reason } => reason.clone(),
            _ => None,
        });
    Some((winner, reason))
}

/// Kind filter, disabled posture, then the target's own resolver. Absent
/// `can_drop` accepts, matching the registration's documented default.
fn eligibility_for(
    registration: &NodeDropTarget,
    intent: &DropIntent,
    subject: &DragSubject,
) -> DropEligibility {
    if !registration.accepts(subject) {
        return DropEligibility::Rejected {
            reason: Some(format!("`{}` does not accept this item", registration.label)),
        };
    }
    match registration.can_drop.as_ref() {
        Some(can_drop) => can_drop(intent, subject),
        None => DropEligibility::Accepted {
            intent: intent.clone(),
        },
    }
}

fn default_announcement(event: &DragAnnouncementEvent) -> String {
    let source = &event.source_label;
    let placement = || match (&event.position, &event.target_label) {
        (Some(position), Some(target)) => format!("{position} {target}"),
        (_, Some(target)) => format!("on {target}"),
        _ => "no target".to_string(),
    };
    match event.kind {
        DragAnnouncementKind::Pickup => format!("Picked up {source}."),
        DragAnnouncementKind::IntentChanged => format!("{source}: {}.", placement()),
        DragAnnouncementKind::IntentCleared => format!("{source}: no drop target."),
        DragAnnouncementKind::Dropped => format!("Dropped {source} {}.", placement()),
        DragAnnouncementKind::Rejected => match &event.reason {
            Some(reason) => format!("{source} was not dropped. {reason}"),
            None => format!("{source} was not dropped."),
        },
        DragAnnouncementKind::Failed => match &event.reason {
            Some(reason) => format!("Moving {source} failed. {reason}"),
            None => format!("Moving {source} failed."),
        },
        DragAnnouncementKind::Cancelled => format!("Cancelled moving {source}."),
    }
}

/// The dragged-item view GPUI requires at drag start.
///
/// Empty unless the controller was given a preview renderer: components draw
/// their own drop indicator from the intent the controller reports, and the
/// substrate must never move the real source node out of layout.
struct NodeDragPreview {
    view: Option<AnyView>,
}

impl Render for NodeDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().children(self.view.clone())
    }
}


// ── Window host ────────────────────────────────────────────────────────────

/// One window's census of the drag controllers mounted inside it.
///
/// A [`DragDropController`] can only close a session it holds during its own
/// per-frame sweep, and a provider that has been unmounted never sweeps again.
/// A host that removes a provider mid-drag would therefore leave a `Dragging`
/// session with live registrations, no terminal callback, and GPUI's own drag
/// still painting — the consumer's drag state latched with nothing left to
/// clear it. Spec 069 makes provider unmount a cancellation, and this is where
/// that cancellation can actually run.
///
/// **It is per window, and that is the whole design.** An earlier attempt used
/// a thread-global "did this controller sweep this frame" mark, which is
/// wrong: rendering window A resets and sweeps controllers owned by window B,
/// so a live drag in B is cancelled merely because B did not happen to render
/// during A's frame. A census that belongs to one host can only ever name that
/// host's own controllers, so A's frame has nothing of B's to look at.
///
/// Cheap to clone: every clone is the same host, the way a handle is.
#[derive(Clone)]
pub struct DragDropWindowHost {
    state: Rc<RefCell<WindowHostState>>,
}

#[derive(Default)]
struct WindowHostState {
    /// Controllers that registered during the frame currently being built.
    building: Vec<DragDropController>,
    /// Controllers present at the end of the last completed frame.
    census: Vec<DragDropController>,
}

impl Default for DragDropWindowHost {
    fn default() -> Self {
        Self::new()
    }
}

impl DragDropWindowHost {
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(WindowHostState::default())),
        }
    }

    /// How many controllers this window currently owns. Diagnosis and tests.
    pub fn census_len(&self) -> usize {
        self.state.borrow().census.len()
    }

    fn frame_begin(&self) {
        self.state.borrow_mut().building.clear();
    }

    /// Record a provider building inside this window's root.
    ///
    /// Idempotent per frame: a provider that renders twice in one frame is one
    /// mounted provider, not two.
    fn note(&self, controller: &DragDropController) {
        let mut state = self.state.borrow_mut();
        if state
            .building
            .iter()
            .any(|known| Rc::ptr_eq(&known.state, &controller.state))
        {
            return;
        }
        state.building.push(controller.clone());
    }

    /// Close the frame: anything in the census that did not build is gone.
    ///
    /// Runs from a zero-size paint canvas appended last at this window's root,
    /// so it reaches a `Window` as well as an `App`. That matters: a terminal
    /// reached with no provider left has no controller host to drain
    /// `pending_stop_active_drag`, so semantic idle and empty registries are
    /// not enough — GPUI's own active drag and preview have to be cleared
    /// here or nowhere.
    fn frame_end(&self, window: &mut Window, cx: &mut App) {
        let departed: Vec<DragDropController> = {
            let state = self.state.borrow();
            state
                .census
                .iter()
                .filter(|known| {
                    !state
                        .building
                        .iter()
                        .any(|live| Rc::ptr_eq(&live.state, &known.state))
                })
                .cloned()
                .collect()
        };

        for controller in departed {
            // Order matters and is the point: cancel through the kernel so the
            // consumer's terminal callback runs exactly once, drop the
            // registrations the vanished tree left behind, and only then stop
            // the runtime's own drag. Forgetting the controller first would
            // leave nothing to drain.
            controller.cancel(cx);
            controller.forget_registrations();
            controller.drain_pending_stop(window, cx);
        }

        let mut state = self.state.borrow_mut();
        state.census = std::mem::take(&mut state.building);
    }
}

/// The window host a provider building right now belongs to.
fn current_window_host() -> Option<DragDropWindowHost> {
    WINDOW_HOST_STACK.with(|stack| stack.borrow().last().cloned())
}

/// Establish this window's provider census and its end-of-frame sweep.
///
/// One per window, wrapped around the one root element — the same shape
/// `attach_overlay_host` already has. `build` returns the window's own root;
/// no wrapper is inserted, so the host never changes layout or hit testing.
///
/// The sweep rides a zero-size paint canvas appended last at the root, after
/// every provider's own canvas, so providers close their frames first and the
/// host then closes over whatever did not appear. It is not `App::defer`:
/// deferring reaches an `App` alone, and stopping GPUI's active drag needs a
/// `Window`.
pub fn drag_drop_window_host<E>(host: &DragDropWindowHost, build: impl FnOnce() -> E) -> E
where
    E: InteractiveElement + ParentElement + 'static,
{
    host.frame_begin();
    WINDOW_HOST_STACK.with(|stack| stack.borrow_mut().push(host.clone()));
    let root = build();
    WINDOW_HOST_STACK.with(|stack| {
        stack.borrow_mut().pop();
    });

    let sweep = host.clone();
    root.child(
        canvas(
            |_bounds, _window, _cx| {},
            move |_bounds, _prepaint, window, cx| sweep.frame_end(window, cx),
        )
        .absolute()
        .top(px(0.0))
        .left(px(0.0))
        .size(px(0.0)),
    )
}

/// Build a host element with `controller` current, then give it that
/// controller's window-level listeners and frame sweep.
///
/// `build` returns the host's *own* root element — no wrapper is inserted, so
/// a provider never changes the layout or hit-testing of the tree it manages.
/// Every drag source and drop target created inside `build` registers with
/// this controller and nothing else can reach them, so two providers in one
/// window own two independent sessions and a nested provider claims its own
/// registrations because the innermost one is current.
///
/// The frame boundary is the provider's own build: `build` runs between
/// [`DragDropController::frame_begin`] and the sweep, so a source or target
/// that did not re-register this frame is gone. The sweep rides a zero-size
/// paint canvas appended last rather than `App::defer`, for two reasons — it
/// reaches a `Window`, so a rebuild-driven cancellation can also stop GPUI's
/// own drag, and a host that wires the provider correctly cannot forget a
/// second call.
pub fn drag_drop_provider<E>(controller: &DragDropController, build: impl FnOnce() -> E) -> E
where
    E: InteractiveElement + ParentElement + 'static,
{
    // Join this window's census before building. A provider that never
    // appears again is only detectable against a record of the providers that
    // used to appear, and that record belongs to the window, not the thread.
    if let Some(host) = current_window_host() {
        host.note(controller);
    }
    controller.frame_begin();
    // A top-level provider claims the frame before building, so a nested one
    // started inside its closure cannot claim it first.
    let top_level = PROVIDER_STACK.with(|stack| stack.borrow().is_empty());
    if top_level {
        FRAME_CONTROLLER.with(|current| *current.borrow_mut() = Some(controller.clone()));
    }
    PROVIDER_STACK.with(|stack| stack.borrow_mut().push(controller.clone()));
    let host = build();
    PROVIDER_STACK.with(|stack| {
        stack.borrow_mut().pop();
    });

    let sweep = controller.clone();
    let host = host.child(
        canvas(
            |_bounds, _window, _cx| {},
            move |_bounds, _prepaint, window, cx| sweep.frame_end(window, cx),
        )
        .absolute()
        .top(px(0.0))
        .left(px(0.0))
        .size(px(0.0)),
    );
    controller.attach_host(host)
}

/// Register this node's drag source and drop target with the building
/// controller, and wire the native input the registrations imply.
///
/// Without a provider the node is inert: a registration with no controller has
/// no session to join, and inventing a document-global one is exactly the
/// collision this card removes.
pub(crate) fn apply_drag_listeners(
    mut el: gpui::Stateful<Div>,
    node: &Node,
    id: &str,
) -> gpui::Stateful<Div> {
    let Some(controller) = current_controller() else {
        return el;
    };

    // Both registrations go in even when disabled. A disabled target is
    // *ineligible*, not gone: arbitration must be able to skip it and hand the
    // intent to a surviving ancestor, whereas an unregistered target reads as
    // removal and takes the kernel's `TargetLost` cancellation. A disabled
    // source is registered for the mirror reason — the sweep can then tell a
    // source that went inert from one the host deleted.
    if let Some(source) = &node.interaction.drag_source {
        if controller.register_source(source, id) && !source.disabled {
            let payload = NativeDragPayload {
                controller: controller.id(),
                source_id: source.source_id.clone(),
            };
            let start = controller.clone();
            let preview_source = source.clone();
            el = el.on_drag(payload, move |payload, _offset, window, cx| {
                start.begin_session(&payload.source_id, NodeDragInputKind::Mouse, cx);
                start.sync_intent_notifications();
                let snapshot = DragPreviewSnapshot {
                    source_id: preview_source.source_id.clone(),
                    subject: preview_source.subject.clone(),
                    operation: preview_source.operation,
                    label: preview_source.label.clone(),
                };
                let renderer = start.state.borrow().preview.clone();
                let view = renderer.map(|render| render(&snapshot, window, cx));
                cx.new(|_| NodeDragPreview { view })
            });
        }
    }

    if let Some(target) = &node.interaction.drop_target {
        if controller.register_target(target) {
            let bounds_controller = controller.clone();
            let target_id = target.target_id.clone();
            el = el.child(
                canvas(
                    move |bounds, _window, _cx| {
                        bounds_controller.record_target_bounds(&target_id, bounds);
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .top(px(0.0))
                .left(px(0.0))
                .size_full(),
            );
        }
    }
    el
}

/// Record drop-target nesting depth for the tree about to be built.
///
/// Depth is a property of the node tree, not of paint order or bounds
/// containment: two equally-sized nested targets must still arbitrate
/// deepest-first, which measured rectangles alone cannot decide.
pub(crate) fn collect_drop_depths(node: &Node) {
    if let Some(controller) = current_controller() {
        controller.record_depths(node, 0);
    }
}

#[cfg(test)]
mod tests {
    //! Claims that do not need a window. Mounted GPUI dispatch — real pointer
    //! and key input through the event tree — lives in
    //! `packages/gpui/preview/tests/headless_regressions.rs`; a unit test here
    //! could never prove it.

    use super::*;

    fn subject() -> DragSubject {
        DragSubject {
            kind: "track".to_string(),
            id: "kick".to_string(),
        }
    }

    /// The capability matrix is the card's central honesty claim: stock GPUI
    /// 0.2.2 delivers mouse and keyboard and an in-window capture-equivalent
    /// move stream, and delivers no touch contact, no pen identity, and no
    /// device-originated cancel.
    #[test]
    fn the_published_capabilities_match_stock_gpui() {
        let capabilities = DragDropController::new().capabilities();

        assert!(capabilities.mouse);
        assert!(capabilities.keyboard);
        assert!(capabilities.in_window_capture);
        assert!(!capabilities.pen);
        assert!(!capabilities.touch);
        assert!(!capabilities.device_cancel);
        assert_eq!(capabilities, GPUI_DRAG_CAPABILITIES);
    }

    /// Two controllers are two identities. `on_drag_move` dispatches by
    /// payload *type*, so a provider that could not tell its own gesture apart
    /// would drive its session from a neighbour's pointer.
    #[test]
    fn two_controllers_never_share_an_identity_or_a_session() {
        let first = DragDropController::new();
        let second = DragDropController::new();

        assert_ne!(first.id(), second.id());
        assert_eq!(first.snapshot().phase, DragSessionPhase::Idle);
        assert_eq!(second.snapshot().phase, DragSessionPhase::Idle);
        assert!(first.clone().id() == first.id(), "a clone is the same controller");
    }

    /// A target refuses a subject whose kind it never accepted, with a reason
    /// suitable for an announcement rather than silence.
    #[test]
    fn eligibility_gates_on_kind_before_the_consumer_resolver_runs() {
        let registration = NodeDropTarget::new("row", "clip", "Row");
        let intent = DropIntent {
            target_id: "row".to_string(),
            position: "after".to_string(),
            operation: DragOperation::Move,
        };

        assert!(matches!(
            eligibility_for(&registration, &intent, &subject()),
            DropEligibility::Rejected { reason: Some(_) }
        ));
    }

    /// Absent `can_drop` accepts — the registration documents that default,
    /// and a target that silently rejected instead would make every custom
    /// surface look broken until it wrote a resolver it does not need.
    #[test]
    fn a_target_without_an_eligibility_resolver_accepts() {
        let registration = NodeDropTarget::new("row", "track", "Row");
        let intent = DropIntent {
            target_id: "row".to_string(),
            position: "after".to_string(),
            operation: DragOperation::Move,
        };

        assert_eq!(
            eligibility_for(&registration, &intent, &subject()),
            DropEligibility::Accepted {
                intent: intent.clone()
            }
        );
    }

    /// A refusal is arbitrated by the same rule as an acceptance: deepest
    /// first, then priority, then registration order. Taking whichever refusal
    /// happened to be visited first would make a nested surface report a
    /// different refusing target depending on registry iteration.
    #[test]
    fn the_refused_target_is_arbitrated_deepest_first() {
        let refused = |target_id: &str, depth: i32, order: i32, reason: &str| DropTargetCandidate {
            target_id: target_id.to_string(),
            depth,
            order,
            priority: 0,
            contains_point: true,
            eligibility: DropEligibility::Rejected {
                reason: Some(reason.to_string()),
            },
        };
        let candidates = vec![
            refused("outer", 0, 0, "outer says no"),
            refused("inner", 1, 1, "inner says no"),
        ];

        assert_eq!(
            resolve_rejected_target(&candidates),
            Some(("inner".to_string(), Some("inner says no".to_string())))
        );

        let mut reversed = candidates.clone();
        reversed.reverse();
        assert_eq!(
            resolve_rejected_target(&reversed),
            resolve_rejected_target(&candidates),
            "the answer cannot depend on collection order"
        );
    }

    /// A candidate the pointer is not over is not refusing anything.
    #[test]
    fn a_target_the_pointer_has_left_is_not_the_refusing_one() {
        let candidates = vec![DropTargetCandidate {
            target_id: "away".to_string(),
            depth: 0,
            order: 0,
            priority: 0,
            contains_point: false,
            eligibility: DropEligibility::Rejected {
                reason: Some("no".to_string()),
            },
        }];

        assert_eq!(resolve_rejected_target(&candidates), None);
    }

    /// The announcement tail is bounded: a controller lives as long as its
    /// host, and an unbounded log would grow for every hover of every drag.
    #[test]
    fn the_announcement_log_keeps_a_bounded_tail() {
        let controller = DragDropController::new();
        {
            let mut state = controller.state.borrow_mut();
            for index in 0..(ANNOUNCEMENT_LOG_LIMIT * 3) {
                state.announcements.push_back(format!("line {index}"));
                while state.announcements.len() > ANNOUNCEMENT_LOG_LIMIT {
                    state.announcements.pop_front();
                }
            }
        }

        let announcements = controller.announcements();
        assert_eq!(announcements.len(), ANNOUNCEMENT_LOG_LIMIT);
        assert_eq!(
            announcements.last().map(String::as_str),
            Some(format!("line {}", ANNOUNCEMENT_LOG_LIMIT * 3 - 1).as_str()),
            "the tail keeps the newest, not the oldest"
        );
    }

    /// Announcements name the source and the placement. A terminal rejection
    /// carries its reason, because "not dropped" without a reason is the
    /// silence this substrate exists to remove.
    #[test]
    fn default_announcements_name_the_source_placement_and_reason() {
        let event = |kind, reason: Option<&str>| DragAnnouncementEvent {
            kind,
            source_label: "Kick".to_string(),
            target_label: Some("Snare".to_string()),
            position: Some("after".to_string()),
            operation: Some(DragOperation::Move),
            reason: reason.map(str::to_string),
        };

        assert_eq!(
            default_announcement(&event(DragAnnouncementKind::Pickup, None)),
            "Picked up Kick."
        );
        assert_eq!(
            default_announcement(&event(DragAnnouncementKind::IntentChanged, None)),
            "Kick: after Snare."
        );
        assert_eq!(
            default_announcement(&event(DragAnnouncementKind::Dropped, None)),
            "Dropped Kick after Snare."
        );
        assert_eq!(
            default_announcement(&event(DragAnnouncementKind::Rejected, Some("Locked"))),
            "Kick was not dropped. Locked"
        );
        assert_eq!(
            default_announcement(&event(DragAnnouncementKind::Cancelled, None)),
            "Cancelled moving Kick."
        );
    }
}
