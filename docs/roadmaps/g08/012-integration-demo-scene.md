# g08.012 — Integration Demo Scene in Jetstream

Status: Completed
Updated: 2026-03-14

## Objective

Build a demo scene module that exercises Pug components in 4 game-appropriate
screen types, verifying that the adapter renders specs correctly.

## Deliverables

### Demo scenes (demo_scene.rs)

| Screen | ID | Components | Specs Used |
|--------|-----|-----------|------------|
| Main Menu | `main-menu` | 8 | Surface, Stack, PageHeader, Separator, Button×3, Menu |
| Settings | `settings` | 11 | Surface, PageHeader, Tabs, Slider×2, Switch×2, Select, TextInput, Button×2 |
| HUD Overlay | `hud` | 8 | Surface, Progress×2, StatusIndicator, Badge, StateTile×2, ToastStack |
| Pause Dialog | `pause-dialog` | 5 | Dialog, ConfirmAction, Button×3 |

### Public API

- `DemoSceneScreen` struct: id, title, nodes vec
- `render_all_scenes(adapter)` → Vec<DemoSceneScreen>
- Per-screen render functions

### Test coverage

6 tests:
- `all_four_scenes_render` — verifies 4 screens with non-empty node counts
- `main_menu_has_navigation_buttons` — at least 3 ButtonSpec nodes
- `settings_has_input_controls` — SliderSpec and SwitchSpec present
- `hud_has_feedback_elements` — ProgressSpec and StateTileSpec present
- `pause_dialog_has_confirm_action` — DialogSpec and ConfirmActionSpec present
- `total_nodes_across_all_scenes` — at least 30 total nodes

## Verification

```
cargo test — 6 demo scene tests passing
Total nodes: 32 across 4 screens
```
