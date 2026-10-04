use crate::app_state::{AppState, NodeSpecimenEvent};
use crate::node_compat::{Button, Eyebrow, MediaPicker};
use crate::specimens::specimen_layout::{specimen_layout, SpecimenAxes};
use crate::PreviewRoot;
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{
    ButtonSpec, ButtonVariant, ControlDensity, ControlSize, EyebrowSpec, MediaKind,
    MediaPickerItem, MediaPickerSpec, MediaPickerTab, SemanticControlSizeRole,
};
use std::sync::Arc;

const OPEN_KEY_PREFIX: &str = "media-picker-open-";

fn sample_items() -> Vec<MediaPickerItem> {
    vec![
        MediaPickerItem::new("img-1", "Banner image", MediaKind::Image).with_thumbnail(true),
        MediaPickerItem::new("img-2", "Profile photo", MediaKind::Image).with_thumbnail(true),
        MediaPickerItem::new("icon-1", "Icon set", MediaKind::Image),
        MediaPickerItem::new("doc-1", "Readme.pdf", MediaKind::Document),
        MediaPickerItem::new("vid-1", "Demo video", MediaKind::Video),
        MediaPickerItem::new("img-3", "Screenshot", MediaKind::Image).with_thumbnail(true),
    ]
}

fn open_change(state: &AppState, key: impl Into<String>) -> Arc<dyn Fn(bool) + Send + Sync> {
    let queue = Arc::clone(&state.node_events);
    let key = key.into();
    Arc::new(move |open| {
        queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value: open,
        });
    })
}

fn open_picker_click(state: &AppState, key: impl Into<String>) -> Arc<dyn Fn() + Send + Sync> {
    let queue = Arc::clone(&state.node_events);
    let key = key.into();
    Arc::new(move || {
        queue.lock().unwrap().push(NodeSpecimenEvent::SetToggle {
            key: key.clone(),
            value: true,
        });
    })
}

fn open_button(
    state: &AppState,
    theme: &GpuiThemeProvider,
    open_key: &str,
    id: impl Into<String>,
    label: impl Into<String>,
) -> AnyElement {
    Button::from_spec(
        ButtonSpec::new()
            .with_variant(ButtonVariant::Secondary)
            .with_label(label),
        theme,
    )
    .with_id(id)
    .on_click(open_picker_click(state, open_key))
    .into_any_element()
}

fn picker_dialog(
    state: &AppState,
    theme: &GpuiThemeProvider,
    open_key: &str,
    spec: MediaPickerSpec,
) -> AnyElement {
    MediaPicker::from_spec(spec.with_open(state.specimens.is_on(open_key)), theme)
        .with_items(sample_items())
        .on_open_change(open_change(state, open_key))
        .into_any_element()
}

fn example_group(
    state: &AppState,
    theme: &GpuiThemeProvider,
    key: &'static str,
    label: &'static str,
    spec: MediaPickerSpec,
) -> Div {
    let open_key = format!("{OPEN_KEY_PREFIX}{key}");
    div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .child(Eyebrow::from_spec(
            EyebrowSpec::new().with_content(label),
            theme,
        ))
        .child(open_button(
            state,
            theme,
            &open_key,
            format!("media-picker-trigger-{key}"),
            format!("Open {label}"),
        ))
        .child(picker_dialog(state, theme, &open_key, spec))
}

pub(crate) fn render(state: &AppState, cx: &mut Context<PreviewRoot>) -> Div {
    let theme = &state.theme;
    let examples = div()
        .flex()
        .flex_col()
        .gap(px(24.0))
        .child(example_group(
            state,
            theme,
            "browse",
            "Media picker dialog",
            MediaPickerSpec::new("Select an asset"),
        ))
        .child(example_group(
            state,
            theme,
            "upload",
            "Upload tab",
            MediaPickerSpec::new("Select an asset")
                .with_active_tab(MediaPickerTab::Upload)
                .with_accept("image/*"),
        ))
        .child(example_group(
            state,
            theme,
            "empty",
            "Empty state",
            MediaPickerSpec::new("Select an asset").with_empty_message("No media items yet."),
        ))
        .child(example_group(
            state,
            theme,
            "compact",
            "Compact asset picker",
            MediaPickerSpec::new("Compact asset picker")
                .with_size(ControlSize::Sm)
                .with_density(ControlDensity::Compact),
        ))
        .child(example_group(
            state,
            theme,
            "prominent",
            "Prominent asset picker",
            MediaPickerSpec::new("Prominent asset picker")
                .with_size_role(SemanticControlSizeRole::Prominent)
                .with_density(ControlDensity::Compact),
        ))
        .into_any_element();

    specimen_layout(
        state,
        cx,
        "media-picker",
        examples,
        SpecimenAxes::examples_only()
            .with_sizes(|size, theme| {
                let key = format!("{OPEN_KEY_PREFIX}size-{size:?}");
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(open_button(
                        state,
                        theme,
                        &key,
                        format!("media-picker-size-trigger-{size:?}"),
                        format!("Open {size:?} picker"),
                    ))
                    .child(picker_dialog(
                        state,
                        theme,
                        &key,
                        MediaPickerSpec::new("Select an asset").with_size(size),
                    ))
                    .into_any_element()
            })
            .with_densities(|density, theme| {
                let key = format!("{OPEN_KEY_PREFIX}density-{density:?}");
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(open_button(
                        state,
                        theme,
                        &key,
                        format!("media-picker-density-trigger-{density:?}"),
                        format!("Open {density:?} picker"),
                    ))
                    .child(picker_dialog(
                        state,
                        theme,
                        &key,
                        MediaPickerSpec::new("Select an asset").with_density(density),
                    ))
                    .into_any_element()
            }),
    )
}
