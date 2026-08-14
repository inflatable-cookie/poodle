//! Headless offscreen UI snapshot — renders a JsEl scene to a PNG without a window.
//!
//! Quads-only (backgrounds, borders, **shadows**, focus rings) — no glyph pass, so
//! text labels don't appear, but everything needed to eyeball elevation / rings / layout
//! / color does. Reuses the same `collect_draw_commands → convert_draw_commands_scaled →
//! UiPass::encode → readback` path the preview's `capture_screenshot` uses, on a headless
//! wgpu device (the renderer GPU-test pattern).
//!
//! Run: `cargo run --bin snap` → writes /tmp/poodle-snap-*.png

use glam::Mat4;
use jetstream_renderer::camera::CameraGpu;
use jetstream_renderer::pipeline::create_sprite_pipeline;
use jetstream_renderer::shader::create_sprite_shader;
use jetstream_renderer::sprite::{QUAD_INDICES, QUAD_VERTICES};
use jetstream_renderer::texture::GpuTexture;
use jetstream_renderer::ui_pass::UiPass;
use jetstream_ui::ui_element::{self, JsEl};
use jetstream_ui::*;
use wgpu::util::DeviceExt;

use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_preview::jsx::{elevation_dialog, elevation_overlay, resolve_color};

fn headless_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        // wgpu 30 added this; the default keeps the adapter's own limits.
        apply_limit_buckets: false,
    }))
    .expect("no GPU adapter");
    pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("poodle-snap"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        memory_hints: wgpu::MemoryHints::default(),
        trace: wgpu::Trace::Off,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
    }))
    .expect("no device")
}

/// Render a JsEl scene to a PNG at the given logical size.
fn snapshot(el: &JsEl, w: u32, h: u32, path: &str) {
    snapshot_opts(el, w, h, path, false, None, 0.0);
}

/// As [`snapshot`], but advances animations by `dt` seconds after the build —
/// for capturing animated components (spinner rotation, skeleton pulse) at a
/// chosen point in their timeline.
fn snapshot_at(el: &JsEl, w: u32, h: u32, path: &str, dt: f32) {
    snapshot_opts(el, w, h, path, false, None, dt);
}

/// As [`snapshot`], optionally focusing the first focusable node (focus rings) and/or
/// applying hover/active overrides at a pointer position `(x, y, pressed)`.
fn snapshot_opts(
    el: &JsEl,
    w: u32,
    h: u32,
    path: &str,
    focus_first: bool,
    hover: Option<(f32, f32, bool)>,
    dt: f32,
) {
    let (device, queue) = headless_device();
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;

    // Layout the scene (real GameUi materialize + Taffy layout). with_text enables the
    // glyph atlases so the text pass below can render labels.
    let mut ui = GameUi::with_text(w as f32, h as f32);
    ui.render_immediate(el);
    if dt > 0.0 {
        ui.advance_animations(dt);
    }
    if focus_first {
        ui.focus.navigate(&ui.tree, NavDirection::Next);
    }
    if let Some((hx, hy, pressed)) = hover {
        ui.set_pointer_state(hx, hy, pressed);
    }
    let cmds = collect_draw_commands(
        &ui.tree,
        &ui.focus,
        &poodle_jetstream_preview::theme_bridge::build_draw_theme(
            &JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE),
        ),
    );
    let quads = convert_draw_commands_scaled(&cmds, 1.0);

    // Offscreen target.
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("snap_tex"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

    let ui_pass = UiPass::new(&device, format);
    ui_pass.upload_quad_geometry(&queue);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("snap_enc"),
    });
    // Clear to a dark canvas so shadows are visible.
    {
        let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("snap_clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                depth_slice: None,
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.92,
                        g: 0.93,
                        b: 0.95,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
    ui_pass.encode(
        &device,
        &queue,
        &mut encoder,
        &view,
        &quads,
        w as f32,
        h as f32,
    );
    queue.submit(std::iter::once(encoder.finish()));

    // ── Text pass (glyphs) — mirrors the preview's render_all_text at 1× scale ──
    // Same path the live app uses: per font size, rasterize into the atlas via
    // convert_text_commands, upload the atlas as a texture, draw the sprite instances.
    if let Some(atlases) = ui.text_atlases.as_mut() {
        let ortho = Mat4::orthographic_rh(0.0, w as f32, h as f32, 0.0, -1.0, 1.0);
        let camera = CameraGpu::from_matrix(&device, ortho);
        let shader = create_sprite_shader(&device);
        let tex_bgl = GpuTexture::bind_group_layout(&device);
        let pipeline = create_sprite_pipeline(
            &device,
            &shader,
            format,
            &camera.bind_group_layout,
            &tex_bgl,
        );
        let vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("snap_text_vbo"),
            contents: bytemuck::cast_slice(QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("snap_text_ibo"),
            contents: bytemuck::cast_slice(QUAD_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Unique font sizes among text commands (1× scale).
        let mut sizes: Vec<f32> = Vec::new();
        for c in &cmds {
            if c.text.is_some() {
                let s = c.text_size.round().max(1.0);
                if !sizes.iter().any(|x| (x - s).abs() < 0.5) {
                    sizes.push(s);
                }
            }
        }

        for size in sizes {
            let filtered: Vec<_> = cmds
                .iter()
                .filter(|c| c.text.is_some() && (c.text_size.round().max(1.0) - size).abs() < 0.5)
                .cloned()
                .collect();
            if filtered.is_empty() {
                continue;
            }
            let instances = convert_text_commands(&filtered, atlases, 1.0);
            if instances.is_empty() {
                continue;
            }
            let Some(atlas) = atlases.get_mut(size) else {
                continue;
            };
            let atlas_tex = GpuTexture::from_rgba8(
                &device,
                &queue,
                &atlas.pixels,
                atlas.width,
                atlas.height,
                "snap_atlas",
                &tex_bgl,
            );
            atlas.mark_clean();
            let ibuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("snap_text_instances"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let mut enc = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("snap_text_enc"),
            });
            {
                let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("snap_text_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        depth_slice: None,
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
                pass.set_pipeline(&pipeline);
                pass.set_bind_group(0, &camera.bind_group, &[]);
                pass.set_bind_group(1, &atlas_tex.bind_group, &[]);
                pass.set_vertex_buffer(0, vbo.slice(..));
                pass.set_vertex_buffer(1, ibuf.slice(..));
                pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint16);
                pass.draw_indexed(0..QUAD_INDICES.len() as u32, 0, 0..instances.len() as u32);
            }
            queue.submit(std::iter::once(enc.finish()));
        }

        // ── Icon pass — SVG icons rasterized + drawn as tinted quads ──
        let icon_cmds = collect_icon_commands(
            &ui.tree,
            &poodle_jetstream_preview::theme_bridge::build_draw_theme(
                &JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE),
            ),
            1.0,
        );
        if !icon_cmds.is_empty() {
            let mut icon_cache = IconCache::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../render/assets/icons"
            ));
            let mut groups: std::collections::HashMap<
                (String, u32),
                Vec<jetstream_renderer::sprite::SpriteInstance>,
            > = std::collections::HashMap::new();
            for c in &icon_cmds {
                groups.entry((c.name.clone(), c.size_px)).or_default().push(
                    jetstream_renderer::sprite::SpriteInstance {
                        position_scale: [
                            c.rect.x + c.rect.width * 0.5,
                            c.rect.y + c.rect.height * 0.5,
                            c.rect.width,
                            c.rect.height,
                        ],
                        rotation_layer: [c.rotation, 0.0, 0.0, 0.0],
                        color: c.tint,
                        uv_rect: [0.0, 0.0, 1.0, 1.0],
                        clip_rect: [0.0; 4],
                    },
                );
            }
            for ((name, size_px), instances) in groups {
                let Some((pixels, iw, ih)) = icon_cache
                    .rasterize(&name, size_px)
                    .map(|(p, w2, h2)| (p.to_vec(), w2, h2))
                else {
                    continue;
                };
                let itex =
                    GpuTexture::from_rgba8(&device, &queue, &pixels, iw, ih, "snap_icon", &tex_bgl);
                let ibuf2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("snap_icon_instances"),
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let mut enc =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut pass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("snap_icon_pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            depth_slice: None,
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });
                    pass.set_pipeline(&pipeline);
                    pass.set_bind_group(0, &camera.bind_group, &[]);
                    pass.set_bind_group(1, &itex.bind_group, &[]);
                    pass.set_vertex_buffer(0, vbo.slice(..));
                    pass.set_vertex_buffer(1, ibuf2.slice(..));
                    pass.set_index_buffer(ibo.slice(..), wgpu::IndexFormat::Uint16);
                    pass.draw_indexed(0..QUAD_INDICES.len() as u32, 0, 0..instances.len() as u32);
                }
                queue.submit(std::iter::once(enc.finish()));
            }
        }
    }

    // Readback.
    let bpp = 4u32;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded = (w * bpp).div_ceil(align) * align;
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("snap_staging"),
        size: (padded * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    enc2.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &staging,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(enc2.finish()));

    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    rx.recv().unwrap().unwrap();

    // wgpu 30 made this fallible.
    let data = slice.get_mapped_range().expect("map readback buffer");
    let mut img = image::RgbaImage::new(w, h);
    for y in 0..h {
        let row = &data[(y * padded) as usize..(y * padded + w * bpp) as usize];
        for x in 0..w {
            let p = (x * bpp) as usize;
            img.put_pixel(
                x,
                y,
                image::Rgba([row[p], row[p + 1], row[p + 2], row[p + 3]]),
            );
        }
    }
    img.save(path).unwrap();
    println!("wrote {path}");
}

/// Render registered specimens to /tmp/poodle-specimens/{slug}.png for batch
/// visual triage against the Svelte reference.
///
/// `only` narrows the sweep to named slugs. Checking one component otherwise
/// renders all 138, which is the difference between a few seconds and the whole
/// sweep on every iteration.
fn snap_all_specimens(only: &[String]) {
    use poodle_jetstream_preview::{app_state::AppState, component_registry, specimens};
    std::fs::create_dir_all("/tmp/poodle-specimens").ok();
    let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
    let canvas = resolve_color(&theme, "color.background.canvas");
    let state = AppState::new();
    let mut done = 0;
    for entry in component_registry::all_components() {
        if !entry.has_specimen {
            continue;
        }
        if !only.is_empty() && !only.iter().any(|slug| slug == entry.slug) {
            continue;
        }
        let Some(specimen) = specimens::render_specimen(entry.slug, &theme, &state) else {
            eprintln!("no renderer despite has_specimen: {}", entry.slug);
            continue;
        };
        let scene = ui_element::div()
            .w(900.0)
            .h(640.0)
            .p(24.0)
            .flex_col()
            .bg(canvas)
            .child(specimen);
        let path = format!("/tmp/poodle-specimens/{}.png", entry.slug);
        snapshot(&scene, 900, 640, &path);
        done += 1;
        // Streamed so the runner's ~80s render phase is not a silent block:
        // a stalled specimen should be visible while it is stalling.
        eprintln!("snap: {done} {}", entry.slug);
    }
    // The two chrome pages below are not per-component, so a filtered sweep
    // skips them: rendering them for a one-slug check is most of the cost the
    // filter exists to avoid, and the runner would compare them anyway.
    if !only.is_empty() {
        eprintln!(
            "snap: rendered {done} of {} requested specimen(s)",
            only.len()
        );
        return;
    }

    // Landing catalogue page (default state = no active component).
    let landing = specimens::build_content(&state, &theme);
    let landing_scene = ui_element::div()
        .w(1200.0)
        .h(1400.0)
        .p(24.0)
        .flex_col()
        .bg(canvas)
        .child(landing);
    snapshot(
        &landing_scene,
        1200,
        1400,
        "/tmp/poodle-specimens/_landing.png",
    );

    // Specimen page with view tabs (checkbox has Sizes + Densities sections).
    {
        let mut st = AppState::new();
        let idx = component_registry::all_components()
            .iter()
            .position(|c| c.slug == "checkbox");
        st.set_active_component(idx);
        let page = specimens::build_content(&st, &theme);
        let scene = ui_element::div()
            .w(1100.0)
            .h(900.0)
            .p(24.0)
            .flex_col()
            .bg(canvas)
            .child(page);
        snapshot(
            &scene,
            1100,
            900,
            "/tmp/poodle-specimens/_specimen-page.png",
        );
    }

    // Full app shell (header chrome + sidebar + landing).
    let shell_el = poodle_jetstream_preview::shell::build_shell(&state, &theme);
    snapshot(&shell_el, 1500, 900, "/tmp/poodle-specimens/_shell.png");

    eprintln!("rendered {done} specimens to /tmp/poodle-specimens/");
}

fn main() {
    if std::env::args().any(|a| a == "specimens") {
        // `--slug=a,b` narrows the sweep; absent, everything renders.
        let only: Vec<String> = std::env::args()
            .find_map(|a| a.strip_prefix("--slug=").map(str::to_string))
            .map(|list| {
                list.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        snap_all_specimens(&only);
        return;
    }
    let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::ECLIPSE);
    let panel = resolve_color(&theme, "color.background.panel");
    let border = resolve_color(&theme, "color.border.default");

    // A rounded card on the canvas; same card with: no shadow / token elevation overlay /
    // token elevation dialog / old shadow_md preset — to compare elevation rendering.
    let card = |label_shadow: i32| -> JsEl {
        let base = ui_element::div()
            .w(170.0)
            .h(90.0)
            .bg(panel)
            .rounded(12.0)
            .border(1.0)
            .border_color(border);
        match label_shadow {
            1 => elevation_overlay(base),
            2 => elevation_dialog(base),
            3 => base.shadow_md(),
            _ => base,
        }
    };

    // Comparison row (shadows overlap — for relative hierarchy).
    let scene = ui_element::div()
        .w(820.0)
        .h(260.0)
        .flex_row()
        .items_center()
        .gap(30.0)
        .pl(30.0)
        .pr(30.0)
        .child(card(0)) // flat
        .child(card(1)) // elevation.overlay (token)
        .child(card(2)) // elevation.dialog (token)
        .child(card(3)); // shadow_md (old preset)
    snapshot(&scene, 820, 260, "/tmp/poodle-snap-elevation.png");

    // Single isolated overlay card (no neighbour overlap) — clean shadow inspection.
    let solo = ui_element::div()
        .w(420.0)
        .h(300.0)
        .flex_row()
        .items_center()
        .justify_center()
        .child(card(1));
    snapshot(&solo, 420, 300, "/tmp/poodle-snap-solo.png");

    // Focus-ring test: two input-like focusable boxes; focus the first.
    let input_color = resolve_color(&theme, "color.background.input");
    let field = || {
        ui_element::div()
            .w(220.0)
            .h(40.0)
            .bg(input_color)
            .rounded(8.0)
            .border(1.0)
            .border_color(border)
            .focusable()
    };
    let focus_scene = ui_element::div()
        .w(540.0)
        .h(120.0)
        .flex_row()
        .items_center()
        .gap(40.0)
        .pl(30.0)
        .child(field()) // focused → should show focus ring
        .child(field()); // unfocused → plain border
    snapshot_opts(
        &focus_scene,
        540,
        120,
        "/tmp/poodle-snap-focus.png",
        true,
        None,
        0.0,
    );

    // Gradient test: linear (white→transparent over red) + radial (cyan center → transparent).
    use glam::Vec4;
    let white = Vec4::new(1.0, 1.0, 1.0, 1.0);
    let white_t = Vec4::new(1.0, 1.0, 1.0, 0.0);
    let red = Vec4::new(0.90, 0.22, 0.22, 1.0);
    let cyan = Vec4::new(0.20, 0.80, 0.92, 1.0);
    let cyan_t = Vec4::new(0.20, 0.80, 0.92, 0.0);
    let dark = Vec4::new(0.10, 0.11, 0.13, 1.0);
    // Layered like the color-picker saturation square: hue bg on parent, white→transparent
    // gradient on a child overlay (no bg) that composites over it.
    let lin_overlay = ui_element::div()
        .absolute()
        .inset_0()
        .bg_gradient_linear(90.0, vec![(white.into(), 0.0), (white_t.into(), 1.0)]);
    let lin = ui_element::div()
        .w(240.0)
        .h(140.0)
        .rounded(10.0)
        .relative()
        .overflow_hidden()
        .bg(red)
        .child(lin_overlay); // expect: left=white → right=red
    let rad = ui_element::div()
        .w(240.0)
        .h(140.0)
        .rounded(10.0)
        .bg(dark)
        .bg_gradient_radial(
            [0.3, 0.3],
            0.75,
            vec![(cyan.into(), 0.0), (cyan_t.into(), 1.0)],
        ); // cyan blob top-left
    let grad_scene = ui_element::div()
        .w(580.0)
        .h(200.0)
        .flex_row()
        .items_center()
        .gap(40.0)
        .pl(30.0)
        .child(lin)
        .child(rad);
    snapshot(&grad_scene, 580, 200, "/tmp/poodle-snap-gradient.png");

    // Real component: color picker (open) — its saturation square (hue bg + white→
    // transparent + transparent→black overlays) and rainbow hue strip exercise gradients.
    let panel = resolve_color(&theme, "color.background.panel");
    let cp = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_color_picker(
        &poodle_specs::ColorPickerSpec::new()
            .with_value("#6366f1")
            .with_open(true),
        &theme,
    ));
    let cp_scene = ui_element::div()
        .w(560.0)
        .h(360.0)
        .p(24.0)
        .bg(panel)
        .flex_row()
        .child(cp);
    snapshot(&cp_scene, 560, 360, "/tmp/poodle-snap-colorpicker.png");

    // Progress (determinate + indeterminate) — gradient fill bars (no bg → were invisible
    // before the gate fix).
    let mk_prog = |spec| {
        poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_progress(
            spec, &theme,
        ))
    };
    let prog_scene = ui_element::div()
        .w(420.0)
        .h(120.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(20.0)
        .child(
            ui_element::div()
                .w(360.0)
                .child(mk_prog(&poodle_specs::ProgressSpec::new().with_value(0.6))),
        )
        .child(
            ui_element::div()
                .w(360.0)
                .child(mk_prog(&poodle_specs::ProgressSpec::new())),
        );
    snapshot(&prog_scene, 420, 120, "/tmp/poodle-snap-progress.png");

    // Text scene — sizes + weights + color, the canonical text-pass verification target.
    let ink = resolve_color(&theme, "color.text.primary");
    let muted = resolve_color(&theme, "color.text.tertiary");
    let row = |s: &str, size: f32, weight: u16, c: glam::Vec4| {
        ui_element::label(s)
            .text_size(size)
            .text_weight(weight)
            .text_color(c)
    };
    let text_scene = ui_element::div()
        .w(460.0)
        .h(200.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(12.0)
        .child(row("Heading 24 / 700", 24.0, 700, ink))
        .child(row("Body 16 / 400 — the quick brown fox", 16.0, 400, ink))
        .child(row("Medium 14 / 500", 14.0, 500, ink))
        .child(row("Caption 11 / 400 muted", 11.0, 400, muted));
    snapshot(&text_scene, 460, 200, "/tmp/poodle-snap-text.png");

    // Font-family: same string sans vs mono. In mono "iiii MMMM 0000" the columns line up.
    use jetstream_ui::FontFamily;
    let s = "iiii MMMM 0000 — code()";
    let fam_scene = ui_element::div()
        .w(460.0)
        .h(120.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(16.0)
        .child(ui_element::label(s).text_size(18.0).text_color(ink)) // sans (default)
        .child(
            ui_element::label(s)
                .text_size(18.0)
                .text_color(ink)
                .font_family(FontFamily::Mono),
        );
    snapshot(&fam_scene, 460, 120, "/tmp/poodle-snap-fontfamily.png");

    // Real component spot-check: code block should render its source in mono.
    let code = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_code(
        &poodle_specs::CodeSpec::new()
            .with_content("fn main() {\n    let xs = [1, 2, 3];\n}")
            .with_language("rust")
            .with_show_line_numbers(true),
        &theme,
    ));
    let code_scene = ui_element::div()
        .w(460.0)
        .h(160.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(10.0)
        .child(
            ui_element::label("Source (expect monospace):")
                .text_size(13.0)
                .text_color(muted),
        )
        .child(code);
    snapshot(&code_scene, 460, 160, "/tmp/poodle-snap-code.png");

    // Letter-spacing: same eyebrow label normal vs 0.12em tracked (contract eyebrow value).
    let ls_scene = ui_element::div()
        .w(460.0)
        .h(120.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(18.0)
        .child(
            ui_element::label("SECTION TITLE")
                .text_size(13.0)
                .text_weight(600)
                .text_color(muted),
        )
        .child(
            ui_element::label("SECTION TITLE")
                .text_size(13.0)
                .text_weight(600)
                .text_color(muted)
                .letter_spacing_em(0.12),
        );
    snapshot(&ls_scene, 460, 120, "/tmp/poodle-snap-letterspacing.png");

    // Real component spot-check: eyebrow (uppercased + 0.12em tracked from its spec).
    let eb = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_eyebrow(
        &poodle_specs::EyebrowSpec::new().with_content("overview"),
        &theme,
    ));
    let eb_scene = ui_element::div()
        .w(460.0)
        .h(90.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .child(eb);
    snapshot(&eb_scene, 460, 90, "/tmp/poodle-snap-eyebrow.png");

    // Hover/active: two rows with a .hover() bg override; pointer over the first.
    let accent = resolve_color(&theme, "color.accent.base");
    let surface = resolve_color(&theme, "color.background.surface");
    let row_hov = || {
        ui_element::div()
            .w(400.0)
            .h(48.0)
            .rounded(8.0)
            .bg(surface)
            .hover(move |s| s.bg(accent))
    };
    let hov_scene = ui_element::div()
        .w(460.0)
        .h(160.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(16.0)
        .child(row_hov()) // y≈24..72 — pointer here → hover bg
        .child(row_hov()); // y≈88..136 — untouched
    // Pointer at (230, 48): over the first row.
    snapshot_opts(
        &hov_scene,
        460,
        160,
        "/tmp/poodle-snap-hover.png",
        false,
        Some((230.0, 48.0, false)),
        0.0,
    );

    // Real component spot-check: menu (items call .hover(|s| s.bg(hover))); pointer over
    // the 2nd item should show the hover fill.
    use poodle_specs::MenuEntry;
    let menu = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_menu(
        &poodle_specs::MenuSpec::new(vec![
            MenuEntry::new("new", "New file"),
            MenuEntry::new("open", "Open…"),
            MenuEntry::new("save", "Save"),
        ]),
        &theme,
    ));
    let menu_scene = ui_element::div()
        .w(320.0)
        .h(180.0)
        .p(20.0)
        .bg(panel)
        .flex_col()
        .items_start()
        .child(menu);
    // Pointer over the 2nd item ("Open…") — items start ~y20, ~32px each → ~y 68.
    snapshot_opts(
        &menu_scene,
        320,
        180,
        "/tmp/poodle-snap-menuhover.png",
        false,
        Some((160.0, 68.0, false)),
        0.0,
    );

    // Border styles: solid / dashed / dotted (contract drop-zone / underline frames).
    use jetstream_ui::ui_element::BorderStyle;
    let bbox = |style| {
        ui_element::div()
            .w(130.0)
            .h(80.0)
            .rounded(10.0)
            .bg(surface)
            .border(2.0)
            .border_color(accent)
            .border_style(style)
    };
    let border_scene = ui_element::div()
        .w(520.0)
        .h(140.0)
        .p(24.0)
        .bg(panel)
        .flex_row()
        .items_center()
        .gap(24.0)
        .child(bbox(BorderStyle::Solid))
        .child(bbox(BorderStyle::Dashed))
        .child(bbox(BorderStyle::Dotted));
    snapshot(&border_scene, 520, 140, "/tmp/poodle-snap-border.png");

    // Real component spot-check: empty-state renders a dashed container border.
    let es = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_empty_state(
        &poodle_specs::EmptyStateSpec::new("No items yet"),
        &theme,
    ));
    let es_scene = ui_element::div()
        .w(420.0)
        .h(200.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .child(es);
    snapshot(&es_scene, 420, 200, "/tmp/poodle-snap-emptystate.png");

    // Multi-layer shadow: single token layer vs a 3-layer contract stack.
    use jetstream_ui::ui_element::BoxShadow;
    let blk = |a: f32| Vec4::new(0.0, 0.0, 0.0, a);
    let single = ui_element::div()
        .w(180.0)
        .h(110.0)
        .rounded(12.0)
        .bg(panel)
        .shadow(0.0, 8.0, 16.0, -2.0, blk(0.18));
    let stacked = ui_element::div()
        .w(180.0)
        .h(110.0)
        .rounded(12.0)
        .bg(panel)
        .shadow_layers(vec![
            BoxShadow {
                offset_x: 0.0,
                offset_y: 1.0,
                blur: 2.0,
                spread: 0.0,
                color: blk(0.20),
                inset: false,
            },
            BoxShadow {
                offset_x: 0.0,
                offset_y: 6.0,
                blur: 14.0,
                spread: -2.0,
                color: blk(0.16),
                inset: false,
            },
            BoxShadow {
                offset_x: 0.0,
                offset_y: 18.0,
                blur: 36.0,
                spread: -6.0,
                color: blk(0.18),
                inset: false,
            },
        ]);
    let ml_scene = ui_element::div()
        .w(520.0)
        .h(220.0)
        .flex_row()
        .items_center()
        .justify_center()
        .gap(60.0)
        .bg(Vec4::new(0.92, 0.93, 0.95, 1.0))
        .child(single)
        .child(stacked);
    snapshot(&ml_scene, 520, 220, "/tmp/poodle-snap-multilayer.png");

    // Inset shadows: hard inner ring (selection ring) + soft inner shadow.
    let ring = ui_element::div()
        .w(160.0)
        .h(90.0)
        .rounded(10.0)
        .bg(surface)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 0.0,
            spread: 2.5,
            color: accent,
            inset: true,
        }]);
    let inner = ui_element::div()
        .w(160.0)
        .h(90.0)
        .rounded(10.0)
        .bg(surface)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: 0.0,
            blur: 14.0,
            spread: 0.0,
            color: blk(0.55),
            inset: true,
        }]);
    // Offset highlight: inset 0 1px 0 white@45% (top inner highlight, à la button).
    let hilite = ui_element::div()
        .w(160.0)
        .h(90.0)
        .rounded(10.0)
        .bg(surface)
        .shadow_layers(vec![BoxShadow {
            offset_x: 0.0,
            offset_y: 3.0,
            blur: 0.0,
            spread: 0.0,
            color: Vec4::new(1.0, 1.0, 1.0, 0.6),
            inset: true,
        }]);
    let inset_scene = ui_element::div()
        .w(640.0)
        .h(150.0)
        .p(24.0)
        .bg(panel)
        .flex_row()
        .items_center()
        .gap(36.0)
        .child(ring)
        .child(inner)
        .child(hilite);
    snapshot(&inset_scene, 640, 150, "/tmp/poodle-snap-inset.png");

    // Real component spot-check: list-card highlighted → inset accent ring.
    let lc = |hl: bool| {
        poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_list_card(
            &poodle_specs::ListCardSpec::new()
                .with_title("Project Alpha")
                .with_highlighted(hl),
            &theme,
        ))
    };
    let lc_scene = ui_element::div()
        .w(460.0)
        .h(200.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .gap(16.0)
        .child(lc(true)) // highlighted → inset accent ring
        .child(lc(false)); // resting
    snapshot(&lc_scene, 460, 200, "/tmp/poodle-snap-listcard.png");

    // Grayscale filter: same accent box in color vs fully desaturated.
    let gbox = |g: f32| {
        ui_element::div()
            .w(150.0)
            .h(90.0)
            .rounded(10.0)
            .bg(accent)
            .grayscale(g)
    };
    let gray_scene = ui_element::div()
        .w(420.0)
        .h(140.0)
        .p(24.0)
        .bg(panel)
        .flex_row()
        .items_center()
        .gap(40.0)
        .child(gbox(0.0))
        .child(gbox(1.0));
    snapshot(&gray_scene, 420, 140, "/tmp/poodle-snap-grayscale.png");

    // Real component spot-check: tabs with a drop-target tab (inset accent ring) +
    // a drag-source tab (opacity 0.4).
    use poodle_specs::TabDefinition;
    let tabs = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_tabs(
        &poodle_specs::TabsSpec::new(vec![
            TabDefinition::new("over", "Overview"),
            TabDefinition::new("det", "Details"),
            TabDefinition::new("set", "Settings"),
        ])
        .with_drag_value(Some("over".into()))
        .with_drop_target_value(Some("set".into())),
        &theme,
    ));
    let tabs_scene = ui_element::div()
        .w(460.0)
        .h(110.0)
        .p(24.0)
        .bg(panel)
        .flex_col()
        .child(tabs);
    snapshot(&tabs_scene, 460, 110, "/tmp/poodle-snap-tabsdrop.png");

    // Button treatment shadows: primary (inset highlight + drop) / secondary (highlight) / ghost.
    use poodle_specs::ButtonVariant;
    let mkbtn = |v: ButtonVariant| {
        poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_button(
            &poodle_specs::ButtonSpec::new()
                .with_label("Save")
                .with_variant(v),
            &theme,
        ))
    };
    let btn_scene = ui_element::div()
        .w(460.0)
        .h(110.0)
        .p(24.0)
        .flex_row()
        .items_center()
        .gap(20.0)
        .bg(Vec4::new(0.90, 0.91, 0.93, 1.0))
        .child(mkbtn(ButtonVariant::Primary))
        .child(mkbtn(ButtonVariant::Secondary))
        .child(mkbtn(ButtonVariant::Ghost));
    snapshot(&btn_scene, 460, 110, "/tmp/poodle-snap-btnshadow.png");

    // Treatment spot-check: switch (track inset highlight + thumb outset drop), on light bg.
    let sw = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_switch(
        &poodle_specs::SwitchSpec::new().with_checked(true),
        &theme,
    ));
    let sw_scene = ui_element::div()
        .w(300.0)
        .h(100.0)
        .p(28.0)
        .flex_row()
        .items_center()
        .bg(Vec4::new(0.90, 0.91, 0.93, 1.0))
        .child(sw);
    snapshot(&sw_scene, 300, 100, "/tmp/poodle-snap-switch.png");

    // Card elevated 4-layer shadow stack, on a light bg.
    use poodle_specs::CardVariant;
    let card = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_card(
        &poodle_specs::CardSpec::new().with_variant(CardVariant::Elevated),
        &theme,
        vec![poodle_jetstream_preview::nel::div().w(180.0).h(70.0)],
    ));
    let card_scene = ui_element::div()
        .w(320.0)
        .h(180.0)
        .p(36.0)
        .flex_row()
        .items_center()
        .justify_center()
        .bg(Vec4::new(0.91, 0.92, 0.94, 1.0))
        .child(card);
    snapshot(&card_scene, 320, 180, "/tmp/poodle-snap-card.png");

    // Color-pipeline proof: four token swatches, hard-coded positions. Readback
    // must match the CSS token hex (post contrast-transform) — pins down whether
    // the sRGB→linear→sRGB round trip is correct end to end.
    let swatch = |tok: &str| {
        let c = resolve_color(&theme, tok);
        ui_element::div().w(40.0).h(40.0).bg(c)
    };
    let color_scene = ui_element::div()
        .w(200.0)
        .h(50.0)
        .flex_row()
        .gap(8.0)
        .p(5.0)
        .child(swatch("color.background.canvas"))
        .child(swatch("color.background.surface"))
        .child(swatch("color.background.panel"))
        .child(swatch("color.accent.base"));
    snapshot(&color_scene, 200, 50, "/tmp/poodle-snap-colorproof.png");

    // Spinner rotation proof: the ring's bright arc starts at the top (t=0)
    // and must sit on the right after a quarter of the 0.8s spin (t=0.2).
    // Same scene, two timeline points — pixel-different PNGs prove the engine
    // animation actually rotates the rendered quads.
    let spinner_scene = || {
        let sp = poodle_jetstream_preview::jsx::jel(poodle_jetstream_preview::compat::js_spinner(
            &poodle_specs::SpinnerSpec::new().with_size(poodle_specs::SpinnerSize::Xl),
            &theme,
        ));
        ui_element::div()
            .w(160.0)
            .h(160.0)
            .flex_row()
            .items_center()
            .justify_center()
            .bg(Vec4::new(0.10, 0.11, 0.13, 1.0))
            .child(sp)
    };
    snapshot_at(
        &spinner_scene(),
        160,
        160,
        "/tmp/poodle-snap-spin-t0.png",
        0.001,
    );
    snapshot_at(
        &spinner_scene(),
        160,
        160,
        "/tmp/poodle-snap-spin-t02.png",
        0.2,
    );
}
