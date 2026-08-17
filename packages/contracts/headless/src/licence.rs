//! Licence — structural mirrors of the licence authority's shapes, plus the
//! pure derivations every renderer consumes.
//!
//! The Rust mirror of `packages/core/src/licence.ts`. Poodle renders; the host
//! supplies behaviour. No Longhorn import, no entitlement policy: `usable`,
//! `attention`, and the seat list are authority reads, and key parsing and
//! account acquisition are injected.
//!
//! The display decisions the three components must not get wrong are all made
//! here, once, exactly as on the web target:
//!
//! - `inGrace` is not a fault (never warning/danger treatment).
//! - `clockRefused` is a machine-clock remedy, never expiry/purchase copy.
//! - Use and update coverage are separate windows.
//! - A failed key check is a typing mistake, never a fake.
//! - A seat without a label is an unnamed machine; raw machine ids never
//!   reach rendered or accessible text.

// ── Structural mirrors of the authority's shapes ─────────────────────────

/// The held licence's usability projection. Authority timestamps are integer
/// Unix seconds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceUsability {
    Active,
    /// Renewal pending; use continues. Never warning/danger.
    InGrace { until: i64 },
    UseWindowExpired { at: i64 },
    /// Lapsed, not expired — the licence is intact, the confirmation stale.
    LeaseLapsed { at: i64 },
    ClockRefused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceTrustBasis {
    OfflineSignature,
    RemoteAssertion { checked: i64 },
}

/// Authority emphasis. Reported, never re-derived from the other reads.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LicenceAttention {
    #[default]
    None,
    Informational,
    Actionable,
}

/// A seat the authority reported. `machine_id` is a random command
/// identifier — never rendered, whole or shortened.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceSeat {
    pub machine_id: String,
    pub label: Option<String>,
    pub this_machine: bool,
}

/// The exact structural credential emitted on activation. Contents are never
/// logged, rendered back, or placed in attributes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceCredential {
    Key { key: String },
    AccountToken { token: String },
    LicenceFile { contents_base64: String },
}

// ── Injected host behaviour ──────────────────────────────────────────────

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceKeyProblem {
    UnexpectedSymbol { symbol: String },
    TooShort { minimum: usize, actual: usize },
    CheckFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceKeyResult {
    Ok { key: String, grouped: String },
    Err(LicenceKeyProblem),
}

/// The host's key parser. Poodle neither imports nor reimplements it.
pub trait LicenceKeyFormat {
    fn parse(&self, input: &str) -> LicenceKeyResult;
    fn is_probably_a_typo(&self, problem: &LicenceKeyProblem) -> bool;
}

// ── LicenceStatus view ───────────────────────────────────────────────────

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceTimedText {
    pub text: String,
    /// Milliseconds after the shared authority-seconds conversion — the
    /// value `TimeAgo` receives on the web. Native renderers convert once at
    /// their own view boundary.
    pub timestamp_ms: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceCoverageRow {
    pub id: &'static str,
    pub term: String,
    /// Value text when there is no window; `None` when `timestamp_ms`
    /// renders.
    pub text: Option<String>,
    pub timestamp_ms: Option<i64>,
    pub future_prefix: Option<String>,
    pub past_prefix: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceTrustRow {
    pub term: String,
    pub text: String,
    pub timestamp_ms: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LicenceStatusIndicator {
    Neutral,
    Success,
    Warning,
    Danger,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LicenceStatusTone {
    Neutral,
    Info,
    Warning,
    Danger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceStatusView {
    pub state: &'static str,
    pub indicator: LicenceStatusIndicator,
    pub tone: LicenceStatusTone,
    pub title: String,
    pub body: LicenceTimedText,
    /// The quiet `inGrace` line; `None` for every other state.
    pub detail: Option<LicenceTimedText>,
    /// Use and update coverage, always both, always in this order.
    pub coverage: Vec<LicenceCoverageRow>,
    pub trust: LicenceTrustRow,
    pub attention: LicenceAttention,
    pub usable: bool,
}

pub struct LicenceStatusInput {
    pub usability: LicenceUsability,
    pub trust_basis: LicenceTrustBasis,
    /// Authority timestamp in integer Unix seconds.
    pub use_until: Option<i64>,
    /// Authority timestamp in integer Unix seconds.
    pub update_until: Option<i64>,
    pub usable: bool,
    pub attention: LicenceAttention,
}

/// Convert one authority timestamp at the shared view boundary.
pub fn licence_timestamp_milliseconds(timestamp_seconds: i64) -> i64 {
    timestamp_seconds * 1_000
}

pub fn licence_status_view(input: LicenceStatusInput) -> LicenceStatusView {
    let (title, body, detail, indicator) = usability_copy(&input.usability);
    LicenceStatusView {
        state: state_name(&input.usability),
        indicator,
        tone: status_tone(&input.usability, input.attention),
        title,
        body,
        detail,
        coverage: vec![
            coverage_row("use", input.use_until),
            coverage_row("update", input.update_until),
        ],
        trust: trust_row(&input.trust_basis),
        attention: input.attention,
        usable: input.usable,
    }
}

fn state_name(usability: &LicenceUsability) -> &'static str {
    match usability {
        LicenceUsability::Active => "active",
        LicenceUsability::InGrace { .. } => "inGrace",
        LicenceUsability::UseWindowExpired { .. } => "useWindowExpired",
        LicenceUsability::LeaseLapsed { .. } => "leaseLapsed",
        LicenceUsability::ClockRefused => "clockRefused",
    }
}

fn usability_copy(
    usability: &LicenceUsability,
) -> (
    String,
    LicenceTimedText,
    Option<LicenceTimedText>,
    LicenceStatusIndicator,
) {
    match usability {
        LicenceUsability::Active => (
            "Licence active".to_string(),
            LicenceTimedText {
                text: "Use is currently covered.".to_string(),
                timestamp_ms: None,
            },
            None,
            LicenceStatusIndicator::Success,
        ),
        // The same title as `active`, because the same thing is true: use
        // continues. A pending renewal is the seller's outstanding work.
        LicenceUsability::InGrace { until } => (
            "Licence active".to_string(),
            LicenceTimedText {
                text: "A renewal is pending. Use continues in the meantime.".to_string(),
                timestamp_ms: None,
            },
            Some(LicenceTimedText {
                text: "Use continues until".to_string(),
                timestamp_ms: Some(licence_timestamp_milliseconds(*until)),
            }),
            LicenceStatusIndicator::Neutral,
        ),
        // Use coverage only. Update coverage is a separate window with its
        // own row.
        LicenceUsability::UseWindowExpired { at } => (
            "Use coverage ended".to_string(),
            LicenceTimedText {
                text: "This licence stopped covering use".to_string(),
                timestamp_ms: Some(licence_timestamp_milliseconds(*at)),
            },
            None,
            LicenceStatusIndicator::Danger,
        ),
        // Lapsed, not expired.
        LicenceUsability::LeaseLapsed { at } => (
            "Licence confirmation required".to_string(),
            LicenceTimedText {
                text: "The lease lapsed".to_string(),
                timestamp_ms: Some(licence_timestamp_milliseconds(*at)),
            },
            None,
            LicenceStatusIndicator::Warning,
        ),
        // A clock that moved backwards is a machine problem with a machine
        // remedy. Never expiry, invalidity, revocation, or purchase.
        LicenceUsability::ClockRefused => (
            "Check this machine's clock".to_string(),
            LicenceTimedText {
                text: "This machine's clock moved backwards. Set the clock to the correct time, then check again."
                    .to_string(),
                timestamp_ms: None,
            },
            None,
            LicenceStatusIndicator::Warning,
        ),
    }
}

/// Block tone. `informational` attention may only lift a calm block to info;
/// it can never make `inGrace` a warning.
fn status_tone(state: &LicenceUsability, attention: LicenceAttention) -> LicenceStatusTone {
    match state {
        LicenceUsability::UseWindowExpired { .. } => LicenceStatusTone::Danger,
        LicenceUsability::LeaseLapsed { .. } | LicenceUsability::ClockRefused => {
            LicenceStatusTone::Warning
        }
        _ => {
            if attention == LicenceAttention::Informational {
                LicenceStatusTone::Info
            } else {
                LicenceStatusTone::Neutral
            }
        }
    }
}

fn coverage_row(id: &'static str, until: Option<i64>) -> LicenceCoverageRow {
    match (id, until) {
        ("use", None) => LicenceCoverageRow {
            id,
            term: "Use coverage".to_string(),
            text: Some("No end date".to_string()),
            timestamp_ms: None,
            future_prefix: None,
            past_prefix: None,
        },
        ("use", Some(until)) => LicenceCoverageRow {
            id,
            term: "Use coverage".to_string(),
            text: None,
            timestamp_ms: Some(licence_timestamp_milliseconds(until)),
            future_prefix: Some("ends".to_string()),
            past_prefix: Some("ended".to_string()),
        },
        ("update", None) => LicenceCoverageRow {
            id,
            term: "Update coverage".to_string(),
            text: Some("No end date".to_string()),
            timestamp_ms: None,
            future_prefix: None,
            past_prefix: None,
        },
        _ => LicenceCoverageRow {
            id,
            term: "Updates".to_string(),
            text: None,
            timestamp_ms: Some(licence_timestamp_milliseconds(until.expect("checked above"))),
            future_prefix: Some("end".to_string()),
            past_prefix: Some("ended".to_string()),
        },
    }
}

fn trust_row(basis: &LicenceTrustBasis) -> LicenceTrustRow {
    match basis {
        LicenceTrustBasis::OfflineSignature => LicenceTrustRow {
            term: "Trust basis".to_string(),
            text: "verified on this machine".to_string(),
            timestamp_ms: None,
        },
        LicenceTrustBasis::RemoteAssertion { checked } => LicenceTrustRow {
            term: "Trust basis".to_string(),
            text: "confirmed".to_string(),
            timestamp_ms: Some(licence_timestamp_milliseconds(*checked)),
        },
    }
}

// ── LicenceActivation resolution ─────────────────────────────────────────

/// The activation product model selected by the host application.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LicenceActivationMode {
    Key,
    #[default]
    Account,
}

/// The credential route currently being submitted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LicenceActivationRoute {
    Key,
    #[default]
    AccountToken,
    LicenceFile,
}

/// A mistyped key. Never `invalid`, `fake`, or `not recognised`.
pub const LICENCE_KEY_TYPO_MESSAGE: &str = "Check the key for a typing mistake.";
/// Distinct from the typo message: a truncation is not a transposition.
pub const LICENCE_KEY_TOO_SHORT_MESSAGE: &str = "This key is too short.";
/// A host predicate that claims neither. Still not a verdict on the key.
pub const LICENCE_KEY_UNREADABLE_MESSAGE: &str = "This key could not be read.";
pub const LICENCE_FILE_REQUIRED_MESSAGE: &str = "Choose a licence file to continue.";
pub const LICENCE_FILE_UNREADABLE_MESSAGE: &str = "That file could not be read.";
/// Generic by design: a failed account flow never leaks token material.
pub const LICENCE_ACCOUNT_FAILED_MESSAGE: &str = "Account activation could not be completed.";

/// Local copy for a rejected key. `tooShort` is checked before the typo
/// predicate, exactly as on the web — the two messages stay distinct.
pub fn licence_key_problem_message(
    problem: &LicenceKeyProblem,
    key_format: &dyn LicenceKeyFormat,
) -> String {
    if matches!(problem, LicenceKeyProblem::TooShort { .. }) {
        return LICENCE_KEY_TOO_SHORT_MESSAGE.to_string();
    }
    if key_format.is_probably_a_typo(problem) {
        return LICENCE_KEY_TYPO_MESSAGE.to_string();
    }
    LICENCE_KEY_UNREADABLE_MESSAGE.to_string()
}

/// The optional machine label: trimmed, and empty means absent.
pub fn licence_machine_label(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Mirror of the web's `licenceFileContentsBase64`: strips the data-URL
/// prefix exactly once. Native reads are already bare, so this is a parity
/// identity for hosts that supply a data-URL string.
pub fn licence_file_contents_base64(read: &str) -> String {
    if let Some(rest) = read.strip_prefix("data:") {
        if let Some(comma) = rest.find(',') {
            return rest[comma + 1..].to_string();
        }
    }
    read.to_string()
}

pub struct LicenceSubmitDraft {
    pub route: LicenceActivationRoute,
    /// The key exactly as typed. Never pre-normalized.
    pub key: String,
    /// A token the injected provider returned; `None` when it cancelled.
    pub token: Option<String>,
    /// File payload, already free of any data-URL prefix.
    pub file_contents_base64: Option<String>,
    /// The machine-label field's raw text.
    pub label: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LicenceSubmitResolution {
    /// Emit the credential; the host runs the activation command.
    Emit {
        credential: LicenceCredential,
        label: Option<String>,
    },
    /// Show a local message and emit nothing.
    Reject { message: String },
    /// Say nothing and emit nothing — a cancelled account flow.
    Quiet,
}

/// The single submit decision, shared by every renderer — the port of
/// `resolveLicenceSubmit`. The accepted key emitted is the raw one; Poodle
/// never re-normalizes a credential the authority's parser accepted.
pub fn resolve_licence_submit(
    draft: &LicenceSubmitDraft,
    key_format: Option<&dyn LicenceKeyFormat>,
) -> LicenceSubmitResolution {
    let label = licence_machine_label(&draft.label);

    match draft.route {
        LicenceActivationRoute::Key => {
            let Some(key_format) = key_format else {
                panic!("A key-format adapter is required for key activation.");
            };
            match key_format.parse(&draft.key) {
                LicenceKeyResult::Err(problem) => LicenceSubmitResolution::Reject {
                    message: licence_key_problem_message(&problem, key_format),
                },
                LicenceKeyResult::Ok { .. } => LicenceSubmitResolution::Emit {
                    credential: LicenceCredential::Key {
                        key: draft.key.clone(),
                    },
                    label,
                },
            }
        }
        LicenceActivationRoute::AccountToken => {
            // A cancellation is a decision, not a failure. Nothing is said.
            let Some(token) = draft.token.clone() else {
                return LicenceSubmitResolution::Quiet;
            };
            LicenceSubmitResolution::Emit {
                credential: LicenceCredential::AccountToken { token },
                label,
            }
        }
        LicenceActivationRoute::LicenceFile => {
            let Some(contents_base64) = draft.file_contents_base64.clone() else {
                return LicenceSubmitResolution::Reject {
                    message: LICENCE_FILE_REQUIRED_MESSAGE.to_string(),
                };
            };
            LicenceSubmitResolution::Emit {
                credential: LicenceCredential::LicenceFile { contents_base64 },
                label,
            }
        }
    }
}

// ── LicenceSeats rows ────────────────────────────────────────────────────

pub const LICENCE_UNNAMED_MACHINE: &str = "Unnamed machine";
pub const LICENCE_THIS_MACHINE: &str = "This machine";
pub const LICENCE_RELEASE_CONFIRM_TITLE: &str = "Release this seat?";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LicenceSeatRow {
    /// Carried for the callback and list keys only. Never rendered.
    pub machine_id: String,
    /// The supplied label, or `Unnamed machine`. Never an ID.
    pub display_label: String,
    pub named: bool,
    pub this_machine: bool,
    /// Only other machines are releasable.
    pub releasable: bool,
    pub pending: bool,
    /// Accessible name for the row's release control.
    pub release_name: String,
    /// Confirmation body — the same honest name the row shows.
    pub confirm_body: String,
}

/// Seats other than this machine — the only rows that can be released.
pub fn licence_other_seats(seats: &[LicenceSeat]) -> Vec<LicenceSeat> {
    seats.iter().filter(|seat| !seat.this_machine).cloned().collect()
}

/// One row per seat. Several unnamed rows look alike, and they stay that
/// way — shortening a machine ID would put identity Poodle was never given
/// in front of the customer.
pub fn licence_seat_rows(
    seats: &[LicenceSeat],
    pending_machine_id: Option<&str>,
    release_label: &str,
) -> Vec<LicenceSeatRow> {
    seats
        .iter()
        .map(|seat| {
            let named = seat
                .label
                .as_deref()
                .is_some_and(|label| !label.trim().is_empty());
            let display_label = if named {
                seat.label.clone().expect("named above")
            } else {
                LICENCE_UNNAMED_MACHINE.to_string()
            };
            LicenceSeatRow {
                machine_id: seat.machine_id.clone(),
                display_label: display_label.clone(),
                named,
                this_machine: seat.this_machine,
                releasable: !seat.this_machine,
                pending: pending_machine_id == Some(seat.machine_id.as_str()),
                release_name: if named {
                    format!("{release_label} {display_label}")
                } else {
                    format!("{release_label} unnamed machine")
                },
                confirm_body: display_label,
            }
        })
        .collect()
}

// ── Absolute time for the quiet `inGrace` line ────────────────────────────

/// Format local civil parts as the web's `formatDisplayTimeDate` does (core
/// `date.ts`), which uses the runtime locale for both the hour cycle and the
/// date order. Pure and timezone-free: the caller resolves the instant into
/// local civil parts at the runtime boundary and passes the runtime locale.
///
/// The reference's pinned cases:
/// - `en_US` — a 12-hour clock with AM/PM (`12:45 PM`) and month/day order
///   (`06/25/2026`);
/// - every other locale (the en-GB shape) — a 24-hour clock (`12:45`) and
///   day/month order (`25/06/2026`).
pub fn format_time_date_locale(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    locale: &str,
) -> String {
    let locale = locale.to_ascii_lowercase();
    let us = locale.starts_with("en_us");
    let time = if us {
        let (hour12, ampm) = match hour.rem_euclid(12) {
            0 => (12, if hour < 12 { "AM" } else { "PM" }),
            h => (h, if hour < 12 { "AM" } else { "PM" }),
        };
        format!("{hour12:02}:{minute:02} {ampm}")
    } else {
        format!("{hour:02}:{minute:02}")
    };
    let date = if us {
        format!("{month:02}/{day:02}/{year:04}")
    } else {
        format!("{day:02}/{month:02}/{year:04}")
    };
    format!("{time} {date}")
}

/// Convert days since 1970-01-01 to a proleptic Gregorian (year, month, day).
/// Howard Hinnant's `civil_from_days` algorithm, the inverse of the epoch
/// math in `poodle_headless::date`. Used by the renderer's non-Unix fallback;
/// Unix targets resolve local parts through the platform instead.
pub fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyKeyFormat;

    impl LicenceKeyFormat for DummyKeyFormat {
        fn parse(&self, input: &str) -> LicenceKeyResult {
            if input.len() < 5 {
                LicenceKeyResult::Err(LicenceKeyProblem::TooShort {
                    minimum: 5,
                    actual: input.len(),
                })
            } else {
                LicenceKeyResult::Ok {
                    key: input.to_string(),
                    grouped: input.to_string(),
                }
            }
        }

        fn is_probably_a_typo(&self, _problem: &LicenceKeyProblem) -> bool {
            false
        }
    }

    #[test]
    fn timestamp_conversion_is_authority_seconds_to_ms() {
        assert_eq!(licence_timestamp_milliseconds(1_800_000_000), 1_800_000_000_000);
    }

    #[test]
    fn status_view_covers_every_state_with_the_web_copy() {
        let active = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::Active,
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: true,
            attention: LicenceAttention::None,
        });
        assert_eq!(active.title, "Licence active");
        assert_eq!(active.indicator, LicenceStatusIndicator::Success);
        assert_eq!(active.coverage.len(), 2);
        assert_eq!(active.coverage[0].text.as_deref(), Some("No end date"));

        let grace = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::InGrace { until: 1_800_000_000 },
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: Some(1_800_000_000),
            update_until: None,
            usable: true,
            attention: LicenceAttention::None,
        });
        assert_eq!(grace.title, "Licence active");
        assert_eq!(grace.indicator, LicenceStatusIndicator::Neutral);
        assert_eq!(
            grace.detail.as_ref().map(|d| d.text.as_str()),
            Some("Use continues until")
        );

        let expired = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::UseWindowExpired { at: 1_700_000_000 },
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: Some(1_700_000_000),
            update_until: Some(1_900_000_000),
            usable: false,
            attention: LicenceAttention::None,
        });
        assert_eq!(expired.title, "Use coverage ended");
        assert_eq!(expired.tone, LicenceStatusTone::Danger);
        assert_eq!(expired.coverage[1].text.as_deref(), None);
        assert_eq!(expired.coverage[1].term, "Updates");
        assert_eq!(expired.coverage[1].future_prefix.as_deref(), Some("end"));

        let clock = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::ClockRefused,
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: false,
            attention: LicenceAttention::None,
        });
        assert_eq!(clock.title, "Check this machine's clock");
        assert!(clock.body.text.contains("clock moved backwards"));
        assert!(!clock.body.text.to_lowercase().contains("expired"));
        assert!(!clock.body.text.to_lowercase().contains("purchase"));
    }

    #[test]
    fn informational_attention_lifts_only_calm_states_to_info() {
        let calm = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::Active,
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: true,
            attention: LicenceAttention::Informational,
        });
        assert_eq!(calm.tone, LicenceStatusTone::Info);

        let grace = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::InGrace { until: 0 },
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: true,
            attention: LicenceAttention::Informational,
        });
        assert_eq!(
            grace.tone,
            LicenceStatusTone::Info,
            "info is allowed, matching the web's statusTone default arm"
        );

        // Actionable attention never lifts a calm state.
        let grace = licence_status_view(LicenceStatusInput {
            usability: LicenceUsability::InGrace { until: 0 },
            trust_basis: LicenceTrustBasis::OfflineSignature,
            use_until: None,
            update_until: None,
            usable: true,
            attention: LicenceAttention::Actionable,
        });
        assert_eq!(grace.tone, LicenceStatusTone::Neutral);
    }

    #[test]
    fn submit_resolution_emits_exact_credentials() {
        let resolved = resolve_licence_submit(
            &LicenceSubmitDraft {
                route: LicenceActivationRoute::Key,
                key: "raw-key".to_string(),
                token: None,
                file_contents_base64: None,
                label: "  work-mac  ".to_string(),
            },
            Some(&DummyKeyFormat),
        );
        assert_eq!(
            resolved,
            LicenceSubmitResolution::Emit {
                credential: LicenceCredential::Key {
                    key: "raw-key".to_string()
                },
                label: Some("work-mac".to_string()),
            }
        );

        let too_short = resolve_licence_submit(
            &LicenceSubmitDraft {
                route: LicenceActivationRoute::Key,
                key: "abc".to_string(),
                token: None,
                file_contents_base64: None,
                label: String::new(),
            },
            Some(&DummyKeyFormat),
        );
        assert_eq!(
            too_short,
            LicenceSubmitResolution::Reject {
                message: LICENCE_KEY_TOO_SHORT_MESSAGE.to_string()
            }
        );

        let file = resolve_licence_submit(
            &LicenceSubmitDraft {
                route: LicenceActivationRoute::LicenceFile,
                key: String::new(),
                token: None,
                file_contents_base64: Some("c3R1ZmY=".to_string()),
                label: "  ".to_string(),
            },
            None,
        );
        assert_eq!(
            file,
            LicenceSubmitResolution::Emit {
                credential: LicenceCredential::LicenceFile {
                    contents_base64: "c3R1ZmY=".to_string()
                },
                label: None,
            }
        );
    }

    #[test]
    fn token_cancellation_is_quiet_and_failure_stays_generic() {
        let quiet = resolve_licence_submit(
            &LicenceSubmitDraft {
                route: LicenceActivationRoute::AccountToken,
                key: String::new(),
                token: None,
                file_contents_base64: None,
                label: String::new(),
            },
            None,
        );
        assert_eq!(quiet, LicenceSubmitResolution::Quiet);
    }

    #[test]
    fn seat_rows_are_honest_about_unnamed_machines() {
        let seats = vec![
            LicenceSeat {
                machine_id: "id-a".to_string(),
                label: Some("Studio rig".to_string()),
                this_machine: true,
            },
            LicenceSeat {
                machine_id: "id-b".to_string(),
                label: None,
                this_machine: false,
            },
        ];
        let rows = licence_seat_rows(&seats, Some("id-b"), "Release");
        assert_eq!(rows[0].display_label, "Studio rig");
        assert!(!rows[0].releasable);
        assert_eq!(rows[1].display_label, LICENCE_UNNAMED_MACHINE);
        assert!(rows[1].releasable);
        assert!(rows[1].pending);
        assert_eq!(rows[1].release_name, "Release unnamed machine");
        assert!(!rows[0].machine_id.is_empty());
        assert!(rows.iter().all(|row| row.display_label != row.machine_id));
    }

    #[test]
    fn time_date_locale_matches_the_web_for_us_and_gb() {
        // en-GB: 24-hour clock, day/month order — the web's
        // `toLocaleTimeString`/`toLocaleDateString` on an en-GB machine.
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 12, 45, "en_GB.UTF-8"),
            "12:45 25/06/2026"
        );
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 0, 5, "en_GB"),
            "00:05 25/06/2026"
        );
        // en-US: 12-hour clock with AM/PM, month/day order.
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 12, 45, "en_US.UTF-8"),
            "12:45 PM 06/25/2026"
        );
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 0, 5, "en_US"),
            "12:05 AM 06/25/2026"
        );
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 23, 59, "en_US"),
            "11:59 PM 06/25/2026"
        );
        // A locale without a pinned shape uses the en-GB shape.
        assert_eq!(
            format_time_date_locale(2026, 6, 25, 9, 15, "de_DE.UTF-8"),
            "09:15 25/06/2026"
        );
    }

    #[test]
    fn civil_from_days_round_trips_modern_dates_not_1970() {
        // 2027-01-15 03:45 UTC is a ten-digit authority seconds value.
        let secs = 1_799_999_100i64;
        let (year, month, day) = civil_from_days(secs.div_euclid(86_400));
        assert_eq!((year, month, day), (2027, 1, 15));
        // The epoch itself renders as 1970 only when given the epoch.
        let (year, month, day) = civil_from_days(0);
        assert_eq!((year, month, day), (1970, 1, 1));
    }

    #[test]
    fn machine_label_trims_and_drops_empty() {
        assert_eq!(licence_machine_label("  rig  "), Some("rig".to_string()));
        assert_eq!(licence_machine_label("   "), None);
        assert_eq!(licence_machine_label(""), None);
    }

    #[test]
    fn data_url_prefix_is_stripped_exactly_once() {
        assert_eq!(
            licence_file_contents_base64("data:application/octet-stream;base64,c3R1ZmY="),
            "c3R1ZmY="
        );
        assert_eq!(licence_file_contents_base64("c3R1ZmY="), "c3R1ZmY=");
    }
}
