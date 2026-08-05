//! Node chrome builder — the old `ui_element` fluent surface, emitting
//! `poodle_node::Node`. Specimens keep their `div().flex_col().gap(..)`
//! shapes; only the vocabulary underneath changed. Colours are sRGB
//! `ColorValue` (the node space); the adapter linearises at the edge.

#![allow(dead_code)]

use poodle_jetstream::JetstreamThemeProvider;
use poodle_node::{
    CrossAxisAlignment, LayoutDirection, LayoutOverflow, LayoutSizing, MainAxisAlignment, Node,
    NodePosition,
};

pub use poodle_node::ColorValue;
pub use poodle_node::FontFamily;

/// Fluent wrapper. `El` derefs into the node at `.into()` / `child()` sites.
#[derive(Clone)]
pub struct El(pub Node);

pub fn div() -> El {
    let mut n = Node::container();
    // The old JsEl div defaulted to Row; Node defaults Column. Keep JsEl
    // semantics so specimen chrome lays out identically.
    n.style.descriptor.layout.direction = LayoutDirection::Row;
    El(n)
}

pub fn label(text: impl Into<String>) -> El {
    El(Node::text(text))
}

pub fn icon(name: impl Into<String>) -> El {
    El(Node::icon(name, 16.0))
}

pub fn button(text: impl Into<String>) -> El {
    El(Node::button(text))
}

impl From<El> for Node {
    fn from(e: El) -> Node {
        e.0
    }
}
impl From<Node> for El {
    fn from(n: Node) -> El {
        El(n)
    }
}

impl El {
    pub fn child(mut self, child: impl Into<Node>) -> Self {
        self.0.children.push(child.into());
        self
    }
    pub fn children<I, T>(mut self, kids: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Node>,
    {
        for k in kids {
            self.0.children.push(k.into());
        }
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.0.id = Some(id.into());
        self
    }

    // ── Layout direction / wrap ──
    pub fn flex_row(mut self) -> Self {
        self.0.style.descriptor.layout.direction = LayoutDirection::Row;
        self
    }
    pub fn flex_col(mut self) -> Self {
        self.0.style.descriptor.layout.direction = LayoutDirection::Column;
        self
    }
    pub fn flex_wrap(mut self) -> Self {
        self.0.style.flex_wrap = true;
        self
    }

    // ── Alignment ──
    pub fn items_center(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Center;
        self
    }
    pub fn items_start(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
        self
    }
    pub fn items_end(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.cross = CrossAxisAlignment::End;
        self
    }
    pub fn justify_center(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.main = MainAxisAlignment::Center;
        self
    }
    pub fn justify_end(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.main = MainAxisAlignment::End;
        self
    }
    pub fn justify_between(mut self) -> Self {
        self.0.style.descriptor.layout.alignment.main = MainAxisAlignment::SpaceBetween;
        self
    }

    // ── Sizing ──
    pub fn w(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.width = LayoutSizing::Fixed(v);
        self
    }
    pub fn h(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.height = LayoutSizing::Fixed(v);
        self
    }
    pub fn size(mut self, v: f32) -> Self {
        if let poodle_node::NodeKind::Icon { size, .. } = &mut self.0.kind {
            *size = v;
        } else {
            self.0.style.descriptor.layout.width = LayoutSizing::Fixed(v);
            self.0.style.descriptor.layout.height = LayoutSizing::Fixed(v);
        }
        self
    }
    pub fn w_full(mut self) -> Self {
        self.0.style.fill_width = true;
        self
    }
    pub fn h_full(mut self) -> Self {
        self.0.style.fill_height = true;
        self
    }
    pub fn min_w(mut self, v: f32) -> Self {
        self.0.style.min_width = Some(v);
        self
    }
    pub fn max_w(mut self, v: f32) -> Self {
        self.0.style.max_width = Some(v);
        self
    }
    pub fn min_h(mut self, v: f32) -> Self {
        self.0.style.min_height = Some(v);
        self
    }
    pub fn max_h(mut self, v: f32) -> Self {
        self.0.style.max_height = Some(v);
        self
    }
    pub fn grow(mut self) -> Self {
        self.0.style.descriptor.layout.width = LayoutSizing::Grow;
        self
    }
    pub fn flex_grow(mut self) -> Self {
        self.0.style.flex_fill = true;
        self
    }
    pub fn flex_1(mut self) -> Self {
        self.0.style.flex_grow = Some(1.0);
        self.0.style.flex_basis = Some(0.0);
        self
    }
    pub fn flex_none(mut self) -> Self {
        self.0.style.flex_none = true;
        self
    }
    pub fn flex_shrink_0(mut self) -> Self {
        self.0.style.flex_shrink_zero = true;
        self
    }
    pub fn self_stretch(mut self) -> Self {
        self.0.style.self_stretch = true;
        self
    }

    // ── Spacing ──
    pub fn gap(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.gap = v;
        self
    }
    pub fn p(mut self, v: f32) -> Self {
        let pad = &mut self.0.style.descriptor.layout.spacing.padding;
        pad.left = v;
        pad.right = v;
        pad.top = v;
        pad.bottom = v;
        self
    }
    pub fn px(mut self, v: f32) -> Self {
        let pad = &mut self.0.style.descriptor.layout.spacing.padding;
        pad.left = v;
        pad.right = v;
        self
    }
    pub fn py(mut self, v: f32) -> Self {
        let pad = &mut self.0.style.descriptor.layout.spacing.padding;
        pad.top = v;
        pad.bottom = v;
        self
    }
    pub fn pl(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.padding.left = v;
        self
    }
    pub fn pr(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.padding.right = v;
        self
    }
    pub fn pt(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.padding.top = v;
        self
    }
    pub fn pb(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.padding.bottom = v;
        self
    }
    pub fn m(mut self, v: f32) -> Self {
        let m = &mut self.0.style.descriptor.layout.spacing.margin;
        m.left = v;
        m.right = v;
        m.top = v;
        m.bottom = v;
        self
    }
    pub fn ml(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.margin.left = v;
        self
    }
    pub fn mr(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.margin.right = v;
        self
    }
    pub fn mt(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.margin.top = v;
        self
    }
    pub fn mb(mut self, v: f32) -> Self {
        self.0.style.descriptor.layout.spacing.margin.bottom = v;
        self
    }

    // ── Paint ──
    pub fn bg(mut self, c: ColorValue) -> Self {
        self.0.style.descriptor.background = Some(c);
        self
    }
    pub fn text_color(mut self, c: ColorValue) -> Self {
        self.0.style.descriptor.text_color = Some(c);
        self
    }
    pub fn text_size(mut self, v: f32) -> Self {
        self.0.style.text_size = Some(v);
        self
    }
    pub fn text_weight(mut self, w: u16) -> Self {
        self.0.style.text_weight = Some(w);
        self
    }
    pub fn font_family(mut self, f: FontFamily) -> Self {
        self.0.style.font_family = Some(f);
        self
    }
    pub fn letter_spacing_em(mut self, v: f32) -> Self {
        self.0.style.letter_spacing_em = Some(v);
        self
    }
    pub fn text_ellipsis(mut self) -> Self {
        self.0.style.text_ellipsis = true;
        self
    }
    pub fn whitespace_nowrap(mut self) -> Self {
        self.0.style.no_wrap = true;
        self
    }
    pub fn border(mut self, w: f32) -> Self {
        self.0.style.descriptor.border.width = w;
        self
    }
    pub fn border_1(mut self) -> Self {
        self.0.style.descriptor.border.width = 1.0;
        self
    }
    pub fn border_color(mut self, c: ColorValue) -> Self {
        self.0.style.descriptor.border.color = c;
        self
    }
    pub fn rounded(mut self, r: f32) -> Self {
        let c = &mut self.0.style.descriptor.corner_radii;
        c.top_left = r;
        c.top_right = r;
        c.bottom_right = r;
        c.bottom_left = r;
        self
    }
    pub fn opacity(mut self, v: f32) -> Self {
        self.0.style.descriptor.opacity = v;
        self
    }
    pub fn overflow_hidden(mut self) -> Self {
        self.0.style.descriptor.layout.overflow_x = LayoutOverflow::Hidden;
        self.0.style.descriptor.layout.overflow_y = LayoutOverflow::Hidden;
        self
    }
    pub fn overflow_scroll(mut self) -> Self {
        self.0.style.descriptor.layout.overflow_x = LayoutOverflow::Scroll;
        self.0.style.descriptor.layout.overflow_y = LayoutOverflow::Scroll;
        self
    }
    pub fn relative(mut self) -> Self {
        self.0.position = NodePosition::Relative;
        self
    }
    pub fn focusable(mut self) -> Self {
        self.0.interaction.focusable = true;
        self
    }
}

// ── Theme helpers (old theme_ext surface, sRGB) ─────────────────────────────

use poodle_adapter::ThemeProvider;

/// Resolve a colour token to sRGB (the node space).
pub fn resolve_color(theme: &JetstreamThemeProvider, token: &str) -> ColorValue {
    ThemeProvider::resolve_color(theme, token)
}

/// Resolve a space/size token to logical pixels.
pub fn resolve_px(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    ThemeProvider::resolve_space(theme, token)
}

/// Resolve a radius token to logical pixels.
pub fn resolve_radius(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    ThemeProvider::resolve_radius(theme, token)
}

/// Resolve an opacity token.
pub fn resolve_opacity(theme: &JetstreamThemeProvider, token: &str) -> f32 {
    ThemeProvider::resolve_opacity(theme, token)
}

/// Alpha tint: multiply the colour's alpha by `fraction`.
pub fn tint(c: ColorValue, fraction: f32) -> ColorValue {
    ColorValue(c.0, c.1, c.2, c.3 * fraction)
}

/// Token-resolved `elevation.overlay` shadow.
pub fn elevation_overlay(mut el: El) -> El {
    el.0.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_OVERLAY);
    el
}

/// Token-resolved `elevation.dialog` shadow.
pub fn elevation_dialog(mut el: El) -> El {
    el.0.style.descriptor.shadow = Some(poodle_tokens::typed::semantic::ELEVATION_DIALOG);
    el
}

/// 1rem = 16 logical px (the preview's fixed rem base).
pub fn rem_to_px(rem: f32) -> f32 {
    rem * 16.0
}

impl El {
    pub fn font_weight(self, w: u16) -> Self {
        self.text_weight(w)
    }
    pub fn border_b_1(mut self) -> Self {
        self.0.style.border_bottom_width = Some(1.0);
        self
    }
    pub fn border_t_1(mut self) -> Self {
        self.0.style.border_top_width = Some(1.0);
        self
    }
    pub fn cursor_pointer(mut self) -> Self {
        self.0.style.descriptor.cursor = poodle_node::CursorHint::Pointer;
        self
    }
    pub fn text_center(mut self) -> Self {
        self.0.style.text_align = Some(poodle_node::TextAlign::Center);
        self
    }
    pub fn min_w_0(mut self) -> Self {
        self.0.style.min_width = Some(0.0);
        self
    }
    pub fn min_h_0(mut self) -> Self {
        self.0.style.min_height = Some(0.0);
        self
    }
    /// Small drop shadow (the old `shadow_sm`).
    pub fn shadow_sm(mut self) -> Self {
        self.0.style.shadow_layers = vec![poodle_node::ShadowLayer {
            offset_x: 0.0,
            offset_y: 1.0,
            blur: 3.0,
            spread: 0.0,
            color: ColorValue(0.0, 0.0, 0.0, 0.25),
            inset: false,
        }];
        self
    }
    /// Render above normal content (the old `overlay`).
    pub fn overlay(mut self) -> Self {
        self.0.style.overlay = true;
        self
    }
    /// Conditional combinator (the old `when`).
    pub fn when(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
    }
}

impl El {
    /// Absolute positioning with 0-insets seeded; combine with left/top.
    pub fn absolute(mut self) -> Self {
        self.0.position = NodePosition::Absolute {
            top: None,
            left: None,
            right: None,
            bottom: None,
        };
        self
    }
    pub fn left(mut self, v: f32) -> Self {
        if let NodePosition::Absolute { left, .. } = &mut self.0.position {
            *left = Some(v);
        }
        self
    }
    pub fn top(mut self, v: f32) -> Self {
        if let NodePosition::Absolute { top, .. } = &mut self.0.position {
            *top = Some(v);
        }
        self
    }
    pub fn right(mut self, v: f32) -> Self {
        if let NodePosition::Absolute { right, .. } = &mut self.0.position {
            *right = Some(v);
        }
        self
    }
    pub fn bottom(mut self, v: f32) -> Self {
        if let NodePosition::Absolute { bottom, .. } = &mut self.0.position {
            *bottom = Some(v);
        }
        self
    }
    pub fn inset_0(mut self) -> Self {
        self.0.position = NodePosition::Absolute {
            top: Some(0.0),
            left: Some(0.0),
            right: Some(0.0),
            bottom: Some(0.0),
        };
        self
    }
}
