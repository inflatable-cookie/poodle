// Compare two production Svelte preview builds through the shipped specimens.
// effigy visual:typography -- BEFORE_DIST AFTER_DIST OUTPUT
import { strict as assert } from "node:assert";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium, webkit, type Browser, type Page } from "playwright";
import { PNG } from "pngjs";
import { pinPage } from "../../../../../test/visual/capture";

const themes = ["eclipse", "iceberg"] as const;
const sizes = ["xs", "sm", "md", "lg", "xl"] as const;
const densities = ["compact", "default", "comfortable"] as const;
const specimens = {
  "page-header": { panel: ".poodle-page-header", text: ".poodle-page-header__title", group: "Operational metadata" },
  "sidebar-nav": { panel: ".poodle-sidebar-nav", text: ".poodle-sidebar-nav__group-title", group: "Library counts" },
  dialog: { panel: ".poodle-dialog", text: ".poodle-dialog__title", group: "" },
  "data-table": { panel: ".poodle-data-table", text: ".poodle-data-table__cell > span", group: "With filters and pagination" },
  button: { panel: ".poodle-button", text: ".poodle-button", group: "" },
  "text-input": { panel: ".poodle-text-input", text: ".poodle-text-input__control", group: "Prefix and suffix" },
};
const slugs = Object.keys(specimens) as (keyof typeof specimens)[];

type Style = "default" | "quiet";
type Size = typeof sizes[number];
type Density = typeof densities[number];
type Theme = typeof themes[number];
type Slug = keyof typeof specimens;

async function openSpecimen(page: Page, dist: string, slug: Slug, theme: Theme, style: Style, size: Size, density: Density) {
  await pinPage(page);
  await page.route("https://poodle.test/**", (route) => {
    const path = new URL(route.request().url()).pathname.slice(1);
    return route.fulfill({ path: resolve(dist, path || "index.html") });
  });
  const params = new URLSearchParams({ theme, style, controlSize: size, density, capture: "specimen" });
  await page.goto(`https://poodle.test/?${params}#components/${slug}`);
  await page.locator(`[data-capture-ready="${slug}"]`).waitFor();
  await page.addStyleTag({ content: `
    *, *::before, *::after {
      animation: none !important;
      transition: none !important;
      caret-color: transparent !important;
    }
    .poodle-specimen-capture-frame { height: auto !important; min-height: 100vh; }
  ` });
  const { group, panel } = specimens[slug];
  if (slug === "dialog") {
    await page.getByRole("button", { name: "Delete item", exact: true }).click();
    await page.mouse.move(0, 0);
  }
  const container = group ? page.locator(".poodle-specimen-group").filter({ hasText: group }).first() : page;
  const target = container.locator(slug === "sidebar-nav" ? panel : `${panel}[data-size="${size}"]`).first();
  // SidebarNav only writes data-size for an explicit prop. Its size variants
  // are entirely CSS-driven; set the same attribute on the library specimen.
  if (slug === "sidebar-nav") await target.evaluate((nav, size) => nav.setAttribute("data-size", size), size);
  await target.waitFor();
  await page.evaluate(async () => {
    await document.fonts.ready;
    await new Promise((done) => requestAnimationFrame(() => requestAnimationFrame(done)));
  });
  return target;
}

const roleValues = {
  pageTitle: [30, 36, 600],
  dialogTitle: [22, 28, 600],
  sectionTitle: [19, 24, 600],
  cardTitle: [15, 20, 600],
  body: [15, 22, 400],
  secondary: [13, 18, 400],
  groupLabel: [13, 18, 600],
  overline: [12, 16, 600],
  tableHeader: [13, 18, 600],
  tablePrimary: [15, 20, 600],
  tableSecondary: [13, 18, 400],
  badge: [12, 16, 500],
  caption: [12, 16, 500],
  code: [13, 18, 400],
  keyCap: [12, 16, 500],
  metricValue: [24, 28, 600],
  tooltip: [13, 18, 400],
  "controlLabel-md": [15, 20, 500],
  "inputText-md": [15, 20, 400],
};

async function readRoles(page: Page) {
  return page.evaluate((names) => {
    const probe = document.createElement("div");
    document.body.append(probe);
    const results = names.map((name) => {
      const role = `--poodle-style-typography-${name}`;
      probe.style.fontSize = `var(${role}-size)`;
      probe.style.lineHeight = `var(${role}-lineHeight)`;
      probe.style.fontWeight = `var(${role}-weight)`;
      probe.style.letterSpacing = `var(${role}-letterSpacing, normal)`;
      probe.style.textTransform = `var(${role}-textTransform, none)`;
      const style = getComputedStyle(probe);
      return {
        role: name,
        size: parseFloat(style.fontSize),
        lineHeight: parseFloat(style.lineHeight),
        weight: parseFloat(style.fontWeight),
        tracking: style.letterSpacing,
        case: style.textTransform,
      };
    });
    probe.dataset.style = "default";
    for (const role of names) {
      if (getComputedStyle(probe).getPropertyValue(`--poodle-style-typography-${role}-size`)) {
        throw new Error(`Nested default kept the quiet ${role} size`);
      }
    }
    probe.remove();
    return results;
  }, Object.keys(roleValues));
}

function differentPixels(a: Buffer, b: Buffer) {
  const first = PNG.sync.read(a);
  const second = PNG.sync.read(b);
  if (first.width !== second.width || first.height !== second.height) return null;
  let changes = 0;
  for (let i = 0; i < first.data.length; i += 4) {
    if (first.data.readUInt32LE(i) !== second.data.readUInt32LE(i)) changes += 1;
  }
  return changes;
}

async function contactSheet(browser: Browser, slug: Slug, theme: Theme, pairs: [Buffer, Buffer][], path: string) {
  const page = await browser.newPage({ viewport: { width: 1800, height: 900 } });
  try {
    const image = (png: Buffer) => `<img src="data:image/png;base64,${png.toString("base64")}">`;
    await page.setContent(`
      <style>
        html { color-scheme: dark; background: #131518; color: #eef2f6; font: 15px system-ui; }
        body { margin: 0; padding: 16px; width: max-content; }
        article { display: grid; grid-template-columns: 2.5rem 1fr 1fr; gap: 12px; }
        b, span { padding: 4px 0; }
        img { display: block; max-width: 810px; height: auto; }
      </style>
      <article>
        <b></b><b>Default</b><b>Quiet</b>
        ${pairs.map(([before, after], index) => `<span>${sizes[index]}</span>${image(before)}${image(after)}`).join("")}
      </article>`,
    );
    await page.locator("body").screenshot({ path: resolve(path, `${theme}-${slug}.png`) });
  } finally {
    await page.close();
  }
}

const [beforeDist, afterDist, output] = process.argv.slice(2);
assert(beforeDist && afterDist && output, "Supply BEFORE_DIST AFTER_DIST OUTPUT");
await mkdir(output, { recursive: true });
const browser = await chromium.launch();
const differences: { slug: Slug; theme: Theme; size: Size; pixels: number | null }[] = [];
const controlGeometry: object[] = [];
const familyOverrides: object[] = [];
const quietSpecimens: object[] = [];

async function render(dist: string, slug: Slug, theme: Theme, style: Style, size: Size) {
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  try {
    const target = await openSpecimen(page, dist, slug, theme, style, size, "default");
    const visible = slug === "dialog" ? target.locator(".poodle-dialog__surface") : target;
    const png = await visible.screenshot();
    if (dist === afterDist && size === "md" && theme === "eclipse") {
      const selector = specimens[slug].text;
      const text = slug === "button" ? target : target.locator(selector).first();
      const read = () => text.evaluate((element) => {
        const { fontSize, lineHeight, fontWeight, fontFamily, textTransform, color } = getComputedStyle(element);
        return { fontSize, lineHeight, fontWeight, fontFamily, textTransform, color };
      });
      const actual = await read();
      if (style === "quiet") quietSpecimens.push({ slug, ...actual });
      await page.evaluate(() => document.documentElement.style.setProperty("--poodle-font-sans", "Georgia, serif"));
      assert.match((await read()).fontFamily, /Georgia/);
      familyOverrides.push({ slug, style, before: actual.fontFamily, root: (await read()).fontFamily });
      await page.evaluate(() => document.documentElement.style.removeProperty("--poodle-font-sans"));
      await target.evaluate((node) => {
        const wrapper = document.createElement("div");
        wrapper.style.setProperty("--poodle-font-sans", "Verdana, sans-serif");
        node.before(wrapper);
        wrapper.append(node);
      });
      assert.match((await read()).fontFamily, /Verdana/);
    }
    return png;
  } finally {
    await page.close();
  }
}

const componentFamilyRoles = {
  "page-header": "heading",
  "sidebar-nav": "label",
  dialog: "heading",
  "data-table": "body",
  button: "label",
  "text-input": "body",
} as const;

const legacyFamilies = {
  body: "Cambria, serif",
  label: "Georgia, serif",
  caption: "Arial, sans-serif",
  heading: "Palatino, serif",
  code: "Courier New, monospace",
} as const;

type FamilyRole = keyof typeof legacyFamilies;
const familyRoles = Object.keys(legacyFamilies) as FamilyRole[];
const legacyToken = (role: FamilyRole) => `--poodle-typography-${role}-family`;

const previouslyInheritedParts = `
  <div class="poodle-page-header" data-has-title="true">
    <a class="poodle-page-header__back" data-font-role="label">Back</a>
    <p class="poodle-page-header__section" data-font-role="label">Section</p>
    <p class="poodle-page-header__eyebrow" data-font-role="caption">Eyebrow</p>
    <p class="poodle-page-header__subtitle" data-font-role="body">Subtitle</p>
    <p class="poodle-page-header__body" data-font-role="body">Body</p>
  </div>
  <div class="poodle-dialog__header">
    <p data-font-role="body">Dialog description</p>
  </div>
  <div class="poodle-data-table">
    <button class="poodle-data-table__toolbar-btn" data-font-role="label">Toolbar</button>
    <label class="poodle-data-table__col-menu-item" data-font-role="label">Column</label>
    <table>
      <thead><tr class="poodle-data-table__filters-row"><td data-font-role="body">Filter</td></tr></thead>
      <tbody><tr><td data-font-role="body">Primary cell</td></tr></tbody>
    </table>
    <div class="poodle-data-table__footer" data-font-role="caption">
      <span class="poodle-data-table__pagination-summary" data-font-role="caption">Summary</span>
      <label class="poodle-data-table__limit" data-font-role="body">Limit</label>
      <span class="poodle-data-table__pagination-page" data-font-role="label">Page</span>
    </div>
  </div>
  <span class="poodle-text-input__char-count" data-font-role="caption">Character count</span>
`;

const legacyDeclarations = familyRoles.map((role) => `${legacyToken(role)}: ${legacyFamilies[role]};`).join(" ");
const sharedDeclarations = "--poodle-font-sans: Verdana, sans-serif; --poodle-font-mono: Menlo, monospace;";

const cascadeCases = [
  { name: "style-default", selector: ":root", declarations: "", shared: false, legacy: false, boundary: false },
  { name: "shared-root", selector: ":root", declarations: sharedDeclarations, shared: true, legacy: false, boundary: false },
  { name: "shared-inner", selector: "#font-probe", declarations: sharedDeclarations, shared: true, legacy: false, boundary: false },
  { name: "legacy-inner", selector: "#font-probe", declarations: legacyDeclarations, shared: false, legacy: true, boundary: false },
  { name: "legacy-root", selector: ":root", declarations: legacyDeclarations, shared: false, legacy: true, boundary: false },
  { name: "shared-inner-style-boundary", selector: "#font-probe", declarations: sharedDeclarations, shared: true, legacy: false, boundary: true },
] as const;

function expectedFamily(role: FamilyRole, style: Style, useShared: boolean, useLegacy: boolean) {
  if (useShared) return role === "code" ? "Menlo" : "Verdana";
  if (useLegacy) return legacyFamilies[role].split(",")[0];
  if (style === "quiet") return role === "code" ? "ui-monospace" : "system-ui";
  return role === "code" ? "IBM Plex Mono" : "Inter";
}

async function checkFontCascade() {
  const results = [];
  for (const engine of [chromium, webkit]) {
    const browser = await engine.launch();
    try {
      for (const style of ["default", "quiet"] as const) {
        const cases = cascadeCases.map((config) => ({
          config,
          components: {} as Partial<Record<Slug, string>>,
          inheritedParts: {} as Record<string, string>,
          aliases: {} as Partial<Record<FamilyRole, string>>,
        }));
        for (const slug of slugs) {
          const page = await browser.newPage();
          try {
            const target = await openSpecimen(page, afterDist, slug, "eclipse", style, "md", "default");
            const includeParts = slug === "data-table";
            await target.evaluate((target, { slug, selector, markup, aliases }) => {
              const scope = document.createElement("div");
              scope.id = "font-probe";
              scope.style.fontFamily = "Tahoma, sans-serif";
              const component = target.cloneNode(true) as HTMLElement;
              const text = slug === "button" ? component : component.querySelector(selector);
              if (!text) throw new Error(`Missing specimen text for ${slug}`);
              text.setAttribute("data-component-font", "");
              scope.append(component);
              scope.insertAdjacentHTML("beforeend", markup);
              for (const { role, token } of aliases) {
                const reader = document.createElement("span");
                reader.dataset.fontAlias = role;
                reader.style.fontFamily = `var(${token})`;
                scope.append(reader);
              }
              document.body.append(scope);
              // Put app CSS before the package stylesheets to test specificity.
              const rules = document.createElement("style");
              rules.id = "font-probe-rules";
              document.head.prepend(rules);
            }, {
              slug,
              selector: specimens[slug].text,
              markup: includeParts ? previouslyInheritedParts : "",
              aliases: includeParts ? familyRoles.map((role) => ({ role, token: legacyToken(role) })) : [],
            });
            assert.equal(await page.locator("html").getAttribute("data-style"), style);

            for (const { config, components, inheritedParts, aliases } of cases) {
              const { name, selector, declarations, shared, legacy, boundary } = config;
              await page.locator("#font-probe-rules").evaluate((rules, css) => { rules.textContent = css; }, `${selector} { ${declarations} }`);
              await page.locator("#font-probe").evaluate((scope, mode) => {
                if (mode) scope.setAttribute("data-style", mode);
                else scope.removeAttribute("data-style");
              }, boundary ? style : null);

              const inheritedDefault = style === "default" && !shared;
              const context = `${engine.name()} ${style} ${name}`;
              const face = (node: Element) => getComputedStyle(node).fontFamily;
              const font = await page.locator("[data-component-font]").evaluate(face);
              const expected = inheritedDefault && slug === "data-table"
                ? "Tahoma" : expectedFamily(componentFamilyRoles[slug], style, shared, legacy);
              assert(font.includes(expected), `${context} ${slug}: ${font}`);
              components[slug] = font;

              if (!includeParts) continue;
              for (const part of await page.locator("[data-font-role]").evaluateAll((nodes) =>
                nodes.map((node) => ({
                  name: node.textContent?.trim() ?? "",
                  role: node.getAttribute("data-font-role"),
                  font: getComputedStyle(node).fontFamily,
                })),
              )) {
                const expected = inheritedDefault ? "Tahoma" : expectedFamily(part.role as FamilyRole, style, shared, legacy);
                assert(part.font.includes(expected), `${context} ${part.name}: ${part.font}`);
                inheritedParts[part.name] = part.font;
              }
              for (const role of familyRoles) {
                const alias = await page.locator(`[data-font-alias="${role}"]`).evaluate(face);
                const scopedAlias = await page.locator("#font-probe").evaluate((scope, token) =>
                  getComputedStyle(scope).getPropertyValue(token),
                  legacyToken(role),
                );
                const expected = expectedFamily(role, style, shared, legacy);
                assert(alias.includes(expected), `${context} legacy ${role}: ${alias}`);
                assert(scopedAlias.includes(expected), `${context} scope legacy ${role}: ${scopedAlias}`);
                aliases[role] = alias;
              }
            }
          } finally {
            await page.close();
          }
        }
        for (const { config, components, inheritedParts, aliases } of cases) {
          results.push({ engine: engine.name(), style, case: config.name, components, inheritedParts, aliases });
        }
      }
    } finally {
      await browser.close();
    }
  }
  return results;
}

try {
  // Separate rows run concurrently, with a bounded browser page count.
  for (const theme of themes) {
    for (const slug of slugs) {
      const pairs: [Buffer, Buffer][] = [];
      for (const size of sizes) {
        const [before, after, quiet] = await Promise.all([
          render(beforeDist, slug, theme, "default", size),
          render(afterDist, slug, theme, "default", size),
          render(afterDist, slug, theme, "quiet", size),
        ]);
        const pixels = differentPixels(before, after);
        differences.push({ slug, theme, size, pixels });
        if (pixels !== 0) {
          for (const [phase, png] of [["before", before], ["after", after]] as const) {
            await writeFile(resolve(output, `${theme}-${slug}-${size}-${phase}.png`), png);
          }
        }
        pairs.push([after, quiet]);
      }
      await contactSheet(browser, slug, theme, pairs, output);
      console.log(theme, slug);
    }
  }

  const page = await browser.newPage();
  await openSpecimen(page, afterDist, "button", "eclipse", "quiet", "md", "default");
  const roles = await readRoles(page);
  for (const role of roles) {
    assert.deepEqual([role.size, role.lineHeight, role.weight], roleValues[role.role as keyof typeof roleValues], role.role);
  }
  await page.close();
  await writeFile(resolve(output, "computed-roles.md"), [
    "# Quiet typography at md", "",
    "| Role | Size (px) | Line height (px) | Weight | Tracking | CSS case |",
    "| --- | ---: | ---: | ---: | --- | --- |",
    ...roles.map(({ role, size, lineHeight, weight, tracking, case: textCase }) =>
      `| ${role} | ${size} | ${lineHeight} | ${weight} | ${tracking} | ${textCase} |`),
    "",
  ].join("\n"));

  for (const slug of ["button", "text-input"] as const) {
    for (const size of sizes) {
      for (const style of ["default", "quiet"] as const) {
        const byDensity = await Promise.all(densities.map(async (density) => {
          const page = await browser.newPage();
          try {
            const target = await openSpecimen(page, afterDist, slug, "eclipse", style, size, density);
            const text = slug === "button" ? target : target.locator(".poodle-text-input__control");
            const height = await target.evaluate((node) => node.getBoundingClientRect().height);
            const type = await text.evaluate((node) => {
              const { fontSize, lineHeight, fontWeight } = getComputedStyle(node);
              return [parseFloat(fontSize), parseFloat(lineHeight), parseFloat(fontWeight)];
            });
            if (style === "quiet") {
              const step = sizes.indexOf(size);
              assert.deepEqual(type, [13 + step, 16 + step * 2, slug === "button" ? 500 : 400]);
            }
            return { density, height, type };
          } finally {
            await page.close();
          }
        }));
        assert.equal(new Set(byDensity.map(({ height }) => height)).size, 1, `${slug} ${size} ${style}`);
        controlGeometry.push({ slug, size, style, byDensity });
      }
    }
  }

  const fontCascade = await checkFontCascade();
  const receipt = { compared: differences.length, differences, quietSpecimens, controlGeometry, familyOverrides, fontCascade };
  await writeFile(resolve(output, "receipt.json"), JSON.stringify(receipt, null, 2) + "\n");
  assert(differences.every(({ pixels }) => pixels === 0), "Default specimen diff detected");
} finally {
  await browser.close();
}
