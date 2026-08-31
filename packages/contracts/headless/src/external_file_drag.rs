//! External-file boundaries — Rust mirror of core `external-file-drag.ts`.
//!
//! Architecture: `docs/architecture/011-drag-and-drop-substrate.md`.
//! Spec: `docs/specs/069-dependable-drag-and-drop-substrate.md`.
//!
//! Two directions, one rule: the artifact never crosses into Poodle. Going
//! out, a consumer names an opaque subject and its host answers with a
//! *receipt* — never a path, descriptor, temporary directory, or file handle.
//! Coming in, the host resolves the platform's own drag into *receipts* plus
//! display metadata, and Poodle validates that metadata before any target is
//! allowed to say yes.
//!
//! Nothing here reaches a filesystem or a shell, and nothing here deletes
//! anything. Materialization, naming, retention, and cleanup are host policy.
//! A native drag ending is not permission to remove a temporary file — the
//! destination may still be reading it, and Poodle has no way to know whether
//! it was consumed at all.
//!
//! As in the cross-window bridge, hosts answer through completion callbacks
//! rather than futures: this crate is renderer-neutral and runs inside a
//! single-threaded frame loop with no executor to await on. The lifecycle,
//! the abort channel, and the exactly-once rules are identical to the
//! TypeScript shapes; only the delivery differs.
//!
//! [`CrossWindowAbort`] and [`CrossWindowCleanup`] are reused rather than
//! copied. They are named for the transfer they shipped with, and an export
//! that is superseded abandons its work for exactly the same reasons a lease
//! does; a second channel would be a second place for the idempotence rule to
//! drift.

use crate::cross_window_drag::{CrossWindowAbort, CrossWindowCleanup};
use crate::drag_drop::{DragCancelReason, DragOperation, DragSubject};
use crate::file_upload::file_accepts;

// ── Shared bounds ──────────────────────────────────────────────────────────

/// The longest opaque receipt id this build will carry.
///
/// A receipt is an identifier, not a payload. The same bound as the
/// cross-window token, for the same reason: an id long enough to smuggle a
/// document through is not an id.
pub const EXTERNAL_FILE_MAX_RECEIPT_LENGTH: usize = 512;

/// The longest display name this build will present.
pub const EXTERNAL_FILE_MAX_NAME_LENGTH: usize = 255;

/// The most files one inbound batch or prepared export may name.
pub const EXTERNAL_FILE_MAX_COUNT: usize = 1024;

/// Whether a name is presentable rather than a location.
///
/// The whole point of the boundary is that Poodle never receives a path, and
/// a display name is the one field with a plausible-looking excuse to carry
/// one. A separator, a drive letter, a parent-directory hop, or a URL scheme
/// means the host handed over a location, so the value is refused rather than
/// trimmed down to its last segment — quietly presenting `secret.wav` for
/// `/Users/tom/private/secret.wav` would hide the leak instead of stopping it.
pub fn is_presentable_file_name(name: &str) -> bool {
    if name.is_empty() || name.len() > EXTERNAL_FILE_MAX_NAME_LENGTH {
        return false;
    }
    if name.contains('/') || name.contains('\\') || name.contains('\0') {
        return false;
    }
    if name == "." || name == ".." {
        return false;
    }
    if has_scheme_or_drive(name) {
        return false;
    }
    !name.trim().is_empty()
}

/// `C:` and `file:` are locations wearing a name's clothes.
fn has_scheme_or_drive(name: &str) -> bool {
    let Some(colon) = name.find(':') else {
        return false;
    };
    let head = &name[..colon];
    !head.is_empty()
        && head.starts_with(|c: char| c.is_ascii_alphabetic())
        && head
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '.' || c == '-')
}

fn is_opaque_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= EXTERNAL_FILE_MAX_RECEIPT_LENGTH
}

// ── Native file drag-out ───────────────────────────────────────────────────

/// What this host can actually carry out of the surface.
///
/// Resolved once per adapter rather than negotiated per gesture, and never
/// inferred from a platform name. `files` is the portable baseline; the rest
/// are advertised extensions, and a capability that is false stays inert —
/// the source keeps its ordinary local drag instead of advertising a transfer
/// that cannot happen.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DragExportCapabilities {
    pub files: bool,
    pub multiple_files: bool,
    pub promised_files: bool,
    pub custom_data_types: Vec<String>,
}

/// Which of the four distinct export forms a preparation produced.
///
/// They are distinct because they have different costs and different
/// lifetimes, and collapsing them would let an adapter answer a capability it
/// does not have: an existing file needs no cleanup at all, a materialized one
/// is a temporary artifact the host must retain past the gesture, a promised
/// one is not written until the destination asks, and custom data is not a
/// file in the first place.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragExportForm {
    ExistingFile,
    MaterializedFile,
    PromisedFile,
    CustomData,
}

/// The armed export. Opaque by construction.
///
/// `receipt_id` is the host's own name for whatever it prepared; Poodle
/// compares it, hands it back, and never parses it. `display_name` is
/// presentation only and is refused when it looks like a location. There is
/// no field for a path, a descriptor, a directory, or a byte, because there is
/// no honest use for one on this side of the boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedFileExport {
    pub receipt_id: String,
    pub display_name: Option<String>,
    pub form: DragExportForm,
    /// How many files this receipt stands for. `None` means one.
    pub file_count: Option<u32>,
    /// Declared types for [`DragExportForm::CustomData`]; every one must be
    /// advertised by the adapter.
    pub data_types: Vec<String>,
}

impl PreparedFileExport {
    /// One file, no custom types — the portable baseline shape.
    pub fn file(receipt_id: impl Into<String>, form: DragExportForm) -> Self {
        Self {
            receipt_id: receipt_id.into(),
            display_name: None,
            form,
            file_count: None,
            data_types: Vec::new(),
        }
    }

    pub fn count(&self) -> u32 {
        self.file_count.unwrap_or(1)
    }
}

/// What Poodle knows when it asks a host to prepare an export.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DragExportPrepareRequest {
    pub session_id: String,
    pub source_id: String,
    pub subject: DragSubject,
    pub operation: DragOperation,
    pub allowed_operations: Vec<DragOperation>,
}

/// The end of a native drag-out, in the only three qualities that are honest.
///
/// There is deliberately no "committed": a native drag ending does not prove a
/// destination consumed anything, and no OS drag operation is authority
/// Poodle is willing to relay. `Ended` says the gesture finished and nothing
/// more; whether a DAW took the file is downstream evidence, not a callback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragExportTerminal {
    Ended,
    Cancelled { reason: DragCancelReason },
    Failed { reason: Option<String> },
}

/// The completion a host calls once its export is prepared, or declined.
pub type DragExportPrepareComplete = Box<dyn FnOnce(Option<PreparedFileExport>) + Send>;

/// The one authoritative terminal subscription a `start` installs.
pub type DragExportTerminalCallback = Box<dyn Fn(DragExportTerminal) + Send>;

/// Per draggable source.
///
/// `prepare` runs on the accepted pre-drag gesture, *before* activation,
/// because rendering a clip or writing a temporary file is not something that
/// can happen inside a synchronous native drag start. It is abortable for the
/// same reason: a preparation that is superseded or cancelled has to reach the
/// host so it can stop the work and release what it allocated.
///
/// `start` is the moment the native drag begins. The host owns it, and its
/// terminal callback is the only thing that ends the export. `cancel` runs
/// only while the receipt is still live and never after a terminal, so one
/// receipt receives exactly one closing command.
///
/// Neither call authorizes deletion. Retention and cleanup stay with the host
/// that made the artifact.
pub trait DragExportBridge: Send + Sync {
    fn capabilities(&self) -> DragExportCapabilities;

    /// Prepare the artifact. `complete` receives `None` for a decline and is
    /// called at most once; a completion for a superseded session is rejected
    /// by the kernel on session id.
    fn prepare(
        &self,
        request: DragExportPrepareRequest,
        abort: CrossWindowAbort,
        complete: DragExportPrepareComplete,
    );

    /// Begin the native drag. Returns the subscription's own cleanup, run
    /// exactly once when the session leaves the host's hands.
    fn start(
        &self,
        prepared: PreparedFileExport,
        on_terminal: DragExportTerminalCallback,
    ) -> CrossWindowCleanup;

    /// Idempotent at the boundary, and called only while the receipt is still
    /// live. Never an instruction to delete.
    fn cancel(&self, prepared: PreparedFileExport, reason: DragCancelReason);
}

/// Why a prepared export cannot arm a native drag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragExportRefusal {
    NoReceipt,
    FilesUnsupported,
    MultipleFilesUnsupported,
    PromisedFilesUnsupported,
    CustomDataUnsupported,
    CountOutOfRange,
    NameIsAPath,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DragExportValidation {
    Accepted,
    Refused { reason: DragExportRefusal },
}

/// Check an armed receipt against what its own adapter said it could do.
///
/// The adapter is not the adversary here so much as the drift: a host that
/// advertises single files and then returns three, or advertises no promised
/// files and then returns a promise, has produced a drag that will fail
/// somewhere far away from here. Refusing at the boundary keeps an unsupported
/// capability inert instead of half-armed, and the same check catches the one
/// shape that must never pass — a display name that is really a path.
pub fn validate_file_export(
    prepared: &PreparedFileExport,
    capabilities: &DragExportCapabilities,
) -> DragExportValidation {
    let refuse = |reason: DragExportRefusal| DragExportValidation::Refused { reason };

    if !is_opaque_id(&prepared.receipt_id) {
        return refuse(DragExportRefusal::NoReceipt);
    }
    if let Some(name) = &prepared.display_name {
        if !is_presentable_file_name(name) {
            return refuse(DragExportRefusal::NameIsAPath);
        }
    }

    let count = prepared.count();
    if count < 1 || count as usize > EXTERNAL_FILE_MAX_COUNT {
        return refuse(DragExportRefusal::CountOutOfRange);
    }

    if prepared.form == DragExportForm::CustomData {
        if prepared.data_types.is_empty() {
            return refuse(DragExportRefusal::CustomDataUnsupported);
        }
        // Both sides opt in explicitly: the adapter by advertising the type,
        // the consumer by asking for it. One alone is not a negotiated format.
        for declared in &prepared.data_types {
            if !capabilities.custom_data_types.contains(declared) {
                return refuse(DragExportRefusal::CustomDataUnsupported);
            }
        }
        return DragExportValidation::Accepted;
    }

    if !capabilities.files {
        return refuse(DragExportRefusal::FilesUnsupported);
    }
    if count > 1 && !capabilities.multiple_files {
        return refuse(DragExportRefusal::MultipleFilesUnsupported);
    }
    if prepared.form == DragExportForm::PromisedFile && !capabilities.promised_files {
        return refuse(DragExportRefusal::PromisedFilesUnsupported);
    }

    DragExportValidation::Accepted
}

/// Whether this adapter can export anything at all.
pub fn can_export_anything(capabilities: &DragExportCapabilities) -> bool {
    capabilities.files || !capabilities.custom_data_types.is_empty()
}

/// The export's own visible lifecycle, beside the semantic session phase.
///
/// The session says what the *drag* is doing; this says what the *artifact* is
/// doing, and they are not the same story. A source whose host cannot export
/// is `Unavailable` before any gesture exists, a slow materialization is
/// `Preparing` while the pointer is already down, and `Ended` is the honest
/// close of a drag that left for the operating system — where the kernel,
/// which only knows about Poodle targets, correctly records that nothing
/// local committed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DragExportState {
    Unavailable,
    #[default]
    Idle,
    Preparing,
    Armed,
    Dragging,
    Ended,
    Cancelled,
    Failed,
}

/// An immutable presentation read of the current export.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DragExportSnapshot {
    pub state: DragExportState,
    pub form: Option<DragExportForm>,
    pub file_count: u32,
    /// Presentation only, and never a path.
    pub display_name: Option<String>,
    /// A refusal or failure reason suitable for presentation.
    pub reason: Option<String>,
}

// ── Inbound files ──────────────────────────────────────────────────────────

/// The subject family every inbound external drag uses.
///
/// A target opts into external files by accepting this kind, exactly the way
/// it opts into any other kind. There is no second eligibility path and no
/// bypass: an inbound file reaches the eligibility resolver and the commit
/// handler through the same arbitration as a reordered row.
pub const INBOUND_FILE_SUBJECT_KIND: &str = "poodle.external-file";

/// The inbound batch protocol version this build accepts, and the only one.
///
/// A batch is untrusted input assembled by an adapter that ships separately
/// from this crate — a shell plugin pinned to an older Poodle, a bridge
/// someone forgot to update. Deliberately strict, for the same reason the
/// cross-window receipt is: a batch whose shape this build cannot fully
/// understand is one it cannot honestly claim to have validated, and a
/// best-effort read of it would be a guess made about a user's files.
pub const INBOUND_FILE_PROTOCOL_VERSION: u32 = 1;

/// Which transport owns inbound files in this window.
///
/// Not a preference — an exclusive claim. A native file-drop capture and a
/// webview's own drag events can both be live at once on some platforms, and
/// a surface that enabled both would take one user gesture as two drops. The
/// host declares the owner and Poodle listens to exactly that one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InboundFileTransport {
    DataTransfer,
    Host,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InboundFileCapabilities {
    pub files: bool,
    pub multiple_files: bool,
    pub transport: InboundFileTransport,
    pub custom_data_types: Vec<String>,
}

/// One inbound file, as far as Poodle is ever allowed to know it.
///
/// A host-issued opaque id, a display name, a declared media type, and a size.
/// `name` and `size` are `None` while the platform is still hiding them — a
/// browser exposes nothing but item kinds and declared types during
/// `dragover`, so a hover-time batch can honestly answer count and type
/// questions and cannot answer name or size ones.
///
/// The unknown is modelled rather than guessed. Inventing a name and a zero
/// would make a hover refusal or acceptance that the drop then contradicts;
/// `None` defers exactly the rules that cannot be decided yet, and the full
/// check runs again at drop where every answer exists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InboundFileReceipt {
    pub receipt_id: String,
    pub name: Option<String>,
    pub media_type: String,
    pub size: Option<u64>,
}

/// One inbound gesture's files, named by one host-issued batch id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InboundFileBatch {
    /// Must equal [`INBOUND_FILE_PROTOCOL_VERSION`].
    pub protocol_version: u32,
    pub batch_id: String,
    pub transport: InboundFileTransport,
    pub files: Vec<InboundFileReceipt>,
}

/// What one target will take. Declared per target, not per window.
///
/// `accept` is the same vocabulary the file-upload surfaces already use
/// (`.ext`, `type/*`, an exact media type, or `*`), because a consumer should
/// not have to learn a second one to describe the same thing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InboundFileConstraints {
    pub max_files: Option<usize>,
    /// Per file, in bytes.
    pub max_size: Option<u64>,
    pub accept: Option<String>,
}

/// Why an inbound batch cannot be offered to a target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InboundFileRefusal {
    UnsupportedProtocol,
    FilesUnsupported,
    Empty,
    Malformed,
    Unidentified,
    NameIsAPath,
    TooMany,
    TooLarge,
    UnsupportedType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InboundFileValidation {
    Accepted,
    Refused { reason: InboundFileRefusal },
}

/// Validate an inbound batch before any target is asked whether it wants it.
///
/// External data is untrusted input: the protocol version, the count, the
/// sizes, the declared types, the names, and the host's own identifiers all
/// arrive from outside and are all checked here, before eligibility, before
/// hover posture, and again before commit. A batch that fails is refused with a reason a surface can
/// announce — not dropped silently, and not passed through for the
/// consumer's eligibility resolver to discover.
///
/// A `None` size passes the size rule rather than failing it: at hover the
/// platform has not said, and refusing an unknown would reject every browser
/// file drag before it could be inspected. The drop-time batch carries real
/// sizes and is validated again, which is where an oversized file is caught.
pub fn validate_inbound_files(
    batch: &InboundFileBatch,
    constraints: &InboundFileConstraints,
    capabilities: &InboundFileCapabilities,
) -> InboundFileValidation {
    let refuse = |reason: InboundFileRefusal| InboundFileValidation::Refused { reason };

    if batch.protocol_version != INBOUND_FILE_PROTOCOL_VERSION {
        // First, and before anything reads the rest of the shape: a batch from
        // a protocol this build does not speak has no fields it can trust.
        return refuse(InboundFileRefusal::UnsupportedProtocol);
    }
    if !capabilities.files {
        return refuse(InboundFileRefusal::FilesUnsupported);
    }
    if batch.transport != capabilities.transport {
        // A batch from a transport this window did not hand ownership to is
        // not a batch this window agreed to receive.
        return refuse(InboundFileRefusal::Malformed);
    }
    if !is_opaque_id(&batch.batch_id) {
        return refuse(InboundFileRefusal::Unidentified);
    }
    if batch.files.is_empty() {
        return refuse(InboundFileRefusal::Empty);
    }
    if batch.files.len() > EXTERNAL_FILE_MAX_COUNT {
        return refuse(InboundFileRefusal::TooMany);
    }

    let max_files = constraints
        .max_files
        .or(if capabilities.multiple_files { None } else { Some(1) });
    if let Some(max) = max_files {
        if batch.files.len() > max {
            return refuse(InboundFileRefusal::TooMany);
        }
    }
    if batch.files.len() > 1 && !capabilities.multiple_files {
        return refuse(InboundFileRefusal::TooMany);
    }

    let mut seen: Vec<&str> = Vec::with_capacity(batch.files.len());
    for file in &batch.files {
        if !is_opaque_id(&file.receipt_id) || seen.contains(&file.receipt_id.as_str()) {
            return refuse(InboundFileRefusal::Unidentified);
        }
        seen.push(&file.receipt_id);

        if let Some(name) = &file.name {
            if !is_presentable_file_name(name) {
                return refuse(InboundFileRefusal::NameIsAPath);
            }
        }
        if file.media_type.len() > EXTERNAL_FILE_MAX_NAME_LENGTH {
            return refuse(InboundFileRefusal::Malformed);
        }
        if let (Some(max), Some(size)) = (constraints.max_size, file.size) {
            if size > max {
                return refuse(InboundFileRefusal::TooLarge);
            }
        }
        if let Some(accept) = &constraints.accept {
            if !accepts_inbound_file(accept, file) {
                return refuse(InboundFileRefusal::UnsupportedType);
            }
        }
    }

    InboundFileValidation::Accepted
}

/// The accept rule over a receipt that may still be half-disclosed.
///
/// With a name in hand this is the ordinary file-upload matcher. Without one —
/// hover — an extension rule cannot be decided, and neither can any rule at
/// all when the platform declared no media type. Undecidable defers to the
/// drop check rather than guessing either way: refusing would show a refusal
/// the drop contradicts, and matching would claim a rule was satisfied when
/// nothing was compared.
fn accepts_inbound_file(accept: &str, file: &InboundFileReceipt) -> bool {
    if let Some(name) = &file.name {
        return file_accepts(Some(accept), name, Some(&file.media_type));
    }

    accept.split(',').map(str::trim).any(|token| {
        if token == "*" {
            true
        } else if token.starts_with('.') {
            // An extension rule needs the name the platform has not disclosed.
            true
        } else if file.media_type.is_empty() {
            true
        } else if let Some(prefix) = token.strip_suffix("/*") {
            file.media_type.starts_with(prefix)
        } else {
            file.media_type == token
        }
    })
}

/// What the host tells this window about an external drag it is carrying.
///
/// Coordinates are the window's own client space, because the host is the only
/// thing that observed the pointer — a native file drag delivers no ordinary
/// pointer input to the surface it is over. Poodle hit-tests its own
/// registered targets with them and arbitrates exactly as it would for a local
/// gesture.
#[derive(Clone, Debug, PartialEq)]
pub enum InboundFileEvent {
    Entered {
        batch: InboundFileBatch,
        x: f32,
        y: f32,
    },
    Moved {
        batch_id: String,
        x: f32,
        y: f32,
    },
    Dropped {
        /// The final batch: names, types, and the sizes hover could not see.
        batch: InboundFileBatch,
        x: f32,
        y: f32,
    },
    Cancelled {
        batch_id: String,
    },
}

/// How one inbound batch finished, from Poodle's side of the boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InboundFileOutcome {
    Committed,
    Rejected,
    Failed,
    Cancelled,
}

/// Per document or native window.
///
/// `subscribe` is the host's live account of an external drag. `release` is
/// the single terminal notification, delivered exactly once per batch, and it
/// is a *notification*: the host decides whether the temporary copy it made
/// survives, whether a rejected batch is discarded, and when. Poodle does not
/// hold the files and does not remove them.
pub trait InboundFileHostBridge: Send + Sync {
    fn capabilities(&self) -> InboundFileCapabilities;

    fn subscribe(&self, listener: Box<dyn Fn(InboundFileEvent) + Send>) -> CrossWindowCleanup;

    fn release(&self, batch_id: &str, outcome: InboundFileOutcome);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn export_capabilities() -> DragExportCapabilities {
        DragExportCapabilities {
            files: true,
            multiple_files: false,
            promised_files: false,
            custom_data_types: Vec::new(),
        }
    }

    fn inbound_capabilities() -> InboundFileCapabilities {
        InboundFileCapabilities {
            files: true,
            multiple_files: true,
            transport: InboundFileTransport::DataTransfer,
            custom_data_types: Vec::new(),
        }
    }

    fn receipt(id: &str, name: &str, media_type: &str, size: Option<u64>) -> InboundFileReceipt {
        InboundFileReceipt {
            receipt_id: id.to_string(),
            name: Some(name.to_string()),
            media_type: media_type.to_string(),
            size,
        }
    }

    /// A hover-time receipt: the platform has declared a type and nothing else.
    fn hover_receipt(id: &str, media_type: &str) -> InboundFileReceipt {
        InboundFileReceipt {
            receipt_id: id.to_string(),
            name: None,
            media_type: media_type.to_string(),
            size: None,
        }
    }

    fn batch(files: Vec<InboundFileReceipt>) -> InboundFileBatch {
        InboundFileBatch {
            protocol_version: INBOUND_FILE_PROTOCOL_VERSION,
            batch_id: "batch-1".to_string(),
            transport: InboundFileTransport::DataTransfer,
            files,
        }
    }

    fn refused(validation: InboundFileValidation) -> Option<InboundFileRefusal> {
        match validation {
            InboundFileValidation::Refused { reason } => Some(reason),
            InboundFileValidation::Accepted => None,
        }
    }

    fn export_refused(validation: DragExportValidation) -> Option<DragExportRefusal> {
        match validation {
            DragExportValidation::Refused { reason } => Some(reason),
            DragExportValidation::Accepted => None,
        }
    }

    /// A display name is the one field that could plausibly smuggle a
    /// location across the boundary, so every location shape is refused —
    /// including the Windows and URL forms that contain no separator at all.
    #[test]
    fn a_display_name_that_is_a_location_is_not_presentable() {
        assert!(is_presentable_file_name("take-01.wav"));
        assert!(is_presentable_file_name("mix (final).aiff"));
        assert!(!is_presentable_file_name("/Users/tom/take-01.wav"));
        assert!(!is_presentable_file_name("..\\take-01.wav"));
        assert!(!is_presentable_file_name("C:take-01.wav"));
        assert!(!is_presentable_file_name("file:take-01.wav"));
        assert!(!is_presentable_file_name(".."));
        assert!(!is_presentable_file_name(""));
        assert!(!is_presentable_file_name("   "));
        assert!(!is_presentable_file_name(&"n".repeat(
            EXTERNAL_FILE_MAX_NAME_LENGTH + 1
        )));
    }

    /// An adapter that returns more than it advertised has produced a drag
    /// that fails somewhere far from here, so the receipt is refused instead
    /// of half-arming a capability the host does not have.
    #[test]
    fn a_prepared_export_cannot_exceed_its_own_advertised_capabilities() {
        let capabilities = export_capabilities();
        let single = PreparedFileExport::file("lease-1", DragExportForm::ExistingFile);
        assert_eq!(validate_file_export(&single, &capabilities), DragExportValidation::Accepted);

        let many = PreparedFileExport {
            file_count: Some(3),
            ..PreparedFileExport::file("lease-1", DragExportForm::MaterializedFile)
        };
        assert_eq!(
            export_refused(validate_file_export(&many, &capabilities)),
            Some(DragExportRefusal::MultipleFilesUnsupported)
        );

        let promised = PreparedFileExport::file("lease-1", DragExportForm::PromisedFile);
        assert_eq!(
            export_refused(validate_file_export(&promised, &capabilities)),
            Some(DragExportRefusal::PromisedFilesUnsupported)
        );

        let no_files = DragExportCapabilities::default();
        assert_eq!(
            export_refused(validate_file_export(&single, &no_files)),
            Some(DragExportRefusal::FilesUnsupported)
        );
        assert!(!can_export_anything(&no_files));
    }

    /// Custom data is a negotiated extension: the adapter advertises the type
    /// and the consumer asks for it. One side alone is not a format.
    #[test]
    fn custom_data_needs_both_sides_to_name_the_same_type() {
        let capabilities = DragExportCapabilities {
            files: false,
            multiple_files: false,
            promised_files: false,
            custom_data_types: vec!["application/x-loophole-clip".to_string()],
        };
        let agreed = PreparedFileExport {
            data_types: vec!["application/x-loophole-clip".to_string()],
            ..PreparedFileExport::file("lease-1", DragExportForm::CustomData)
        };
        assert_eq!(validate_file_export(&agreed, &capabilities), DragExportValidation::Accepted);
        assert!(can_export_anything(&capabilities));

        let unadvertised = PreparedFileExport {
            data_types: vec!["application/x-something-else".to_string()],
            ..PreparedFileExport::file("lease-1", DragExportForm::CustomData)
        };
        assert_eq!(
            export_refused(validate_file_export(&unadvertised, &capabilities)),
            Some(DragExportRefusal::CustomDataUnsupported)
        );

        let untyped = PreparedFileExport::file("lease-1", DragExportForm::CustomData);
        assert_eq!(
            export_refused(validate_file_export(&untyped, &capabilities)),
            Some(DragExportRefusal::CustomDataUnsupported)
        );
    }

    /// The receipt id is the whole handle, so an absent one is not an export.
    #[test]
    fn an_export_without_a_bounded_receipt_id_is_refused() {
        let capabilities = export_capabilities();
        let empty = PreparedFileExport::file("", DragExportForm::ExistingFile);
        assert_eq!(
            export_refused(validate_file_export(&empty, &capabilities)),
            Some(DragExportRefusal::NoReceipt)
        );

        let oversized = PreparedFileExport::file(
            "t".repeat(EXTERNAL_FILE_MAX_RECEIPT_LENGTH + 1),
            DragExportForm::ExistingFile,
        );
        assert_eq!(
            export_refused(validate_file_export(&oversized, &capabilities)),
            Some(DragExportRefusal::NoReceipt)
        );

        let path_named = PreparedFileExport {
            display_name: Some("/tmp/poodle/take-01.wav".to_string()),
            ..PreparedFileExport::file("lease-1", DragExportForm::MaterializedFile)
        };
        assert_eq!(
            export_refused(validate_file_export(&path_named, &capabilities)),
            Some(DragExportRefusal::NameIsAPath)
        );
    }

    /// Every rule that can be answered at hover is answered at hover; the one
    /// that cannot — size — passes as unknown and is caught at drop.
    #[test]
    fn an_unknown_hover_size_passes_and_the_real_drop_size_does_not() {
        let capabilities = inbound_capabilities();
        let constraints = InboundFileConstraints {
            max_size: Some(1_000),
            ..InboundFileConstraints::default()
        };

        let hovering = batch(vec![receipt("f1", "take.wav", "audio/wav", None)]);
        assert_eq!(
            validate_inbound_files(&hovering, &constraints, &capabilities),
            InboundFileValidation::Accepted
        );

        let dropped = batch(vec![receipt("f1", "take.wav", "audio/wav", Some(2_000))]);
        assert_eq!(
            refused(validate_inbound_files(&dropped, &constraints, &capabilities)),
            Some(InboundFileRefusal::TooLarge)
        );
    }

    /// Count, type, identity, and name are all checked before a target sees
    /// the batch, and a host that claims the wrong transport is refused
    /// outright — this window handed ownership to exactly one.
    #[test]
    fn inbound_validation_gates_count_type_identity_and_transport() {
        let capabilities = inbound_capabilities();
        let accept_audio = InboundFileConstraints {
            max_files: Some(2),
            accept: Some("audio/*".to_string()),
            ..InboundFileConstraints::default()
        };

        let two = batch(vec![
            receipt("f1", "a.wav", "audio/wav", Some(10)),
            receipt("f2", "b.aiff", "audio/aiff", Some(10)),
        ]);
        assert_eq!(
            validate_inbound_files(&two, &accept_audio, &capabilities),
            InboundFileValidation::Accepted
        );

        let three = batch(vec![
            receipt("f1", "a.wav", "audio/wav", Some(10)),
            receipt("f2", "b.wav", "audio/wav", Some(10)),
            receipt("f3", "c.wav", "audio/wav", Some(10)),
        ]);
        assert_eq!(
            refused(validate_inbound_files(&three, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::TooMany)
        );

        let wrong_type = batch(vec![receipt("f1", "notes.txt", "text/plain", Some(10))]);
        assert_eq!(
            refused(validate_inbound_files(&wrong_type, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::UnsupportedType)
        );

        let duplicate = batch(vec![
            receipt("f1", "a.wav", "audio/wav", Some(10)),
            receipt("f1", "b.wav", "audio/wav", Some(10)),
        ]);
        assert_eq!(
            refused(validate_inbound_files(&duplicate, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::Unidentified)
        );

        let path_named = batch(vec![receipt("f1", "/tmp/a.wav", "audio/wav", Some(10))]);
        assert_eq!(
            refused(validate_inbound_files(&path_named, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::NameIsAPath)
        );

        let foreign = InboundFileBatch {
            transport: InboundFileTransport::Host,
            ..two.clone()
        };
        assert_eq!(
            refused(validate_inbound_files(&foreign, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::Malformed)
        );

        let empty = batch(Vec::new());
        assert_eq!(
            refused(validate_inbound_files(&empty, &accept_audio, &capabilities)),
            Some(InboundFileRefusal::Empty)
        );
    }

    /// Hover discloses a type and nothing else, so an extension rule cannot be
    /// answered yet and defers rather than refusing a drag the drop would
    /// accept. A type rule the platform *can* answer is still enforced.
    #[test]
    fn an_undisclosed_hover_name_defers_extension_rules_and_keeps_type_rules() {
        let capabilities = inbound_capabilities();
        let by_extension = InboundFileConstraints {
            accept: Some(".wav".to_string()),
            ..InboundFileConstraints::default()
        };
        let by_type = InboundFileConstraints {
            accept: Some("audio/*".to_string()),
            ..InboundFileConstraints::default()
        };

        let hovering = batch(vec![hover_receipt("f1", "audio/wav")]);
        assert_eq!(
            validate_inbound_files(&hovering, &by_extension, &capabilities),
            InboundFileValidation::Accepted
        );
        assert_eq!(
            validate_inbound_files(&hovering, &by_type, &capabilities),
            InboundFileValidation::Accepted
        );

        let wrong_type = batch(vec![hover_receipt("f1", "text/plain")]);
        assert_eq!(
            refused(validate_inbound_files(&wrong_type, &by_type, &capabilities)),
            Some(InboundFileRefusal::UnsupportedType)
        );

        // The drop discloses the name the hover deferred on, and the
        // extension rule is answered there.
        let dropped = batch(vec![receipt("f1", "notes.txt", "text/plain", Some(1))]);
        assert_eq!(
            refused(validate_inbound_files(&dropped, &by_extension, &capabilities)),
            Some(InboundFileRefusal::UnsupportedType)
        );
    }

    /// A batch from a protocol this build does not speak is refused before any
    /// other field is read: an adapter pinned to a different Poodle may mean
    /// anything by the rest of the shape, and guessing would be a guess about
    /// somebody's files.
    #[test]
    fn a_batch_from_another_protocol_version_is_refused_first() {
        let capabilities = inbound_capabilities();
        let good = batch(vec![receipt("f1", "a.wav", "audio/wav", Some(10))]);
        assert_eq!(
            validate_inbound_files(&good, &InboundFileConstraints::default(), &capabilities),
            InboundFileValidation::Accepted
        );

        for version in [0, INBOUND_FILE_PROTOCOL_VERSION + 1] {
            let foreign = InboundFileBatch {
                protocol_version: version,
                ..good.clone()
            };
            assert_eq!(
                refused(validate_inbound_files(
                    &foreign,
                    &InboundFileConstraints::default(),
                    &capabilities
                )),
                Some(InboundFileRefusal::UnsupportedProtocol)
            );
        }

        // Refused *first*: a batch that is also empty and on the wrong
        // transport still reports the version, because nothing after it was
        // trustworthy enough to check.
        let unreadable = InboundFileBatch {
            protocol_version: INBOUND_FILE_PROTOCOL_VERSION + 1,
            transport: InboundFileTransport::Host,
            files: Vec::new(),
            ..good.clone()
        };
        assert_eq!(
            refused(validate_inbound_files(
                &unreadable,
                &InboundFileConstraints::default(),
                &capabilities
            )),
            Some(InboundFileRefusal::UnsupportedProtocol)
        );
    }

    /// A single-file window refuses a multi-file batch even when the target
    /// asked for no limit of its own: the transport's ceiling is not the
    /// target's to raise.
    #[test]
    fn a_single_file_transport_refuses_a_multi_file_batch() {
        let capabilities = InboundFileCapabilities {
            multiple_files: false,
            ..inbound_capabilities()
        };
        let two = batch(vec![
            receipt("f1", "a.wav", "audio/wav", Some(10)),
            receipt("f2", "b.wav", "audio/wav", Some(10)),
        ]);
        assert_eq!(
            refused(validate_inbound_files(
                &two,
                &InboundFileConstraints::default(),
                &capabilities
            )),
            Some(InboundFileRefusal::TooMany)
        );

        let unsupported = InboundFileCapabilities {
            files: false,
            ..inbound_capabilities()
        };
        assert_eq!(
            refused(validate_inbound_files(
                &two,
                &InboundFileConstraints::default(),
                &unsupported
            )),
            Some(InboundFileRefusal::FilesUnsupported)
        );
    }
}
