use crate::node_compat::{Eyebrow, LicenceStatus};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::licence::{LicenceAttention, LicenceTrustBasis, LicenceUsability};
use poodle_specs::{EyebrowSpec, LicenceStatusSpec};

fn now_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn group(theme: &GpuiThemeProvider, label: &str, specimen: LicenceStatus) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(EyebrowSpec::new().with_content(label), theme))
        .child(specimen)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let now = now_seconds();
    let soon = now + 12 * 86_400;
    let later = now + 240 * 86_400;
    let past = now - 9 * 86_400;
    let checked = now - 3_600;

    let offline = LicenceTrustBasis::OfflineSignature;
    let remote = LicenceTrustBasis::RemoteAssertion { checked };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .max_w(px(420.0))
        .child(group(theme, "Active", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(offline.clone())
                .with_use_until(Some(later))
                .with_update_until(Some(later))
                .with_usable(true),
            theme,
        )))
        // A pending renewal is the seller's outstanding work. It stays calm.
        .child(group(theme, "In grace", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::InGrace { until: soon })
                .with_trust_basis(remote.clone())
                .with_use_until(Some(soon))
                .with_update_until(Some(later))
                .with_usable(true),
            theme,
        )))
        .child(group(theme, "Use window expired", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::UseWindowExpired { at: past })
                .with_trust_basis(offline.clone())
                .with_use_until(Some(past))
                .with_update_until(Some(later))
                .with_usable(false)
                .with_attention(LicenceAttention::Actionable),
            theme,
        )))
        .child(group(theme, "Lease lapsed", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::LeaseLapsed { at: past })
                .with_trust_basis(remote.clone())
                .with_use_until(Some(later))
                .with_update_until(Some(later))
                .with_usable(false)
                .with_attention(LicenceAttention::Actionable),
            theme,
        )))
        // The remedy is the machine's clock, never a purchase.
        .child(group(theme, "Clock refused", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::ClockRefused)
                .with_trust_basis(remote.clone())
                .with_use_until(Some(later))
                .with_update_until(Some(later))
                .with_usable(false)
                .with_attention(LicenceAttention::Actionable),
            theme,
        )))
        .child(group(theme, "No coverage expiry", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(offline.clone())
                .with_use_until(None)
                .with_update_until(None)
                .with_usable(true),
            theme,
        )))
        // Perpetual use, lapsed updates. Two windows, two rows.
        .child(group(theme, "Updates expired", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(offline.clone())
                .with_use_until(None)
                .with_update_until(Some(past))
                .with_usable(true)
                .with_attention(LicenceAttention::Informational),
            theme,
        )))
        .child(group(theme, "Use window only", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(offline.clone())
                .with_use_until(Some(later))
                .with_update_until(None)
                .with_usable(true),
            theme,
        )))
        .child(group(theme, "Offline verification", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(offline.clone())
                .with_use_until(Some(later))
                .with_update_until(Some(later))
                .with_usable(true),
            theme,
        )))
        .child(group(theme, "Remote verification", LicenceStatus::from_spec(
            LicenceStatusSpec::new()
                .with_usability(LicenceUsability::Active)
                .with_trust_basis(remote.clone())
                .with_use_until(Some(later))
                .with_update_until(Some(later))
                .with_usable(true),
            theme,
        )))
        }

