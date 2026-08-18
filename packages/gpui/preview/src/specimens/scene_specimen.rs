//! Generated display-specimen interpreter. Catalogue routes now live in named
//! per-component modules; this file remains so the fixture can still be
//! rendered in isolation.
#![allow(dead_code)]

use crate::app_state::AppState;
use crate::node_compat::{Avatar, Callout, EmptyState, Eyebrow, Pill, Spinner};
use crate::specimens::specimen_layout::SpecimenAxes;
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    AvatarShape, AvatarSize, AvatarSpec, AvatarTone, CallOutSpec, ControlDensity, ControlSize,
    EmptyStateSize, EmptyStateSpec, EmptyStateVariant, EyebrowSpec, InlineTypographyMode, PillAppearance, PillFont,
    PillSize, PillSpec, PillTone, SemanticControlSizeRole, SpinnerSize, SpinnerSpec, SpinnerTone,
    SpinnerVariant, StatusTone,
};

#[path = "../generated/specimens/specimens.rs"]
mod fixture;

use fixture::{SpecimenInstance, SpecimenProp, SPECIMEN_SCENES};

/// Looks up the scene for a component slug (`callout` → `callout-specimen`).
fn scene_for(slug: &str) -> Option<&'static fixture::SpecimenScene<'static>> {
    SPECIMEN_SCENES
        .iter()
        .find(|scene| scene.id == format!("{slug}-specimen"))
}

fn prop<'a>(instance: &'a SpecimenInstance<'a>, name: &str) -> Option<&'a str> {
    instance
        .props
        .iter()
        .rfind(|p: &&SpecimenProp| p.prop == name)
        .map(|p| p.value)
}

fn status_tone(value: &str) -> StatusTone {
    match value {
        "info" => StatusTone::Info,
        "success" => StatusTone::Success,
        "warning" => StatusTone::Warning,
        "danger" => StatusTone::Danger,
        "pending" => StatusTone::Pending,
        _ => StatusTone::Neutral,
    }
}

fn pill_tone(value: &str) -> PillTone {
    match value {
        "info" => PillTone::Info,
        "success" => PillTone::Success,
        "warning" => PillTone::Warning,
        "danger" => PillTone::Danger,
        _ => PillTone::Neutral,
    }
}

fn pill_appearance(value: &str) -> PillAppearance {
    match value {
        "subtle" => PillAppearance::Subtle,
        "badge" => PillAppearance::Badge,
        _ => PillAppearance::Solid,
    }
}

fn pill_size(value: &str) -> PillSize {
    match value {
        "xs" => PillSize::Xs,
        "sm" => PillSize::Sm,
        "lg" => PillSize::Lg,
        "xl" => PillSize::Xl,
        _ => PillSize::Md,
    }
}

fn pill_font(value: &str) -> PillFont {
    match value {
        "mono" => PillFont::Mono,
        _ => PillFont::Normal,
    }
}

fn typography(value: &str) -> InlineTypographyMode {
    match value {
        "inherit" => InlineTypographyMode::Inherit,
        _ => InlineTypographyMode::Default,
    }
}

fn size_role(value: &str) -> SemanticControlSizeRole {
    match value {
        "control" => SemanticControlSizeRole::Control,
        "prominent" => SemanticControlSizeRole::Prominent,
        _ => SemanticControlSizeRole::Chrome,
    }
}

fn control_size(value: &str) -> ControlSize {
    match value {
        "xs" => ControlSize::Xs,
        "sm" => ControlSize::Sm,
        "lg" => ControlSize::Lg,
        "xl" => ControlSize::Xl,
        _ => ControlSize::Md,
    }
}

fn control_density(value: &str) -> ControlDensity {
    match value {
        "compact" => ControlDensity::Compact,
        "comfortable" => ControlDensity::Comfortable,
        _ => ControlDensity::Default,
    }
}

fn avatar_size(value: &str) -> AvatarSize {
    match value {
        "xs" => AvatarSize::Xs,
        "sm" => AvatarSize::Sm,
        "lg" => AvatarSize::Lg,
        "xl" => AvatarSize::Xl,
        _ => AvatarSize::Md,
    }
}

fn avatar_shape(value: &str) -> AvatarShape {
    match value {
        "rounded" => AvatarShape::Rounded,
        _ => AvatarShape::Circle,
    }
}

fn avatar_tone(value: &str) -> AvatarTone {
    match value {
        "accent" => AvatarTone::Accent,
        _ => AvatarTone::Neutral,
    }
}

fn spinner_variant(value: &str) -> SpinnerVariant {
    match value {
        "grid" => SpinnerVariant::Grid,
        "dots" => SpinnerVariant::Dots,
        _ => SpinnerVariant::Ring,
    }
}

fn spinner_size(value: &str) -> SpinnerSize {
    match value {
        "xs" => SpinnerSize::Xs,
        "sm" => SpinnerSize::Sm,
        "lg" => SpinnerSize::Lg,
        "xl" => SpinnerSize::Xl,
        _ => SpinnerSize::Md,
    }
}

fn spinner_tone(value: &str) -> SpinnerTone {
    match value {
        "accent" => SpinnerTone::Accent,
        "muted" => SpinnerTone::Muted,
        _ => SpinnerTone::Current,
    }
}

fn empty_state_variant(value: &str) -> EmptyStateVariant {
    match value {
        "search" => EmptyStateVariant::Search,
        "firstRun" => EmptyStateVariant::FirstRun,
        _ => EmptyStateVariant::Neutral,
    }
}

/// Renders one fixture instance through the shared native renderer.
fn render_instance(instance: &SpecimenInstance, theme: &GpuiThemeProvider) -> AnyElement {
    match instance.component {
        "callout" => {
            let mut spec = CallOutSpec::new()
                .with_tone(status_tone(prop(instance, "tone").unwrap_or("neutral")))
                .dismissible(prop(instance, "dismissible") == Some("true"));
            if let Some(title) = prop(instance, "title") {
                spec = spec.with_title(title);
            }
            if let Some(message) = prop(instance, "message") {
                spec = spec.with_content(message);
            } else if let Some(content) = prop(instance, "content") {
                spec = spec.with_content(content);
            }
            if let Some(size) = prop(instance, "size") {
                spec = spec.with_size(control_size(size));
            }
            if let Some(density) = prop(instance, "density") {
                spec = spec.with_density(control_density(density));
            }
            Callout::from_spec(spec, theme).into_any_element()
        }
        "pill" => {
            let mut spec = PillSpec::new()
                .with_label(prop(instance, "content").unwrap_or(""))
                .with_tone(pill_tone(prop(instance, "tone").unwrap_or("neutral")))
                .with_appearance(pill_appearance(
                    prop(instance, "appearance").unwrap_or("solid"),
                ));
            if let Some(role) = prop(instance, "sizeRole") {
                spec = spec.with_size_role(size_role(role));
            }
            if let Some(font) = prop(instance, "font") {
                spec = spec.with_font(pill_font(font));
            }
            if let Some(mode) = prop(instance, "typography") {
                spec = spec.with_typography(typography(mode));
            }
            if let Some(accent) = prop(instance, "accent") {
                spec = spec.with_accent_color(accent);
            }
            if let Some(size) = prop(instance, "size") {
                spec = spec.with_size(pill_size(size));
            }
            if let Some(density) = prop(instance, "density") {
                spec = spec.with_density(control_density(density));
            }
            if prop(instance, "muted") == Some("true") {
                spec = spec.with_muted(true);
            }
            Pill::from_spec(spec, theme).into_any_element()
        }
        "spinner" => {
            let mut spec = SpinnerSpec::new()
                .with_variant(spinner_variant(prop(instance, "variant").unwrap_or("ring")))
                .with_size(spinner_size(prop(instance, "size").unwrap_or("md")))
                .with_tone(spinner_tone(prop(instance, "tone").unwrap_or("current")));
            if let Some(density) = prop(instance, "density") {
                spec = spec.with_density(control_density(density));
            }
            Spinner::from_spec(spec, theme)
        }
        "avatar" => {
            let mut spec = AvatarSpec::new();
            if let Some(initials) = prop(instance, "initials") {
                spec = spec.with_initials(initials);
            }
            if let Some(src) = prop(instance, "src") {
                spec = spec.with_src(src);
            }
            if let Some(alt) = prop(instance, "alt") {
                spec = spec.with_alt(alt);
            }
            if let Some(size) = prop(instance, "size") {
                spec = spec.with_size(avatar_size(size));
            }
            if let Some(tone) = prop(instance, "tone") {
                spec = spec.with_tone(avatar_tone(tone));
            }
            if let Some(shape) = prop(instance, "shape") {
                spec = spec.with_shape(avatar_shape(shape));
            }
            Avatar::from_spec(spec, theme)
        }
        "empty-state" => {
            let mut spec = EmptyStateSpec::new(prop(instance, "title").unwrap_or("")).with_variant(
                empty_state_variant(prop(instance, "variant").unwrap_or("neutral")),
            );
            if let Some(message) = prop(instance, "message") {
                spec = spec.with_message(message);
            }
            if prop(instance, "size") == Some("compact") {
                spec = spec.with_size(EmptyStateSize::Compact);
            }
            if let Some(density) = prop(instance, "density") {
                spec = spec.with_density(control_density(density));
            }
            EmptyState::from_spec(spec, theme).into_any_element()
        }
        _ => div().child("unhandled component").into_any_element(),
    }
}

/// One fixture group as a labelled section (the interpreter owns the group
/// chrome; the fixture owns the text).
fn render_group(group: &fixture::SpecimenGroup, theme: &GpuiThemeProvider) -> Div {
    let mut content = div().flex().flex_col().gap(px(8.0));
    for instance in group.instances {
        content = content.child(render_instance(instance, theme));
    }
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(group.label),
            theme,
        ))
        .child(content)
}

/// The matrix renders the scene's first instance at each axis value (the
/// projection convention the web renderers share).
fn render_matrix_instance(
    instance: &SpecimenInstance,
    theme: &GpuiThemeProvider,
    prop_name: &str,
    value: &str,
) -> AnyElement {
    let mut props: Vec<SpecimenProp> = instance
        .props
        .iter()
        .filter(|p| p.prop != prop_name)
        .cloned()
        .collect();
    props.push(SpecimenProp {
        prop: prop_name,
        value,
    });
    let adjusted = SpecimenInstance {
        component: instance.component,
        caption: instance.caption,
        props: &props,
    };
    render_instance(&adjusted, theme)
}

#[cfg(test)]
mod matrix_override_tests {
    use super::*;

    #[test]
    fn prop_last_binding_wins() {
        let props = [
            SpecimenProp {
                prop: "size",
                value: "xs",
            },
            SpecimenProp {
                prop: "size",
                value: "md",
            },
        ];
        let instance = SpecimenInstance {
            component: "avatar",
            caption: Some("TA xs"),
            props: &props,
        };
        assert_eq!(prop(&instance, "size"), Some("md"));
    }

    #[test]
    fn matrix_size_overrides_fixture_binding() {
        let base_props = [
            SpecimenProp {
                prop: "initials",
                value: "TA",
            },
            SpecimenProp {
                prop: "size",
                value: "xs",
            },
        ];
        let instance = SpecimenInstance {
            component: "avatar",
            caption: Some("TA xs"),
            props: &base_props,
        };
        let mut props: Vec<SpecimenProp> = instance
            .props
            .iter()
            .filter(|p| p.prop != "size")
            .cloned()
            .collect();
        props.push(SpecimenProp {
            prop: "size",
            value: "md",
        });
        let adjusted = SpecimenInstance {
            component: instance.component,
            caption: instance.caption,
            props: &props,
        };
        assert_eq!(prop(&instance, "size"), Some("xs"));
        assert_eq!(prop(&adjusted, "size"), Some("md"));
    }
}

/// Renders the scene for a slug, or `None` when the slug is not scene-driven.
pub(crate) fn render(slug: &str, state: &AppState, cx: &mut Context<PreviewRoot>) -> Option<Div> {
    let theme = &state.theme;
    let scene = scene_for(slug)?;

    let mut examples = div().flex().flex_col().gap(px(24.0));
    for group in scene.groups {
        examples = examples.child(render_group(group, theme));
    }
    let examples = examples.into_any_element();

    let first = scene.groups.first().and_then(|g| g.instances.first());
    let size_axis = if scene.size_axis.is_empty() {
        None
    } else {
        Some(scene.size_axis)
    };
    let density_axis = if scene.density_axis.is_empty() {
        None
    } else {
        Some(scene.density_axis)
    };

    let mut axes = SpecimenAxes::examples_only();
    if let (Some(instance), Some(axis)) = (first, size_axis) {
        axes = axes.with_named_sizes(axis, move |value, theme| {
            render_matrix_instance(instance, theme, "size", value)
        });
    }
    if let (Some(instance), Some(axis)) = (first, density_axis) {
        axes = axes.with_named_densities(axis, move |value, theme| {
            render_matrix_instance(instance, theme, "density", value)
        });
    }

    Some(super::specimen_layout::specimen_layout(
        state, cx, slug, examples, axes,
    ))
}
