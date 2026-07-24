// Modular theme controller for Svelte. Serves the available Poodle themes,
// holds the current selection, applies it to the DOM (`data-theme`), and
// optionally persists it. Provided via context so a `ThemeSelect` (or anything
// else) auto-wires to it. Entirely optional — the component also works with
// plain props.

import { getContext, setContext } from "svelte";
import { get, writable, type Writable } from "svelte/store";
import { applyThemeAttributes, themeOptions, type ThemeOption } from "@poodle/svelte-tokens";

const THEME_CONTROLLER_KEY = Symbol("poodle-theme-controller");

export interface ThemeControllerConfig {
  /** Theme catalogue. Defaults to every registered Poodle theme. */
  themes?: ThemeOption[];
  /** Initial theme value. Defaults to a persisted value, else the first theme. */
  initial?: string;
  /** Element to stamp `data-theme` on (or a getter). Defaults to
   * `document.documentElement`. */
  target?: HTMLElement | (() => HTMLElement | null) | null;
  /** localStorage key for persistence. `null` disables it; defaults to
   * `"poodle-theme"`. */
  persistKey?: string | null;
}

export interface ThemeController {
  themes: ThemeOption[];
  current: Writable<string>;
  setTheme: (value: string) => void;
}

/** Create a theme controller and publish it on Svelte context. Call in a root
 * component's script; descendants read it via `getThemeController()`. */
export function createThemeController(config: ThemeControllerConfig = {}): ThemeController {
  const themes = config.themes ?? themeOptions();
  const persistKey = config.persistKey === undefined ? "poodle-theme" : config.persistKey;
  const canPersist = persistKey != null && typeof localStorage !== "undefined";
  const stored = canPersist ? localStorage.getItem(persistKey as string) : null;
  const known = (value: string | null | undefined): value is string =>
    !!value && themes.some((theme) => theme.value === value);

  const initial = known(stored)
    ? stored
    : known(config.initial)
      ? config.initial
      : (themes[0]?.value ?? "");
  const current = writable(initial);

  const resolveTarget = (): HTMLElement | null => {
    if (typeof config.target === "function") return config.target();
    if (config.target) return config.target;
    return typeof document !== "undefined" ? document.documentElement : null;
  };

  current.subscribe((theme) => {
    if (!theme) return;
    const element = resolveTarget();
    if (element) applyThemeAttributes(element, { theme: theme as Parameters<typeof applyThemeAttributes>[1]["theme"] });
    if (canPersist) localStorage.setItem(persistKey as string, theme);
  });

  const controller: ThemeController = {
    themes,
    current,
    setTheme: (value) => {
      if (known(value)) current.set(value);
    },
  };

  setContext(THEME_CONTROLLER_KEY, controller);
  return controller;
}

/** Read the theme controller from context, or `null` if none was created. */
export function getThemeController(): ThemeController | null {
  return getContext<ThemeController | undefined>(THEME_CONTROLLER_KEY) ?? null;
}

/** Convenience: the controller's current theme value, read once (non-reactive). */
export function currentTheme(controller: ThemeController): string {
  return get(controller.current);
}
