use crate::node_compat::Eyebrow;
use gpui::*;
use poodle_adapter::ThemeProvider;
use poodle_gpui::GpuiThemeProvider;
use poodle_node::Node;
use poodle_render::RenderContext;
use poodle_specs::{PaddingScale, SurfaceBorder, SurfaceRole, SurfaceSpec, SurfaceTone};

fn node_surface(spec: SurfaceSpec, theme: &GpuiThemeProvider, content: Node) -> AnyElement {
    poodle_gpui_node_backend::to_gpui(&poodle_render::surface(
        &spec,
        &RenderContext::new(theme),
        vec![content],
    ))
}

pub(crate) fn render(theme: &GpuiThemeProvider) -> Div {
    let text_secondary = theme.resolve_color("color.text.secondary");

    let body = |content: &str| {
        let mut node = Node::text(content);
        node.style.text_size = Some(14.0);
        node.style.descriptor.text_color = Some(text_secondary);
        node
    };

    div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        // --- Tone: Panel (default) ---
        .child(group(
            theme,
            "Panel tone (default)",
            node_surface(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Panel)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
                body("Panel surface with subtle border \u{2014} the standard container."),
            ),
        ))
        // --- Tone: Canvas ---
        .child(group(
            theme,
            "Canvas tone",
            node_surface(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Canvas)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md),
                theme,
                body("Canvas surface sits behind panels as a background layer."),
            ),
        ))
        // --- Tone: Elevated ---
        .child(group(
            theme,
            "Elevated tone",
            node_surface(
                SurfaceSpec::new()
                    .with_tone(SurfaceTone::Elevated)
                    .with_border(SurfaceBorder::Subtle)
                    .with_padding(PaddingScale::Md)
                    .with_elevation(true),
                theme,
                body("Elevated surface with shadow for overlays and cards."),
            ),
        ))
        // --- Border: subtle (default) / default / none ---
        .child(group(
            theme,
            "Border emphasis",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                    body("Subtle border (default) \u{2014} mixed border color."),
                ))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Default)
                        .with_padding(PaddingScale::Md),
                    theme,
                    body("Default border \u{2014} full border-default color."),
                ))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::None)
                        .with_padding(PaddingScale::Md),
                    theme,
                    body("No border \u{2014} just padding and background fill."),
                )),
        ))
        // --- Padding scale: none / sm / md / lg ---
        .child(group(
            theme,
            "Padding scale",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::None),
                    theme,
                    body("padding=none"),
                ))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Sm),
                    theme,
                    body("padding=sm"),
                ))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Md),
                    theme,
                    body("padding=md (default)"),
                ))
                .child(node_surface(
                    SurfaceSpec::new()
                        .with_border(SurfaceBorder::Subtle)
                        .with_padding(PaddingScale::Lg),
                    theme,
                    body("padding=lg"),
                )),
        ))
        // --- Semantic role (region with accessible label) ---
        .child(group(
            theme,
            "Region role (asRole=region)",
            node_surface(
                SurfaceSpec::new()
                    .with_role(SurfaceRole::Region)
                    .with_label("Account settings")
                    .with_padding(PaddingScale::Md),
                theme,
                body("Surface as a semantic region with an accessible label."),
            ),
        ))
}

fn group(theme: &GpuiThemeProvider, title: &str, content: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            poodle_specs::EyebrowSpec::new().with_content(title),
            theme,
        ))
        .child(content)
}
