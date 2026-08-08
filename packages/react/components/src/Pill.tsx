import type { CSSProperties, ReactNode } from "react";

import "@inflatable-cookie/poodle-styles/pill.css";

import { usePillContext } from "./pill-context";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  PillAppearance,
  PillFont,
  PillSize,
  PillTone,
  PillTypography,
  SemanticControlSizeRole,
} from "./types";

export interface PillProps {
  tone?: PillTone;
  appearance?: PillAppearance;
  size?: PillSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  font?: PillFont;
  typography?: PillTypography;
  accent?: string | null;
  muted?: boolean;
  adaptiveWidth?: boolean;
  ariaLabel?: string | null;
  children?: ReactNode;
}

export function Pill({
  tone = "neutral",
  appearance = "solid",
  size = null,
  sizeRole = "chrome",
  density = null,
  font = "normal",
  typography = "label",
  accent = null,
  muted = false,
  adaptiveWidth = false,
  ariaLabel = null,
  children,
}: PillProps) {
  const uiPresentation = useUiPresentation();
  const pillContext = usePillContext();

  const resolvedSize = (pillContext?.size ??
    size ??
    resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole)) as PillSize;
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedTypography = pillContext?.typography ?? typography;

  const style: (CSSProperties & Record<string, string>) | undefined = accent
    ? { "--poodle-pill-accent": accent }
    : undefined;

  return (
    <span
      className="poodle-pill"
      data-tone={tone}
      data-appearance={appearance}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-font={font}
      data-typography={resolvedTypography}
      data-muted={muted}
      data-adaptive-width={adaptiveWidth || undefined}
      data-accent={accent ? "custom" : undefined}
      aria-label={ariaLabel ?? undefined}
      style={style}
    >
      {children}
    </span>
  );
}
