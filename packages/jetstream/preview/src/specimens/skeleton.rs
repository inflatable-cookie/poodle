//! Skeleton specimen — placeholder loading elements.

use jetstream_runtime::ui_element::*;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::skeleton::js_skeleton;
use poodle_jetstream_components::theme_ext::*;
use poodle_components::SkeletonSpec;

pub fn render(theme: &JetstreamThemeProvider) -> JsEl {
    let secondary = resolve_color(theme, "color.text.secondary");

    div().flex_col().gap(24.0)
        .child(group("Default skeleton", secondary,
            div().flex_col().gap(8.0).w(300.0)
                .child(js_skeleton(&SkeletonSpec::new(), theme))
                .child(js_skeleton(&SkeletonSpec::new(), theme))
                .child(js_skeleton(&SkeletonSpec::new(), theme))
        ))
        .child(group("Circle skeleton", secondary,
            div().flex_row().gap(12.0)
                .child(js_skeleton(&SkeletonSpec::new().with_shape("circle"), theme).w(40.0).h(40.0))
                .child(div().flex_col().gap(8.0).grow()
                    .child(js_skeleton(&SkeletonSpec::new(), theme))
                    .child(js_skeleton(&SkeletonSpec::new(), theme))
                )
        ))
}

fn group(title: &str, text_secondary: glam::Vec4, content: JsEl) -> JsEl {
    div().flex_col().gap(8.0)
        .child(label(title).text_color(text_secondary).text_size(11.0))
        .child(content)
}
