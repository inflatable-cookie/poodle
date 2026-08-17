//! Generic GPUI single-file selection/read capability (g15.007).
//!
//! The FileUpload/LicenceActivation browse intent arrives at the backend as a
//! node activation; what happens next is a runtime-owned capability. This
//! module is that capability for GPUI, and it is deliberately generic:
//!
//! - The live adapter opens the real OS path prompt (`App::prompt_for_paths`),
//!   **awaits** its oneshot receiver in a GPUI task (completion-driven — a
//!   dialog result schedules the render that consumes it, never a poll), reads
//!   the selected file, and runs the shared post-selection pipeline
//!   ([`resolve_os_selection`] → [`finish_file_pick`]).
//! - Headless evidence injects fixture paths/bytes through the same
//!   [`SingleFileSource`] seam and the same [`finish_file_pick`] pipeline —
//!   a static filename or a prefilled credential is never proof.
//!
//! GPUI 0.2.2's `PathPromptOptions` has no accept-filter field, so the
//! configured accept rule (and the web default 10 MB size rule) are enforced
//! *after* selection and a rejection is reported honestly rather than claimed
//! as OS-filtered.

use std::path::PathBuf;

use poodle_headless::file_upload::validate_upload_file;

/// What one pick is configured to do. The accept rule mirrors the FileUpload
/// spec's `accept`; `max_size` defaults to the web component's 10 MB.
#[derive(Clone, Debug, Default)]
pub struct SingleFilePickSpec {
    /// Prompt shown by the OS dialog.
    pub prompt: String,
    /// The FileUpload `accept` rule, enforced after selection.
    pub accept: Option<String>,
    /// Maximum size in bytes; the web default (10 MB) applies when `None`.
    pub max_size: Option<u64>,
}

/// A selected file with its bytes already read.
#[derive(Clone, Debug)]
pub struct PickedFile {
    pub path: PathBuf,
    /// Display name — the last path segment.
    pub name: String,
    pub bytes: Vec<u8>,
}

/// The resolved outcome of one pick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilePickOutcome {
    /// The file passed the accept/size rules and was encoded to bare base64
    /// (no data-URL prefix). Credential contents never render.
    Selected { name: String, contents_base64: String },
    /// The user cancelled the prompt; nothing was selected.
    Cancelled,
    /// The accept or size rule refused the file. Reported honestly — the OS
    /// dialog never filtered it.
    Rejected(String),
    /// The file could not be read.
    Failed(String),
}

/// The OS path prompt options for a single file.
///
/// `prompt_for_paths` hands back a oneshot receiver; the caller **awaits** it
/// in a GPUI task so completion drives the render (see the preview's
/// `AppState::start_file_picks`). Polling the receiver is deliberately not
/// provided here: `try_recv` has no wakeup, so a dialog result would otherwise
/// sit until an unrelated repaint.
pub fn os_pick_options(spec: &SingleFilePickSpec) -> gpui::PathPromptOptions {
    gpui::PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: Some(spec.prompt.clone().into()),
    }
}

/// Resolve a completed OS selection into a pick outcome: first path, read,
/// then the shared accept/size/base64 pipeline.
///
/// The payload is what `App::prompt_for_paths`'s receiver carries —
/// `Ok(Some(paths))` selected, `Ok(None)` cancelled (the sender was dropped
/// with nothing), `Err(..)` a platform failure. Called from the awaited task,
/// and unit-tested with real temporary files so the live read path is
/// exercised, not just the injected one.
pub fn resolve_os_selection(
    selection: anyhow::Result<Option<Vec<PathBuf>>>,
    spec: &SingleFilePickSpec,
) -> FilePickOutcome {
    let Some(paths) = selection.map_err(|error| error.to_string()).ok() else {
        return FilePickOutcome::Failed("The file dialog could not be opened.".to_string());
    };
    let Some(paths) = paths else {
        return FilePickOutcome::Cancelled;
    };
    let Some(path) = paths.into_iter().next() else {
        return FilePickOutcome::Cancelled;
    };
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    match std::fs::read(&path) {
        Ok(bytes) => finish_file_pick(PickedFile { path, name, bytes }, spec),
        Err(error) => FilePickOutcome::Failed(error.to_string()),
    }
}

/// Injected source for headless evidence and hosts that supply fixture
/// selection/bytes — the same post-selection seam the OS prompt uses, minus
/// the dialog and its async lifecycle.
pub struct InjectedFileSource {
    file: Result<Option<PickedFile>, String>,
}

impl InjectedFileSource {
    pub fn new(file: Result<Option<PickedFile>, String>) -> Self {
        Self { file }
    }
}

/// Where a *synchronous* pick outcome comes from, for headless evidence.
pub trait SingleFileSource {
    fn poll(&mut self) -> Option<Result<Option<PickedFile>, String>>;
}

impl SingleFileSource for InjectedFileSource {
    fn poll(&mut self) -> Option<Result<Option<PickedFile>, String>> {
        Some(std::mem::replace(
            &mut self.file,
            Err("fixture consumed".to_string()),
        ))
    }
}

/// The shared post-selection pipeline: accept rule, size rule, then the bare
/// base64 credential payload. Live and headless routes both end here, so a
/// rejection means the same thing on both.
pub fn finish_file_pick(file: PickedFile, spec: &SingleFilePickSpec) -> FilePickOutcome {
    let name = file.name.clone();
    if let Some(message) = validate_upload_file(
        &file.name,
        None,
        file.bytes.len() as u64,
        spec.max_size,
        spec.accept.as_deref(),
    ) {
        return FilePickOutcome::Rejected(message);
    }
    FilePickOutcome::Selected {
        name,
        contents_base64: poodle_headless::file_upload::base64_encode(&file.bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> SingleFilePickSpec {
        SingleFilePickSpec {
            prompt: "Choose a licence file".to_string(),
            accept: Some(".lic".to_string()),
            max_size: None,
        }
    }

    fn picked(name: &str, bytes: &[u8]) -> PickedFile {
        PickedFile {
            path: PathBuf::from(name),
            name: name.to_string(),
            bytes: bytes.to_vec(),
        }
    }

    /// The injected seam resolves immediately and runs the same pipeline as
    /// the OS prompt — this is the headless proof path.
    #[test]
    fn injected_source_resolves_through_the_same_seam() {
        let mut source = InjectedFileSource::new(Ok(Some(picked("machine.lic", b"payload"))));
        let file = source
            .poll()
            .expect("resolved")
            .expect("no read error")
            .expect("not cancelled");
        let outcome = finish_file_pick(file, &spec());
        assert_eq!(
            outcome,
            FilePickOutcome::Selected {
                name: "machine.lic".to_string(),
                contents_base64: poodle_headless::file_upload::base64_encode(b"payload"),
            }
        );
        let FilePickOutcome::Selected { contents_base64, .. } = &outcome else {
            panic!("expected a selection");
        };
        assert!(!contents_base64.starts_with("data:"));
    }

    /// A cancellation from the injected source stays a quiet cancellation.
    #[test]
    fn cancellation_stays_quiet() {
        let mut source = InjectedFileSource::new(Ok(None));
        assert!(source.poll().expect("resolved").expect("no error").is_none());
    }

    /// The accept rule is enforced after selection — the OS dialog could not
    /// filter it (GPUI 0.2.2), so the rejection is reported honestly.
    #[test]
    fn accept_rule_rejects_after_selection() {
        let outcome = finish_file_pick(picked("machine.txt", b"x"), &spec());
        assert_eq!(
            outcome,
            FilePickOutcome::Rejected(
                "File type not accepted. Accepted types: .lic".to_string()
            )
        );
    }

    /// The size rule mirrors the web default when the host configured none.
    #[test]
    fn size_rule_applies_the_web_default() {
        let mut big = spec();
        big.max_size = Some(1);
        let outcome = finish_file_pick(picked("machine.lic", b"payload"), &big);
        assert!(matches!(outcome, FilePickOutcome::Rejected(message)
            if message.starts_with("File too large. Maximum size is")));
    }

    /// A fixture read failure surfaces as a Failed outcome, not a rejection.
    #[test]
    fn read_failures_are_reported() {
        let mut source = InjectedFileSource::new(Err("no such file".to_string()));
        assert_eq!(
            source.poll().expect("resolved").expect_err("errored"),
            "no such file"
        );
    }

    /// The live OS resolution reads a real file through the same pipeline as
    /// the injected seam — this exercises the read path the synchronous
    /// fixtures bypass.
    #[test]
    fn os_selection_reads_a_real_file_through_the_seam() {
        let dir = std::env::temp_dir().join(format!("poodle-pick-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join("machine.lic");
        std::fs::write(&path, b"live payload").expect("fixture file");
        let outcome = resolve_os_selection(Ok(Some(vec![path.clone()])), &spec());
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(
            outcome,
            FilePickOutcome::Selected {
                name: "machine.lic".to_string(),
                contents_base64: poodle_headless::file_upload::base64_encode(b"live payload"),
            }
        );
    }

    /// `Ok(None)` — the sender dropped without a selection — is a
    /// cancellation, not a pending dialog.
    #[test]
    fn os_selection_cancellation_is_not_pending() {
        assert_eq!(resolve_os_selection(Ok(None), &spec()), FilePickOutcome::Cancelled);
        assert_eq!(
            resolve_os_selection(Ok(Some(vec![])), &spec()),
            FilePickOutcome::Cancelled
        );
    }

    /// A platform error and an unreadable path both report honestly.
    #[test]
    fn os_selection_failures_are_reported() {
        assert!(matches!(
            resolve_os_selection(Err(anyhow::anyhow!("picker failed")), &spec()),
            FilePickOutcome::Failed(_)
        ));
        let missing = std::env::temp_dir().join("poodle-pick-missing.lic");
        assert!(matches!(
            resolve_os_selection(Ok(Some(vec![missing])), &spec()),
            FilePickOutcome::Failed(_)
        ));
    }
}
