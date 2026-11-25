import { alpha, hex, over, type ThemeColor } from "./theme-color";
import type { ThemeRoles } from "./theme-resolver";
import type { cssVars } from "./tokens/generated/ts/index";

type ColorProperty = (typeof cssVars)[Extract<keyof typeof cssVars, `color.${string}`>];

// CSS color-mix(in srgb, …) interpolates premultiplied channels.
function mix(front: ThemeColor, back: ThemeColor, weight: number): ThemeColor {
  const frontAlpha = front[3] * weight;
  const backAlpha = back[3] * (1 - weight);
  const opacity = frontAlpha + backAlpha;
  if (opacity === 0) return [0, 0, 0, 0];
  return [
    (front[0] * frontAlpha + back[0] * backAlpha) / opacity,
    (front[1] * frontAlpha + back[1] * backAlpha) / opacity,
    (front[2] * frontAlpha + back[2] * backAlpha) / opacity,
    opacity,
  ];
}

/** Preview-only bridge from resolved colours to Poodle's current semantic CSS properties. */
export function previewThemeCssProperties(roles: ThemeRoles): Record<ColorProperty, string> {
  const selection = roles["select.accent"];
  const selectionOpacity = selection[3] / 255;
  const selectionLine = roles["accent.text"];
  const surface = roles["layer.surface"];
  const subtleBorder = roles["border.subtle"];
  const defaultBorder = mix(roles["border.strong"], subtleBorder, 0.25);
  const subtleSelection = alpha(selection, selectionOpacity * 14 / 18);

  return {
    "--poodle-color-background-canvas": hex(roles["layer.canvas"]),
    "--poodle-color-background-surface": hex(surface),
    "--poodle-color-background-panel": hex(roles["layer.raised"]),
    "--poodle-color-background-elevated": hex(roles["layer.overlay"]),
    "--poodle-color-background-overlay": hex(roles.scrim),
    "--poodle-color-surface-hover": hex(roles["fill.hover"]),
    "--poodle-color-text-primary": hex(roles["text.primary"]),
    "--poodle-color-text-secondary": hex(roles["text.secondary"]),
    "--poodle-color-text-tertiary": hex(roles["text.secondary"]),
    "--poodle-color-text-inverse": hex(roles["accent.on"]),
    "--poodle-color-border-subtle": hex(subtleBorder),
    "--poodle-color-border-default": hex(defaultBorder),
    "--poodle-color-border-strong": hex(roles["border.strong"]),
    "--poodle-color-accent-base": hex(roles["accent.fill"]),
    "--poodle-color-accent-hover": hex(roles["accent.fill-hover"]),
    "--poodle-color-accent-focusRing": hex(roles["accent.line"]),
    "--poodle-color-status-success": hex(roles["status.success.fg"]),
    "--poodle-color-status-warning": hex(roles["status.warning.fg"]),
    "--poodle-color-status-danger": hex(roles["status.danger.fg"]),
    "--poodle-color-status-info": hex(roles["status.info.fg"]),
    "--poodle-color-icon-primary": hex(roles["text.primary"]),
    "--poodle-color-icon-muted": hex(roles["text.secondary"]),
    "--poodle-color-syntax-comment": hex(roles["syntax.comment"]),
    "--poodle-color-syntax-keyword": hex(roles["syntax.keyword"]),
    "--poodle-color-syntax-string": hex(roles["syntax.string"]),
    "--poodle-color-syntax-literal": hex(roles["syntax.literal"]),
    "--poodle-color-syntax-type": hex(roles["syntax.type"]),
    "--poodle-color-syntax-callable": hex(roles["syntax.callable"]),
    "--poodle-color-syntax-property": hex(roles["syntax.property"]),
    "--poodle-color-syntax-operator": hex(roles["syntax.operator"]),
    "--poodle-color-syntax-punctuation": hex(roles["syntax.punctuation"]),
    "--poodle-color-syntax-invalid": hex(roles["syntax.invalid"]),
    "--poodle-color-fill-rest": hex(roles["fill.rest"]),
    "--poodle-color-fill-hover": hex(roles["fill.hover"]),
    "--poodle-color-fill-active": hex(roles["fill.active"]),
    "--poodle-color-select-accent": hex(selection),
    "--poodle-color-select-accentSubtle": hex(subtleSelection),
    "--poodle-color-select-accentStrong": hex(alpha(selection, selectionOpacity * 22 / 18)),
    "--poodle-color-select-accentSurface": hex(over(subtleSelection, surface)),
    "--poodle-color-select-accentSurfaceHover": hex(over(selection, surface)),
    "--poodle-color-select-accentBorder": hex(mix(selectionLine, subtleBorder, 0.32)),
    "--poodle-color-select-accentBorderStrong": hex(mix(selectionLine, defaultBorder, 0.42)),
    "--poodle-color-select-neutral": hex(roles["select.neutral"]),
  };
}
