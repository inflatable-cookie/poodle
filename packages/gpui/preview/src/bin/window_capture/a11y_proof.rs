//! Live, non-activating GPUI AccessKit proof scene and command line.

use anyhow::{bail, Result};
use gpui::{
    div, px, AnyElement, App, AppContext as _, Context, IntoElement, ParentElement, Render, Styled,
    Window,
};
use poodle_gpui::GpuiThemeProvider;
use poodle_node::{Node, NodeRole};
use poodle_specs::{ButtonSpec, ButtonVariant, CheckboxSpec, SliderSpec};
use std::path::PathBuf;

use crate::{fixture_capture::FixtureAssets, transport};

const USAGE: &str = "usage: poodle-window-capture --a11y-proof [--plant-unnamed]";

pub fn parse_args(args: &[String]) -> Result<bool> {
    let mut proof_seen = false;
    let mut plant_unnamed = false;
    for arg in args {
        match arg.as_str() {
            "--a11y-proof" if !proof_seen => proof_seen = true,
            "--plant-unnamed" if !plant_unnamed => plant_unnamed = true,
            "--a11y-proof" | "--plant-unnamed" => bail!("duplicate argument '{arg}'\n{USAGE}"),
            other => bail!("unknown argument '{other}'\n{USAGE}"),
        }
    }
    if !proof_seen {
        bail!("--a11y-proof is required\n{USAGE}");
    }
    Ok(plant_unnamed)
}

struct A11yProofRoot {
    theme: GpuiThemeProvider,
    plant_unnamed: bool,
}

impl Render for A11yProofRoot {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let ctx = poodle_render::RenderContext::new(&self.theme);
        let save = poodle_render::button(
            &ButtonSpec::new()
                .with_label("Save")
                .with_aria_label("GPUI AX proof: Save")
                .with_pressed(true)
                .with_variant(ButtonVariant::Primary),
            &ctx,
            None,
        );
        let details = poodle_render::button(
            &ButtonSpec::new()
                .with_label("Details")
                .with_aria_label("GPUI AX proof: Details")
                .with_aria_expanded(true)
                .with_chevron(true),
            &ctx,
            None,
        );
        let mixed = poodle_render::checkbox(
            &CheckboxSpec::new()
                .with_mixed(true)
                .with_label("GPUI AX proof: Mixed state")
                .with_aria_label("GPUI AX proof: Mixed state"),
            &ctx,
            None,
        );
        let mut slider_spec = SliderSpec::new(42.5).with_bounds(0.0, 100.0);
        slider_spec.aria_label = Some("GPUI AX proof: Output level".to_owned());
        slider_spec.value_text = Some("42.5 percent".to_owned());
        let slider_ctx = ctx.with_block_layout_width(160.0);
        let slider = poodle_render::slider(
            &slider_spec,
            &slider_ctx,
            &poodle_render::SliderHandlers::default(),
        );
        let mut selected = Node::container()
            .role(NodeRole::Tab)
            .aria_label("GPUI AX proof: Selected tab");
        selected.a11y.selected = Some(true);
        let mut tabs = Node::container()
            .role(NodeRole::TabList)
            .aria_label("GPUI AX proof: Tabs");
        tabs.children.push(selected);
        let mut disabled = Node::container()
            .role(NodeRole::Button)
            .aria_label("GPUI AX proof: Disabled button");
        disabled.interaction.disabled = true;

        poodle_gpui_node_backend::reset_element_ids();
        let save: AnyElement = poodle_gpui_node_backend::to_gpui(&save);
        let details: AnyElement = poodle_gpui_node_backend::to_gpui(&details);
        let mixed: AnyElement = poodle_gpui_node_backend::to_gpui(&mixed);
        let slider: AnyElement = poodle_gpui_node_backend::to_gpui(&slider);
        let tabs: AnyElement = poodle_gpui_node_backend::to_gpui(&tabs);
        let disabled: AnyElement = poodle_gpui_node_backend::to_gpui(&disabled);

        let mut root = div()
            .size_full()
            .p(px(24.0))
            .flex()
            .flex_col()
            .gap(px(12.0))
            .bg(gpui::rgb(0xffffff))
            .child(save)
            .child(details)
            .child(mixed)
            .child(slider)
            .child(tabs)
            .child(disabled);
        if self.plant_unnamed {
            let unnamed: AnyElement =
                poodle_gpui_node_backend::to_gpui(&Node::container().role(NodeRole::Button));
            root = root.child(unnamed);
        }
        root
    }
}

pub fn run(plant_unnamed: bool) -> ! {
    let theme = GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE);
    transport::accessibility_window(
        FixtureAssets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        },
        Vec::new(),
        Box::new(move |_window, cx: &mut App| {
            cx.new(|_| A11yProofRoot {
                theme,
                plant_unnamed,
            })
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_arguments_are_closed_and_allow_the_planted_control() {
        assert!(!parse_args(&["--a11y-proof".into()]).unwrap());
        assert!(parse_args(&["--a11y-proof".into(), "--plant-unnamed".into()]).unwrap());
        assert!(parse_args(&["--plant-unnamed".into()]).is_err());
        assert!(parse_args(&["--a11y-proof".into(), "--other".into()]).is_err());
    }
}
