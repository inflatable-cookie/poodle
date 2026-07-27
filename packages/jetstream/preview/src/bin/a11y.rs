//! Project every specimen's accessibility tree, headlessly.
//!
//! `test/native-visual/ax-audit.ts` reads the *real* macOS tree through
//! `AXUIElement`, which is the only way to know the projection actually reaches
//! an assistive technology. But it needs a window, an unlocked display and an
//! activated app, and it can only see whichever screen the preview is showing —
//! so it covers one screen out of 135.
//!
//! This covers the other 134. `GameUi::accessibility_update()` needs no window:
//! it walks the materialized `UiTree`, which exists as soon as a specimen is
//! rendered. So every component can be projected in one process, in seconds,
//! deterministically — the same trade that made the offscreen visual gate beat
//! the window-capture one.
//!
//! The two are complements, not alternatives. This one proves *what the tree
//! says* for every component; the AX probe proves *that macOS receives it*.
//!
//!   cargo run --bin a11y            # audit, non-zero exit on findings
//!   cargo run --bin a11y -- --roles # per-slug role breakdown
//!   cargo run --bin a11y -- --slug=checkbox

use jetstream_ui::accesskit::Role;
use jetstream_ui::GameUi;
use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_preview::{app_state::AppState, component_registry, specimens};

/// Roles that carry no meaning on their own and need no accessible name.
///
/// `GenericContainer` is the default a node gets when nothing claims it, so it
/// is also the marker for "this component has no role yet" — counted, but not
/// failed on, because that gap is tracked as work rather than as a defect.
const STRUCTURAL: &[Role] = &[
    Role::GenericContainer,
    Role::Group,
    Role::Unknown,
    Role::ScrollView,
    Role::Window,
];

/// Roles that mean something specific and are meaningless unnamed.
///
/// A button announced as "button" tells the user there is something to press
/// and nothing about what it does. These are the ones worth failing on.
///
/// `TabList` is deliberately absent. A name on it is good practice — the
/// preview's own nav got one — but the tabs inside carry the meaning, so an
/// unnamed tab list is imperfect rather than unusable. Failing on it would put
/// 28 findings from one specimen ahead of controls a user genuinely cannot
/// identify.
fn needs_name(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::CheckBox
            | Role::Switch
            | Role::RadioButton
            | Role::TextInput
            | Role::Slider
            | Role::ComboBox
            | Role::Tab
            | Role::MenuItem
            | Role::Link
            | Role::SpinButton
            | Role::TreeItem
    )
}

struct Finding {
    slug: &'static str,
    role: Role,
    count: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let show_roles = args.iter().any(|a| a == "--roles");
    let only: Option<String> = args
        .iter()
        .find_map(|a| a.strip_prefix("--slug=").map(str::to_owned));

    let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
    let state = AppState::new();

    let mut findings: Vec<Finding> = Vec::new();
    let mut audited = 0;
    let mut role_less = Vec::new();
    let mut total_nodes = 0;

    for entry in component_registry::ALL_COMPONENTS {
        if !entry.has_specimen {
            continue;
        }
        if let Some(slug) = &only
            && entry.slug != slug
        {
            continue;
        }
        let Some(specimen) = specimens::render_specimen(entry.slug, &theme, &state) else {
            continue;
        };

        // A generous canvas: a specimen laid out at 0x0 collapses and the tree
        // it projects would describe nothing.
        let mut ui = GameUi::new(900.0, 640.0);
        ui.render_immediate(
            &jetstream_ui::ui_element::div()
                .w(900.0)
                .h(640.0)
                .p(24.0)
                .flex_col()
                .child(specimen),
        );

        let Some(update) = ui.accessibility_update() else {
            eprintln!("{}: projected no tree at all", entry.slug);
            continue;
        };
        audited += 1;
        total_nodes += update.nodes.len();

        let mut unnamed: std::collections::HashMap<Role, usize> = Default::default();
        let mut meaningful = 0;
        for (_, node) in &update.nodes {
            let role = node.role();
            if !STRUCTURAL.contains(&role) {
                meaningful += 1;
            }
            if needs_name(role) && node.label().is_none() {
                *unnamed.entry(role).or_default() += 1;
            }
        }
        if meaningful == 0 {
            role_less.push(entry.slug);
        }

        if show_roles {
            let mut roles: std::collections::HashMap<Role, usize> = Default::default();
            for (_, node) in &update.nodes {
                *roles.entry(node.role()).or_default() += 1;
            }
            let mut listed: Vec<_> = roles.into_iter().collect();
            listed.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let summary = listed
                .iter()
                .map(|(r, n)| format!("{r:?}={n}"))
                .collect::<Vec<_>>()
                .join(" ");
            println!("{:28} {summary}", entry.slug);
        }

        for (role, count) in unnamed {
            findings.push(Finding {
                slug: entry.slug,
                role,
                count,
            });
        }
    }

    println!("\naudited {audited} specimens, {total_nodes} nodes");

    if !role_less.is_empty() {
        println!(
            "\n{} specimen(s) project only structural nodes — no role claims anything:\n  {}",
            role_less.len(),
            role_less.join(", ")
        );
    }

    if findings.is_empty() {
        println!("\nevery role that needs an accessible name has one.");
        return;
    }

    findings.sort_by_key(|f| std::cmp::Reverse(f.count));
    let total: usize = findings.iter().map(|f| f.count).sum();
    println!("\n{total} unnamed element(s) whose role needs a name:");
    for finding in &findings {
        println!("  {:28} {:?} x{}", finding.slug, finding.role, finding.count);
    }
    std::process::exit(1);
}
