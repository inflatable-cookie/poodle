#![recursion_limit = "512"]

use std::cell::RefCell;
use std::rc::Rc;

use gpui::{div, px, AnyElement, IntoElement, ParentElement, Styled, TestAppContext};
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::{StatusIndicatorSpec, StatusTone};

#[path = "../src/headless_driver.rs"]
mod headless_driver;

#[path = "../src/block_slider_host.rs"]
mod block_slider_host;

mod app_state {
    #[derive(Clone, Debug)]
    pub enum NodeSpecimenEvent {
        FileBrowse {
            key: String,
            spec: poodle_gpui_node_backend::file_capability::SingleFilePickSpec,
            failed_message: Option<String>,
        },
        SetToggle {
            key: String,
            value: bool,
        },
        SetValue {
            key: String,
            value: String,
        },
    }
}

#[path = "../src/node_compat.rs"]
mod node_compat;

use headless_driver::HeadlessDriver;

fn run_headless(body: impl FnOnce(&mut TestAppContext)) {
    poodle_gpui_node_backend::reset_focus_registry();
    let mut cx = TestAppContext::single();
    body(&mut cx);
    cx.dispatcher.run_until_parked();
    cx.background_executor.forbid_parking();
    cx.quit();
    cx.dispatcher.run_until_parked();
}

fn theme() -> GpuiThemeProvider {
    GpuiThemeProvider::new().with_theme(&poodle_tokens::themes::ECLIPSE)
}

#[test]
fn status_indicator_status_reason_tokens_and_identity_rebuild_through_mounted_backend() {
    run_headless(|cx| {
        let left_status = Rc::new(RefCell::new(StatusTone::Info));
        let theme_provider = theme();
        let build: Rc<dyn Fn() -> AnyElement> = {
            let left_status = Rc::clone(&left_status);
            let theme_provider = theme_provider.clone();
            Rc::new(move || {
                let indicator = |scope, status, reason| {
                    node_compat::StatusIndicator::from_spec(
                        StatusIndicatorSpec::new()
                            .with_status(status)
                            .with_label("Connected")
                            .with_aria_label(reason),
                        &theme_provider,
                    )
                    .with_instance_id(scope)
                    .into_element()
                };
                div()
                    .flex()
                    .flex_col()
                    .gap(px(20.0))
                    .child(indicator(
                        "left",
                        *left_status.borrow(),
                        "Left connection is healthy",
                    ))
                    .child(indicator(
                        "right",
                        StatusTone::Info,
                        "Right connection is healthy",
                    ))
                    .into_any_element()
            })
        };

        poodle_gpui_node_backend::begin_probe_capture();
        let mut driver = HeadlessDriver::new_element_in_box(cx, build, 320.0, 180.0);
        for id in [
            "status-indicator:left",
            "status-indicator:left:dot",
            "status-indicator:left:label",
            "status-indicator:right",
            "status-indicator:right:dot",
            "status-indicator:right:label",
        ] {
            assert!(
                poodle_gpui_node_backend::bounds_for(id).is_some(),
                "production IntoElement path paints {id}"
            );
        }

        *left_status.borrow_mut() = StatusTone::Danger;
        driver.draw_frame();
        let left = poodle_gpui_node_backend::painted_node_for("status-indicator:left")
            .expect("rebuilt left indicator");
        let right = poodle_gpui_node_backend::painted_node_for("status-indicator:right")
            .expect("stable right indicator");
        assert_eq!(left.roles.get("status").map(String::as_str), Some("danger"));
        assert_eq!(right.roles.get("status").map(String::as_str), Some("info"));
        assert_eq!(
            left.a11y_label.as_deref(),
            Some("Left connection is healthy")
        );
        assert_eq!(
            right.a11y_label.as_deref(),
            Some("Right connection is healthy")
        );

        let channels = poodle_gpui_node_backend::take_probe_capture();
        assert!(channels.contains(&"content.text-icon.icon"));
        assert!(channels.contains(&"content.text-icon.text"));
    });
}
