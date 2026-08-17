//! Generic GPUI single-file selection/read capability (g15.007).
//!
//! The FileUpload/LicenceActivation browse intent arrives at the backend as a
//! node activation; what happens next is a runtime-owned capability. This
//! module is that capability for GPUI, and it is deliberately generic:
//!
//! - The live adapter opens the real OS path prompt
//!   ([`OsFilePrompt::start`] → `App::prompt_for_paths`), reads the selected
//!   file, and runs the shared post-selection pipeline.
//! - Headless evidence injects fixture paths/bytes through the same
//!   [`SingleFileSource`] seam and the same [`finish_file_pick`] pipeline —
//!   a static filename or a prefilled credential is never proof.
//!
//! GPUI 0.2.2's `PathPromptOptions` has no accept-filter field, so the
//! configured accept rule (and the web default 10 MB size rule) are enforced
//! *after* selection and a rejection is reported honestly rather than claimed
//! as OS-filtered.

use std::path::PathBuf;

use gpui::App;

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

/// Where one file pick comes from. The OS source owns the modal prompt and
/// its asynchronous receiver; headless evidence injects fixture bytes through
/// this same trait.
pub trait SingleFileSource {
    /// Continue the pick. `None` means still pending — poll again on a later
    /// frame. `Some(Ok(Some(file)))`, `Some(Ok(None))` (cancelled), and
    /// `Some(Err(..))` (read failed) resolve it.
    fn poll(&mut self) -> Option<Result<Option<PickedFile>, String>>;
}

/// The live source: GPUI's OS path prompt plus the file read.
///
/// `App::prompt_for_paths` returns a oneshot receiver immediately and the
/// platform delivers asynchronously, so [`Self::start`] begins the prompt and
/// later polls try the receiver. The test platform does not implement the
/// prompt, so headless evidence never routes through this source.
pub struct OsFilePrompt {
    receiver: futures::channel::oneshot::Receiver<anyhow::Result<Option<Vec<PathBuf>>>>,
}

impl OsFilePrompt {
    /// Open the OS path prompt for one file and return the source that will
    /// resolve it. Call with the app context (a render frame has one); poll
    /// from later frames.
    pub fn start(cx: &mut App, spec: &SingleFilePickSpec) -> Self {
        let options = gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some(spec.prompt.clone().into()),
        };
        Self {
            receiver: cx.prompt_for_paths(options),
        }
    }
}

impl SingleFileSource for OsFilePrompt {
    fn poll(&mut self) -> Option<Result<Option<PickedFile>, String>> {
        let payload = match self.receiver.try_recv() {
            // Receiver still parked: the dialog is open.
            Err(_) => return None,
            Ok(payload) => payload,
        };
        let selection = payload?;
        let paths = match selection {
            Ok(paths) => paths,
            Err(error) => return Some(Err(error.to_string())),
        };
        let Some(paths) = paths else {
            return Some(Ok(None));
        };
        let Some(path) = paths.into_iter().next() else {
            return Some(Ok(None));
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        match std::fs::read(&path) {
            Ok(bytes) => Some(Ok(Some(PickedFile { path, name, bytes }))),
            Err(error) => Some(Err(error.to_string())),
        }
    }
}

/// Injected source for headless evidence and hosts that supply fixture
/// selection/bytes — the same seam the OS prompt uses, minus the dialog.
pub struct InjectedFileSource {
    file: Result<Option<PickedFile>, String>,
}

impl InjectedFileSource {
    pub fn new(file: Result<Option<PickedFile>, String>) -> Self {
        Self { file }
    }
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
}
