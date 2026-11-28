//! In-memory GPUI test platform for conformance observation.
//!
//! Captures GPUI's raw AccessKit updates and drawn scenes at the same
//! `Platform` and `PlatformWindow` boundaries a desktop platform receives,
//! without depending on GPUI's `bench-support` feature. All window layout and
//! painting remain GPUI's.

use std::cell::{Cell, RefCell};
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
    dispatcher: Arc<ThreadedDispatcher>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    text_system: Arc<dyn PlatformTextSystem>,
    clipboard: RefCell<Option<ClipboardItem>>,
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    primary: RefCell<Option<ClipboardItem>>,
    #[cfg(target_os = "macos")]
    find_pasteboard: RefCell<Option<ClipboardItem>>,
    pub output: Rc<RefCell<Output>>,
}

impl PlatformTap {
    pub fn new() -> Self {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());
        Self {
            dispatcher,
            background_executor,
            foreground_executor,
            text_system: Arc::new(NoopTextSystem),
            clipboard: RefCell::default(),
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            primary: RefCell::default(),
            #[cfg(target_os = "macos")]
            find_pasteboard: RefCell::default(),
            output: Rc::default(),
        }
    }

    pub fn run_until_idle(&self) {
        self.dispatcher.run_until_idle();
    }
}

struct HeadlessKeyboardLayout;

impl PlatformKeyboardLayout for HeadlessKeyboardLayout {
    fn id(&self) -> &str {
        "poodle.headless"
    }

    fn name(&self) -> &str {
        "Poodle Headless"
    }
}

struct WindowTap {
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    is_fullscreen: Cell<bool>,
    input_handler: Option<PlatformInputHandler>,
    sprite_atlas: Arc<dyn PlatformAtlas>,
    output: Rc<RefCell<Output>>,
    a11y: RefCell<Option<A11yCallbacks>>,
}

impl WindowTap {
    fn new(bounds: Bounds<Pixels>, output: Rc<RefCell<Output>>) -> Self {
        Self {
            bounds,
            // Preserve GPUI's TestWindow 2x default so physical-pixel AccessKit
            // bounds and scene coordinates match the stock test platform.
            scale_factor: 2.0,
            is_fullscreen: Cell::new(false),
            input_handler: None,
            sprite_atlas: Arc::new(HeadlessAtlas::default()),
            output,
            a11y: RefCell::default(),
        }
    }
}

impl HasWindowHandle for WindowTap {
    fn window_handle(&self) -> Result<RawWindowHandle<'_>, HandleError> {
        Err(HandleError::NotSupported)
    }
}

impl HasDisplayHandle for WindowTap {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        Err(HandleError::NotSupported)
    }
}

impl Platform for PlatformTap {
    fn background_executor(&self) -> BackgroundExecutor {
        self.background_executor.clone()
    }

    fn foreground_executor(&self) -> ForegroundExecutor {
        self.foreground_executor.clone()
    }

    fn text_system(&self) -> Arc<dyn PlatformTextSystem> {
        self.text_system.clone()
    }

    fn run(&self, launch: Box<dyn FnOnce()>) {
        launch();
    }

    fn quit(&self) {}

    fn restart(&self, _binary_path: Option<PathBuf>, _arguments: Vec<OsString>) {}

    fn activate(&self, _ignoring_other_apps: bool) {}

    fn hide(&self) {}

    fn hide_other_apps(&self) {}

    fn unhide_other_apps(&self) {}

    fn displays(&self) -> Vec<Rc<dyn PlatformDisplay>> {
        Vec::new()
    }

    fn primary_display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn active_window(&self) -> Option<AnyWindowHandle> {
        None
    }

    fn open_window(
        &self,
        _handle: AnyWindowHandle,
        options: WindowParams,
    ) -> Result<Box<dyn PlatformWindow>> {
        Ok(Box::new(WindowTap::new(
            options.bounds,
            self.output.clone(),
        )))
    }

    fn window_appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn open_url(&self, _url: &str) {}

    fn on_open_urls(&self, _callback: Box<dyn FnMut(Vec<String>)>) {}

    fn register_url_scheme(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn prompt_for_paths(
        &self,
        _options: PathPromptOptions,
    ) -> oneshot::Receiver<Result<Option<Vec<PathBuf>>>> {
        oneshot::channel().1
    }

    fn prompt_for_new_path(
        &self,
        _directory: &FsPath,
        _suggested_name: Option<&str>,
    ) -> oneshot::Receiver<Result<Option<PathBuf>>> {
        oneshot::channel().1
    }

    fn can_select_mixed_files_and_dirs(&self) -> bool {
        true
    }

    fn reveal_path(&self, _path: &FsPath) {}

    fn open_with_system(&self, _path: &FsPath) {}

    fn on_quit(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_reopen(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_sleep(&self, _callback: Box<dyn FnMut()>) {}

    fn on_system_wake(&self, _callback: Box<dyn FnMut()>) {}

    fn set_menus(&self, _menus: Vec<Menu>, _keymap: &Keymap) {}

    fn set_dock_menu(&self, _menu: Vec<MenuItem>, _keymap: &Keymap) {}

    fn on_app_menu_action(&self, _callback: Box<dyn FnMut(&dyn Action)>) {}

    fn on_will_open_app_menu(&self, _callback: Box<dyn FnMut()>) {}

    fn on_validate_app_menu_command(&self, _callback: Box<dyn FnMut(&dyn Action) -> bool>) {}

    fn thermal_state(&self) -> ThermalState {
        ThermalState::Nominal
    }

    fn on_thermal_state_change(&self, _callback: Box<dyn FnMut()>) {}

    fn prevent_idle_sleep(&self, _reason: &str) -> Task<Result<ActivityGuard>> {
        Task::ready(Ok(ActivityGuard::noop()))
    }

    fn app_path(&self) -> Result<PathBuf> {
        anyhow::bail!("headless conformance platform has no application path")
    }

    fn path_for_auxiliary_executable(&self, name: &str) -> Result<PathBuf> {
        anyhow::bail!("headless conformance platform has no auxiliary executable {name:?}")
    }

    fn set_cursor_style(&self, _style: CursorStyle) {}

    fn hide_cursor_until_mouse_moves(&self) {}

    fn is_cursor_visible(&self) -> bool {
        true
    }

    fn should_auto_hide_scrollbars(&self) -> bool {
        false
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        self.clipboard.borrow().clone()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        *self.clipboard.borrow_mut() = Some(item);
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn read_from_primary(&self) -> Option<ClipboardItem> {
        self.primary.borrow().clone()
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    fn write_to_primary(&self, item: ClipboardItem) {
        *self.primary.borrow_mut() = Some(item);
    }

    #[cfg(target_os = "macos")]
    fn read_from_find_pasteboard(&self) -> Option<ClipboardItem> {
        self.find_pasteboard.borrow().clone()
    }

    #[cfg(target_os = "macos")]
    fn write_to_find_pasteboard(&self, item: ClipboardItem) {
        *self.find_pasteboard.borrow_mut() = Some(item);
    }

    fn write_credentials(&self, _url: &str, _username: &str, _password: &[u8]) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn read_credentials(&self, _url: &str) -> Task<Result<Option<(String, Vec<u8>)>>> {
        Task::ready(Ok(None))
    }

    fn delete_credentials(&self, _url: &str) -> Task<Result<()>> {
        Task::ready(Ok(()))
    }

    fn keyboard_layout(&self) -> Box<dyn PlatformKeyboardLayout> {
        Box::new(HeadlessKeyboardLayout)
    }

    fn keyboard_mapper(&self) -> Rc<dyn PlatformKeyboardMapper> {
        Rc::new(DummyKeyboardMapper)
    }

    fn on_keyboard_layout_change(&self, _callback: Box<dyn FnMut()>) {}
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

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }

    fn is_maximized(&self) -> bool {
        false
    }

    fn window_bounds(&self) -> WindowBounds {
        WindowBounds::Windowed(self.bounds)
    }

    fn content_size(&self) -> Size<Pixels> {
        self.bounds.size
    }

    fn resize(&mut self, size: Size<Pixels>) {
        self.bounds.size = size;
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn appearance(&self) -> WindowAppearance {
        WindowAppearance::Light
    }

    fn display(&self) -> Option<Rc<dyn PlatformDisplay>> {
        None
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::default()
    }

    fn modifiers(&self) -> Modifiers {
        Modifiers::default()
    }

    fn capslock(&self) -> Capslock {
        Capslock::default()
    }

    fn set_input_handler(&mut self, handler: PlatformInputHandler) {
        self.input_handler = Some(handler);
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.input_handler.take()
    }

    fn prompt(
        &self,
        _level: PromptLevel,
        _message: &str,
        _detail: Option<&str>,
        _answers: &[PromptButton],
    ) -> Option<oneshot::Receiver<usize>> {
        None
    }

    fn activate(&self) {}

    fn is_active(&self) -> bool {
        false
    }

    fn visibility(&self) -> WindowVisibility {
        WindowVisibility::Visible
    }

    fn is_hovered(&self) -> bool {
        false
    }

    fn background_appearance(&self) -> WindowBackgroundAppearance {
        WindowBackgroundAppearance::Opaque
    }

    fn set_title(&mut self, _title: &str) {}

    fn set_background_appearance(&self, _appearance: WindowBackgroundAppearance) {}

    fn minimize(&self) {}

    fn zoom(&self) {}

    fn toggle_fullscreen(&self) {
        self.is_fullscreen.set(!self.is_fullscreen.get());
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen.get()
    }

    fn on_input(&self, _callback: Box<dyn FnMut(PlatformInput) -> DispatchEventResult>) {}

    fn on_active_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_visibility_change(&self, _callback: Box<dyn FnMut(WindowVisibility)>) {}

    fn on_hover_status_change(&self, _callback: Box<dyn FnMut(bool)>) {}

    fn on_resize(&self, _callback: Box<dyn FnMut(Size<Pixels>, f32)>) {}

    fn on_moved(&self, _callback: Box<dyn FnMut()>) {}

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {}

    fn on_hit_test_window_control(&self, _callback: Box<dyn FnMut() -> Option<WindowControlArea>>) {
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {}

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {}

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.sprite_atlas.clone()
    }

    fn is_subpixel_rendering_supported(&self) -> bool {
        false
    }

    fn gpu_specs(&self) -> Option<GpuSpecs> {
        None
    }

    fn update_ime_position(&self, _bounds: Bounds<Pixels>) {}
}
