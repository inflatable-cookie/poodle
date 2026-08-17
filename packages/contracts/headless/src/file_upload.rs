//! FileUpload pure validation and the single-file read payload (g15.007).
//!
//! Mirrors `isAcceptedFileType` / `validateUploadFile` in
//! `packages/core/src/file-upload.ts` so a rejected file means the same thing
//! on every target. GPUI 0.2.2's path prompt has no accept-filter field, so
//! the configured accept rule is enforced *after* selection — this module
//! owns that rule and the size rule, and the GPUI capability seam owns
//! selecting and reading.
//!
//! `base64_encode` is the credential payload formatter: the authority wants
//! the bare payload, never a data-URL prefix, so the licence-file route
//! encodes through here and the web target strips the prefix through its
//! mirror (`licenceFileContentsBase64`).

/// The file-name extension token, exactly as the web target derives it:
/// `"." + last dot-segment`, lower-cased, and a name without a dot still
/// yields `".<whole-name>"` — the reference's own quirk, kept for parity.
fn extension_token(file_name: &str) -> String {
    let lower = file_name.to_ascii_lowercase();
    match lower.rsplit_once('.') {
        Some((_, ext)) if !ext.is_empty() => format!(".{ext}"),
        _ => format!(".{lower}"),
    }
}

/// Whether `accept` admits the given file. Mirrors `isAcceptedFileType`:
/// `.ext` tokens match the extension, `type/*` wildcards and exact MIME types
/// need a MIME read (the GPUI path cannot supply one and fails those rules
/// honestly rather than guessing), and `*` admits everything.
pub fn file_accepts(accept: Option<&str>, file_name: &str, mime_type: Option<&str>) -> bool {
    let Some(accept) = accept else {
        return true;
    };
    let accept = accept.trim();
    if accept.is_empty() || accept == "*" {
        return true;
    }
    let ext = extension_token(file_name);
    accept.split(',').map(str::trim).any(|accepted| {
        if let Some(stripped) = accepted.strip_prefix('.') {
            !stripped.is_empty() && ext == accepted.to_ascii_lowercase()
        } else if let Some(prefix) = accepted.strip_suffix("/*") {
            mime_type.is_some_and(|mime| mime.starts_with(prefix))
        } else {
            mime_type == Some(accepted)
        }
    })
}

/// The web component's default maximum size (10 MB), applied when the host
/// did not configure one — the same default the Svelte/React targets enforce.
pub const DEFAULT_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Mirror of `formatFileSize` (B/KB/MB/GB with one decimal). Distinct from
/// the specs' list-row formatter, which is the `bytes`/`KB`/`MB` copy the
/// file list itself displays.
pub fn format_file_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let k = 1024.0;
    let sizes = ["B", "KB", "MB", "GB"];
    let value = bytes as f64;
    let i = (value.log(k).floor() as usize).min(sizes.len() - 1);
    format!("{:.1} {}", value / k.powi(i as i32), sizes[i])
}

/// Mirror of `validateUploadFile` without the host `validate` closure: the
/// size rule first, then the accept rule, with the reference's copy.
pub fn validate_upload_file(
    file_name: &str,
    mime_type: Option<&str>,
    size: u64,
    max_size: Option<u64>,
    accept: Option<&str>,
) -> Option<String> {
    let max_size = max_size.unwrap_or(DEFAULT_MAX_FILE_SIZE);
    if size > max_size {
        return Some(format!(
            "File too large. Maximum size is {}",
            format_file_size(max_size)
        ));
    }
    if !file_accepts(accept, file_name, mime_type) {
        return Some(format!(
            "File type not accepted. Accepted types: {}",
            accept.unwrap_or_default()
        ));
    }
    None
}

/// Standard base64 encoding of raw bytes — the licence-file credential
/// payload. Dependency-free on purpose (the shared Rust target has no runtime
/// base64 and a new crate dependency is not this card's to add).
pub fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            TABLE[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_tokens_match_case_insensitively() {
        assert!(file_accepts(Some(".lic"), "machine.lic", None));
        assert!(file_accepts(Some(".LIC"), "MACHINE.LIC", None));
        assert!(file_accepts(Some(".pdf,.lic"), "machine.lic", None));
        assert!(!file_accepts(Some(".lic"), "machine.txt", None));
        assert!(!file_accepts(Some(".pdf"), "machine.lic", None));
    }

    #[test]
    fn mime_rules_need_a_mime_read_and_never_guess() {
        assert!(file_accepts(Some("image/*"), "photo.png", Some("image/png")));
        assert!(file_accepts(Some("image/png"), "photo.png", Some("image/png")));
        assert!(!file_accepts(Some("image/*"), "photo.png", None));
        assert!(!file_accepts(Some("image/png"), "photo.png", None));
        assert!(!file_accepts(Some("image/*"), "photo.png", Some("text/plain")));
    }

    #[test]
    fn wildcard_and_absent_accept_admit_everything() {
        assert!(file_accepts(Some("*"), "anything.bin", None));
        assert!(file_accepts(None, "anything.bin", None));
        assert!(file_accepts(Some("  "), "anything.bin", None));
    }

    #[test]
    fn extension_token_keeps_the_reference_quirk_for_dotless_names() {
        // Core derives ".noext" for a name without a dot; `.noext` matches.
        assert!(file_accepts(Some(".noext"), "noext", None));
        assert!(!file_accepts(Some(".txt"), "noext", None));
    }

    #[test]
    fn size_rule_precedes_accept_and_uses_the_web_copy() {
        assert_eq!(
            validate_upload_file("big.lic", None, 5 * 1024 * 1024, Some(1024), None),
            Some("File too large. Maximum size is 1.0 KB".to_string())
        );
        assert_eq!(
            validate_upload_file("ok.lic", None, 512, Some(1024), Some(".lic")),
            None
        );
        assert_eq!(
            validate_upload_file("ok.txt", None, 512, Some(1024), Some(".lic")),
            Some("File type not accepted. Accepted types: .lic".to_string())
        );
    }

    #[test]
    fn default_max_size_is_ten_megabytes() {
        assert_eq!(DEFAULT_MAX_FILE_SIZE, 10 * 1024 * 1024);
        assert_eq!(
            validate_upload_file("big.bin", None, DEFAULT_MAX_FILE_SIZE + 1, None, None),
            Some("File too large. Maximum size is 10.0 MB".to_string())
        );
    }

    #[test]
    fn format_file_size_mirrors_core() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512.0 B");
        assert_eq!(format_file_size(2048), "2.0 KB");
        assert_eq!(format_file_size(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn base64_matches_rfc4648_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
        // Binary payloads keep every byte.
        assert_eq!(base64_encode(&[0x00, 0xff, 0x10, 0x80]), "AP8QgA==");
    }

    #[test]
    fn base64_never_carries_a_data_url_prefix() {
        let encoded = base64_encode(b"licence payload");
        assert!(!encoded.starts_with("data:"));
        assert!(!encoded.contains(','));
    }
}
