/**
 * Fail when the native presentation cascade (architecture 010, g15.043)
 * drifts back to pre-cascade shapes.
 *
 * Three standing rules:
 *
 * A. Component specs preserve omission. No semantic `ControlSize` /
 *    `ControlDensity` component input may be reintroduced as a concrete
 *    field. `UiPresentationProviderSpec` is exempt: a provider's own two
 *    values are always concrete. Component-specific size domains
 *    (`AvatarSize`, `IconSize`, `SpinnerSize`, numeric sizes) use other
 *    types and are invisible to this check by construction.
 *
 * B. Public renderers take the context. No function in `poodle-render` may
 *    accept a bare `&dyn ThemeProvider`; token resolution reaches internal
 *    helpers through `ctx.theme()`. `context.rs` is exempt: it owns the one
 *    borrowed theme.
 *
 * C. The GPUI preview never reintroduces a passthrough or manual-equivalent
 *    UiPresentationProvider: no preview-local provider facade, and the
 *    specimen must route scoped content through
 *    `poodle_render::context::ui_presentation_provider` without copying the
 *    provider's own scope variables into child specs (an explicit literal
 *    reset like `.with_size(ControlSize::Md)` remains legal — that is the
 *    explicit-reset case, not a manual copy).
 */

const ROOT = new URL("..", import.meta.url).pathname;
const SELF = "scripts/check-presentation-cascade.ts";

const SPEC_DIR = "packages/contracts/components/src";
const RENDER_DIR = "packages/render/src";
const PROVIDER_SPEC = "ui_presentation_provider.rs";
const CONTEXT_MODULE = "context.rs";
const PREVIEW_PROVIDERS = "packages/gpui/preview/src/providers.rs";
const PREVIEW_SPECIMEN =
  "packages/gpui/preview/src/specimens/ui_presentation_provider.rs";

// A. Concrete semantic presentation field on a component spec.
const CONCRETE_FIELD =
  /^\s*pub (size|density): (crate::types::)?Control(Size|Density),/;

// B. Bare theme parameter (or any bare trait-object mention) in a renderer.
const BARE_THEME = /&dyn (poodle_adapter::)?ThemeProvider/;

// C2. Manual-equivalent copy: the provider's scope variables handed straight
// into a child spec builder. Configuring the provider's OWN spec from an axis
// variable (UiPresentationProviderSpec::new().with_density(density)) is
// legitimate — that is how a scope is declared, not a copy into a child.
const MANUAL_COPY = /\.with_(size|density)\((size|density)\)/;

const failures: string[] = [];
let checked = 0;

async function scan(
  dir: string,
  exemptFile: string,
  pattern: RegExp,
  label: string,
): Promise<void> {
  const glob = new Bun.Glob("**/*.rs");
  for await (const path of glob.scan({ cwd: `${ROOT}/${dir}`, onlyFiles: true })) {
    if (path.split("/").pop() === exemptFile) continue;
    const full = `${dir}/${path}`;
    if (full === SELF) continue;
    checked += 1;
    const lines = (await Bun.file(`${ROOT}/${full}`).text()).split("\n");
    for (const [index, line] of lines.entries()) {
      if (pattern.test(line)) {
        failures.push(`${label}: ${full}:${index + 1}: ${line.trim()}`);
      }
    }
  }
}

await scan(SPEC_DIR, PROVIDER_SPEC, CONCRETE_FIELD, "concrete semantic presentation field");
await scan(RENDER_DIR, CONTEXT_MODULE, BARE_THEME, "renderer bypasses RenderContext");

// C1. No preview-local UiPresentationProvider facade. Doc comments (`//!`,
// `///`) may mention the provider by name; code may not.
{
  checked += 1;
  const lines = (await Bun.file(`${ROOT}/${PREVIEW_PROVIDERS}`).text()).split("\n");
  for (const [index, line] of lines.entries()) {
    if (/UiPresentationProvider/.test(line) && !/^\s*\/\//.test(line)) {
      failures.push(
        `preview provider facade: ${PREVIEW_PROVIDERS}:${index + 1}: ${line.trim()}`,
      );
    }
  }
}

// C2. The specimen demonstrates the real cascade.
{
  checked += 1;
  const source = await Bun.file(`${ROOT}/${PREVIEW_SPECIMEN}`).text();
  const lines = source.split("\n");
  for (const [index, line] of lines.entries()) {
    if (MANUAL_COPY.test(line)) {
      // A variable flowed into a UiPresentationProviderSpec construction is
      // scope declaration, not a manual copy into a child spec.
      const window = lines.slice(Math.max(0, index - 3), index + 1).join("\n");
      if (window.includes("UiPresentationProviderSpec")) continue;
      failures.push(
        `manual-equivalent provider copy: ${PREVIEW_SPECIMEN}:${index + 1}: ${line.trim()}`,
      );
    }
  }
  if (!source.includes("ui_presentation_provider(")) {
    failures.push(
      `preview provider specimen no longer routes through ui_presentation_provider: ${PREVIEW_SPECIMEN}`,
    );
  }
}

if (failures.length > 0) {
  console.error(
    `presentation cascade drift: ${failures.length} violation(s):\n${failures
      .map((failure) => `  ${failure}`)
      .join("\n")}`,
  );
  process.exit(1);
}

console.log(`presentation cascade drift: checked ${checked} files, 0 violations`);
