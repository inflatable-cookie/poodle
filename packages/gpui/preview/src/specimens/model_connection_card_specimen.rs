use std::sync::Arc;

use crate::app_state::{AppState, ModelConnectionEvent, NodeSpecimenEvent};
use crate::node_compat::{Eyebrow, ModelConnectionCard};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_headless::model_connection::{model_catalogue_fixtures, ModelConnectionReadiness};
use poodle_node::Node;
use poodle_render::RenderContext;
use poodle_specs::{
    ButtonVariant, EyebrowSpec, IconButtonSpec, ModelCatalogueEditorSpec, ModelConnectionCardSpec,
    PillAppearance, PillSpec, PillTone, SemanticControlSizeRole,
};

fn group(theme: &GpuiThemeProvider, label: &str, specimen: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(specimen)
}

/// The web specimen's card fixtures, restated. Every field is safe display
/// copy the host supplied — no credential, no probe result.
fn work() -> ModelConnectionCardSpec {
    ModelConnectionCardSpec::new("conn-openai-work", "OpenAI · Work", "OpenAI")
        .with_route_label("Responses API")
        .with_version("2026-08")
        .with_access_summary("API key on file")
        .with_readiness(ModelConnectionReadiness::Ready, "Ready")
}

fn personal() -> ModelConnectionCardSpec {
    ModelConnectionCardSpec::new("conn-openai-personal", "OpenAI · Personal", "OpenAI")
        .with_route_label("Responses API")
        .with_version("2026-08")
        .with_access_summary("API key on file")
        .with_readiness(ModelConnectionReadiness::Ready, "Ready")
        .with_enabled(false)
}

fn codex() -> ModelConnectionCardSpec {
    ModelConnectionCardSpec::new("conn-codex-checking", "Codex", "Codex")
        .with_route_label("App install")
        .with_access_summary("Signed in")
        .with_readiness(ModelConnectionReadiness::Checking, "Checking install")
}

fn anthropic() -> ModelConnectionCardSpec {
    ModelConnectionCardSpec::new("conn-anthropic-attention", "Anthropic", "Anthropic")
        .with_route_label("Messages API")
        .with_version("2026-07")
        .with_access_summary("Needs re-authorisation")
        .with_readiness(ModelConnectionReadiness::Attention, "Needs attention")
}

fn ollama() -> ModelConnectionCardSpec {
    ModelConnectionCardSpec::new("conn-ollama-unavailable", "Ollama", "Ollama")
        .with_route_label("Local runtime")
        .with_access_summary("No credentials required")
        .with_readiness(
            ModelConnectionReadiness::Unavailable,
            "Runtime not reachable",
        )
}

/// The connection behind the one live card on this page.
const CARD_LIVE_ID: &str = "conn-openai-work";

/// Several groups show the same connection, so each instance carries its own
/// backend-state scope: two cards for one connection id would otherwise share
/// a disclosure focus handle.
fn plain(
    theme: &GpuiThemeProvider,
    spec: ModelConnectionCardSpec,
    scope: &str,
) -> ModelConnectionCard {
    ModelConnectionCard::from_spec(spec, theme).with_instance_id(scope)
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let ctx = RenderContext::new(theme);
    let queue = Arc::clone(&state.node_events);
    let host = &state.model_connection;
    let _ = cx;

    // One live card: disclosure and the enable preference are host state, and
    // each callback moves only its own dimension.
    // The details example seeds open, so the section shows the evidence it
    // exists for rather than a closed summary. The disclosure stays live.
    let live_spec = work()
        .with_open(host.card_is_open(CARD_LIVE_ID, true))
        .with_enabled(host.card_is_enabled(CARD_LIVE_ID, true));
    let live = ModelConnectionCard::from_spec(live_spec, theme)
        .with_instance_id("card-live")
        .with_details(poodle_render::model_catalogue_editor(
            &ModelCatalogueEditorSpec::new().with_items(model_catalogue_fixtures()),
            &ctx,
            poodle_render::ModelCatalogueEditorHandlers {
                instance_id: Some("card-live-details".to_string()),
                ..poodle_render::ModelCatalogueEditorHandlers::default()
            },
        ))
        .on_open_change({
            let queue = Arc::clone(&queue);
            Arc::new(move |open: bool| {
                queue
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::ModelConnection(
                        ModelConnectionEvent::CardOpen {
                            id: CARD_LIVE_ID.to_string(),
                            open,
                        },
                    ));
            })
        })
        .on_enabled_change({
            let queue = Arc::clone(&queue);
            Arc::new(move |enabled: bool| {
                queue
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::ModelConnection(
                        ModelConnectionEvent::CardEnabled {
                            id: CARD_LIVE_ID.to_string(),
                            enabled,
                        },
                    ));
            })
        })
        .on_focus_request({
            let queue = Arc::clone(&queue);
            Arc::new(move |id: &str| {
                queue
                    .lock()
                    .unwrap()
                    .push(NodeSpecimenEvent::ModelConnection(
                        ModelConnectionEvent::FocusRequest(id.to_string()),
                    ));
            })
        });

    div()
        .flex()
        .flex_col()
        .gap(px(32.0))
        // Two configured connections of one provider. They differ only by
        // instance label and opaque id; the second is switched off by host
        // preference, not by readiness.
        .child(group(
            theme,
            "Ready and enabled",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(plain(theme, work(), "card-ready"))
                .child(plain(theme, personal(), "card-off")),
        ))
        .child(group(
            theme,
            "Readiness and preference states",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(plain(theme, codex(), "card-checking"))
                .child(plain(theme, anthropic(), "card-attention"))
                .child(plain(theme, ollama(), "card-unavailable"))
                // The whole card is inert; readiness copy stays readable.
                .child(plain(theme, work().with_disabled(true), "card-disabled"))
                // Only the enable Switch is locked; the card still opens.
                .child(plain(
                    theme,
                    codex().with_enable_disabled(true),
                    "card-enable-locked",
                )),
        ))
        // Host composition: the provider mark sits inline before the name,
        // badges follow it, and the actions menu is the host's own.
        .child(group(
            theme,
            "Host mark, badges, actions, and closed accessory",
            div()
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(
                    plain(theme, work(), "card-host-content")
                        .with_leading(Node::icon("star", 16.0))
                        .with_badges(poodle_render::pill(
                            &PillSpec::new()
                                .with_label("Preview")
                                .with_tone(PillTone::Info)
                                .with_appearance(PillAppearance::Subtle),
                            &ctx,
                        ))
                        .with_actions(poodle_render::icon_button(
                            &IconButtonSpec::new()
                                .with_icon("ellipsis")
                                .with_variant(ButtonVariant::Ghost)
                                .with_size_role(SemanticControlSizeRole::Chrome)
                                .with_aria_label("More actions for OpenAI · Work"),
                            &ctx,
                            None,
                        )),
                )
                .child(
                    plain(theme, work(), "card-accessory")
                        .with_closed_accessory(Node::text("Update 1.4.0 available")),
                ),
        ))
        .child(group(theme, "Open details with catalogue", live))
        .child(group(
            theme,
            "Narrow summary wrapping",
            div()
                .max_w(px(288.0))
                .child(plain(theme, anthropic(), "card-narrow")),
        ))
}
