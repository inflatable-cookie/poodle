use poodle_node::Node;
use poodle_render::{token_input, RenderContext};
use poodle_specs::{ControlSize, TokenInputSpec};

use crate::{column, control_density, control_size, section, View};

fn size_label(size: ControlSize) -> &'static str {
    match size {
        ControlSize::Xs => "XS",
        ControlSize::Sm => "SM",
        ControlSize::Md => "MD",
        ControlSize::Lg => "LG",
        ControlSize::Xl => "XL",
    }
}

pub(crate) fn examples(ctx: &RenderContext<'_>) -> Node {
    let mut default_spec = TokenInputSpec::new()
        .with_values(vec!["ifrs".into(), "tax".into(), "audit".into()])
        .with_placeholder("Type a tag...");
    default_spec.id = "blog-tags".into();
    default_spec.name = Some("tags".into());

    let mut multi_sep = TokenInputSpec::new()
        .with_values(vec!["draft".into(), "review".into()])
        .with_placeholder("draft; review; live");
    multi_sep.separators = vec![",".into(), ";".into()];

    let empty = TokenInputSpec::new().with_placeholder("Type a tag...");

    let mut max_len = TokenInputSpec::new()
        .with_values(vec!["abbr".into()])
        .with_placeholder("Max 8 chars...");
    max_len.max_length = Some(8);

    let long_tags = TokenInputSpec::new()
        .with_values(vec![
            "legacy-migration".into(),
            "a-very-long-tag-that-should-wrap-inside-the-token-without-breaking-the-field".into(),
        ])
        .with_placeholder("Type a label...");
    let mut narrow_wrap = column(0.0, vec![token_input(&long_tags, ctx, None)]);
    narrow_wrap.style.max_width = Some(288.0);

    let read_only = TokenInputSpec::new()
        .with_values(vec!["premium".into(), "acca".into(), "2027".into()])
        .with_read_only(true);

    let disabled = TokenInputSpec::new()
        .with_values(vec!["finance".into(), "ethics".into()])
        .with_disabled(true);

    let mut body = column(
        20.0,
        vec![
            section("Default", [token_input(&default_spec, ctx, None)], ctx),
            section(
                "Multiple separators (comma + semicolon)",
                [token_input(&multi_sep, ctx, None)],
                ctx,
            ),
            section(
                "Empty (placeholder only)",
                [token_input(&empty, ctx, None)],
                ctx,
            ),
            section(
                "Max length draft (8 chars)",
                [token_input(&max_len, ctx, None)],
                ctx,
            ),
            section("Narrow and long values", [narrow_wrap], ctx),
            section("Read only", [token_input(&read_only, ctx, None)], ctx),
            section("Disabled", [token_input(&disabled, ctx, None)], ctx),
        ],
    );
    body.style.max_width = Some(560.0);
    body
}

pub(crate) fn axis(view: View, ctx: &RenderContext<'_>) -> Node {
    let spec = match view {
        View::Size(value) => {
            let size = control_size(value);
            TokenInputSpec::new()
                .with_values(vec!["alpha".into(), "beta".into()])
                .with_size(size)
                .with_placeholder(size_label(size))
        }
        View::Density(value) => TokenInputSpec::new()
            .with_values(vec!["audit".into(), "reporting".into()])
            .with_density(control_density(value))
            .with_placeholder("Type a tag..."),
        View::Examples => unreachable!("Examples uses the session"),
    };
    token_input(&spec, ctx, None)
}
