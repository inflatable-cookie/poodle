//! BlockEditor — block-based content editor backed by BlockEditorSpec.
//!
//! Pure shell: renders the contract chrome (per-block toolbar with drag grip,
//! TypeSelect, move buttons, AddSelect, remove button) plus a content area that
//! renders each block by its type. Block payloads are consumer-owned; the shell
//! resolves all spacing/typography from tokens (recipe rems via `rem_to_px`).
//!
//! Interactivity (editing, type change, add/remove/reorder via the selects and
//! buttons) is preview-event-loop bound and intentionally not wired here — the
//! controls render at the current spec state. ARIA (`role="group"`, aria-labels)
//! is an accepted GPUI limitation (no accessibility channel).

use crate::presentation::rem_to_px;
use crate::primitives::{Icon, Select};
use crate::theme_ext::{resolve_color, resolve_opacity, resolve_radius};
use gpui::*;
use poodle_gpui::GpuiThemeProvider;
use poodle_specs::BlockEditorSpec;
use poodle_specs::{
    BlockEditorMode, ChoiceOption, ControlDensity, ControlSize, EditorBlock, IconSize, IconSpec,
    SelectSpec, SelectVariant, SemanticControlSizeRole,
};

pub struct BlockEditor {
    spec: BlockEditorSpec,
    theme: GpuiThemeProvider,
    children: Vec<AnyElement>,
}

impl std::ops::Deref for BlockEditor {
    type Target = BlockEditorSpec;
    fn deref(&self) -> &BlockEditorSpec {
        &self.spec
    }
}

impl BlockEditor {
    pub fn new(theme: &GpuiThemeProvider) -> Self {
        Self {
            spec: BlockEditorSpec::new(),
            theme: theme.clone(),
            children: Vec::new(),
        }
    }
    pub fn from_spec(spec: BlockEditorSpec, theme: &GpuiThemeProvider) -> Self {
        Self {
            spec,
            theme: theme.clone(),
            children: Vec::new(),
        }
    }
    /// Legacy: provide pre-rendered block content elements (used by older
    /// specimens that don't drive blocks through the spec). When `spec.blocks`
    /// is populated these are ignored in favour of spec-driven rendering.
    pub fn with_child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    pub fn with_size(mut self, v: ControlSize) -> Self {
        self.spec.size = v;
        self
    }
    pub fn with_size_role(mut self, v: SemanticControlSizeRole) -> Self {
        self.spec.size_role = v;
        self
    }
    pub fn with_density(mut self, v: ControlDensity) -> Self {
        self.spec.density = v;
        self
    }
}

/// Contract `--poodle-block-editor-control-size` per size (rem → px).
fn control_size_px(size: ControlSize) -> Pixels {
    let rem = match size {
        ControlSize::Xs => 1.25,
        ControlSize::Sm => 1.5,
        ControlSize::Md => 1.75,
        ControlSize::Lg => 2.0,
        ControlSize::Xl => 2.25,
    };
    px(rem_to_px(rem))
}

/// Contract density recipe rems: (toolbar-y, toolbar-x, content-x, content-y,
/// stack-gap). toolbar-gap is the same across densities (`0.125rem`).
fn density_recipe(density: ControlDensity) -> (f32, f32, f32, f32, f32) {
    match density {
        // toolbar-y, toolbar-x, content-x, content-y, stack-gap
        ControlDensity::Compact => (0.1875, 0.25, 0.375, 0.25, 0.375),
        ControlDensity::Default => (0.25, 0.375, 0.5, 0.375, 0.5),
        ControlDensity::Comfortable => (0.3125, 0.5, 0.625, 0.5, 0.625),
    }
}

/// Build a contract tool button (`.block-editor__tool-btn`): control-size box,
/// rounded, hover-bg; `disabled` dims to the contract `0.3` opacity. Optional
/// `danger` flag tints hover for the remove button.
fn tool_btn(
    theme: &GpuiThemeProvider,
    icon_name: &str,
    icon_size: IconSize,
    color: Hsla,
    hover_bg: Hsla,
    control_size: Pixels,
    radius: Pixels,
    disabled: bool,
) -> Div {
    let mut btn = div()
        .flex()
        .items_center()
        .justify_center()
        .w(control_size)
        .h(control_size)
        .rounded(radius)
        .child(Icon::from_spec(IconSpec::new(icon_name).with_size(icon_size), theme).with_color(color));

    if disabled {
        // Contract `.block-editor__tool-btn:disabled` opacity 0.3
        btn = btn.opacity(0.3);
    } else {
        btn = btn
            .cursor(CursorStyle::PointingHand)
            .hover(move |s| s.bg(hover_bg));
    }
    btn
}

impl BlockEditor {
    /// Render a single block's content by its type. Block payloads are
    /// consumer-owned; this renders the legacy `content` string with type-aware
    /// presentation (heading / quote / code / list / paragraph). Unknown types
    /// fall back to body paragraph rendering.
    fn render_block_content(
        &self,
        block: &EditorBlock,
        content_x: Pixels,
        content_y: Pixels,
    ) -> AnyElement {
        let theme = &self.theme;
        let text_primary = resolve_color(theme, "color.text.primary");
        let text_secondary = resolve_color(theme, "color.text.secondary");
        let accent = resolve_color(theme, "color.accent.base");
        let elevated = resolve_color(theme, "color.background.elevated");
        let radius_control = resolve_radius(theme, "radius.control");

        let body_size = px(rem_to_px(0.875)); // contract input font-size
        let heading_size = px(rem_to_px(1.125));
        let code_size = px(rem_to_px(0.8125));

        // Content padding (`.block-editor__content`), min-height 1.5rem.
        let container = div()
            .flex()
            .flex_col()
            .px(content_x)
            .py(content_y)
            .min_h(px(rem_to_px(1.5)));

        let text = block.content.clone().unwrap_or_default();

        match block.block_type.as_str() {
            "heading" | "heading-1" | "heading-2" => container
                .child(
                    div()
                        .text_size(heading_size)
                        .font_weight(FontWeight::BOLD)
                        .text_color(text_primary)
                        .child(text),
                )
                .into_any_element(),
            "quote" | "blockquote" => container
                .child(
                    div()
                        .border_l_2()
                        .border_color(accent)
                        .pl(content_x)
                        .text_size(body_size)
                        .italic()
                        .text_color(text_secondary)
                        .child(text),
                )
                .into_any_element(),
            "code" => container
                .child(
                    div()
                        .bg(Hsla {
                            a: elevated.a * 0.4,
                            ..elevated
                        })
                        .rounded(radius_control)
                        .px(content_x)
                        .py(content_y)
                        .font_family("monospace")
                        .text_size(code_size)
                        .text_color(text_primary)
                        .child(text),
                )
                .into_any_element(),
            "list" | "bulleted-list" => {
                let mut list = div().flex().flex_col();
                for line in text.lines() {
                    list = list.child(
                        div()
                            .text_size(body_size)
                            .text_color(text_primary)
                            .child(format!("\u{2022} {}", line)),
                    );
                }
                container.child(list).into_any_element()
            }
            // paragraph / unknown → body paragraph
            _ => container
                .child(
                    div()
                        .text_size(body_size)
                        .text_color(text_primary)
                        .child(text),
                )
                .into_any_element(),
        }
    }

    /// Build a ghost Select seeded from `block_types`. When `value` is `Some`,
    /// the trigger shows the matching label (TypeSelect); when `None` the menu
    /// is the bare add-block picker (AddSelect). Callbacks are preview-loop
    /// bound and intentionally omitted. Icons per option are not rendered —
    /// `ChoiceOption` carries no icon field (accepted Select-primitive gap).
    fn build_type_select(&self, value: Option<&str>, disabled: bool) -> Select {
        let options: Vec<ChoiceOption> = self
            .spec
            .block_types
            .iter()
            .map(|t| ChoiceOption::new(t.block_type.clone(), t.label.clone()))
            .collect();

        let mut sel = SelectSpec::default()
            .with_variant(SelectVariant::Ghost)
            .with_menu_min_width("10rem");
        sel.options = options;
        sel.is_disabled = disabled;
        sel.size = self.spec.size;
        sel.size_role = self.spec.size_role;
        sel.density = self.spec.density;
        if let Some(v) = value {
            sel = sel.with_value(v);
        }

        Select::from_spec(sel, &self.theme)
    }
}

impl IntoElement for BlockEditor {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        let theme = &self.theme;

        // ── Resolve chrome from tokens / contract recipe rems ───────────────
        let fill = resolve_color(theme, self.spec.fill_token());
        let radius_control = resolve_radius(theme, "radius.control");
        let elevated = resolve_color(theme, "color.background.elevated");
        let text_tertiary = resolve_color(theme, "color.text.tertiary");
        let danger = resolve_color(theme, "color.status.danger");
        let accent = resolve_color(theme, "color.accent.base");

        let control_size = control_size_px(self.spec.size);
        let (toolbar_y, toolbar_x, content_x, content_y, stack_gap) =
            density_recipe(self.spec.density);
        let toolbar_y = px(rem_to_px(toolbar_y));
        let toolbar_x = px(rem_to_px(toolbar_x));
        let content_x = px(rem_to_px(content_x));
        let content_y = px(rem_to_px(content_y));
        let stack_gap = px(rem_to_px(stack_gap));
        let toolbar_gap = px(rem_to_px(0.125)); // `--poodle-block-editor-toolbar-gap`

        // Per-block background mixes (contract: elevated 42%).
        let block_bg = Hsla {
            a: elevated.a * 0.42,
            ..elevated
        };
        // Tool-button hover bg: accent 16% (contract `.tool-btn:hover`).
        let tool_hover_bg = Hsla {
            a: accent.a * 0.16,
            ..accent
        };
        // Remove-button hover bg: danger 16% (contract `.remove-btn:hover`).
        let remove_hover_bg = Hsla {
            a: danger.a * 0.16,
            ..danger
        };

        // ── Posture flags (mode + explicit overrides) ───────────────────────
        let is_multi = self.spec.mode == BlockEditorMode::Multi;
        let can_reorder = self.spec.allow_reorder.unwrap_or(is_multi);
        let can_add = self.spec.allow_add.unwrap_or(is_multi);
        let can_remove = self.spec.allow_remove.unwrap_or(is_multi);
        let can_type_change = self.spec.allow_type_change.unwrap_or(true);
        let disabled = self.spec.is_disabled;

        // ── Root: flex column, surface bg, stack-gap, no border/radius/pad ──
        let mut root = div().flex().flex_col().gap(stack_gap).bg(fill);

        if !self.spec.blocks.is_empty() {
            let block_count = self.spec.blocks.len();
            for (i, block) in self.spec.blocks.iter().enumerate() {
                // ── Toolbar left: drag grip + TypeSelect ───────────────────
                let mut toolbar_left = div().flex().items_center().gap(toolbar_gap);
                if can_reorder {
                    // `.block-editor__drag-grip` — control-size box, tertiary icon.
                    toolbar_left = toolbar_left.child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(control_size)
                            .h(control_size)
                            .rounded(radius_control)
                            .cursor(CursorStyle::OpenHand)
                            .child(
                                Icon::from_spec(
                                    IconSpec::new("grip-vertical").with_size(IconSize::Sm),
                                    theme,
                                )
                                .with_color(text_tertiary),
                            ),
                    );
                }
                if can_type_change {
                    // TypeSelect inset margin-left when reorder is disabled
                    // (grip hidden) to keep alignment with block content:
                    // calc(content-x + input-x − toolbar-x). input-x default 0.375rem.
                    let mut type_wrap = div().flex_shrink_0();
                    if !can_reorder {
                        let input_x = px(rem_to_px(0.375));
                        let inset = content_x + input_x - toolbar_x;
                        if inset > px(0.0) {
                            type_wrap = type_wrap.ml(inset);
                        }
                    }
                    toolbar_left = toolbar_left.child(
                        type_wrap.child(self.build_type_select(Some(&block.block_type), disabled)),
                    );
                }

                // ── Toolbar right: move up/down, AddSelect, remove ─────────
                let mut toolbar_right = div().flex().items_center().gap(toolbar_gap);
                if can_reorder {
                    toolbar_right = toolbar_right
                        .child(tool_btn(
                            theme,
                            "arrow-up",
                            IconSize::Sm,
                            text_tertiary,
                            tool_hover_bg,
                            control_size,
                            radius_control,
                            disabled || i == 0,
                        ))
                        .child(tool_btn(
                            theme,
                            "arrow-down",
                            IconSize::Sm,
                            text_tertiary,
                            tool_hover_bg,
                            control_size,
                            radius_control,
                            disabled || i == block_count - 1,
                        ));
                }
                if can_add {
                    // AddSelect: ghost Select with plus-icon trigger. The Select
                    // primitive has no trigger-slot override, so the plus tool
                    // button sits ahead of the (value-less) picker — closest
                    // faithful subset of the contract's trigger-slot pattern.
                    toolbar_right = toolbar_right
                        .child(tool_btn(
                            theme,
                            "plus",
                            IconSize::Sm,
                            text_tertiary,
                            tool_hover_bg,
                            control_size,
                            radius_control,
                            disabled,
                        ))
                        .child(div().flex_shrink_0().child(self.build_type_select(None, disabled)));
                }
                if can_remove && block_count > 1 {
                    toolbar_right = toolbar_right.child(tool_btn(
                        theme,
                        "x",
                        IconSize::Sm,
                        text_tertiary,
                        remove_hover_bg,
                        control_size,
                        radius_control,
                        disabled,
                    ));
                }

                let toolbar = div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .py(toolbar_y)
                    .px(toolbar_x)
                    .child(toolbar_left)
                    .child(toolbar_right);

                // ── Block: flex column, elevated-42% bg, control radius ────
                let block_el = div()
                    .flex()
                    .flex_col()
                    .rounded(radius_control)
                    .bg(block_bg)
                    .child(toolbar)
                    .child(self.render_block_content(block, content_x, content_y));

                root = root.child(block_el);
            }
        } else {
            // Legacy `with_child` path: wrap each pre-rendered child in a block
            // shell so spacing/background stay contract-consistent. No toolbar
            // (no block-type metadata available for the selects).
            for child in self.children {
                root = root.child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(radius_control)
                        .bg(block_bg)
                        .child(div().px(content_x).py(content_y).child(child)),
                );
            }
        }

        if disabled {
            root = root.opacity(resolve_opacity(theme, "state.opacity.disabled"));
        }
        root.into_any_element()
    }
}
