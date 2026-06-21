use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_gpui_components::{Avatar, Eyebrow};
use poodle_specs::{AvatarShape, AvatarSize, AvatarSpec, AvatarTone, EyebrowSpec};

// A small embedded SVG portrait so the image group renders a real image node
// (object-fit: cover) rather than a hand-rolled box. Matches AvatarSpecimen.svelte.
const SAMPLE_IMAGE: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 80 80'%3E%3Crect width='80' height='80' fill='%232563eb'/%3E%3Ccircle cx='40' cy='30' r='16' fill='%23fff'/%3E%3Cpath d='M14 74c5-18 17-28 26-28s21 10 26 28' fill='%23fff'/%3E%3C/svg%3E";

fn group(theme: &GpuiThemeProvider, label: &str, content: Div) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label.to_string()),
            theme,
        ))
        .child(content)
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(20.0))
        // ── Size ladder: xs → xl (contract §3, §5) ──────────────────────
        .child(group(
            theme,
            "Initials — size scale",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Xs),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Sm),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Md),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Lg),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("TA")
                        .with_size(AvatarSize::Xl),
                    theme,
                )),
        ))
        // ── Initials fallback (multi-char, trims to three) ──────────────
        .child(group(
            theme,
            "Initials fallback",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(AvatarSpec::new().with_initials("A"), theme))
                .child(Avatar::from_spec(
                    AvatarSpec::new().with_initials("AB"),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new().with_initials("ABC"),
                    theme,
                ))
                .child(Avatar::from_spec(AvatarSpec::new(), theme)),
        ))
        // ── Tone: neutral vs accent fallback background (contract §3) ────
        .child(group(
            theme,
            "Tone",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Neutral),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Accent),
                    theme,
                )),
        ))
        // ── Shape: circle vs rounded-square (contract §3) ───────────────
        .child(group(
            theme,
            "Shape",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_shape(AvatarShape::Circle)
                        .with_tone(AvatarTone::Accent),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_initials("AC")
                        .with_shape(AvatarShape::Rounded)
                        .with_tone(AvatarTone::Accent),
                    theme,
                )),
        ))
        // ── Image: object-fit cover over initials (contract §3, §5) ─────
        .child(group(
            theme,
            "Image",
            div()
                .flex()
                .items_center()
                .gap(px(12.0))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_src(SAMPLE_IMAGE)
                        .with_alt("Example user")
                        .with_size(AvatarSize::Lg),
                    theme,
                ))
                .child(Avatar::from_spec(
                    AvatarSpec::new()
                        .with_src(SAMPLE_IMAGE)
                        .with_alt("Example user")
                        .with_shape(AvatarShape::Rounded)
                        .with_size(AvatarSize::Lg),
                    theme,
                )),
        ))
}
