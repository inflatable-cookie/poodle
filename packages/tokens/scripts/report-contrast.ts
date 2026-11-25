import { readFile } from "node:fs/promises";
import { chromium } from "playwright";
import { cssVars } from "../artifacts/ts/index";
import { themes } from "../artifacts/ts/themes";

const AA = 4.5;
const MIN_TONAL_STEP = 0.04;
const layers = ["canvas", "surface", "panel", "elevated", "overlay"] as const;
const textRoles = {
  primary: "color.text.primary",
  secondary: "color.text.secondary",
  tertiary: "color.text.tertiary",
  inverse: "color.text.inverse",
  accent: "color.selection.accent.text",
  neutral: "color.selection.neutral.text",
} satisfies Record<string, keyof typeof cssVars>;
type TextRole = keyof typeof textRoles;
type TokenPath = keyof typeof cssVars;
type RGB = readonly [number, number, number];

type Fill = {
  role: TokenPath;
  over?: "canvas" | "surface" | "panel" | "elevated" | "toggle";
  check: readonly TextRole[];
};

const fills: readonly Fill[] = [
  { role: "color.control.neutral.fill", check: ["primary", "secondary"] },
  { role: "color.control.neutral.hover", check: ["primary", "secondary", "neutral"] },
  { role: "color.control.neutral.active", check: ["primary", "secondary"] },
  { role: "color.surface.tonal", check: ["primary", "secondary"] },
  { role: "color.selection.accent.fill", over: "canvas", check: ["accent"] },
  { role: "color.selection.accent.fillSubtle", over: "elevated", check: ["accent"] },
  { role: "color.selection.accent.fillStrong", over: "toggle", check: ["accent"] },
  { role: "color.selection.accent.fillSurface", check: ["accent"] },
  { role: "color.selection.accent.fillSurfaceHover", check: ["accent"] },
  { role: "color.selection.accent.fillSolid", check: [] }, // Legacy inverse-on-accent pair.
  { role: "color.selection.neutral.fill", check: ["neutral", "secondary"] },
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
  console.log("Layer order is for comparison. The overlay scrim is composited over elevated.");
  console.log("Translucent selection fills are composited over their default specimen backdrops.");
  console.log("Every text role is reported. Each new fill lists the foregrounds checked for AA.");
  console.log("The default solid tab still uses inverse on the existing accent, and the default highlighted option still uses secondary descriptions.");

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
      const toggle = sample("color-mix(in srgb, var(--poodle-color-background-canvas) 93%, var(--poodle-color-text-primary))");
      const backgrounds = fills.map(({ role, over, check }) => {
        const under = over === "toggle" ? toggle : layerColors.find(({ name }) => name === over)?.rgb;
        return { role, over, check, rgb: sample(`var(${cssVars[role]})`, under) };
      });
      const text = Object.entries(textRoles).map(([name, role]) =>
        ({ name, rgb: sample(`var(${cssVars[role]})`) }));
      probe.remove();
      return { layers: layerColors, fills: backgrounds, text };
    }, { theme, cssVars, layers, textRoles, fills });

    console.log(`\n### ${theme}\n`);
    console.log(`| Background | vs previous | ${samples.text.map(({ name }) => name).join(" | ")} | Checked |`);
    console.log("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |");

    function row(name: string, rgb: RGB, difference: string, check: readonly string[] = []) {
      const ratios = samples.text.map(({ name: text, rgb: foreground }) => {
        const ratio = contrast(foreground, rgb);
        if (ratio < AA && check.includes(text)) failures.push(`${theme} ${text} on ${name}`);
        return ratio.toFixed(2) + (ratio < AA ? " !" : "");
      });
      console.log(`| ${name} | ${difference} | ${ratios.join(" | ")} | ${check.join(", ") || "—"} |`);
    }

    for (const [index, { name, rgb }] of samples.layers.entries()) {
      const previous = samples.layers[index - 1];
      row(name, rgb, previous ? contrast(previous.rgb, rgb).toFixed(2) : "—");
    }
    for (const { role, over, check, rgb } of samples.fills) {
      row(role.slice("color.".length) + (over ? ` over ${over}` : ""), rgb, "—", check);
    }
    const tonal = samples.fills.find(({ role }) => role === "color.surface.tonal");
    if (!tonal) throw new Error("Missing tonal surface colour");
    console.log("");
    for (const { name, rgb } of samples.layers) {
      if (name !== "canvas" && name !== "panel") continue;
      const step = Math.abs(lightness(tonal.rgb) - lightness(rgb));
      console.log(`Tonal surface against ${name}: ${contrast(tonal.rgb, rgb).toFixed(2)}:1, ΔL ${step.toFixed(3)}.`);
      if (step < MIN_TONAL_STEP) failures.push(`${theme} tonal lightness step against ${name}`);
    }
  }
} finally {
  await browser.close();
}

if (failures.length > 0) {
  console.error(`\nNew fill checks failed:\n${failures.join("\n")}`);
  process.exitCode = 1;
}
