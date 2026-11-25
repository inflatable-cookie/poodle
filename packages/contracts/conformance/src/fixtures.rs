use std::sync::{Arc, Mutex};

use poodle_node::{
    ColorValue, CrossAxisAlignment, HasPopup, LayoutDirection, LayoutEdges, LayoutSizing,
    MainAxisAlignment, Node, NodeKind, NodePosition, NodeRole, NodeToggled, ResolvedIconContour,
    ResolvedIconGeometryFrame,
};
use serde_json::{json, Value};

use crate::inventory::{kind_name, role_name, ROLES};
use crate::{Action, Case, Check, Probe, Proof, Sprite};

/// Named icon fixture asset, resolved by the backend's ordinary icon host.
pub const NAMED_ICON: &str = "conformance-icon";
/// SVG bytes for the named icon fixture.
pub const NAMED_ICON_SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24"><rect x="4" y="4" width="16" height="16" fill="black"/></svg>"#;
/// Embedded image fixture asset, resolved by the backend's ordinary image host.
pub const IMAGE_ASSET: &str = "conformance-image.svg";
/// SVG bytes for the image fixture.
pub const IMAGE_SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="12"><rect width="16" height="12" fill="blue"/></svg>"#;

const TARGET: &str = "target";
const VIEWPORT: [f32; 2] = [96.0, 72.0];
const RED: [u8; 3] = [255, 0, 0];
const BLUE: [u8; 3] = [0, 0, 255];

fn id(mut node: Node, id: &str) -> Node {
    node.id = Some(id.into());
    node
}

fn sized(mut node: Node, width: f32, height: f32) -> Node {
    node.style.descriptor.layout.width = LayoutSizing::Fixed(width);
    node.style.descriptor.layout.height = LayoutSizing::Fixed(height);
    node
}

fn box_with(id_value: &str, width: f32, height: f32) -> Node {
    id(sized(Node::container(), width, height), id_value)
}

fn case(id_value: impl Into<String>, proof: Proof, root: Node, checks: Vec<Check>) -> Case {
    Case {
        id: id_value.into(),
        proof,
        viewport: VIEWPORT,
        root: id(root, "root"),
        checks,
        actions: Vec::new(),
        expected_events: &[],
        recorded: Arc::new(Mutex::new(Vec::new())),
    }
}

fn scene(children: impl IntoIterator<Item = Node>) -> Node {
    let mut root = sized(Node::container(), VIEWPORT[0], VIEWPORT[1]);
    root.style.descriptor.layout.alignment.cross = CrossAxisAlignment::Start;
    root.children(children)
}

fn exact(target: &'static str, probe: Probe, expected: Value) -> Check {
    Check {
        target,
        probe,
        expected,
        tolerance: None,
    }
}

fn bounds(target: &'static str, expected: [f32; 4]) -> Check {
    Check {
        target,
        probe: Probe::Bounds,
        expected: json!(expected),
        tolerance: Some(0.5),
    }
}

fn paint(rgb: [u8; 3], points: &[([f32; 2], bool)]) -> Vec<Check> {
    points
        .iter()
        .map(|&(point, covered)| exact(TARGET, Probe::Paint { point, rgb }, json!(covered)))
        .collect()
}

fn kind(node: Node, proof: Proof, siblings: Vec<Node>, checks: Vec<Check>) -> Case {
    let name = kind_name(&node.kind);
    case(
        format!("kind.{name}"),
        proof,
        scene([id(node, TARGET)].into_iter().chain(siblings)),
        checks,
    )
}

fn intrinsic_text(node: Node, short: Node, mut extra: Vec<Node>) -> Case {
    extra.insert(0, id(short, "short"));
    kind(
        node,
        Proof::Layout,
        extra,
        vec![exact(TARGET, Probe::WiderThan("short"), json!(true))],
    )
}

fn kind_cases() -> Vec<Case> {
    let mut container = sized(Node::container(), 24.0, 24.0);
    container.children.push(box_with("child", 8.0, 6.0));

    let mut bar = sized(
        Node {
            kind: NodeKind::Progress { fraction: 0.25 },
            ..Node::default()
        },
        40.0,
        8.0,
    );
    bar.style.descriptor.text_color = Some(ColorValue::rgba(1.0, 0.0, 0.0, 1.0));

    let mut ring = sized(Node::progress_ring(0.25), 24.0, 24.0);
    ring.style.descriptor.border.color = ColorValue::rgba(0.0, 0.0, 1.0, 1.0);
    ring.style.descriptor.text_color = bar.style.descriptor.text_color;
    let mut ring_checks = paint(
        RED,
        &[
            ([14.15, 1.2], true),
            ([19.78, 4.22], true),
            ([22.8, 9.85], true),
            ([23.0, 14.0], false),
            ([12.0, 23.0], false),
            ([1.0, 12.0], false),
            ([12.0, 12.0], false),
        ],
    );
    ring_checks.extend(paint(
        BLUE,
        &[
            ([1.0, 12.0], true),
            ([12.0, 23.0], true),
            ([12.0, 1.0], true),
            ([23.0, 12.0], true),
            ([12.0, 12.0], false),
        ],
    ));

    let mut contour = Node::resolved_icon_geometry(
        24.0,
        ResolvedIconGeometryFrame {
            contours: vec![ResolvedIconContour {
                closed: true,
                points: vec![(40_000, 40_000), (200_000, 40_000), (200_000, 180_000)],
            }],
        },
    );
    contour.style.descriptor.text_color = bar.style.descriptor.text_color;

    let input = kind(
        sized(Node::input("filled value", "unused prompt"), 80.0, 20.0),
        Proof::Layout,
        vec![id(
            sized(Node::input("", "shown prompt"), 80.0, 20.0),
            "empty",
        )],
        vec![
            exact(TARGET, Probe::ShapedText, json!("filled value")),
            exact("empty", Probe::ShapedText, json!("shown prompt")),
        ],
    );

    vec![
        kind(
            container,
            Proof::Layout,
            vec![],
            vec![
                bounds(TARGET, [0.0, 0.0, 24.0, 24.0]),
                bounds("child", [0.0, 0.0, 8.0, 6.0]),
            ],
        ),
        intrinsic_text(Node::text("text content"), Node::text("x"), vec![]),
        intrinsic_text(Node::button("button label"), Node::button("x"), vec![]),
        input,
        kind(
            Node::icon(NAMED_ICON, 24.0),
            Proof::Paint,
            vec![],
            vec![
                bounds(TARGET, [0.0, 0.0, 24.0, 24.0]),
                exact(TARGET, Probe::Sprite(Sprite::Mask), json!(true)),
            ],
        ),
        kind(
            sized(
                Node {
                    kind: NodeKind::Image {
                        source: IMAGE_ASSET.into(),
                    },
                    ..Node::default()
                },
                32.0,
                24.0,
            ),
            Proof::Paint,
            vec![],
            vec![
                bounds(TARGET, [0.0, 0.0, 32.0, 24.0]),
                exact(TARGET, Probe::Sprite(Sprite::Image), json!(true)),
            ],
        ),
        kind(
            bar,
            Proof::Paint,
            vec![],
            paint(RED, &[([6.0, 4.0], true), ([11.0, 4.0], false)]),
        ),
        kind(ring, Proof::Paint, vec![], ring_checks),
        kind(
            contour,
            Proof::Paint,
            vec![],
            paint(
                RED,
                &[
                    ([11.0, 4.0], true),
                    ([20.0, 11.0], true),
                    ([12.0, 11.0], true),
                    ([15.0, 8.0], false),
                    ([4.0, 18.0], false),
                ],
            ),
        ),
    ]
}

fn a11y_node(role: NodeRole, target: &str) -> Node {
    let mut node = box_with(target, 24.0, 16.0);
    node.a11y.role = Some(role);
    node
}

fn a11y_cases() -> impl Iterator<Item = Case> {
    use NodeRole::*;
    // Each field rides on a role that gives it meaning. Relationship targets
    // are actual siblings rather than fabricated identifiers in the output.
    macro_rules! a11y {
        ($field:ident, $role:ident, $value:expr, $expected:expr) => {{
            let mut node = a11y_node($role, TARGET);
            node.a11y.$field = $value;
            case(
                concat!("a11y.", stringify!($field)),
                Proof::Accessibility,
                scene([node]),
                vec![exact(
                    TARGET,
                    Probe::A11y(stringify!($field)),
                    json!($expected),
                )],
            )
        }};
    }
    macro_rules! relation {
        ($field:ident, $role:ident, $related:ident) => {{
            let mut node = a11y_node($role, TARGET);
            node.a11y.$field = Some("related".into());
            case(
                concat!("a11y.", stringify!($field)),
                Proof::Accessibility,
                scene([node, a11y_node($related, "related")]),
                vec![exact(
                    TARGET,
                    Probe::A11y(stringify!($field)),
                    json!(["related"]),
                )],
            )
        }};
    }

    let mut first = a11y_node(Button, TARGET);
    first.interaction.focusable = true;
    first.a11y.initial_focus = true;
    let initial_focus = case(
        "a11y.initial_focus",
        Proof::Accessibility,
        scene([first]),
        vec![exact(TARGET, Probe::Focused, json!(true))],
    );

    let mut skipped = a11y_node(Button, TARGET);
    skipped.interaction.focusable = true;
    skipped.a11y.tab_index = Some(-1);
    let tab_index = case(
        "a11y.tab_index",
        Proof::Accessibility,
        scene([skipped]),
        vec![
            exact(TARGET, Probe::A11y("focus_action"), json!(true)),
            exact(TARGET, Probe::TabStops, json!(0)),
        ],
    );

    [
        a11y!(role, Group, Some(Heading), "Heading"),
        a11y!(label, Button, Some("Run report".into()), "Run report"),
        a11y!(expanded, TreeItem, Some(true), true),
        a11y!(selected, ListBoxOption, Some(true), true),
        a11y!(multiselectable, ListBox, Some(true), true),
        a11y!(hidden, Group, Some(true), true),
        initial_focus,
        tab_index,
        relation!(controls, Button, Region),
        a11y!(has_popup, Button, Some(HasPopup::Menu), "Menu"),
        relation!(labelled_by, TextInput, Label),
        a11y!(orientation, Slider, Some("vertical".into()), "Vertical"),
        a11y!(toggled, CheckBox, Some(NodeToggled::Mixed), "Mixed"),
        a11y!(level, Heading, Some(3), 3),
        a11y!(value, Slider, Some(0.25), 0.25),
        a11y!(value_min, Slider, Some(-2.5), -2.5),
        a11y!(value_max, Slider, Some(2.5), 2.5),
        a11y!(value_text, Slider, Some("quarter".into()), "quarter"),
        a11y!(invalid, TextInput, Some(true), true),
        a11y!(busy, Status, Some(true), true),
        relation!(described_by, TextInput, Label),
    ]
    .into_iter()
}

fn layout_cases() -> [Case; 4] {
    let flow = |direction| {
        let mut root = sized(Node::container(), VIEWPORT[0], VIEWPORT[1]);
        root.style.descriptor.layout = root
            .style
            .descriptor
            .layout
            .with_direction(direction)
            .with_padding(LayoutEdges::uniform(4.0))
            .with_gap(6.0)
            .with_alignment(MainAxisAlignment::Start, CrossAxisAlignment::Start);
        root.children([box_with("first", 20.0, 12.0), box_with("second", 10.0, 8.0)])
    };
    let mut grow = box_with(TARGET, 10.0, 10.0);
    grow.style.descriptor.layout.height = LayoutSizing::Grow;
    let column = scene([box_with("first", 10.0, 12.0), grow]);
    let mut absolute = box_with(TARGET, 16.0, 12.0);
    absolute.position = NodePosition::Absolute {
        left: Some(30.0),
        top: Some(18.0),
        right: None,
        bottom: None,
    };
    let mut anchor = scene([absolute]);
    anchor.position = NodePosition::Relative;
    [
        case(
            "layout.row",
            Proof::Layout,
            flow(LayoutDirection::Row),
            vec![
                bounds("first", [4.0, 4.0, 20.0, 12.0]),
                bounds("second", [30.0, 4.0, 10.0, 8.0]),
            ],
        ),
        case(
            "layout.column",
            Proof::Layout,
            flow(LayoutDirection::Column),
            vec![
                bounds("first", [4.0, 4.0, 20.0, 12.0]),
                bounds("second", [4.0, 22.0, 10.0, 8.0]),
            ],
        ),
        case(
            "layout.height_grow",
            Proof::Layout,
            column,
            vec![bounds(TARGET, [0.0, 12.0, 10.0, 60.0])],
        ),
        case(
            "layout.absolute",
            Proof::Layout,
            anchor,
            vec![bounds(TARGET, [30.0, 18.0, 16.0, 12.0])],
        ),
    ]
}

fn activate_case() -> Case {
    let mut case = case(
        "dispatch.activate",
        Proof::Dispatch,
        Node::container(),
        Vec::new(),
    );
    let events = Arc::clone(&case.recorded);
    let mut node = id(Node::button("Activate"), TARGET);
    node.interaction.on_activate = Some(Arc::new(move || {
        events
            .lock()
            .expect("fixture event recorder poisoned")
            .push("activate");
    }));
    case.root = id(scene([node]), "root");
    case.actions = vec![Action::Activate(TARGET)];
    case.expected_events = &["activate"];
    case
}

/// Build a fresh fixture corpus in stable case order.
pub fn cases() -> impl Iterator<Item = Case> {
    kind_cases()
        .into_iter()
        .chain(ROLES.iter().copied().map(|role| {
            let name = role_name(role);
            let expected = if role == NodeRole::Presentation {
                Value::Null
            } else {
                json!(name)
            };
            case(
                format!("role.{name}"),
                Proof::Accessibility,
                scene([a11y_node(role, TARGET)]),
                vec![
                    bounds(TARGET, [0.0, 0.0, 24.0, 16.0]),
                    exact(TARGET, Probe::A11y("role"), expected),
                ],
            )
        }))
        .chain(a11y_cases())
        .chain(layout_cases())
        .chain([activate_case()])
}
