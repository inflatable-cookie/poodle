//! Tap the in-memory GPUI test platform at the same boundaries a desktop
//! platform receives. The stock TestWindow does not activate AccessKit or
//! retain scenes. All window layout and painting remain GPUI's.
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path as FsPath, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use futures::channel::oneshot;
use gpui::*;
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle as RawWindowHandle,
};

type FrameRequest = Box<dyn FnMut(RequestFrameOptions)>;

#[derive(Default)]
pub(super) struct Output {
    pub request: Option<FrameRequest>,
    pub tree: Option<accesskit::TreeUpdate>,
    pub quads: Vec<Quad>,
    pub paths: Vec<Path<ScaledPixels>>,
    pub masks: Vec<Bounds<ScaledPixels>>,
    pub images: Vec<Bounds<ScaledPixels>>,
}

pub(super) struct PlatformTap {
    delegate: Rc<dyn Platform>,
    pub output: Rc<RefCell<Output>>,
}

impl PlatformTap {
    pub fn new() -> Self {
        Self {
            delegate: bench_platform(None, Arc::new(NoopTextSystem)),
            output: Rc::default(),
        }
    }

    pub fn run_until_idle(&self) {
        self.foreground_executor()
            .dispatcher()
            .as_threaded()
            .expect("GPUI benchmark platform dispatcher")
            .run_until_idle();
    }
}

struct WindowTap {
    delegate: Box<dyn PlatformWindow>,
    output: Rc<RefCell<Output>>,
    a11y: RefCell<Option<A11yCallbacks>>,
}

impl HasWindowHandle for WindowTap {
    fn window_handle(&self) -> Result<RawWindowHandle<'_>, HandleError> {
        self.delegate.window_handle()
    }
}

impl HasDisplayHandle for WindowTap {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.delegate.display_handle()
    }
}

macro_rules! forward {
    () => {};
    ($name:ident(&mut $this:ident $(, $arg:ident: $ty:ty)*) $(-> $out:ty)?; $($rest:tt)*) => {
        fn $name(&mut $this $(, $arg: $ty)*) $(-> $out)? {
            $this.delegate.$name($($arg),*)
        }
        forward! { $($rest)* }
    };
    ($name:ident(&$this:ident $(, $arg:ident: $ty:ty)*) $(-> $out:ty)?; $($rest:tt)*) => {
        fn $name(&$this $(, $arg: $ty)*) $(-> $out)? {
            $this.delegate.$name($($arg),*)
        }
        forward! { $($rest)* }
    };
}

impl Platform for PlatformTap {
    fn run(&self, launch: Box<dyn FnOnce()>) {
        launch();
    }

    fn open_window(
        &self,
        handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        let delegate = self.delegate.open_window(handle, options)?;
        Ok(Box::new(WindowTap {
            delegate,
            output: self.output.clone(),
            a11y: RefCell::default(),
        }))
    }

    forward! {
        background_executor(&self) -> BackgroundExecutor;
        foreground_executor(&self) -> ForegroundExecutor;
        text_system(&self) -> Arc<dyn PlatformTextSystem>;
        quit(&self);
        restart(&self, binary_path: Option<PathBuf>, arguments: Vec<OsString>);
        activate(&self, ignoring_other_apps: bool);
        hide(&self);
        hide_other_apps(&self);
        unhide_other_apps(&self);
        displays(&self) -> Vec<Rc<dyn PlatformDisplay>>;
        primary_display(&self) -> Option<Rc<dyn PlatformDisplay>>;
        active_window(&self) -> Option<AnyWindowHandle>;
        window_appearance(&self) -> WindowAppearance;
        open_url(&self, url: &str);
        on_open_urls(&self, callback: Box<dyn FnMut(Vec<String>)>);
        register_url_scheme(&self, url: &str) -> Task<Result<()>>;
        prompt_for_paths(&self, options: PathPromptOptions) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>>;
        prompt_for_new_path(&self, directory: &FsPath, suggested_name: Option<&str>) -> oneshot::Receiver<Result<Option<PathBuf>>>;
        can_select_mixed_files_and_dirs(&self) -> bool;
        reveal_path(&self, path: &FsPath);
        open_with_system(&self, path: &FsPath);
        on_quit(&self, callback: Box<dyn FnMut() -> bool>);
        on_reopen(&self, callback: Box<dyn FnMut()>);
        on_system_sleep(&self, callback: Box<dyn FnMut()>);
        on_system_wake(&self, callback: Box<dyn FnMut()>);
        set_menus(&self, menus: Vec<Menu>, keymap: &Keymap);
        set_dock_menu(&self, menu: Vec<MenuItem>, keymap: &Keymap);
        on_app_menu_action(&self, callback: Box<dyn FnMut(&dyn Action)>);
        on_will_open_app_menu(&self, callback: Box<dyn FnMut()>);
        on_validate_app_menu_command(&self, callback: Box<dyn FnMut(&dyn Action) -> bool>);
        thermal_state(&self) -> ThermalState;
        on_thermal_state_change(&self, callback: Box<dyn FnMut()>);
        prevent_idle_sleep(&self, reason: &str) -> Task<Result<ActivityGuard>>;
        app_path(&self) -> Result<PathBuf>;
        path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf>;
        set_cursor_style(&self, style: CursorStyle);
        hide_cursor_until_mouse_moves(&self);
        is_cursor_visible(&self) -> bool;
        should_auto_hide_scrollbars(&self) -> bool;
        read_from_clipboard(&self) -> Option<ClipboardItem>;
        write_to_clipboard(&self, item: ClipboardItem);
        write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Task<Result<()>>;
        read_credentials(&self, url: &str) -> Task<Result<Option<(String, Vec<u8>)>>>;
        delete_credentials(&self, url: &str) -> Task<Result<()>>;
        keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout>;
        keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper>;
        on_keyboard_layout_change(&self, callback: Box<dyn FnMut()>);
    }
    #[cfg(target_os = "linux")]
    forward! {
        read_from_primary(&self) -> Option<ClipboardItem>;
        write_to_primary(&self, item: ClipboardItem);
    }

    #[cfg(target_os = "macos")]
    forward! {
        read_from_find_pasteboard(&self) -> Option<ClipboardItem>;
        write_to_find_pasteboard(&self, item: ClipboardItem);
    }
}

impl PlatformWindow for WindowTap {
    fn a11y_init(&self, callbacks: A11yCallbacks) {
        (callbacks.activation)();
        // Keep the remaining channels alive for this window's lifetime.
        self.a11y.replace(Some(callbacks));
    }

    fn a11y_tree_update(&self, update: accesskit::TreeUpdate) {
        self.output.borrow_mut().tree = Some(update);
    }

    fn on_request_frame(&self, request: FrameRequest) {
        self.output.borrow_mut().request = Some(request);
    }

    fn draw(&self, scene: &Scene) {
        let mut output = self.output.borrow_mut();
        output.quads.clone_from(&scene.quads);
        output.paths.clone_from(&scene.paths);
        output.masks = scene
            .monochrome_sprites
            .iter()
            .map(|sprite| sprite.bounds)
            .collect();
        output.images = scene
            .polychrome_sprites
            .iter()
            .map(|sprite| sprite.bounds)
            .collect();
    }

    forward! {
        bounds(&self) -> Bounds<Pixels>;
        is_maximized(&self) -> bool;
        window_bounds(&self) -> WindowBounds;
        content_size(&self) -> Size<Pixels>;
        resize(&mut self, size: Size<Pixels>);
        scale_factor(&self) -> f32;
        appearance(&self) -> WindowAppearance;
        display(&self) -> Option<Rc<dyn PlatformDisplay>>;
        mouse_position(&self) -> Point<Pixels>;
        modifiers(&self) -> Modifiers;
        capslock(&self) -> Capslock;
        set_input_handler(&mut self, handler: PlatformInputHandler);
        take_input_handler(&mut self) -> Option<PlatformInputHandler>;
        prompt(&self, level: PromptLevel, message: &str, detail: Option<&str>, answers: &[PromptButton]) -> Option<oneshot::Receiver<usize>>;
        activate(&self);
        is_active(&self) -> bool;
        visibility(&self) -> WindowVisibility;
        is_hovered(&self) -> bool;
        background_appearance(&self) -> WindowBackgroundAppearance;
        set_title(&mut self, title: &str);
        set_background_appearance(&self, appearance: WindowBackgroundAppearance);
        minimize(&self);
        zoom(&self);
        toggle_fullscreen(&self);
        is_fullscreen(&self) -> bool;
        on_input(&self, callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>);
        on_active_status_change(&self, callback: Box<dyn FnMut(bool)>);
        on_visibility_change(&self, callback: Box<dyn FnMut(WindowVisibility)>);
        on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>);
        on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>);
        on_moved(&self, callback: Box<dyn FnMut()>);
        on_should_close(&self, callback: Box<dyn FnMut() -> bool>);
        on_hit_test_window_control(&self, callback: Box<dyn FnMut() -> Option<WindowControlArea>>);
        on_close(&self, callback: Box<dyn FnOnce()>);
        on_appearance_changed(&self, callback: Box<dyn FnMut()>);
        sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;
        is_subpixel_rendering_supported(&self) -> bool;
        gpu_specs(&self) -> Option<GpuSpecs>;
        update_ime_position(&self, bounds: Bounds<Pixels>);
    }
}
