//! Jetstream demo scene — exercises Poodle components in a game engine context.
//!
//! g08.012: Demonstrates 4 game-appropriate screen types:
//! 1. Main menu — title, navigation buttons
//! 2. Settings screen — sliders, toggles, selects
//! 3. HUD overlay — health bar, status indicators, score
//! 4. Pause dialog — modal overlay with actions

use poodle_adapter::{RenderComponent, ThemeProvider};
use poodle_specs::{
    BadgeSpec, ButtonSpec, DialogSpec, MenuSpec, ProgressSpec, SelectSpec, SeparatorSpec,
    SliderSpec, SpinnerSpec, StackSpec, StatusIndicatorSpec, SurfaceSpec, SwitchSpec, TabsSpec,
    TextInputSpec,
};
use poodle_specs::{ConfirmActionSpec, PageHeaderSpec, StateTileSpec, ToastStackSpec};
use poodle_style::StyleDescriptor;

use crate::{JetstreamAdapter, JetstreamNodeHandle};

/// A demo scene screen with rendered node handles.
#[derive(Debug)]
pub struct DemoSceneScreen {
    pub id: &'static str,
    pub title: &'static str,
    pub nodes: Vec<JetstreamNodeHandle>,
}

impl DemoSceneScreen {
    fn new(id: &'static str, title: &'static str) -> Self {
        Self {
            id,
            title,
            nodes: Vec::new(),
        }
    }

    fn push(&mut self, handle: JetstreamNodeHandle) {
        self.nodes.push(handle);
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

/// Render all 4 demo scene screens.
pub fn render_all_scenes(adapter: &JetstreamAdapter) -> Vec<DemoSceneScreen> {
    let theme = adapter.theme();
    vec![
        render_main_menu(adapter, theme),
        render_settings(adapter, theme),
        render_hud(adapter, theme),
        render_pause_dialog(adapter, theme),
    ]
}

fn render_main_menu(a: &JetstreamAdapter, t: &dyn ThemeProvider) -> DemoSceneScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoSceneScreen::new("main-menu", "Main Menu");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&StackSpec::new(), &s, t));
    screen.push(a.render(&PageHeaderSpec::new("Game Title"), &s, t));
    screen.push(a.render(&SeparatorSpec::new(), &s, t));
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Start
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Options
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Quit
    screen.push(a.render(&MenuSpec::new(vec![]), &s, t));

    screen
}

fn render_settings(a: &JetstreamAdapter, t: &dyn ThemeProvider) -> DemoSceneScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoSceneScreen::new("settings", "Settings");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&PageHeaderSpec::new("Settings"), &s, t));
    screen.push(a.render(&TabsSpec::new(vec![]), &s, t));
    screen.push(a.render(&SliderSpec::new(0.75), &s, t)); // Volume
    screen.push(a.render(&SliderSpec::new(0.5), &s, t)); // Brightness
    screen.push(a.render(&SwitchSpec::new(), &s, t)); // Fullscreen
    screen.push(a.render(&SwitchSpec::new(), &s, t)); // VSync
    screen.push(a.render(&SelectSpec::new(vec![]), &s, t)); // Resolution
    screen.push(a.render(&TextInputSpec::new(), &s, t)); // Player name
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Apply
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Back

    screen
}

fn render_hud(a: &JetstreamAdapter, t: &dyn ThemeProvider) -> DemoSceneScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoSceneScreen::new("hud", "HUD Overlay");

    screen.push(a.render(&SurfaceSpec::new(), &s, t));
    screen.push(a.render(&ProgressSpec::new(), &s, t)); // Health bar
    screen.push(a.render(&ProgressSpec::new(), &s, t)); // Energy bar
    screen.push(a.render(&SpinnerSpec::new(), &s, t)); // Loading/status activity
    screen.push(a.render(&StatusIndicatorSpec::new(), &s, t)); // Status
    screen.push(a.render(&BadgeSpec::new(), &s, t)); // Level badge
    screen.push(a.render(&StateTileSpec::new("Score", "1250"), &s, t));
    screen.push(a.render(&StateTileSpec::new("Time", "2:34"), &s, t));
    screen.push(a.render(&ToastStackSpec::new(), &s, t)); // Notifications

    screen
}

fn render_pause_dialog(a: &JetstreamAdapter, t: &dyn ThemeProvider) -> DemoSceneScreen {
    let s = StyleDescriptor::new();
    let mut screen = DemoSceneScreen::new("pause-dialog", "Pause Dialog");

    screen.push(a.render(&DialogSpec::new(), &s, t));
    screen.push(a.render(
        &ConfirmActionSpec::new("Paused", "Game paused", "Resume", "Quit"),
        &s,
        t,
    ));
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Resume
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Settings
    screen.push(a.render(&ButtonSpec::new(), &s, t)); // Quit to menu

    screen
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::JetstreamThemeProvider;

    fn adapter() -> JetstreamAdapter {
        JetstreamAdapter::new(JetstreamThemeProvider::default())
    }

    #[test]
    fn all_four_scenes_render() {
        let scenes = render_all_scenes(&adapter());
        assert_eq!(scenes.len(), 4);
        for scene in &scenes {
            assert!(scene.node_count() > 0, "Scene '{}' has no nodes", scene.id);
        }
    }

    #[test]
    fn main_menu_has_navigation_buttons() {
        let scene = render_main_menu(&adapter(), adapter().theme());
        let button_count = scene
            .nodes
            .iter()
            .filter(|n| n.spec_type == "ButtonSpec")
            .count();
        assert!(button_count >= 3, "Main menu needs at least 3 buttons");
    }

    #[test]
    fn settings_has_input_controls() {
        let scene = render_settings(&adapter(), adapter().theme());
        let has_slider = scene.nodes.iter().any(|n| n.spec_type == "SliderSpec");
        let has_switch = scene.nodes.iter().any(|n| n.spec_type == "SwitchSpec");
        assert!(has_slider && has_switch);
    }

    #[test]
    fn hud_has_feedback_elements() {
        let scene = render_hud(&adapter(), adapter().theme());
        let has_progress = scene.nodes.iter().any(|n| n.spec_type == "ProgressSpec");
        let has_spinner = scene.nodes.iter().any(|n| n.spec_type == "SpinnerSpec");
        let has_state_tile = scene.nodes.iter().any(|n| n.spec_type == "StateTileSpec");
        assert!(has_progress && has_spinner && has_state_tile);
    }

    #[test]
    fn pause_dialog_has_confirm_action() {
        let scene = render_pause_dialog(&adapter(), adapter().theme());
        let has_dialog = scene.nodes.iter().any(|n| n.spec_type == "DialogSpec");
        let has_confirm = scene
            .nodes
            .iter()
            .any(|n| n.spec_type == "ConfirmActionSpec");
        assert!(has_dialog && has_confirm);
    }

    #[test]
    fn total_nodes_across_all_scenes() {
        let scenes = render_all_scenes(&adapter());
        let total: usize = scenes.iter().map(|s| s.node_count()).sum();
        assert!(
            total >= 30,
            "Total node count {total} below expected minimum"
        );
    }
}
