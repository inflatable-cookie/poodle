//! Headless offscreen UI snapshot — renders a JsEl scene to a PNG without a window.
//!
//! Quads-only (backgrounds, borders, **shadows**, focus rings) — no glyph pass, so
//! text labels don't appear, but everything needed to eyeball elevation / rings / layout
//! / color does. Reuses the same `collect_draw_commands → convert_draw_commands_scaled →
//! UiPass::encode → readback` path the preview's `capture_screenshot` uses, on a headless
//! wgpu device (the renderer GPU-test pattern).
//!
//! Run: `cargo run --bin snap` → writes /tmp/poodle-snap-*.png

use jetstream_runtime::game_ui::*;
use jetstream_runtime::ui_element::{self, JsEl};
use jetstream_renderer::ui_pass::UiPass;

use poodle_jetstream::JetstreamThemeProvider;
use poodle_jetstream_components::theme_ext::{elevation_overlay, elevation_dialog, resolve_color};

fn headless_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
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
    snapshot_opts(el, w, h, path, false);
}

/// As [`snapshot`], optionally focusing the first focusable node (to show focus rings).
fn snapshot_opts(el: &JsEl, w: u32, h: u32, path: &str, focus_first: bool) {
    let (device, queue) = headless_device();
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;

    // Layout the scene (real GameUi materialize + Taffy layout).
    let mut ui = GameUi::new(w as f32, h as f32);
    ui.render_immediate(el);
    if focus_first {
        ui.focus.navigate(&ui.tree, NavDirection::Next);
    }
    let cmds = collect_draw_commands(&ui.tree, &ui.focus, &Theme::default());
    let quads = convert_draw_commands_scaled(&cmds, 1.0);

    // Offscreen target.
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("snap_tex"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
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
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.92, g: 0.93, b: 0.95, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
    ui_pass.encode(&device, &queue, &mut encoder, &view, &quads, w as f32, h as f32);
    queue.submit(std::iter::once(encoder.finish()));

    // Readback.
    let bpp = 4u32;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded = (w * bpp + align - 1) / align * align;
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
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    queue.submit(std::iter::once(enc2.finish()));

    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { let _ = tx.send(r); });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    rx.recv().unwrap().unwrap();

    let data = slice.get_mapped_range();
    let mut img = image::RgbaImage::new(w, h);
    for y in 0..h {
        let row = &data[(y * padded) as usize..(y * padded + w * bpp) as usize];
        for x in 0..w {
            let p = (x * bpp) as usize;
            img.put_pixel(x, y, image::Rgba([row[p], row[p + 1], row[p + 2], row[p + 3]]));
        }
    }
    img.save(path).unwrap();
    println!("wrote {path}");
}

fn main() {
    let theme = JetstreamThemeProvider::from_theme(&poodle_tokens::themes::DARK);
    let panel = resolve_color(&theme, "color.background.panel");
    let border = resolve_color(&theme, "color.border.default");

    // A rounded card on the canvas; same card with: no shadow / token elevation overlay /
    // token elevation dialog / old shadow_md preset — to compare elevation rendering.
    let card = |label_shadow: i32| -> JsEl {
        let base = ui_element::div()
            .w(170.0).h(90.0)
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
        .w(820.0).h(260.0)
        .flex_row()
        .items_center()
        .gap(30.0)
        .pl(30.0).pr(30.0)
        .child(card(0)) // flat
        .child(card(1)) // elevation.overlay (token)
        .child(card(2)) // elevation.dialog (token)
        .child(card(3)); // shadow_md (old preset)
    snapshot(&scene, 820, 260, "/tmp/poodle-snap-elevation.png");

    // Single isolated overlay card (no neighbour overlap) — clean shadow inspection.
    let solo = ui_element::div()
        .w(420.0).h(300.0)
        .flex_row()
        .items_center()
        .justify_center()
        .child(card(1));
    snapshot(&solo, 420, 300, "/tmp/poodle-snap-solo.png");

    // Focus-ring test: two input-like focusable boxes; focus the first.
    let input_color = resolve_color(&theme, "color.background.input");
    let field = || ui_element::div()
        .w(220.0).h(40.0)
        .bg(input_color)
        .rounded(8.0)
        .border(1.0)
        .border_color(border)
        .focusable();
    let focus_scene = ui_element::div()
        .w(540.0).h(120.0)
        .flex_row()
        .items_center()
        .gap(40.0)
        .pl(30.0)
        .child(field())  // focused → should show focus ring
        .child(field()); // unfocused → plain border
    snapshot_opts(&focus_scene, 540, 120, "/tmp/poodle-snap-focus.png", true);

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
        .absolute().inset_0()
        .bg_gradient_linear(90.0, vec![(white.into(), 0.0), (white_t.into(), 1.0)]);
    let lin = ui_element::div()
        .w(240.0).h(140.0).rounded(10.0)
        .relative().overflow_hidden()
        .bg(red)
        .child(lin_overlay); // expect: left=white → right=red
    let rad = ui_element::div()
        .w(240.0).h(140.0).rounded(10.0)
        .bg(dark)
        .bg_gradient_radial([0.3, 0.3], 0.75, vec![(cyan.into(), 0.0), (cyan_t.into(), 1.0)]); // cyan blob top-left
    let grad_scene = ui_element::div()
        .w(580.0).h(200.0)
        .flex_row().items_center().gap(40.0).pl(30.0)
        .child(lin)
        .child(rad);
    snapshot(&grad_scene, 580, 200, "/tmp/poodle-snap-gradient.png");

    // Real component: color picker (open) — its saturation square (hue bg + white→
    // transparent + transparent→black overlays) and rainbow hue strip exercise gradients.
    let panel = resolve_color(&theme, "color.background.panel");
    let cp = poodle_jetstream_components::color_picker::js_color_picker(
        &poodle_specs::ColorPickerSpec::new().with_value("#6366f1").with_open(true),
        &theme,
    );
    let cp_scene = ui_element::div()
        .w(560.0).h(360.0).p(24.0).bg(panel).flex_row()
        .child(cp);
    snapshot(&cp_scene, 560, 360, "/tmp/poodle-snap-colorpicker.png");

    // Progress (determinate + indeterminate) — gradient fill bars (no bg → were invisible
    // before the gate fix).
    let mk_prog = |spec| poodle_jetstream_components::progress::js_progress(spec, &theme);
    let prog_scene = ui_element::div()
        .w(420.0).h(120.0).p(24.0).bg(panel).flex_col().gap(20.0)
        .child(ui_element::div().w(360.0).child(mk_prog(&poodle_specs::ProgressSpec::new().with_value(0.6))))
        .child(ui_element::div().w(360.0).child(mk_prog(&poodle_specs::ProgressSpec::new())));
    snapshot(&prog_scene, 420, 120, "/tmp/poodle-snap-progress.png");
}
