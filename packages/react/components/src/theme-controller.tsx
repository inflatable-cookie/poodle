// Modular theme controller for React. Serves the available Poodle themes, holds
// the current selection, applies it to the DOM (`data-theme`), and optionally
// persists it. Provided via context so a `ThemeSelect` (or anything else)
// auto-wires. Entirely optional — the component also works with plain props.

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { applyThemeAttributes, themeOptions } from "@inflatable-cookie/poodle-svelte-tokens";
import type { ThemeOption } from "./types";

export interface ThemeController {
  themes: ThemeOption[];
  current: string;
  setTheme: (value: string) => void;
}

export interface ThemeControllerConfig {
  themes?: ThemeOption[];
  initial?: string;
  target?: HTMLElement | (() => HTMLElement | null) | null;
  persistKey?: string | null;
}

const ThemeControllerContext = createContext<ThemeController | null>(null);

export interface ThemeControllerProviderProps extends ThemeControllerConfig {
  children: ReactNode;
}

export function ThemeControllerProvider({
  children,
  themes: themesProp,
  initial,
  target,
  persistKey = "poodle-theme",
}: ThemeControllerProviderProps) {
  const themes = useMemo(() => themesProp ?? (themeOptions() as ThemeOption[]), [themesProp]);
  const known = useCallback(
    (value: string | null | undefined): value is string =>
      !!value && themes.some((theme) => theme.value === value),
    [themes],
  );

  const [current, setCurrent] = useState<string>(() => {
    const stored =
      persistKey != null && typeof localStorage !== "undefined"
        ? localStorage.getItem(persistKey)
        : null;
    if (known(stored)) return stored;
    if (known(initial)) return initial;
    return themes[0]?.value ?? "";
  });

  useEffect(() => {
    if (!current) return;
    const element =
      typeof target === "function" ? target() : (target ?? (typeof document !== "undefined" ? document.documentElement : null));
    if (element) {
      applyThemeAttributes(element, {
        theme: current as Parameters<typeof applyThemeAttributes>[1]["theme"],
      });
    }
    if (persistKey != null && typeof localStorage !== "undefined") {
      localStorage.setItem(persistKey, current);
    }
  }, [current, target, persistKey]);

  const setTheme = useCallback(
    (value: string) => {
      if (known(value)) setCurrent(value);
    },
    [known],
  );

  const controller = useMemo<ThemeController>(
    () => ({ themes, current, setTheme }),
    [themes, current, setTheme],
  );

  return (
    <ThemeControllerContext.Provider value={controller}>{children}</ThemeControllerContext.Provider>
  );
}

/** Read the theme controller from context, or `null` if no provider is present. */
export function useThemeController(): ThemeController | null {
  return useContext(ThemeControllerContext);
}
