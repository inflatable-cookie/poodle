//! Pug Jetstream Preview — standalone preview application for showcasing
//! Pug design system components rendered via the Jetstream game engine.
//!
//! Run with: `cargo run -p pug-jetstream-preview`

mod app_state;
mod component_registry;
mod shell;
mod specimens;
mod theme_bridge;

use glam::{Mat4, Vec4};
use jetstream_platform::{
    ControlFlow, KeyCode, MouseButton as PlatMouseButton, Platform, PlatformConfig, PlatformEvent,
    VSyncMode, WindowConfig,
};
use jetstream_renderer::camera::CameraGpu;
use jetstream_renderer::pipeline::create_sprite_pipeline;
use jetstream_renderer::shader::create_sprite_shader;
use jetstream_renderer::sprite::{
    SpriteInstance, QUAD_INDICES as SPRITE_QUAD_INDICES, QUAD_VERTICES as SPRITE_QUAD_VERTICES,
};
use jetstream_renderer::texture::GpuTexture;
use jetstream_renderer::ui_pass::UiPass;
use jetstream_runtime::game_ui::*;
use pug_adapter::ThemeProvider;
use wgpu::util::DeviceExt;

use app_state::{AppState, ControlSize, DemoScreen, Density, Section, ThemePreset};

/// Maximum number of glyph instances per frame.
const MAX_TEXT_INSTANCES: usize = 16384;

/// A group of draw commands sharing the same clip_rect, rendered together
/// with a single wgpu scissor rect.
struct ClipGroup {
    /// Logical clip rectangle (None = full screen, no clipping).
    clip: Option<Rect>,
    /// Draw commands in this group.
    commands: Vec<UiDrawCommand>,
}

impl ClipGroup {
    /// Convert the logical clip rect to a physical-pixel scissor rect
    /// suitable for `wgpu::RenderPass::set_scissor_rect`.
    fn scissor_physical(&self, scale: f32, phys_w: f32, phys_h: f32) -> Option<[u32; 4]> {
        let r = self.clip?;
        let x = (r.x * scale).round().max(0.0) as u32;
        let y = (r.y * scale).round().max(0.0) as u32;
        let right = ((r.x + r.width) * scale).round().min(phys_w) as u32;
        let bottom = ((r.y + r.height) * scale).round().min(phys_h) as u32;
        let w = right.saturating_sub(x).max(1);
        let h = bottom.saturating_sub(y).max(1);
        Some([x, y, w, h])
    }
}

/// Partition draw commands into groups sharing the same `clip_rect`.
/// Preserves draw order within each group. Groups are returned in the
/// order they first appear, so unclipped commands (clip_rect=None)
/// typically come first.
fn group_by_clip_rect(commands: &[UiDrawCommand]) -> Vec<ClipGroup> {
    let mut groups: Vec<ClipGroup> = Vec::new();

    for cmd in commands {
        // Find existing group with matching clip_rect.
        let found = groups.iter_mut().find(|g| match (&g.clip, &cmd.clip_rect) {
            (None, None) => true,
            (Some(a), Some(b)) => rects_eq(a, b),
            _ => false,
        });
        if let Some(group) = found {
            group.commands.push(cmd.clone());
        } else {
            groups.push(ClipGroup {
                clip: cmd.clip_rect,
                commands: vec![cmd.clone()],
            });
        }
    }

    groups
}

/// Compare two Rects for approximate equality (pixel tolerance).
fn rects_eq(a: &Rect, b: &Rect) -> bool {
    (a.x - b.x).abs() < 0.5
        && (a.y - b.y).abs() < 0.5
        && (a.width - b.width).abs() < 0.5
        && (a.height - b.height).abs() < 0.5
}

/// Persistent state across frames.
struct PreviewState {
    app: AppState,
    theme: pug_jetstream::JetstreamThemeProvider,
    game_ui: GameUi,
    ui_pass: UiPass,
    shell: Option<shell::ShellNodes>,
    /// Jetstream Theme for draw command collection.
    draw_theme: Theme,
    /// Text overlay rendering.
    text_pipeline: wgpu::RenderPipeline,
    text_camera: CameraGpu,
    text_quad_vbo: wgpu::Buffer,
    text_quad_ibo: wgpu::Buffer,
    text_instance_buffer: wgpu::Buffer,
    text_instance_capacity: usize,
    /// GPU textures for each font-size atlas: (scaled_px_size, GpuTexture).
    text_atlas_textures: Vec<(f32, GpuTexture)>,
    /// Bind group layout for textures — needed to create new atlas textures on demand.
    texture_bgl: wgpu::BindGroupLayout,
    /// Clear color (linear space) for the background.
    clear_color: wgpu::Color,
    /// Mouse position in logical pixels.
    mouse_x: f32,
    mouse_y: f32,
    /// Whether the left mouse button is currently pressed.
    mouse_left_down: bool,
    /// Whether a screenshot was requested this frame.
    screenshot_requested: bool,
    /// Surface format for screenshot readback.
    surface_format: wgpu::TextureFormat,
    /// Frames remaining to suppress scroll events (prevents trackpad momentum
    /// from immediately re-scrolling after a navigation change rebuilds the tree).
    scroll_suppress_frames: u8,
}

impl PreviewState {
    fn new(
        gpu: &jetstream_platform::GpuContext,
        width: u32,
        height: u32,
        scale_factor: f64,
    ) -> Self {
        let logical_w = width as f32 / scale_factor as f32;
        let logical_h = height as f32 / scale_factor as f32;

        let mut game_ui = GameUi::with_text(logical_w, logical_h);
        game_ui.set_scale_factor(scale_factor as f32);
        game_ui.active = true;

        let ui_pass = UiPass::new(&gpu.device, gpu.surface_format);

        // Build a Jetstream Theme with colors from the Pug token system.
        let pug_theme = pug_jetstream::JetstreamThemeProvider::default();
        let draw_theme = build_draw_theme(&pug_theme);

        // Set clear color from Pug canvas background (linear space — already
        // converted by theme_bridge::resolve_vec4).
        let canvas = theme_bridge::canvas_background(&pug_theme);
        let clear_color = wgpu::Color {
            r: canvas.x as f64,
            g: canvas.y as f64,
            b: canvas.z as f64,
            a: 1.0,
        };

        // ── Text rendering setup (mirrors OverlayRenderer pattern) ──
        let ortho = Mat4::orthographic_rh(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);
        let text_camera = CameraGpu::from_matrix(&gpu.device, ortho);

        let shader = create_sprite_shader(&gpu.device);
        let texture_bgl = GpuTexture::bind_group_layout(&gpu.device);
        let text_pipeline = create_sprite_pipeline(
            &gpu.device,
            &shader,
            gpu.surface_format,
            &text_camera.bind_group_layout,
            &texture_bgl,
        );

        let text_quad_vbo =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("text_quad_vbo"),
                    contents: bytemuck::cast_slice(SPRITE_QUAD_VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let text_quad_ibo =
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("text_quad_ibo"),
                    contents: bytemuck::cast_slice(SPRITE_QUAD_INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let text_instance_capacity = MAX_TEXT_INSTANCES;
        let text_instance_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text_instance_buffer"),
            size: (text_instance_capacity * std::mem::size_of::<SpriteInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Atlas pre-creation happens in rebuild_shell after the tree is built,
        // so we can pre-create for all text sizes used. Start with empty textures.
        let text_atlas_textures = Vec::new();

        let mut state = Self {
            app: AppState::new(),
            theme: pug_theme,
            game_ui,
            ui_pass,
            shell: None,
            draw_theme,
            text_pipeline,
            text_camera,
            text_quad_vbo,
            text_quad_ibo,
            text_instance_buffer,
            text_instance_capacity,
            text_atlas_textures,
            texture_bgl,
            clear_color,
            mouse_x: 0.0,
            mouse_y: 0.0,
            mouse_left_down: false,
            screenshot_requested: false,
            surface_format: gpu.surface_format,
            scroll_suppress_frames: 0,
        };

        // Initial build
        state.rebuild_shell();
        state
    }

    fn rebuild_shell(&mut self) {
        // Save current scroll offsets before tearing down the tree.
        let saved_sidebar_scroll = if !self.app.reset_sidebar_scroll {
            self.shell.as_ref().and_then(|s| {
                self.game_ui.tree.get(s.sidebar).and_then(|n| {
                    if let Widget::List { scroll_offset } = &n.widget {
                        Some(*scroll_offset)
                    } else {
                        None
                    }
                })
            })
        } else {
            None
        };
        let saved_content_scroll = if !self.app.reset_content_scroll {
            self.shell.as_ref().and_then(|s| {
                self.game_ui.tree.get(s.content).and_then(|n| {
                    if let Widget::List { scroll_offset } = &n.widget {
                        Some(*scroll_offset)
                    } else {
                        None
                    }
                })
            })
        } else {
            None
        };

        // Clear existing tree
        if let Some(ref shell) = self.shell {
            self.game_ui.tree.remove(shell.root);
        }
        let shell = shell::build_shell(&mut self.game_ui.tree, &self.app, &self.theme);
        self.shell = Some(shell);
        self.app.dirty = false;
        self.app.reset_sidebar_scroll = false;
        self.app.reset_content_scroll = false;

        // Restore saved scroll offsets.
        if let Some(offset) = saved_sidebar_scroll {
            if let Some(ref s) = self.shell {
                if let Some(n) = self.game_ui.tree.get_mut(s.sidebar) {
                    if let Widget::List { scroll_offset } = &mut n.widget {
                        *scroll_offset = offset;
                    }
                }
            }
        }
        if let Some(offset) = saved_content_scroll {
            if let Some(ref s) = self.shell {
                if let Some(n) = self.game_ui.tree.get_mut(s.content) {
                    if let Widget::List { scroll_offset } = &mut n.widget {
                        *scroll_offset = offset;
                    }
                }
            }
        }

        // Update the draw theme (used for draw command collection) so that
        // theme changes are reflected in the rendered output.
        self.draw_theme = build_draw_theme(&self.theme);

        // Update clear color in case the theme changed.
        let canvas = theme_bridge::canvas_background(&self.theme);
        self.clear_color = wgpu::Color {
            r: canvas.x as f64,
            g: canvas.y as f64,
            b: canvas.z as f64,
            a: 1.0,
        };

        // Pre-create glyph atlases for every font size used across shell
        // and specimens. We need atlases at *logical* sizes (for layout text
        // measurement via TextMeasure) AND at *physical* sizes (for GPU
        // rendering via convert_text_commands). Both live in the same
        // TextAtlasSet keyed by their pixel size.
        let scale = self.game_ui.scale_factor;
        let all_logical_sizes: &[f32] = &[
            9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 16.0, 18.0, 20.0,
        ];
        if let Some(ref mut atlases) = self.game_ui.text_atlases {
            for &sz in all_logical_sizes {
                // Logical atlas (for TextMeasure during layout).
                let _ = atlases.get_or_create(sz);
                // Physical atlas (for GPU rendering at Retina scale).
                let phys = (sz * scale).round().max(1.0);
                if (phys - sz).abs() > 0.5 {
                    let _ = atlases.get_or_create(phys);
                }
            }
        }

        // Use GameUi::layout() which includes text measurement, so that
        // Sizing::Fit nodes (buttons, labels) get correct intrinsic sizes.
        self.game_ui.theme = self.draw_theme.clone();
        self.game_ui.layout();
    }


    fn update_and_render(
        &mut self,
        frame: &jetstream_platform::PlatformFrame<'_>,
    ) -> ControlFlow {
        // Handle window resize
        let logical_w = frame.window_width as f32 / frame.scale_factor as f32;
        let logical_h = frame.window_height as f32 / frame.scale_factor as f32;
        if (logical_w - self.game_ui.screen_width).abs() > 0.5
            || (logical_h - self.game_ui.screen_height).abs() > 0.5
        {
            self.game_ui.resize(logical_w, logical_h);
            self.app.dirty = true;

            // Update text camera ortho projection for new window size.
            let ortho = Mat4::orthographic_rh(
                0.0,
                frame.window_width as f32,
                frame.window_height as f32,
                0.0,
                -1.0,
                1.0,
            );
            self.text_camera.update_matrix(&frame.gpu.queue, ortho);
        }

        // Process platform events
        let scale = frame.scale_factor as f32;
        for event in &frame.events {
            match event {
                PlatformEvent::KeyPressed { key, .. } if *key == KeyCode::ESCAPE => {
                    return ControlFlow::Exit;
                }
                PlatformEvent::KeyPressed { key, .. } if *key == KeyCode::F12 => {
                    self.screenshot_requested = true;
                    log::info!("Screenshot requested (F12)");
                }
                PlatformEvent::MouseMoved { x, y } => {
                    // Physical pixels → logical pixels.
                    self.mouse_x = *x as f32 / scale;
                    self.mouse_y = *y as f32 / scale;
                }
                PlatformEvent::MouseButton { button, pressed } => {
                    if matches!(button, PlatMouseButton::Left) {
                        let was_down = self.mouse_left_down;
                        self.mouse_left_down = *pressed;

                        // On mouse-up, activate the node under the cursor.
                        if was_down && !pressed {
                            if let Some(hit_id) =
                                self.game_ui.tree.hit_test(self.mouse_x, self.mouse_y)
                            {
                                self.game_ui.focus.set_focus(Some(hit_id));
                                self.handle_activation(hit_id);
                            }
                        }
                    }
                }
                PlatformEvent::MouseWheel { delta_x: _, delta_y } => {
                    // Suppress trackpad momentum after a navigation rebuild so
                    // that the newly-reset scroll position isn't immediately
                    // re-scrolled by lingering inertial events.
                    if self.scroll_suppress_frames == 0 {
                        let scroll_amount = -*delta_y as f32;
                        self.game_ui.scroll_at(self.mouse_x, self.mouse_y, scroll_amount);
                    }
                }
                _ => {}
            }
        }

        // Process UI input (keyboard/gamepad via InputSystem)
        let ui_events = self.game_ui.process_input(
            &jetstream_input::InputSystem::new(),
            self.mouse_x,
            self.mouse_y,
        );

        for event in &ui_events {
            if let UiEvent::Activated(node_id) = event {
                self.handle_activation(*node_id);
            }
        }

        // Decrement scroll suppression counter each frame.
        if self.scroll_suppress_frames > 0 {
            self.scroll_suppress_frames -= 1;
        }

        // Rebuild if state changed
        if self.app.dirty {
            self.rebuild_shell();
            // Suppress scroll events for a few frames so that trackpad
            // momentum doesn't immediately re-scroll the fresh list.
            self.scroll_suppress_frames = 3;
            // Ensure GPU textures exist for any new font sizes.
            self.sync_atlas_textures(frame.gpu);
        }

        // Tick transitions/animations
        let dt = frame.dt.as_secs_f32();
        self.game_ui.tree.tick_transitions(dt);

        // Ensure GPU textures are available (handles initial frame + any
        // dynamically-created atlases from scroll or rebuild).
        if self.text_atlas_textures.is_empty() {
            self.sync_atlas_textures(frame.gpu);
        }

        // ── Render ──
        if let Some(surface_view) = frame.surface_view {
            let scale = frame.scale_factor as f32;
            let draw_commands = collect_draw_commands(
                &self.game_ui.tree,
                &self.game_ui.focus,
                &self.draw_theme,
            );

            let mut encoder = frame.gpu.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some("pug_preview_encoder"),
                },
            );

            // 1) Clear pass — fill surface with canvas background color.
            {
                let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("clear_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: surface_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_color),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            }

            // 2) UI quad + text passes — grouped by clip_rect for scissor clipping.
            //
            // Group draw commands by their clip_rect so we can render each group
            // with the correct wgpu scissor rect. This ensures scroll container
            // contents are visually clipped to their parent bounds.
            let clip_groups = group_by_clip_rect(&draw_commands);
            let phys_w = frame.window_width as f32;
            let phys_h = frame.window_height as f32;

            self.ui_pass.upload_quad_geometry(&frame.gpu.queue);

            for group in &clip_groups {
                let quad_instances = convert_draw_commands_scaled(&group.commands, scale);
                let scissor = group.scissor_physical(scale, phys_w, phys_h);

                self.ui_pass.encode_clipped(
                    &frame.gpu.device,
                    &frame.gpu.queue,
                    &mut encoder,
                    surface_view,
                    &quad_instances,
                    phys_w,
                    phys_h,
                    scissor,
                );
            }

            frame.gpu.queue.submit(std::iter::once(encoder.finish()));

            // 3) Text sprite passes — one per clip group, each with scissor.
            for group in &clip_groups {
                let scissor = group.scissor_physical(scale, phys_w, phys_h);
                self.render_all_text_clipped(
                    frame.gpu,
                    surface_view,
                    &group.commands,
                    scale,
                    scissor,
                );
            }

            // 4) Screenshot capture (if requested).
            if self.screenshot_requested {
                self.screenshot_requested = false;
                self.capture_screenshot(frame.gpu, frame.window_width, frame.window_height);
            }
        }

        ControlFlow::Continue
    }

    /// Handle activation of a UI node (from click or keyboard).
    fn handle_activation(&mut self, node_id: UiNodeId) {
        if let Some(ref shell) = self.shell {
            if let Some(tab_idx) = shell::tab_index_for_node(shell, node_id) {
                if let Some(&section) = Section::ALL.get(tab_idx) {
                    self.app.set_section(section);
                }
            }
            if let Some(item_idx) = shell::sidebar_index_for_node(shell, node_id) {
                match self.app.section {
                    Section::Demo => {
                        if let Some(&screen) = DemoScreen::ALL.get(item_idx) {
                            self.app.set_demo_screen(screen);
                        }
                    }
                    _ => {
                        self.app.set_active_component(Some(item_idx));
                    }
                }
            }
            if let Some(idx) = shell::theme_index_for_node(shell, node_id) {
                if let Some(&preset) = ThemePreset::ALL.get(idx) {
                    self.app.set_theme(preset);
                }
            }
            if let Some(idx) = shell::density_index_for_node(shell, node_id) {
                if let Some(&density) = Density::ALL.get(idx) {
                    self.app.set_density(density);
                }
            }
            if let Some(idx) = shell::size_index_for_node(shell, node_id) {
                if let Some(&size) = ControlSize::ALL.get(idx) {
                    self.app.set_control_size(size);
                }
            }
            if let Some(probe_idx) = shell::probe_index_for_node(shell, node_id) {
                match probe_idx {
                    0 => self.app.toggle_disabled(),
                    1 => self.app.toggle_invalid(),
                    2 => self.app.toggle_busy(),
                    _ => {}
                }
            }
        }
    }

    /// Upload GPU textures for all physical-sized atlases that don't yet have
    /// a corresponding GPU texture, or re-upload any atlas whose pixel data
    /// has changed (dirty flag from dynamic glyph rasterization).
    fn sync_atlas_textures(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
    ) {
        let scale = self.game_ui.scale_factor;
        let all_logical_sizes: &[f32] = &[
            9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 16.0, 18.0, 20.0,
        ];
        if let Some(ref mut atlases) = self.game_ui.text_atlases {
            for &sz in all_logical_sizes {
                let phys = (sz * scale).round().max(1.0);
                let existing_idx = self.text_atlas_textures.iter().position(|(s, _)| (*s - phys).abs() < 0.5);
                if let Some(atlas) = atlases.get_mut(phys) {
                    if atlas.dirty || existing_idx.is_none() {
                        let gpu_tex = GpuTexture::from_rgba8(
                            &gpu.device,
                            &gpu.queue,
                            &atlas.pixels,
                            atlas.width,
                            atlas.height,
                            &format!("font_atlas_{phys}px"),
                            &self.texture_bgl,
                        );
                        if let Some(idx) = existing_idx {
                            // Replace existing texture.
                            self.text_atlas_textures[idx] = (phys, gpu_tex);
                        } else {
                            log::info!(
                                "Uploaded atlas {}px (logical {}pt) — {}×{}",
                                phys, sz, atlas.width, atlas.height,
                            );
                            self.text_atlas_textures.push((phys, gpu_tex));
                        }
                        atlas.mark_clean();
                    }
                }
            }
        }
    }

    /// Find the GPU texture for the given scaled pixel font size.
    fn find_atlas_texture(&self, scaled_size: f32) -> Option<&GpuTexture> {
        self.text_atlas_textures
            .iter()
            .find(|(s, _)| (*s - scaled_size).abs() < 0.5)
            .map(|(_, t)| t)
    }

    /// Ensure a GPU texture exists for the given scaled font size, creating it
    /// from the TextAtlasSet if needed. Also re-uploads if the atlas is dirty.
    fn ensure_atlas_texture(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
        scaled_size: f32,
    ) {
        let existing_idx = self.text_atlas_textures.iter().position(|(s, _)| (*s - scaled_size).abs() < 0.5);

        if let Some(ref mut atlases) = self.game_ui.text_atlases {
            if let Some(atlas) = atlases.get_mut(scaled_size) {
                let needs_upload = existing_idx.is_none() || atlas.dirty;
                if needs_upload {
                    let gpu_tex = GpuTexture::from_rgba8(
                        &gpu.device,
                        &gpu.queue,
                        &atlas.pixels,
                        atlas.width,
                        atlas.height,
                        &format!("font_atlas_{scaled_size}px"),
                        &self.texture_bgl,
                    );
                    if let Some(idx) = existing_idx {
                        self.text_atlas_textures[idx] = (scaled_size, gpu_tex);
                    } else {
                        log::info!(
                            "Uploaded new text atlas: {}px ({}×{})",
                            scaled_size, atlas.width, atlas.height,
                        );
                        self.text_atlas_textures.push((scaled_size, gpu_tex));
                    }
                    atlas.mark_clean();
                }
            }
        }
    }

    /// Grow the shared text instance buffer if needed, upload instance data.
    fn upload_text_instances(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
        instances: &[SpriteInstance],
    ) {
        if instances.len() > self.text_instance_capacity {
            self.text_instance_capacity = instances.len().next_power_of_two();
            self.text_instance_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("text_instance_buffer"),
                size: (self.text_instance_capacity * std::mem::size_of::<SpriteInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        gpu.queue.write_buffer(
            &self.text_instance_buffer,
            0,
            bytemuck::cast_slice(instances),
        );
    }

    /// Render text sprite instances for a single atlas batch.
    /// `atlas_idx` indexes into `self.text_atlas_textures`.
    fn render_text_batch(
        &self,
        gpu: &jetstream_platform::GpuContext,
        surface_view: &wgpu::TextureView,
        instance_count: u32,
        atlas_idx: usize,
        scissor: Option<[u32; 4]>,
    ) {
        let atlas_texture = &self.text_atlas_textures[atlas_idx].1;

        let mut encoder =
            gpu.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("text_encoder"),
                });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("text_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            if let Some([sx, sy, sw, sh]) = scissor {
                pass.set_scissor_rect(sx, sy, sw, sh);
            }

            pass.set_pipeline(&self.text_pipeline);
            pass.set_bind_group(0, &self.text_camera.bind_group, &[]);
            pass.set_bind_group(1, &atlas_texture.bind_group, &[]);
            pass.set_vertex_buffer(0, self.text_quad_vbo.slice(..));
            pass.set_vertex_buffer(1, self.text_instance_buffer.slice(..));
            pass.set_index_buffer(self.text_quad_ibo.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(
                0..SPRITE_QUAD_INDICES.len() as u32,
                0,
                0..instance_count,
            );
        }

        gpu.queue.submit(std::iter::once(encoder.finish()));
    }

    /// Render all text by grouping draw commands by font size and rendering
    /// each batch with its corresponding glyph atlas texture.
    fn render_all_text(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
        surface_view: &wgpu::TextureView,
        draw_commands: &[UiDrawCommand],
        scale_factor: f32,
    ) {
        self.render_all_text_clipped(gpu, surface_view, draw_commands, scale_factor, None);
    }

    /// Like [`render_all_text`](Self::render_all_text) but with an optional
    /// scissor rect for clip_rect-based clipping.
    fn render_all_text_clipped(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
        surface_view: &wgpu::TextureView,
        draw_commands: &[UiDrawCommand],
        scale_factor: f32,
        scissor: Option<[u32; 4]>,
    ) {
        // Collect unique scaled font sizes from commands that have text.
        let mut font_sizes: Vec<f32> = Vec::new();
        for cmd in draw_commands {
            if cmd.text.is_some() {
                let scaled = (cmd.text_size * scale_factor).round().max(1.0);
                if !font_sizes.iter().any(|s| (*s - scaled).abs() < 0.5) {
                    font_sizes.push(scaled);
                }
            }
        }

        if font_sizes.is_empty() {
            return;
        }

        // For each unique font size, filter commands, generate instances, render.
        for scaled_size in &font_sizes {
            // Filter to commands matching this font size.
            let filtered: Vec<UiDrawCommand> = draw_commands
                .iter()
                .filter(|cmd| {
                    if cmd.text.is_some() {
                        let s = (cmd.text_size * scale_factor).round().max(1.0);
                        (s - *scaled_size).abs() < 0.5
                    } else {
                        false
                    }
                })
                .cloned()
                .collect();

            if filtered.is_empty() {
                continue;
            }

            // Generate sprite instances for this font size batch.
            let instances = if let Some(ref mut atlases) = self.game_ui.text_atlases {
                convert_text_commands(&filtered, atlases, scale_factor)
            } else {
                continue;
            };

            if instances.is_empty() {
                continue;
            }

            // Ensure we have a GPU texture for this atlas.
            self.ensure_atlas_texture(gpu, *scaled_size);

            // Upload instances and render with the matching atlas texture.
            let instance_count = instances.len() as u32;
            self.upload_text_instances(gpu, &instances);

            if let Some(atlas_idx) = self.text_atlas_textures
                .iter()
                .position(|(s, _)| (*s - *scaled_size).abs() < 0.5)
            {
                self.render_text_batch(gpu, surface_view, instance_count, atlas_idx, scissor);
            }
        }
    }

    /// Capture a screenshot of the current frame and save as PNG.
    ///
    /// Re-renders at 1× scale into an offscreen texture, including both
    /// UI quads and text, then copies to a staging buffer for readback.
    fn capture_screenshot(
        &mut self,
        gpu: &jetstream_platform::GpuContext,
        _physical_width: u32,
        _physical_height: u32,
    ) {
        // Render at 1× scale — use logical dimensions for the readback texture
        // so that quad positions and text positions (both in logical pixels)
        // align correctly with the texture size.
        let logical_w = self.game_ui.screen_width;
        let logical_h = self.game_ui.screen_height;
        let width = logical_w.ceil() as u32;
        let height = logical_h.ceil() as u32;

        let bpp: u32 = 4; // BGRA8 or RGBA8
        let unpadded_bytes_per_row = width * bpp;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bytes_per_row = (unpadded_bytes_per_row + align - 1) / align * align;

        // Create an offscreen texture at logical dimensions (1× scale).
        let readback_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("screenshot_texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.surface_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let readback_view = readback_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Re-render the current frame into the readback texture at 1× scale.
        let draw_commands = collect_draw_commands(
            &self.game_ui.tree,
            &self.game_ui.focus,
            &self.draw_theme,
        );
        let quad_instances = convert_draw_commands_scaled(&draw_commands, 1.0);

        let mut encoder =
            gpu.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("screenshot_encoder"),
                });

        // Clear.
        {
            let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("screenshot_clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &readback_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        // Render quads at 1× into the logical-sized texture.
        self.ui_pass.upload_quad_geometry(&gpu.queue);
        self.ui_pass.encode(
            &gpu.device,
            &gpu.queue,
            &mut encoder,
            &readback_view,
            &quad_instances,
            logical_w,
            logical_h,
        );

        gpu.queue.submit(std::iter::once(encoder.finish()));

        // Render text at 1× into the readback texture.
        // Ortho camera matches the logical-sized texture (1× scale).
        let ss_ortho = Mat4::orthographic_rh(0.0, logical_w, logical_h, 0.0, -1.0, 1.0);
        self.text_camera.update_matrix(&gpu.queue, ss_ortho);
        self.render_all_text(gpu, &readback_view, &draw_commands, 1.0);

        // Restore the real ortho camera (physical pixel dimensions will be reset on
        // the next resize check, but let's be safe).
        // The next frame's resize handler will correct this if needed.

        // Copy texture → staging buffer.
        let mut encoder =
            gpu.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("screenshot_copy_encoder"),
                });

        let staging_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("screenshot_staging"),
            size: (padded_bytes_per_row * height) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &readback_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &staging_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        gpu.queue.submit(std::iter::once(encoder.finish()));

        // Map and read back.
        let slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        gpu.device.poll(wgpu::Maintain::Wait);

        match receiver.recv() {
            Ok(Ok(())) => {
                let data = slice.get_mapped_range();
                let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);

                let is_bgra = matches!(
                    self.surface_format,
                    wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb
                );

                for row in 0..height {
                    let start = (row * padded_bytes_per_row) as usize;
                    let end = start + (width * bpp) as usize;
                    let row_data = &data[start..end];

                    for pixel in row_data.chunks_exact(4) {
                        if is_bgra {
                            rgba_data.extend_from_slice(&[pixel[2], pixel[1], pixel[0], pixel[3]]);
                        } else {
                            rgba_data.extend_from_slice(pixel);
                        }
                    }
                }

                drop(data);
                staging_buffer.unmap();

                // Save into docs/screenshots/ when running from the pug repo,
                // otherwise fall back to the current working directory.
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let filename = format!("pug-preview-screenshot-{timestamp}.png");

                let dir = std::path::Path::new("docs/screenshots");
                let path = if dir.parent().map_or(false, |p| p.join("effigy.toml").exists()) {
                    // We're in the pug repo root (or a child thereof).
                    let _ = std::fs::create_dir_all(dir);
                    dir.join(&filename).to_string_lossy().into_owned()
                } else {
                    filename
                };
                match image::save_buffer(
                    &path,
                    &rgba_data,
                    width,
                    height,
                    image::ColorType::Rgba8,
                ) {
                    Ok(()) => log::info!("Screenshot saved: {path}"),
                    Err(e) => log::error!("Failed to save screenshot: {e}"),
                }
            }
            Ok(Err(e)) => log::error!("Screenshot buffer mapping failed: {e}"),
            Err(e) => log::error!("Screenshot channel error: {e}"),
        }
    }
}

/// Build a Jetstream `Theme` from Pug tokens for draw command rendering.
fn build_draw_theme(pug_theme: &dyn ThemeProvider) -> Theme {
    let resolve = |token: &str| -> Vec4 { theme_bridge::resolve_vec4(pug_theme, token) };

    Theme {
        text_color: resolve("semantic.color.text.primary"),
        text_size: 14.0,
        button_bg: resolve("semantic.color.background.surface"),
        button_hover_bg: resolve("semantic.color.accent.hover"),
        button_pressed_bg: resolve("semantic.color.accent.base"),
        panel_bg: resolve("semantic.color.background.panel"),
        focus_color: resolve("semantic.color.accent.focus"),
        slider_track: resolve("semantic.color.border.subtle"),
        slider_fill: resolve("semantic.color.accent.base"),
        progress_track: resolve("semantic.color.border.subtle"),
        progress_fill: resolve("semantic.color.accent.base"),
        input_bg: resolve("semantic.color.background.canvas"),
        input_border: resolve("semantic.color.border.default"),
    }
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("Starting Pug Jetstream Preview...");

    let config = PlatformConfig {
        window: WindowConfig {
            title: "Pug — Jetstream Preview".to_string(),
            width: 1280,
            height: 800,
            resizable: true,
            vsync: VSyncMode::Off,
        },
    };

    let platform = Platform::new(config);
    let mut preview_state: Option<PreviewState> = None;

    platform.run(move |frame| {
        let state = preview_state.get_or_insert_with(|| {
            log::info!("Initializing preview state...");
            PreviewState::new(
                frame.gpu,
                frame.window_width,
                frame.window_height,
                frame.scale_factor,
            )
        });

        state.update_and_render(frame)
    });
}
