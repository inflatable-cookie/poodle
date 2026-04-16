//! Icon registry for GPUI — provides SVG element data for icon rendering.
//!
//! Icons are stored as sequences of SVG elements (paths, circles, rects) from
//! the Lucide icon set. The elements are rendered into GPUI using the svg()
//! element with inline SVG markup.

use std::collections::HashMap;
use std::sync::LazyLock;

/// A single SVG element within an icon.
#[derive(Clone, Debug)]
pub enum SvgElement {
    /// SVG path element with a `d` attribute.
    Path(String),
    /// SVG circle element with cx, cy, r.
    Circle { cx: f32, cy: f32, r: f32 },
    /// SVG rect element with x, y, width, height, rx, ry.
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        rx: f32,
        ry: f32,
    },
}

impl SvgElement {
    /// Convert this element to an SVG string fragment.
    pub fn to_svg_string(&self) -> String {
        match self {
            SvgElement::Path(d) => format!(r#"<path d="{d}"/>"#),
            SvgElement::Circle { cx, cy, r } => {
                format!(r#"<circle cx="{cx}" cy="{cy}" r="{r}"/>"#)
            }
            SvgElement::Rect {
                x,
                y,
                width,
                height,
                rx,
                ry,
            } => {
                format!(
                    r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{rx}" ry="{ry}"/>"#
                )
            }
        }
    }
}

/// An icon definition — a named set of SVG elements.
#[derive(Clone, Debug)]
pub struct IconDefinition {
    pub name: &'static str,
    pub elements: Vec<SvgElement>,
}

impl IconDefinition {
    /// Build a complete SVG string for this icon.
    pub fn to_svg(&self, size: f32) -> String {
        let inner: String = self.elements.iter().map(|e| e.to_svg_string()).collect();
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">{inner}</svg>"#
        )
    }
}

/// The global icon registry.
pub static ICON_REGISTRY: LazyLock<HashMap<&'static str, IconDefinition>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Helper macros for concise registration
    macro_rules! path {
        ($d:expr) => {
            SvgElement::Path($d.to_string())
        };
    }
    macro_rules! circle {
        ($cx:expr, $cy:expr, $r:expr) => {
            SvgElement::Circle {
                cx: $cx,
                cy: $cy,
                r: $r,
            }
        };
    }
    macro_rules! rect {
        ($x:expr, $y:expr, $w:expr, $h:expr, $rx:expr, $ry:expr) => {
            SvgElement::Rect {
                x: $x,
                y: $y,
                width: $w,
                height: $h,
                rx: $rx,
                ry: $ry,
            }
        };
    }
    macro_rules! icon {
        ($name:expr, $($elem:expr),+ $(,)?) => {
            map.insert($name, IconDefinition {
                name: $name,
                elements: vec![$($elem),+],
            });
        };
    }

    // --- Navigation ---
    icon!("arrow-left", path!("m12 19-7-7 7-7"), path!("M19 12H5"));
    icon!("arrow-right", path!("M5 12h14"), path!("m12 5 7 7-7 7"));
    icon!("chevron-down", path!("m6 9 6 6 6-6"));
    icon!("chevron-left", path!("m15 18-6-6 6-6"));
    icon!("chevron-right", path!("m9 18 6-6-6-6"));
    icon!("chevron-up", path!("m18 15-6-6-6 6"));
    icon!(
        "menu",
        path!("M4 5h16"),
        path!("M4 12h16"),
        path!("M4 19h16")
    );
    icon!("x", path!("M18 6 6 18"), path!("m6 6 12 12"));

    // --- Actions ---
    icon!("check", path!("M20 6 9 17l-5-5"));
    icon!(
        "copy",
        rect!(8.0, 8.0, 14.0, 14.0, 2.0, 2.0),
        path!("M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2")
    );
    icon!(
        "download",
        path!("M12 15V3"),
        path!("M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"),
        path!("m7 10 5 5 5-5")
    );
    icon!(
        "edit",
        path!("M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z")
    );
    icon!(
        "external-link",
        path!("M15 3h6v6"),
        path!("M10 14 21 3"),
        path!("M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6")
    );
    icon!("minus", path!("M5 12h14"));
    icon!("plus", path!("M5 12h14"), path!("M12 5v14"));
    icon!(
        "save",
        path!("M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"),
        path!("M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"),
        path!("M7 3v4a1 1 0 0 0 1 1h7")
    );
    icon!(
        "trash-2",
        path!("M10 11v6"),
        path!("M14 11v6"),
        path!("M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"),
        path!("M3 6h18"),
        path!("M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2")
    );
    icon!(
        "upload",
        path!("M12 3v12"),
        path!("m17 8-5-5-5 5"),
        path!("M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4")
    );

    // --- Status / feedback ---
    icon!(
        "triangle-alert",
        path!("m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"),
        path!("M12 9v4"),
        path!("M12 17h.01")
    );
    icon!(
        "alert-circle",
        circle!(12.0, 12.0, 10.0),
        path!("M12 8v4"),
        path!("M12 16h.01")
    );
    icon!(
        "check-circle",
        circle!(12.0, 12.0, 10.0),
        path!("m9 12 2 2 4-4")
    );
    icon!(
        "info",
        circle!(12.0, 12.0, 10.0),
        path!("M12 16v-4"),
        path!("M12 8h.01")
    );
    icon!(
        "loader",
        path!("M12 2v4"),
        path!("m16.2 7.8 2.9-2.9"),
        path!("M18 12h4"),
        path!("m16.2 16.2 2.9 2.9"),
        path!("M12 18v4"),
        path!("m4.9 19.1 2.9-2.9"),
        path!("M2 12h4"),
        path!("m4.9 4.9 2.9 2.9")
    );

    // --- Objects / UI ---
    icon!(
        "bell",
        path!("M10.268 21a2 2 0 0 0 3.464 0"),
        path!("M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326")
    );
    icon!(
        "calendar",
        path!("M8 2v4"),
        path!("M16 2v4"),
        rect!(3.0, 4.0, 18.0, 18.0, 2.0, 0.0),
        path!("M3 10h18")
    );
    icon!("clock", circle!(12.0, 12.0, 10.0), path!("M12 6v6l4 2"));
    icon!(
        "eye",
        path!("M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"),
        circle!(12.0, 12.0, 3.0)
    );
    icon!(
        "file",
        path!("M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"),
        path!("M14 2v5a1 1 0 0 0 1 1h5")
    );
    icon!("filter", path!("M22 3H2l8 9.46V19l4 2v-8.54z"));
    icon!(
        "folder",
        path!("M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z")
    );
    icon!(
        "lock",
        rect!(3.0, 11.0, 18.0, 11.0, 2.0, 2.0),
        path!("M7 11V7a5 5 0 0 1 10 0v4")
    );
    icon!(
        "mail",
        path!("m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7"),
        rect!(2.0, 4.0, 20.0, 16.0, 2.0, 0.0)
    );
    icon!(
        "search",
        path!("m21 21-4.34-4.34"),
        circle!(11.0, 11.0, 8.0)
    );
    icon!(
        "settings",
        path!("M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"),
        circle!(12.0, 12.0, 3.0)
    );
    icon!(
        "user",
        path!("M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"),
        circle!(12.0, 12.0, 4.0)
    );

    // --- More ---
    icon!(
        "heart",
        path!("M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5")
    );
    icon!(
        "star",
        path!("M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z")
    );

    map
});

/// Look up an icon by name from the global registry.
pub fn get_icon(name: &str) -> Option<&'static IconDefinition> {
    ICON_REGISTRY.get(name)
}
