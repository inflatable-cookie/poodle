//! Avatar specimen — initials/icon/image fallback, shape, tone, size ladder.
//!
//! Every avatar is a real `js_avatar` driven by `AvatarSpec`; the component
//! resolves the fallback (initials → image when `src` set), shape, tone, and
//! size from tokens. No hand-rolled circles.

use crate::nel::*;
use poodle_jetstream::JetstreamThemeProvider;
use crate::compat::js_avatar;

use poodle_specs::{AvatarShape, AvatarSize, AvatarSpec, AvatarTone};

pub fn render(theme: &JetstreamThemeProvider) -> El {
    let secondary = resolve_color(theme, "color.text.secondary");

    let row = |children: Vec<El>| -> El {
        let mut r = div().flex_row().items_center().gap(12.0);
        for c in children {
            r = r.child(c);
        }
        r
    };

    div()
        .flex_col()
        .gap(24.0)
        .child(group(
            "Initials fallback",
            secondary,
            row(vec![
                js_avatar(&AvatarSpec::new().with_initials("A"), theme),
                js_avatar(&AvatarSpec::new().with_initials("JD"), theme),
                js_avatar(&AvatarSpec::new().with_initials("ABC"), theme),
                js_avatar(&AvatarSpec::new(), theme), // empty → placeholder glyph
            ]),
        ))
        .child(group(
            "Image",
            secondary,
            row(vec![
                js_avatar(
                    &AvatarSpec::new()
                        .with_src("https://example.com/a.png")
                        .with_alt("User A"),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new()
                        .with_src("https://example.com/b.png")
                        .with_alt("User B")
                        .with_shape(AvatarShape::Rounded),
                    theme,
                ),
            ]),
        ))
        .child(group(
            "Shape",
            secondary,
            row(vec![
                js_avatar(
                    &AvatarSpec::new()
                        .with_initials("CI")
                        .with_shape(AvatarShape::Circle),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new()
                        .with_initials("RS")
                        .with_shape(AvatarShape::Rounded),
                    theme,
                ),
            ]),
        ))
        .child(group(
            "Tone",
            secondary,
            row(vec![
                js_avatar(
                    &AvatarSpec::new()
                        .with_initials("NE")
                        .with_tone(AvatarTone::Neutral),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new()
                        .with_initials("AC")
                        .with_tone(AvatarTone::Accent),
                    theme,
                ),
            ]),
        ))
        .child(group(
            "Sizes",
            secondary,
            row(vec![
                js_avatar(
                    &AvatarSpec::new().with_initials("XS").with_size(AvatarSize::Xs),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new().with_initials("SM").with_size(AvatarSize::Sm),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new().with_initials("MD").with_size(AvatarSize::Md),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new().with_initials("LG").with_size(AvatarSize::Lg),
                    theme,
                ),
                js_avatar(
                    &AvatarSpec::new().with_initials("XL").with_size(AvatarSize::Xl),
                    theme,
                ),
            ]),
        ))
}

fn group(title: &str, text_secondary: ColorValue, content: El) -> El {
    div()
        .flex_col()
        .gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
