//! Reference application demonstrating downstream consumption of poodle-gpui.
//!
//! g07.013: This example shows how an external application would:
//! 1. Create a GpuiAdapter with a theme provider
//! 2. Build component specs from shared contract crates
//! 3. Render specs through the adapter to get GPUI element handles
//! 4. Compose screens from rendered components
//!
//! This serves as the "downstream reference-app adoption proof" for g07.

use poodle_adapter::{AdapterManifest, RenderComponent, ThemeProvider};
use poodle_specs::{DataTableSpec, DetailShellSpec, FormShellSpec, PageHeaderSpec};
use poodle_gpui::{GpuiAdapter, GpuiThemeProvider};
use poodle_specs::{ButtonSpec, StackSpec, SurfaceSpec, TextInputSpec};
use poodle_style::StyleDescriptor;
use poodle_workstation::{AppHeaderSpec, WorkspaceShellSpec};

fn main() {
    // 1. Create the adapter with default theme
    let theme = GpuiThemeProvider::default();
    let adapter = GpuiAdapter::new(theme);

    println!("=== Poodle GPUI Reference App ===\n");
    println!("Adapter: {}", adapter.name());
    println!(
        "Supported components: {}",
        adapter.supported_components().len()
    );
    println!(
        "Unsupported components: {}\n",
        adapter.unsupported_components().len()
    );

    // 2. Resolve tokens through the theme provider
    let theme = adapter.theme();
    let accent = theme.resolve_color("color.accent.base");
    let space = theme.resolve_space("space.stack.md");
    println!("Theme resolution:");
    println!("  accent color: rgba({:.2}, {:.2}, {:.2}, {:.2})", accent.0, accent.1, accent.2, accent.3);
    println!("  stack-md space: {:.1}px\n", space);

    // 3. Build and render component specs
    let style = StyleDescriptor::new();

    println!("Rendering workspace shell...");
    let shell = adapter.render(&WorkspaceShellSpec::new(), &style, theme);
    println!("  → {} ({})", shell.element_id, shell.spec_type);

    let header = adapter.render(&AppHeaderSpec::new(), &style, theme);
    println!("  → {} ({})", header.element_id, header.spec_type);

    println!("\nRendering form screen...");
    let surface = adapter.render(&SurfaceSpec::new(), &style, theme);
    let stack = adapter.render(&StackSpec::new(), &style, theme);
    let page_header = adapter.render(&PageHeaderSpec::new("Create Project"), &style, theme);
    let form = adapter.render(&FormShellSpec::new("project-form"), &style, theme);
    let input = adapter.render(&TextInputSpec::new(), &style, theme);
    let button = adapter.render(&ButtonSpec::new(), &style, theme);

    for handle in [&surface, &stack, &page_header, &form, &input, &button] {
        println!("  → {} ({})", handle.element_id, handle.spec_type);
    }

    println!("\nRendering browse screen...");
    let table = adapter.render(&DataTableSpec::new(vec![], vec![]), &style, theme);
    let detail = adapter.render(&DetailShellSpec::new(), &style, theme);

    for handle in [&table, &detail] {
        println!("  → {} ({})", handle.element_id, handle.spec_type);
    }

    // 4. Demonstrate multi-app consumption via demo_app module
    println!("\n=== Demo App Screens ===\n");
    let screens = poodle_gpui::demo_app::render_all_screens(&adapter);
    for screen in &screens {
        println!(
            "  {} — {} components",
            screen.title,
            screen.component_count()
        );
    }

    let total: usize = screens.iter().map(|s| s.component_count()).sum();
    println!("\nTotal rendered components: {total}");
    println!("\n✓ Reference app completed successfully.");
}
