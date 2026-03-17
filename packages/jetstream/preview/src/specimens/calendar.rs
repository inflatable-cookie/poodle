//! Calendar specimen — monthly date grid for date picking.

use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;

use crate::theme_bridge;

pub fn render(tree: &mut UiTree, theme: &dyn ThemeProvider) -> UiNodeId {
    let text_primary = theme_bridge::text_primary(theme);
    let text_secondary = theme_bridge::text_secondary(theme);
    let text_inverse = theme_bridge::text_inverse(theme);
    let accent = theme_bridge::accent_base(theme);
    let bg_elevated = theme_bridge::elevated_background(theme);
    let border = theme_bridge::border_subtle(theme);

    let root = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Grow(1.0),
        gap: 20.0,
        ..UiStyle::default()
    });

    // ── Standard calendar ──
    section_label(tree, root, "March 2026", text_secondary);
    {
        let cal = calendar_card(tree, bg_elevated, border);
        tree.add_child(root, cal);

        // Navigation header
        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 12.0, bottom: 8.0, left: 12.0 },
            align: Align::Center,
            justify: Justify::SpaceBetween,
            ..UiStyle::default()
        });
        tree.add_child(cal, nav);

        let prev = tree.create(Widget::Label { text: "◂".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(nav, prev);

        let month = tree.create(Widget::Label { text: "March 2026".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(nav, month);

        let next = tree.create(Widget::Label { text: "▸".to_string() }, UiStyle {
            text_color: Some(text_secondary), text_size: Some(14.0), ..UiStyle::default()
        });
        tree.add_child(nav, next);

        // Day-of-week headers
        let dow_row = grid_row(tree, cal);
        for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
            day_header(tree, dow_row, day, text_secondary);
        }

        // Week rows — March 2026 starts on Sunday
        let days: &[&[DayState]] = &[
            &[D(1), D(2), D(3), D(4), D(5), D(6), D(7)],
            &[D(8), D(9), D(10), D(11), D(12), D(13), D(14)],
            &[D(15), SEL, D(17), D(18), D(19), D(20), D(21)],
            &[D(22), D(23), D(24), D(25), D(26), D(27), D(28)],
            &[D(29), D(30), D(31), OFF, OFF, OFF, OFF],
        ];

        for week in days {
            let row = grid_row(tree, cal);
            for &state in *week {
                match state {
                    DayState::Normal(n) => day_cell(tree, row, &n.to_string(), text_primary, None, false),
                    DayState::Selected => day_cell(tree, row, "16", text_inverse, Some(accent), true),
                    DayState::Outside => day_cell(tree, row, "", text_secondary, None, false),
                }
            }
        }
    }

    // ── With today highlight ──
    section_label(tree, root, "Today Highlighted (no selection)", text_secondary);
    {
        let cal = calendar_card(tree, bg_elevated, border);
        tree.add_child(root, cal);

        let nav = tree.create(Widget::Panel, UiStyle {
            direction: Direction::Row,
            width: Sizing::Grow(1.0),
            padding: Edges { top: 12.0, right: 12.0, bottom: 8.0, left: 12.0 },
            align: Align::Center, justify: Justify::Center,
            ..UiStyle::default()
        });
        tree.add_child(cal, nav);

        let month = tree.create(Widget::Label { text: "March 2026".to_string() }, UiStyle {
            text_color: Some(text_primary), text_size: Some(13.0), ..UiStyle::default()
        });
        tree.add_child(nav, month);

        let dow_row = grid_row(tree, cal);
        for day in &["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"] {
            day_header(tree, dow_row, day, text_secondary);
        }

        // Just show one row with today indicator
        let row = grid_row(tree, cal);
        for d in 15..=21 {
            if d == 16 {
                // Today — outlined, not filled
                let cell = tree.create(Widget::Panel, UiStyle {
                    width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
                    corner_radii: [14.0; 4], border_color: Some(accent),
                    border_width: 1.0, align: Align::Center,
                    justify: Justify::Center, ..UiStyle::default()
                });
                tree.add_child(row, cell);

                let lbl = tree.create(Widget::Label { text: "16".to_string() }, UiStyle {
                    text_color: Some(accent), text_size: Some(11.0), ..UiStyle::default()
                });
                tree.add_child(cell, lbl);
            } else {
                day_cell(tree, row, &d.to_string(), text_primary, None, false);
            }
        }
    }

    root
}

#[derive(Clone, Copy)]
enum DayState {
    Normal(u8),
    Selected,
    Outside,
}

const fn D(n: u8) -> DayState { DayState::Normal(n) }
const SEL: DayState = DayState::Selected;
const OFF: DayState = DayState::Outside;

fn section_label(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
    });
    tree.add_child(parent, lbl);
}

fn calendar_card(tree: &mut UiTree, bg: glam::Vec4, border: glam::Vec4) -> UiNodeId {
    tree.create(Widget::Panel, UiStyle {
        direction: Direction::Column,
        width: Sizing::Fixed(232.0),
        background: Some(bg),
        border_color: Some(border),
        border_width: 1.0,
        corner_radii: [8.0; 4],
        padding: Edges { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 },
        ..UiStyle::default()
    })
}

fn grid_row(tree: &mut UiTree, parent: UiNodeId) -> UiNodeId {
    let row = tree.create(Widget::Panel, UiStyle {
        direction: Direction::Row,
        width: Sizing::Grow(1.0),
        padding: Edges { top: 0.0, right: 8.0, bottom: 0.0, left: 8.0 },
        gap: 2.0,
        justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, row);
    row
}

fn day_header(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4) {
    let cell = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(28.0), height: Sizing::Fixed(24.0),
        align: Align::Center, justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, cell);

    let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
        text_color: Some(color), text_size: Some(10.0), ..UiStyle::default()
    });
    tree.add_child(cell, lbl);
}

fn day_cell(tree: &mut UiTree, parent: UiNodeId, text: &str, color: glam::Vec4, bg: Option<glam::Vec4>, _selected: bool) {
    let cell = tree.create(Widget::Panel, UiStyle {
        width: Sizing::Fixed(28.0), height: Sizing::Fixed(28.0),
        corner_radii: [14.0; 4], background: bg,
        align: Align::Center, justify: Justify::Center,
        ..UiStyle::default()
    });
    tree.add_child(parent, cell);

    if !text.is_empty() {
        let lbl = tree.create(Widget::Label { text: text.to_string() }, UiStyle {
            text_color: Some(color), text_size: Some(11.0), ..UiStyle::default()
        });
        tree.add_child(cell, lbl);
    }
}
