import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
import { cssVars } from "../artifacts/ts/index";
import { themes } from "../artifacts/ts/themes";

const AA = 4.5;
const MIN_TONAL_STEP = 0.04;
const contentLayers = ["canvas", "surface", "panel", "elevated"] as const;
const layers = [...contentLayers, "overlay"] as const;
const textRoles = {
  primary: "color.text.primary",
  secondary: "color.text.secondary",
  tertiary: "color.text.tertiary",
  inverse: "color.text.inverse",
} satisfies Record<string, keyof typeof cssVars>;
type TextRole = keyof typeof textRoles;
type TokenPath = keyof typeof cssVars;
type Layer = (typeof contentLayers)[number];
type RGB = readonly [number, number, number];

type Fill = {
  role: TokenPath;
  over?: Layer;
  toggleUnderlay?: true;
  check: readonly TextRole[];
};

const fills: readonly Fill[] = [
  ...(["color.fill.rest", "color.fill.hover", "color.fill.active", "color.select.neutral", "color.select.accent"] as const)
    .flatMap((role) => contentLayers.map((over) => ({ role, over, check: ["primary"] as const }))),
  { role: "color.select.accentSubtle", over: "elevated", check: ["primary"] },
  ...contentLayers.map((over) => ({
    role: "color.select.accentStrong",
    over,
    toggleUnderlay: true,
    check: ["primary"],
  }) satisfies Fill),
  { role: "color.select.accentSurface", check: ["primary"] },
  { role: "color.select.accentSurfaceHover", check: ["primary"] },
];

function linear(channel: number): number {
  const value = channel / 255;
  return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
}

function luminance(rgb: RGB): number {
  return 0.2126 * linear(rgb[0]) + 0.7152 * linear(rgb[1]) + 0.0722 * linear(rgb[2]);
}

function contrast(a: RGB, b: RGB): number {
  const [first, second] = [luminance(a), luminance(b)];
  return (Math.max(first, second) + 0.05) / (Math.min(first, second) + 0.05);
}

function lightness(rgb: RGB): number {
  const [r, g, b] = rgb.map(linear);
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  return 0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s;
}

const stylesheet = (name: string) =>
  readFile(new URL(`../artifacts/css/poodle-${name}.css`, import.meta.url), "utf8");
const browser = await chromium.launch();
const failures: string[] = [];

try {
  const page = await browser.newPage();
  await page.addStyleTag({ content: (await Promise.all(["tokens", "themes"].map(stylesheet))).join("\n") });
  console.log("WCAG AA at the shipped web contrast setting (0.5). ! marks a ratio below 4.5:1.");
  console.log("The overlay scrim is composited over elevated. Ink washes are sampled on all four content layers.");
  console.log("AccentStrong is sampled on each ToggleGroup underlay. AccentSubtle is sampled on the Select listbox.");
  console.log("Every new fill is checked with its foreground. The inverse foreground on the existing solid Tabs accent and the secondary description on the default Select highlight are also reported.");

  for (const theme of Object.keys(themes)) {
    const samples = await page.evaluate(({ theme, cssVars, layers, textRoles, fills }) => {
      document.documentElement.dataset.theme = theme;
      const probe = document.body.appendChild(document.createElement("div"));
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 1;
      const ctx = canvas.getContext("2d", { willReadFrequently: true });
      if (!ctx) throw new Error("Could not create the contrast sampling canvas");

      const sample = (paint: string, under?: RGB): RGB => {
        if (!CSS.supports("background-color", paint)) throw new Error(`Invalid colour ${paint}`);
        probe.style.backgroundColor = paint;
        ctx.clearRect(0, 0, 1, 1);
        if (under) {
          ctx.fillStyle = `rgb(${under.join(", ")})`;
          ctx.fillRect(0, 0, 1, 1);
        }
        ctx.fillStyle = getComputedStyle(probe).backgroundColor;
        ctx.fillRect(0, 0, 1, 1);
        const [r, g, b, alpha] = ctx.getImageData(0, 0, 1, 1).data;
        if (alpha !== 255) throw new Error(`A backdrop is required for ${theme} ${paint}`);
        return [r, g, b];
      };

      const layerColors = layers.map((name) => {
        const paint = `var(${cssVars[`color.background.${name}`]})`;
        const under = name === "overlay" ? sample(`var(${cssVars["color.background.elevated"]})`) : undefined;
        return { name, rgb: sample(paint, under) };
      });
      const backgrounds = fills.map(({ role, over, toggleUnderlay, check }) => {
        const under = toggleUnderlay && over
          ? sample(`color-mix(in srgb, var(${cssVars[`color.background.${over}`]}) 93%, var(${cssVars["color.text.primary"]}))`)
          : layerColors.find(({ name }) => name === over)?.rgb;
        const backdrop = toggleUnderlay ? `ToggleGroup on ${over}` : over;
        return { role, backdrop, check, rgb: sample(`var(${cssVars[role]})`, under) };
      });
      const text = Object.entries(textRoles).map(([name, role]) =>
        ({ name, rgb: sample(`var(${cssVars[role]})`) }));
      const solidTab = sample(`var(${cssVars["color.accent.base"]})`);
      probe.remove();
      return { layers: layerColors, fills: backgrounds, text, solidTab };
    }, { theme, cssVars, layers, textRoles, fills });

    console.log(`\n### ${theme}\n`);
    console.log(`| Background | vs previous | ΔL | ${samples.text.map(({ name }) => name).join(" | ")} | Checked |`);
    console.log("| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |");

    function row(name: string, rgb: RGB, previous?: RGB, check: readonly string[] = []) {
      const ratios = samples.text.map(({ name: text, rgb: foreground }) => {
        const ratio = contrast(foreground, rgb);
        if (ratio < AA && check.includes(text)) failures.push(`${theme} ${text} on ${name}`);
        return ratio.toFixed(2) + (ratio < AA ? " !" : "");
      });
      const ratio = previous ? contrast(previous, rgb).toFixed(2) : "—";
      const step = previous ? Math.abs(lightness(previous) - lightness(rgb)).toFixed(3) : "—";
      console.log(`| ${name} | ${ratio} | ${step} | ${ratios.join(" | ")} | ${check.join(", ") || "—"} |`);
    }

    for (const [index, { name, rgb }] of samples.layers.entries()) {
      row(name, rgb, samples.layers[index - 1]?.rgb);
    }
    for (const { role, backdrop, check, rgb } of samples.fills) {
      row(role.slice("color.".length) + (backdrop ? ` over ${backdrop}` : ""), rgb, undefined, check);
    }
    row("accent.base (existing solid Tabs)", samples.solidTab);

    console.log("");
    for (const [lower, upper] of [["canvas", "panel"], ["panel", "elevated"]]) {
      const below = samples.layers.find(({ name }) => name === lower)?.rgb;
      const above = samples.layers.find(({ name }) => name === upper)?.rgb;
      if (!below || !above) throw new Error(`Missing layer for ${lower} → ${upper}`);
      const step = Math.abs(lightness(above) - lightness(below));
      const shortfall = step < MIN_TONAL_STEP ? " (below 0.04 target)" : "";
      console.log(`Tonal ${lower} → ${upper} is ${contrast(below, above).toFixed(2)}:1, ΔL ${step.toFixed(3)}${shortfall}.`);
    }
  }
} finally {
  await browser.close();
}

if (failures.length > 0) {
  console.error(`\nNew fill checks failed:\n${failures.join("\n")}`);
  process.exitCode = 1;
}
